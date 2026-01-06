use std::path::Path;

pub use paste::paste;

pub trait Compiler {
    fn compile(&self, crate_path: &Path, target_path: &Path) -> Result<CompilationResult, ()>;

    fn emit_platform(&self) -> Result<String, ()>;
}

pub struct CompilationResult {
    pub vm_name: String,

    /// Fully qualified zkvm name.
    pub vm_full_name: String,

    /// Fully qualified program struct name.
    pub program_name: String,

    pub program_impl: String,
}
