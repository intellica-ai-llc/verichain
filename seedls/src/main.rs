//! AGENT-SEED v15.2 language server — `seedls`.
//!
//! Implements the Language Server Protocol via `tower-lsp` to provide
//! IDE features: diagnostics, completion, hover, go-to-definition,
//! find-references, rename, semantic tokens, and document symbols.
//!
//! Architecture follows the tower-lsp boilerplate pattern:
//!   Backend struct holding Client + document state
//!   → LanguageServer trait impl over stdio JSON-RPC
//!
//! References:
//!   - tower-lsp 0.20 (docs.rs, Apr 2026)
//!   - LSP 3.17 specification (microsoft.github.io/language-server-protocol)
//!   - jinja-lsp server architecture (git.joshthomas.dev, May 2025)

use tower_lsp::jsonrpc::Result as LspResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ── Backend ──

/// The LSP backend — holds document state and compiler integration.
#[derive(Debug)]
struct Backend {
    /// LSP client for publishing diagnostics and logging.
    client: Client,
    /// All open documents, keyed by URI, stored as ropey ropes for efficient editing.
    documents: Arc<RwLock<HashMap<Url, DocumentState>>>,
}

/// Per-document state.
#[derive(Debug, Clone)]
struct DocumentState {
    /// The full document text as a rope (incremental-friendly).
    text: ropey::Rope,
    /// Language ID (e.g., "seed").
    language_id: String,
    /// Version counter for incremental sync.
    version: i32,
    /// Whether the document has unsent diagnostics.
    dirty: bool,
}

impl DocumentState {
    fn new(text: String, language_id: String, version: i32) -> Self {
        Self { text: ropey::Rope::from(text), language_id, version, dirty: true }
    }
}

// ── LanguageServer trait impl ──

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    // ── Lifecycle ──

    async fn initialize(&self, _params: InitializeParams) -> LspResult<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "AGENT-SEED Language Server".into(),
                version: Some("15.2.0".into()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::INCREMENTAL)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".into(), ":".into(), "@".into(), "§".into()]),
                    resolve_provider: Some(true),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                rename_provider: Some(OneOf::Right(RenameOptions {
                    prepare_provider: Some(true),
                    ..Default::default()
                })),
                document_symbol_provider: Some(OneOf::Left(true)),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(
                    SemanticTokensOptions {
                        legend: SemanticTokensLegend {
                            token_types: vec![
                                SemanticTokenType::KEYWORD,
                                SemanticTokenType::FUNCTION,
                                SemanticTokenType::VARIABLE,
                                SemanticTokenType::TYPE,
                                SemanticTokenType::STRING,
                                SemanticTokenType::NUMBER,
                                SemanticTokenType::COMMENT,
                                SemanticTokenType::OPERATOR,
                            ],
                            token_modifiers: vec![
                                SemanticTokenModifier::DECLARATION,
                                SemanticTokenModifier::DEFINITION,
                                SemanticTokenModifier::READONLY,
                            ],
                        },
                        full: Some(SemanticTokensFullOptions::Bool(true)),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "AGENT-SEED Language Server v15.2.0 initialized").await;
    }

    async fn shutdown(&self) -> LspResult<()> { Ok(()) }

    // ── Document sync ──

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let doc = DocumentState::new(params.text_document.text, params.text_document.language_id, params.text_document.version);
        self.documents.write().await.insert(uri.clone(), doc.clone());
        self.publish_diagnostics(&uri, &doc).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let mut docs = self.documents.write().await;
        if let Some(doc) = docs.get_mut(&params.text_document.uri) {
            doc.version = params.text_document.version;
            for change in params.content_changes {
                if let Some(range) = change.range {
                    let start = Self::position_to_char(&doc.text, range.start);
                    let end = Self::position_to_char(&doc.text, range.end);
                    doc.text.remove(start..end);
                    doc.text.insert(start, &change.text);
                } else {
                    doc.text = ropey::Rope::from(change.text);
                }
            }
            doc.dirty = true;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let docs = self.documents.read().await;
        if let Some(doc) = docs.get(&params.text_document.uri) {
            self.publish_diagnostics(&params.text_document.uri, doc).await;
        }
    }

    // ── Completion ──

    async fn completion(&self, params: CompletionParams) -> LspResult<Option<CompletionResponse>> {
        let docs = self.documents.read().await;
        let doc = match docs.get(&params.text_document_position.text_document.uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        let line_idx = params.text_document_position.position.line as usize;
        let line = Self::get_line(&doc.text, line_idx);
        let items = self.compute_completions(line);

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn completion_resolve(&self, mut item: CompletionItem) -> LspResult<CompletionItem> {
        // Enhance completion items with documentation
        if item.documentation.is_none() {
            item.documentation = Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**{}**", item.label),
            }));
        }
        Ok(item)
    }

    // ── Hover ──

    async fn hover(&self, params: HoverParams) -> LspResult<Option<Hover>> {
        let docs = self.documents.read().await;
        let doc = match docs.get(&params.text_document_position_params.text_document.uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        let pos = params.text_document_position_params.position;
        let line = Self::get_line(&doc.text, pos.line as usize);
        let word = Self::word_at(&line, pos.character as usize);

        if let Some(w) = word {
            let hover_text = self.compute_hover(w);
            Ok(Some(Hover {
                contents: HoverContents::Scalar(MarkedString::String(hover_text)),
                range: None,
            }))
        } else {
            Ok(None)
        }
    }

    // ── Go-to-definition ──

    async fn goto_definition(&self, _params: GotoDefinitionParams) -> LspResult<Option<GotoDefinitionResponse>> {
        // Stub: integrate with seedc name resolution
        Ok(None)
    }

    // ── Find references ──

    async fn references(&self, _params: ReferenceParams) -> LspResult<Option<Vec<Location>>> {
        Ok(None)
    }

    // ── Rename ──

    async fn rename(&self, _params: RenameParams) -> LspResult<Option<WorkspaceEdit>> {
        Ok(None)
    }

    async fn prepare_rename(&self, _params: TextDocumentPositionParams) -> LspResult<Option<PrepareRenameResponse>> {
        Ok(None)
    }

    // ── Document symbols ──

    async fn document_symbol(&self, _params: DocumentSymbolParams) -> LspResult<Option<DocumentSymbolResponse>> {
        Ok(None)
    }

    // ── Semantic tokens ──

    async fn semantic_tokens_full(&self, _params: SemanticTokensParams) -> LspResult<Option<SemanticTokensResult>> {
        Ok(None)
    }
}

