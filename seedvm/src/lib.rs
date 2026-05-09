pub mod value;
pub mod computation;
pub mod state;
pub mod executor;
pub mod rng;
pub mod schedule;
pub mod memory;
pub mod protocols;

use seedc::ir::Module;
use std::path::Path;

pub use state::VMState;

/// Run a compiled .aslb file from disk, returning the final VM state.
pub fn run_file(path: &Path, seed: u64) -> miette::Result<VMState> {
    let data = std::fs::read(path)
        .map_err(|e| miette::miette!("I/O error reading {}: {}", path.display(), e))?;
    run_bytes(&data, seed)
}

/// Run a compiled .aslb module from bytes, returning the final VM state.
pub fn run_bytes(data: &[u8], seed: u64) -> miette::Result<VMState> {
    let module: Module = seedc::binary::deserialize(data)
        .map_err(|e| miette::miette!("Deserialisation error: {}", e))?;
    let mut vm = executor::VM::new(module, seed);
    vm.run()?;
    Ok(vm.state)
}