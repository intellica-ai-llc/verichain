//! Semantic analysis passes for AGENT-SEED v15.2.
//!
//! Pipeline:
//!   1. Name resolution — build scope graph, link identifiers to definitions
//!   2. Type checking — Hindley-Milner inference with affine tracking
//!   3. Effect checking — row-based effect accumulation
//!   4. Taint checking — lattice-based information flow control
//!   5. Contract checking — discharge/perform scoping, temporal constraints
//!
//! References:
//!   - Algorithm W (Milner 1978)
//!   - Affect: An Affine Type and Effect System (van Rooij & Krebbers, POPL 2025)
//!   - Tant: taint qualifiers in the type system (Bertolo, 2026)
//!   - Scope Graphs (Néron et al., 2015; van Antwerpen et al., 2018)

pub mod nameres;
pub mod typeck;
pub mod effectck;
pub mod taintck;
pub mod contractck;
pub mod types;

use crate::ast::Program;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

// ── Unified semantic error type ──

#[derive(Error, Diagnostic, Debug)]
pub enum TypeError {
    #[error("unresolved name `{name}`")]
    #[diagnostic(help("Did you forget to import or declare this name?"))]
    UnresolvedName {
        name: String,
        #[label("not found in this scope")]
        span: SourceSpan,
    },

    #[error("type mismatch")]
    #[diagnostic(help("Expected `{expected}`, found `{found}`."))]
    Mismatch {
        expected: String,
        found: String,
        #[label("expected {expected}")]
        span: SourceSpan,
        #[label("found {found}")]
        found_span: Option<SourceSpan>,
    },

    #[error("affine type violation")]
    #[diagnostic(help("Capability `{name}` was already consumed or moved."))]
    AffineViolation {
        name: String,
        #[label("used again here")]
        span: SourceSpan,
    },

    #[error("effect not discharged")]
    #[diagnostic(help("Effect `{effect}` is performed outside a discharge block."))]
    UndischargedEffect {
        effect: String,
        #[label("performed here")]
        span: SourceSpan,
    },

    #[error("taint violation")]
    #[diagnostic(help("{message}"))]
    TaintViolation {
        message: String,
        #[label("violation")]
        span: SourceSpan,
    },

    #[error("contract violation")]
    #[diagnostic(help("{message}"))]
    ContractViolation {
        message: String,
        #[label("violates contract")]
        span: SourceSpan,
    },

    #[error(transparent)]
    Other(#[from] Box<dyn Diagnostic + Send + Sync + 'static>),
}

// ── Top-level entry point ──

/// Run all semantic analysis passes and return a typed AST.
pub fn check(program: Program) -> Result<Program, TypeError> {
    let mut program = nameres::resolve(program)?;
    program = typeck::infer_types(program)?;
    program = effectck::check_effects(program)?;
    program = taintck::check_taint(program)?;
    program = contractck::check_contracts(program)?;
    Ok(program)
}