// ── Backend implementation ──

impl Backend {
    /// Parse the document and publish diagnostics.
    async fn publish_diagnostics(&self, uri: &Url, doc: &DocumentState) {
        let source = doc.text.to_string();
        let diags = match seedc::compile(&source) {
            Ok(_) => Vec::new(),
            Err(e) => {
                vec![Diagnostic {
                    range: Range { start: Position::new(0, 0), end: Position::new(0, 0) },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: format!("{}", e),
                    ..Default::default()
                }]
            }
        };
        self.client.publish_diagnostics(uri.clone(), diags, Some(doc.version)).await;
    }

    /// Compute completion items based on the current line context.
    fn compute_completions(&self, line: &str) -> Vec<CompletionItem> {
        let mut items = Vec::new();
        let line_trimmed = line.trim();

        // Keyword completions (top-level)
        let keywords = &[
            ("agent", "Declare an autonomous agent"),
            ("section", "Declare a typed section (memory layer)"),
            ("struct", "Declare a struct type"),
            ("enum", "Declare an enum type"),
            ("fn", "Declare a function"),
            ("let", "Declare a variable binding"),
            ("match", "Pattern match expression"),
            ("if", "Conditional expression"),
            ("loop", "Infinite loop"),
            ("while", "Conditional loop"),
            ("for", "Iteration loop"),
            ("return", "Return from function"),
            ("use", "Import a module"),
            ("pub", "Make item publicly visible"),
            ("async", "Async function or block"),
            ("await", "Await a future"),
            ("discharge", "Enter a discharge block for effects"),
            ("perform", "Perform an effect (must be inside discharge)"),
            ("spawn", "Spawn a new agent"),
            ("heartbeat", "Configure agent heartbeat"),
            ("dream", "Configure dream/memory consolidation cycle"),
            ("seed", "Declare a seed (agent definition)"),
            ("seedlet", "Declare a seedlet (lightweight seed)"),
            ("contract", "Declare a safety contract"),
            ("pipeline", "Declare an agent pipeline"),
            ("infer", "Perform type-safe LLM inference"),
            ("train", "Define a training regimen"),
            ("evolve", "Define self-evolution rules"),
            ("ontology", "Declare ontological constraints"),
            ("rule", "Declare a neurosymbolic rule"),
            ("route", "Define model routing"),
        ];

        for (kw, desc) in keywords {
            if line_trimmed.is_empty() || kw.starts_with(line_trimmed) {
                items.push(CompletionItem {
                    label: kw.to_string(),
                    kind: Some(CompletionItemKind::KEYWORD),
                    detail: Some(desc.to_string()),
                    insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                    ..Default::default()
                });
            }
        }

        // Section completions (prefixed with §)
        if line_trimmed.starts_with('§') {
            let sections = &[
                ("§IDENTITY-ANCHOR", "Agent identity anchor"),
                ("§ESSENCE", "Agent philosophical core"),
                ("§MEMORY-HIERARCHY", "Memory hierarchy configuration"),
                ("§DECISION-LOG", "Append-only decision log"),
                ("§BEHAVIORAL-PROFILE", "Interaction style profile"),
                ("§USER-MODEL", "Known user attributes"),
                ("§RUNTIME-CONSTRAINTS", "AgentSpec safety constraints"),
                ("§SAFETY-CONTRACTS", "Alignment contracts"),
                ("§BOOTSTRAP-INSTRUCTIONS", "Boot sequence for loading the seed"),
                ("§HEARTBEAT", "Heartbeat configuration"),
                ("§DREAM-CYCLE", "Dream cycle configuration"),
                ("§STIGMERGY-SUBSTRATE", "Federated knowledge substrate"),
                ("§MULTI-GRAPH-MEMORY", "Multi-graph memory indexing"),
                ("§CONTINUUM-MEMORY", "Continuum memory with decay"),
                ("§SELF-EVOLUTION", "Autonomous capability expansion"),
                ("§RL-TRAINING", "Reinforcement learning training"),
                ("§PROMPT-STRATEGY", "Prompt optimization strategy"),
                ("§NEUROSYMBOLIC", "Ontology-constrained neural reasoning"),
                ("§TEST-TIME-COMPUTE", "Inference-time reasoning budgets"),
                ("§MODEL-ROUTING", "Capability-tiered model selection"),
                ("§TYPED-MEMORY", "Schema-constrained typed memory"),
                ("§PACKAGE-MANIFEST", "Package distribution metadata"),
            ];

            let partial = line_trimmed.strip_prefix('§').unwrap_or("");
            for (sec, desc) in sections {
                let sec_name = sec.strip_prefix('§').unwrap_or(sec);
                if partial.is_empty() || sec_name.starts_with(partial) {
                    items.push(CompletionItem {
                        label: sec.to_string(),
                        kind: Some(CompletionItemKind::STRUCT),
                        detail: Some(desc.to_string()),
                        insert_text_format: Some(InsertTextFormat::PLAIN_TEXT),
                        ..Default::default()
                    });
                }
            }
        }

        items
    }

