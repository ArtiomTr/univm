use cfg_if::cfg_if;
use univm_interface::compiler::Compiler;

cfg_if! {
    if #[cfg(not(target_arch = "riscv32"))] {
        mod zkvm;
        pub use zkvm::*;
    }
}

#[cfg(not(target_arch = "riscv32"))]
mod compiler;

pub fn compiler() -> impl Compiler {
    cfg_if! {
        if #[cfg(target_arch = "riscv32")] {
            univm_interface::compiler::StubCompiler
        } else {
            compiler::Risc0Compiler::default()
        }
    }
}
