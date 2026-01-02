use cfg_if::cfg_if;
use univm_interface::compiler::Compiler;

mod zkvm;
pub use zkvm::*;

mod compiler;

pub fn compiler() -> impl Compiler {
    compiler::Risc0Compiler::default()
}
