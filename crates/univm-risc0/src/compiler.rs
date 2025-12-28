use std::{fs::File, io::Write as _, path::Path};

use risc0_build::GuestOptions;
use univm_interface::compiler::{CompilationResult, Compiler};

#[derive(Default)]
pub struct Risc0Compiler;

impl Compiler for Risc0Compiler {
    fn compile(
        &self,
        base_program_name: String,
        crate_path: &Path,
        target_path: &Path,
    ) -> Result<CompilationResult, ()> {
        let package = risc0_build::get_package(crate_path);

        let entries =
            risc0_build::build_package(&package, &target_path, GuestOptions::default()).unwrap();

        let Some(entry) = entries.first() else {
            panic!("not a single entry found");
        };

        assert!(entries.len() == 1, "too many entries");

        let elf_path = &entry.path;

        write!(
            generated_code,
            r#"mod risc0 {{
    pub const ELF: &[u8] = include_bytes!({elf_path:?});
}}"#
        )
        .unwrap();

        Ok(CompilationResult {
            vm_name: "univm_risc0::Risc0".to_owned(),
            program_name: "univm_risc0::Risc0Program".to_owned(),
            program_impl: format!(
                r#"pub struct {base_program_name}Risc0(univm_risc0::Risc0Program<$input, $output, $io>);
                
                impl GuestProgram<univm_risc0::Risc0> for {base_program_name}Risc0 {{
                    type Input = $input;
                    type Output = $output;
                    
                    fn execute(&self, input: Self::Input) -> Result<(Self::Output, Risc0ExecutionReport), ()> {{
                        self.0.execute(input);
                    }}
                    
                    fn prove(&self, input: Self::Input) -> Result<(Self::Output, Risc0Proof, Risc0ExecutionReport), ()> "#
            ),
        })
    }
}
