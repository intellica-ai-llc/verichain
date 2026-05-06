//! Token definitions for the AGENT‑SEED v15.2 language.
//!
//! Every token carries its kind, original source text, and a byte‑span
//! for use with `miette` diagnostic reporting.

use miette::SourceSpan;
use std::fmt;

// ── Token ──

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: SourceSpan,
}

impl Token {
    pub fn new(kind: TokenKind, text: impl Into<String>, start: usize, end: usize) -> Self {
        Self { kind, text: text.into(), span: (start..end).into() }
    }
}

// ── Token kind ──

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenKind {
    // ── Keywords (alphabetical) ──
    KwAgent,
    KwAsync,
    KwAwait,
    KwBorrow,
    KwBreak,
    KwCatch,
    KwCompose,
    KwConfident,
    KwContinue,
    KwContract,
    KwDischarge,
    KwDream,
    KwEffect,
    KwElse,
    KwEnum,
    KwEvolve,
    KwExport,
    KwExtern,
    KwFn,
    KwFor,
    KwHandler,
    KwHeartbeat,
    KwIf,
    KwImpl,
    KwIn,            // <-- added here
    KwInfer,
    KwInherit,
    KwLet,
    KwLoop,
    KwMatch,
    KwMemo,
    KwMod,
    KwMove,
    KwMut,
    KwObserve,
    KwOntology,
    KwOptimize,
    KwOrchestrate,
    KwOwn,
    KwPerform,
    KwPipe,
    KwPipeline,
    KwProb,
    KwPub,
    KwReact,
    KwRedirect,
    KwRef,
    KwReturn,
    KwRoute,
    KwRule,
    KwSection,
    KwSeed,
    KwSeedlet,
    KwSelect,
    KwSelf,
    KwSignal,
    KwSpawn,
    KwStruct,
    KwThink,
    KwThrow,
    KwTrait,
    KwTrain,
    KwTry,
    KwType,
    KwUnsafe,
    KwUse,
    KwWhile,

    // ── Literals ──
    Ident,
    IntLiteral,
    FloatLiteral,
    StringLiteral,
    RawStringLiteral,
    CharLiteral,
    TrueLiteral,
    FalseLiteral,
    NullLiteral,

    // ── Operators & punctuation ──
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Not,
    And,
    AndAnd,
    Or,
    OrOr,
    Caret,
    Tilde,
    Shl,
    Shr,
    Arrow,
    FatArrow,
    Dot,
    DotDot,
    DotDotEq,
    Colon,
    ColonColon,
    Semicolon,
    Comma,
    At,
    Question,
    Hash,
    Dollar,
    Backslash,

    // ── Brackets ──
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // ── Pipeline / redirection operators ──
    Pipe,
    PipeGt,
    PipeGtGt,
    PipeAnd,
    RedirectOut,
    RedirectAppend,
    RedirectIn,
    RedirectErr,

    // ── Compound assignment ──
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    AndEq,
    OrEq,
    CaretEq,
    ShlEq,
    ShrEq,

    // ── Misc ──
    Eof,
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// ── Helper: check if a keyword string matches ──
pub fn keyword_from_str(word: &str) -> Option<TokenKind> {
    Some(match word {
        "agent"       => TokenKind::KwAgent,
        "async"       => TokenKind::KwAsync,
        "await"       => TokenKind::KwAwait,
        "borrow"      => TokenKind::KwBorrow,
        "break"       => TokenKind::KwBreak,
        "catch"       => TokenKind::KwCatch,
        "compose"     => TokenKind::KwCompose,
        "confident"   => TokenKind::KwConfident,
        "continue"    => TokenKind::KwContinue,
        "contract"    => TokenKind::KwContract,
        "discharge"   => TokenKind::KwDischarge,
        "dream"       => TokenKind::KwDream,
        "effect"      => TokenKind::KwEffect,
        "else"        => TokenKind::KwElse,
        "enum"        => TokenKind::KwEnum,
        "evolve"      => TokenKind::KwEvolve,
        "export"      => TokenKind::KwExport,
        "extern"      => TokenKind::KwExtern,
        "fn"          => TokenKind::KwFn,
        "for"         => TokenKind::KwFor,
        "handler"     => TokenKind::KwHandler,
        "heartbeat"   => TokenKind::KwHeartbeat,
        "if"          => TokenKind::KwIf,
        "impl"        => TokenKind::KwImpl,
        "in"          => TokenKind::KwIn,            // <-- added here
        "infer"       => TokenKind::KwInfer,
        "inherit"     => TokenKind::KwInherit,
        "let"         => TokenKind::KwLet,
        "loop"        => TokenKind::KwLoop,
        "match"       => TokenKind::KwMatch,
        "memo"        => TokenKind::KwMemo,
        "mod"         => TokenKind::KwMod,
        "move"        => TokenKind::KwMove,
        "mut"         => TokenKind::KwMut,
        "observe"     => TokenKind::KwObserve,
        "ontology"    => TokenKind::KwOntology,
        "optimize"    => TokenKind::KwOptimize,
        "orchestrate" => TokenKind::KwOrchestrate,
        "own"         => TokenKind::KwOwn,
        "perform"     => TokenKind::KwPerform,
        "pipe"        => TokenKind::KwPipe,
        "pipeline"    => TokenKind::KwPipeline,
        "prob"        => TokenKind::KwProb,
        "pub"         => TokenKind::KwPub,
        "react"       => TokenKind::KwReact,
        "redirect"    => TokenKind::KwRedirect,
        "ref"         => TokenKind::KwRef,
        "return"      => TokenKind::KwReturn,
        "route"       => TokenKind::KwRoute,
        "rule"        => TokenKind::KwRule,
        "section"     => TokenKind::KwSection,
        "seed"        => TokenKind::KwSeed,
        "seedlet"     => TokenKind::KwSeedlet,
        "select"      => TokenKind::KwSelect,
        "self"        => TokenKind::KwSelf,
        "signal"      => TokenKind::KwSignal,
        "spawn"       => TokenKind::KwSpawn,
        "struct"      => TokenKind::KwStruct,
        "think"       => TokenKind::KwThink,
        "throw"       => TokenKind::KwThrow,
        "trait"       => TokenKind::KwTrait,
        "train"       => TokenKind::KwTrain,
        "try"         => TokenKind::KwTry,
        "type"        => TokenKind::KwType,
        "unsafe"      => TokenKind::KwUnsafe,
        "use"         => TokenKind::KwUse,
        "while"       => TokenKind::KwWhile,
        "true"        => TokenKind::TrueLiteral,
        "false"       => TokenKind::FalseLiteral,
        "null"        => TokenKind::NullLiteral,
        _ => return None,
    })
}