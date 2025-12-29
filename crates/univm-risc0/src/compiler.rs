use std::path::Path;

use risc0_build::GuestOptions;
use univm_interface::compiler::{CompilationResult, Compiler};

#[derive(Default)]
pub struct Risc0Compiler;

impl Compiler for Risc0Compiler {
    fn compile(&self, crate_path: &Path, target_path: &Path) -> Result<CompilationResult, ()> {
        let package = risc0_build::get_package(crate_path);

        println!("cargo:warn=building {crate_path:?} into {target_path:?}");

        let entries =
            risc0_build::build_package(&package, &target_path, GuestOptions::default()).unwrap();

        let Some(entry) = entries.first() else {
            panic!("not a single entry found");
        };

        assert!(entries.len() == 1, "too many entries");

        let elf_path = &entry.path;
        let image_id = entry.image_id.as_words();

        Ok(CompilationResult {
            vm_name: "Risc0".to_owned(),
            vm_full_name: "univm_risc0::Risc0".to_owned(),
            program_name: "[<$base_program_name Risc0>]".to_owned(),
            program_impl: format!(
                r#"pub struct [<$base_program_name Risc0>](univm_risc0::Risc0Program<$input, $output, $io>);

                impl [<$base_program_name Risc0>] {{
                    fn init() -> Self {{
                        const ELF: &[u8] = include_bytes!({elf_path:?});
                        const DIGEST: [u32; 8] = {image_id:?};

                        Self(univm_risc0::Risc0Program::<$input, $output, $io>::new(ELF, DIGEST, $io))
                    }}
                }}

                impl GuestProgram<univm_risc0::Risc0> for [<$base_program_name Risc0>] {{
                    type Input = $input;
                    type Output = $output;

                    fn execute(&self, zkvm: &univm_risc0::Risc0, input: Self::Input) -> Result<(Self::Output, univm_risc0::Risc0ExecutionReport), ()> {{
                        self.0.execute(zkvm, input)
                    }}

                    fn prove(&self, zkvm: &univm_risc0::Risc0, input: Self::Input) -> Result<(Self::Output, univm_risc0::Risc0Proof, univm_risc0::Risc0ExecutionReport), ()> {{
                        self.0.prove(zkvm, input)
                    }}

                    fn verify(&self, zkvm: &univm_risc0::Risc0, proof: univm_risc0::Risc0Proof) -> bool {{
                        self.0.verify(zkvm, proof)
                    }}
                }}"#
            ),
        })
    }
}
