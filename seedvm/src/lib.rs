//! AGENT-SEED v15.2 virtual machine — `seedvm`.
//!
//! A deterministic, stack-based bytecode interpreter that executes
//! `.aslb` binary modules produced by `seedc`.
//!
//! # Architecture
//!
//! The VM is a structured stack machine inspired by WebAssembly and
//! clox (Crafting Interpreters). It uses:
//!
//! - An **operand stack** for implicit value passing between instructions.
//! - A **local variable array** for mutable function-scoped storage.
//! - A **global variable table** for module-level state.
//! - An **effect accumulator** that tracks uncertainty, taint, and cost.
//! - A **provenance graph** for auditable execution traces.
//! - A **deterministic PRNG** for reproducible randomness.
//!
//! # References
//!
//! - Crafting Interpreters (Nystrom, 2021) — bytecode VM design
//! - WebAssembly Core Specification v3.0 — structured stack machine
//! - Affect (van Rooij & Krebbers, POPL 2025) — effect system integration

pub mod value;
pub mod state;
pub mod executor;
pub mod rng;
pub mod schedule;

use std::path::Path;
use miette::{IntoDiagnostic, WrapErr};

/// Load an `.aslb` module from disk and execute it.
///
/// Returns the final VM state after execution completes.
pub fn run_file(path: &Path, seed: u64) -> miette::Result<state::VMState> {
    let data = std::fs::read(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("failed to read `{}`", path.display()))?;

    let module = seedc::binary::deserialize(&data)
        .wrap_err_with(|| format!("failed to deserialise `{}`", path.display()))?;

    let mut vm = executor::VM::new(module, seed);
    vm.run()?;

    Ok(vm.state)
}

/// Load an `.aslb` module from bytes and execute it.
pub fn run_bytes(data: &[u8], seed: u64) -> miette::Result<state::VMState> {
    let module = seedc::binary::deserialize(data)
        .wrap_err("failed to deserialise module")?;

    let mut vm = executor::VM::new(module, seed);
    vm.run()?;

    Ok(vm.state)
}