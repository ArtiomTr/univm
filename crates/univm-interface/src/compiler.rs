use std::{fs::File, path::Path};

pub trait Compiler {
    fn compile(
        &self,
        base_program_name: String,
        crate_path: &Path,
        target_path: &Path,
    ) -> Result<CompilationResult, ()>;
}

pub struct CompilationResult {
    /// Fully qualified zkvm name.
    pub vm_name: String,

    /// Fully qualified program struct name.
    pub program_name: String,

    pub program_impl: String,
}
