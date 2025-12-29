use std::path::Path;

pub use paste::paste;

pub trait Compiler {
    fn compile(&self, crate_path: &Path, target_path: &Path) -> Result<CompilationResult, ()>;
}

pub struct CompilationResult {
    pub vm_name: String,

    /// Fully qualified zkvm name.
    pub vm_full_name: String,

    /// Fully qualified program struct name.
    pub program_name: String,

    pub program_impl: String,
}

pub struct StubCompiler;

impl Compiler for StubCompiler {
    fn compile(&self, _crate_path: &Path, _target_path: &Path) -> Result<CompilationResult, ()> {
        panic!("stub compiler")
    }
}
