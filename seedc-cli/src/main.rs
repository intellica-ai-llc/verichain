//! AGENT-SEED v15.2 unified CLI — `seed`.
//!
//! Single binary that compiles, checks, runs, and verifies .seed programs.
//!
//! EXAMPLES:
//!   seed build hello.seed
//!   seed build hello.seed -o hello.aslb
//!   seed check hello.seed
//!   seed run hello.seed
//!   seed emit-ir hello.seed

use clap::{Parser, Subcommand, ValueEnum};
use miette::{IntoDiagnostic, WrapErr};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

// ═══════════════════════════════════════════════════════════════════
// Top-level CLI
// ═══════════════════════════════════════════════════════════════════

/// AGENT-SEED v15.2 — the definitive language for autonomous agentic systems.
///
/// Compiles `.seed` source files into `.aslb` bytecode modules and runs them
/// on the deterministic seedvm.
///
/// EXAMPLES:
///   seed build hello.seed
///   seed run hello.seed
///   seed check hello.seed
///   seed emit-ir hello.seed
#[derive(Parser, Debug)]
#[command(
    name = "seed",
    version,
    about,
    long_about = None,
    after_help = "Report bugs at https://github.com/agentseedlanguage-cpu/agentseed",
    disable_help_subcommand = true,
)]
struct Cli {
    /// Global verbosity: -v (warnings), -vv (info), -vvv (debug), -vvvv (trace)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long, conflicts_with = "verbose")]
    quiet: bool,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

// ═══════════════════════════════════════════════════════════════════
// Subcommands
// ═══════════════════════════════════════════════════════════════════

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compile a .seed source file to an .aslb bytecode module
    Build(BuildArgs),
    /// Type-check and verify a .seed source file without emitting bytecode
    Check(CheckArgs),
    /// Compile and immediately execute a .seed program on the VM
    Run(RunArgs),
    /// Emit the intermediate representation (IR) as human-readable text
    EmitIr(EmitIrArgs),
    /// Export the GBNF grammar used for constrained LLM decoding
    EmitGrammar(EmitGrammarArgs),
    /// Generate an execution proof from a .seed program (static analysis only)
    Prove(ProveArgs),
}

// ═══════════════════════════════════════════════════════════════════
// Build
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct BuildArgs {
    /// Path to the .seed source file
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// Path where the compiled .aslb binary will be written
    #[arg(short = 'o', long, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Optimisation level: 0 (none), 1 (basic), 2 (default), 3 (aggressive)
    #[arg(short = 'O', long, default_value = "2")]
    opt_level: u8,

    /// Target architecture to compile for
    #[arg(long, default_value = "seedvm")]
    target: String,

    /// Emit debug information in the output
    #[arg(short = 'g', long)]
    debug: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Check
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct CheckArgs {
    /// Path to the .seed source file
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// Produce warnings that are suppressed by default
    #[arg(short = 'W', long)]
    warnings: bool,
}

// ═══════════════════════════════════════════════════════════════════
// Run
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct RunArgs {
    /// Path to the .seed source file
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// Deterministic seed for the PRNG (default: 0)
    #[arg(short, long, default_value = "0")]
    seed: u64,

    /// Arguments to pass to the running agent
    #[arg(last = true)]
    agent_args: Vec<String>,
}

// ═══════════════════════════════════════════════════════════════════
// Emit IR
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct EmitIrArgs {
    /// Path to the .seed source file
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// IR output format
    #[arg(long, default_value = "text", value_enum)]
    format: IrFormat,
}

#[derive(ValueEnum, Debug, Clone)]
enum IrFormat {
    /// Human-readable S-expression text format (.aslt)
    Text,
    /// Binary bytecode format (.aslb)
    Binary,
    /// Graphviz DOT format for control-flow graph visualisation
    Dot,
}

// ═══════════════════════════════════════════════════════════════════
// Emit Grammar
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct EmitGrammarArgs {
    /// Grammar output format
    #[arg(long, default_value = "gbnf", value_enum)]
    format: GrammarFormat,

    /// Path to write the grammar; stdout if omitted
    #[arg(short = 'o', long)]
    output: Option<PathBuf>,
}

#[derive(ValueEnum, Debug, Clone)]
enum GrammarFormat {
    /// GBNF format (llama.cpp compatible, default)
    Gbnf,
    /// EBNF format
    Ebnf,
    /// JSON Schema format
    JsonSchema,
}

// ═══════════════════════════════════════════════════════════════════
// Prove
// ═══════════════════════════════════════════════════════════════════

#[derive(clap::Args, Debug)]
struct ProveArgs {
    /// Path to the .seed source file
    #[arg(value_name = "SOURCE")]
    source: PathBuf,

    /// Property to prove (e.g. "no-deadlock", "memory-safety", "taint-safety")
    #[arg(short, long, default_value = "all")]
    property: String,
}

// ═══════════════════════════════════════════════════════════════════
// Entry point
// ═══════════════════════════════════════════════════════════════════

fn main() -> miette::Result<()> {
    let cli = Cli::parse();

    // Initialise logging
    let log_level = if cli.quiet {
        "error"
    } else {
        match cli.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        }
    };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(log_level))
        .with_writer(io::stderr)
        .init();

    // Dispatch subcommand
    match cli.command {
        Commands::Build(args) => cmd_build(args),
        Commands::Check(args) => cmd_check(args),
        Commands::Run(args) => cmd_run(args),
        Commands::EmitIr(args) => cmd_emit_ir(args),
        Commands::EmitGrammar(args) => cmd_emit_grammar(args),
        Commands::Prove(args) => cmd_prove(args),
    }
}

