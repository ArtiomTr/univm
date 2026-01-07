use std::{fs::File, io::Read, path::Path};

use sp1_helper::BuildArgs;
use univm_interface::compiler::{CompilationResult, Compiler};

#[derive(Default)]
pub struct Sp1Compiler;

impl Compiler for Sp1Compiler {
    fn compile(&self, crate_path: &Path, target_path: &Path) -> Result<CompilationResult, ()> {
        let crate_name = {
            let mut file = File::open(crate_path.join("Cargo.toml")).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();
            let package: toml::Value = toml::from_str(&buffer).unwrap();
            package
                .get("package")
                .and_then(|x| x.get("name"))
                .and_then(toml::Value::as_str)
                .map(ToOwned::to_owned)
                .unwrap()
        };

        sp1_helper::build_program_with_args(
            crate_path.as_os_str().to_str().unwrap(),
            BuildArgs {
                output_directory: Some(target_path.to_str().unwrap().to_owned()),
                packages: vec![crate_name.clone()],
                ..Default::default()
            },
        );

        Ok(CompilationResult {
            vm_name: "Sp1".to_owned(),
            vm_full_name: "univm_sp1::Sp1".to_owned(),
            program_name: "[<$base_program_name Sp1>]".to_owned(),
            program_impl: format!(
                r#"pub struct [<$base_program_name Sp1>](univm_sp1::Sp1Program<$input, $output, $io>);

                impl [<$base_program_name Sp1>] {{
                    fn init(vm: &univm_sp1::Sp1) -> Self {{
                        const ELF: &[u8] = include_bytes!(env!("SP1_ELF_{crate_name}"));

                        Self(univm_sp1::Sp1Program::<$input, $output, $io>::new(vm, ELF, $io))
                    }}
                }}

                impl univm_interface::GuestProgram<univm_sp1::Sp1> for [<$base_program_name Sp1>] {{
                    type Input = $input;
                    type Output = $output;

                    fn execute(&self, zkvm: &univm_sp1::Sp1, input: Self::Input) -> Result<(Self::Output, univm_sp1::Sp1ExecutionReport), ()> {{
                        self.0.execute(zkvm, input)
                    }}

                    fn prove(&self, zkvm: &univm_sp1::Sp1, input: Self::Input) -> Result<(Self::Output, univm_sp1::Sp1Proof, univm_sp1::Sp1ExecutionReport), ()> {{
                        self.0.prove(zkvm, input)
                    }}

                    fn verify(&self, zkvm: &univm_sp1::Sp1, proof: &univm_sp1::Sp1Proof) -> bool {{
                        self.0.verify(zkvm, proof)
                    }}
                }}"#
            ),
        })
    }

    fn emit_platform(&self) -> Result<String, ()> {
        Ok(PLATFORM_CODE.to_owned())
    }
}

const PLATFORM_CODE: &'static str = r#"
#[cfg_zkvm(sp1)]
#[allow(unused)]
pub type UniVMCurrentPlatform = univm_platform_sp1::Sp1Platform;

#[cfg_zkvm(sp1)]
#[allow(unused)]
pub use univm_platform_sp1::__univm_entrypoint;
"#;