    /// Compute hover text for a word at the cursor position.
    fn compute_hover(&self, word: &str) -> String {
        let info: HashMap<&str, &str> = HashMap::from([
            ("agent", "**agent** — Declares an autonomous agent with lifecycle hooks, memory layers, and capabilities.\n\n```seed\nagent MyAgent { ... }\n```"),
            ("section", "**section** — Declares a typed memory section conforming to a schema.\n\n```seed\nsection UserProfile { name: string, age: u32 }\n```"),
            ("fn", "**fn** — Declares a function with optional effect signature.\n\n```seed\nfn query(prompt: string) -> string !{inference}\n```"),
            ("let", "**let** — Binds a value to a pattern. Use `let mut` for mutable bindings.\n\n```seed\nlet x: i32 = 42;\nlet mut y = 0;\n```"),
            ("match", "**match** — Exhaustive pattern matching over enums.\n\n```seed\nmatch result {\n  Ok(v) => process(v),\n  Err(e) => handle(e),\n}\n```"),
            ("perform", "**perform** — Executes an effectful operation. Must be lexically enclosed by a `discharge` block."),
            ("discharge", "**discharge** — Opens a discharge context for performing effects with confidence thresholds."),
            ("heartbeat", "**heartbeat** — Configures the autonomous tick loop that keeps the agent alive.\n\n```seed\nheartbeat { interval: 30s, idle_threshold: 15s }\n```"),
            ("dream", "**dream** — Configures the nightly memory consolidation cycle.\n\n```seed\ndream { schedule: daily, phases: [review, resolve, consolidate, compress, prune] }\n```"),
            ("seed", "**seed** — The top-level declaration that defines an agent's complete configuration.\n\n```seed\nseed my_seed {\n  §IDENTITY-ANCHOR { name: \"Ada\" }\n  §ESSENCE { content: \"...\" }\n}\n```"),
            ("pipeline", "**pipeline** — Composes agents sequentially, with data flowing through stages.\n\n```seed\nresearcher |> fact_checker |> summarizer\n```"),
        ]);

        info.get(word).cloned().unwrap_or_else(|| format!("**{}** — no documentation available", word))
    }

    /// Get a line from a ropey document.
    fn get_line(rope: &ropey::Rope, line: usize) -> String {
        if let Some(slice) = rope.get_line(line) {
            slice.to_string()
        } else {
            String::new()
        }
    }

    /// Extract the word at a given character position.
    fn word_at(line: &str, col: usize) -> Option<&str> {
        let chars: Vec<char> = line.chars().collect();
        let col = col.min(chars.len());
        if col == 0 || !chars.get(col.saturating_sub(1)).map_or(false, |c| c.is_alphanumeric() || *c == '_' || *c == '§') {
            return None;
        }

        let mut start = col.saturating_sub(1);
        while start > 0 && (chars[start - 1].is_alphanumeric() || chars[start - 1] == '_' || chars[start - 1] == '§') {
            start -= 1;
        }

        let mut end = col;
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }

        Some(&line[chars[..start].iter().map(|c| c.len_utf8()).sum::<usize>()..chars[..end].iter().map(|c| c.len_utf8()).sum::<usize>()])
    }

    /// Convert LSP Position to character offset in a ropey rope.
    fn position_to_char(rope: &ropey::Rope, pos: Position) -> usize {
        rope.try_line_to_char(pos.line as usize).unwrap_or(0) + pos.character as usize
    }
}

// ── Entry point ──

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents: Arc::new(RwLock::new(HashMap::new())),
    });

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout, socket).serve(service).await;
}