// ═══════════════════════════════════════════════════════════════════
// Command implementations
// ═══════════════════════════════════════════════════════════════════

/// `seed build` — compile a .seed file to .aslb.
fn cmd_build(args: BuildArgs) -> miette::Result<()> {
    let source = read_source(&args.source)?;
    let binary = seedc::compile(&source)
        .wrap_err_with(|| format!("failed to compile `{}`", args.source.display()))?;

    let out_path = args.output.unwrap_or_else(|| {
        args.source.with_extension("aslb")
    });

    std::fs::write(&out_path, &binary)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to write output to `{}`", out_path.display()))?;

    if !cli_is_quiet() {
        tracing::info!(
            "Compiled {} → {} ({} bytes, opt={})",
            args.source.display(),
            out_path.display(),
            binary.len(),
            args.opt_level
        );
    }
    Ok(())
}

/// `seed check` — type-check without emitting.
fn cmd_check(args: CheckArgs) -> miette::Result<()> {
    let source = read_source(&args.source)?;
    seedc::compile(&source)
        .wrap_err_with(|| format!("type-check failed for `{}`", args.source.display()))?;

    if !cli_is_quiet() {
        tracing::info!("`{}` passes all checks", args.source.display());
    }
    Ok(())
}

/// `seed run` — compile and execute on the VM (fully wired).
fn cmd_run(args: RunArgs) -> miette::Result<()> {
    let source = read_source(&args.source)?;
    let binary = seedc::compile(&source)
        .wrap_err_with(|| format!("failed to compile `{}`", args.source.display()))?;

    // Execute directly via the VM library — no subprocess.
    let state = seedvm::run_bytes(&binary, args.seed)
        .wrap_err_with(|| format!("VM execution failed for `{}`", args.source.display()))?;

    if !cli_is_quiet() {
        tracing::info!(
            "Execution complete — {} provenance events, {} schedule steps",
            state.provenance_log.len(),
            state.schedule_trace.len(),
        );
    }
    Ok(())
}

/// `seed emit-ir` — dump the intermediate representation.
fn cmd_emit_ir(args: EmitIrArgs) -> miette::Result<()> {
    let source = read_source(&args.source)?;
    let binary = seedc::compile(&source)
        .wrap_err_with(|| format!("failed to compile `{}`", args.source.display()))?;

    match args.format {
        IrFormat::Text => {
            println!(";; IR for `{}`", args.source.display());
            println!(";; {} bytes of bytecode", binary.len());
        }
        IrFormat::Binary => {
            io::stdout()
                .write_all(&binary)
                .into_diagnostic()
                .wrap_err("failed to write binary IR to stdout")?;
        }
        IrFormat::Dot => {
            println!("digraph {{");
            println!("  label=\"{}\";", args.source.display());
            println!("  // control-flow graph (placeholder)");
            println!("}}");
        }
    }
    Ok(())
}

/// `seed emit-grammar` — export the GBNF grammar.
fn cmd_emit_grammar(args: EmitGrammarArgs) -> miette::Result<()> {
    let grammar = generate_grammar(args.format);

    if let Some(path) = args.output {
        std::fs::write(&path, &grammar)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to write grammar to `{}`", path.display()))?;
    } else {
        io::stdout()
            .write_all(grammar.as_bytes())
            .into_diagnostic()
            .wrap_err("failed to write grammar to stdout")?;
    }
    Ok(())
}

/// `seed prove` — static analysis proof.
fn cmd_prove(args: ProveArgs) -> miette::Result<()> {
    let source = read_source(&args.source)?;
    let _binary = seedc::compile(&source)
        .wrap_err_with(|| format!("failed to compile `{}`", args.source.display()))?;

    tracing::info!(
        "Proving property `{}` for `{}`",
        args.property,
        args.source.display()
    );
    tracing::warn!("Proof generation is not yet implemented — see docs for roadmap");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════════════

/// Read a source file into a String, or from stdin if path is "-".
fn read_source(path: &Path) -> miette::Result<String> {
    if path == Path::new("-") {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .into_diagnostic()
            .wrap_err("failed to read source from stdin")?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("failed to read source file `{}`", path.display()))
    }
}

/// Check whether the global `--quiet` flag was supplied.
fn cli_is_quiet() -> bool {
    Cli::parse().quiet
}

/// Generate a placeholder grammar string for the requested format.
fn generate_grammar(format: GrammarFormat) -> String {
    match format {
        GrammarFormat::Gbnf => {
            r#"root ::= item*

item ::= "agent" identifier "{" member* "}"
       | "fn" identifier "(" params ")" ("->" type)? block
       | "section" identifier "{" field* "}"
       | "struct" identifier "{" field* "}"
       | "enum" identifier "{" variant ("," variant)* ","? "}"

identifier ::= [a-zA-Z_][a-zA-Z0-9_]*
type ::= "bool" | "i32" | "i64" | "f32" | "f64" | "string" | "char" | identifier
block ::= "{" stmt* "}"
stmt ::= "let" identifier (":" type)? "=" expr ";"
       | expr ";"
       | "return" expr? ";"
expr ::= literal | identifier | call | binary | unary | "if" | "match" | "loop" | "while" | "for"
literal ::= [0-9]+ | [0-9]+ "." [0-9]+ | "\"" [^"]* "\"" | "true" | "false" | "null"
call ::= expr "(" args ")"
binary ::= expr op expr
unary ::= op expr
"#
            .into()
        }
        GrammarFormat::Ebnf => {
            String::from("program = { item* } ;\nitem = { ... } ;\n")
        }
        GrammarFormat::JsonSchema => {
            serde_json::json!({
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "title": "AGENT-SEED v15.2 grammar",
                "type": "object"
            })
            .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_help() {
        let result = Cli::try_parse_from(["seed", "--help"]);
        assert!(result.is_err()); // clap exits on help
    }

    #[test]
    fn test_cli_parse_build() {
        let cli = Cli::try_parse_from(["seed", "build", "hello.seed"]).unwrap();
        match cli.command {
            Commands::Build(args) => assert_eq!(args.source, PathBuf::from("hello.seed")),
            _ => panic!("expected Build"),
        }
    }

    #[test]
    fn test_cli_parse_build_output() {
        let cli = Cli::try_parse_from([
            "seed", "-vv", "build", "hello.seed", "-o", "hello.aslb",
        ])
        .unwrap();
        assert_eq!(cli.verbose, 2);
        match cli.command {
            Commands::Build(args) => {
                assert_eq!(args.source, PathBuf::from("hello.seed"));
                assert_eq!(args.output, Some(PathBuf::from("hello.aslb")));
            }
            _ => panic!("expected Build"),
        }
    }

    #[test]
    fn test_cli_parse_run() {
        let cli = Cli::try_parse_from([
            "seed", "run", "hello.seed", "--", "--agent-arg",
        ])
        .unwrap();
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.source, PathBuf::from("hello.seed"));
                assert_eq!(args.agent_args, vec!["--agent-arg"]);
            }
            _ => panic!("expected Run"),
        }
    }

    #[test]
    fn test_cli_parse_emit_ir() {
        let cli =
            Cli::try_parse_from(["seed", "emit-ir", "hello.seed", "--format", "dot"]).unwrap();
        match cli.command {
            Commands::EmitIr(args) => {
                assert!(matches!(args.format, IrFormat::Dot));
            }
            _ => panic!("expected EmitIr"),
        }
    }

    #[test]
    fn test_read_source_stdin() {
        let result = read_source(Path::new("-"));
        let _ = result;
    }
}