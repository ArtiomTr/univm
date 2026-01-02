use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use cargo_metadata::MetadataCommand;
use univm_interface::compiler::{CompilationResult, Compiler};

pub struct BuildOptions {
    compilers: Vec<Box<dyn Compiler>>,
    crates: Vec<PathBuf>,
}

fn get_out_dir() -> PathBuf {
    // This code is based on https://docs.rs/cxx-build/latest/src/cxx_build/target.rs.html#10-49

    if let Some(target_dir) = env::var_os("CARGO_TARGET_DIR").map(Into::<PathBuf>::into) {
        if target_dir.is_absolute() {
            return target_dir.join("riscv-guest");
        }
    }

    let mut dir: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    loop {
        if dir.join(".rustc_info.json").exists()
            || dir.join("CACHEDIR.TAG").exists()
            || dir.file_name() == Some(OsStr::new("target"))
                && dir
                    .parent()
                    .is_some_and(|parent| parent.join("Cargo.toml").exists())
        {
            return dir.join("riscv-guest");
        }
        if dir.pop() {
            continue;
        }
        panic!("Cannot find cargo target dir location")
    }
}

const CACHEDIR_TAG_CONTENT: &'static str = r#"Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by univm compiler.
# For information about cache directory tags, see:
#	http://www.brynosaurus.com/cachedir/"#;

impl BuildOptions {
    pub fn new() -> Self {
        BuildOptions {
            compilers: Vec::new(),
            crates: Vec::new(),
        }
    }

    pub fn add_crate(mut self, path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        if path.is_absolute() {
            self.crates.push(path.to_path_buf());
        } else {
            let manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();

            self.crates.push(manifest_dir.join(path));
        }

        self
    }

    pub fn zkvm(mut self, compiler: impl Compiler + 'static) -> Self {
        self.compilers.push(Box::new(compiler));
        self
    }

    fn build_crate(&self, crate_path: &Path) {
        let target_dir = get_out_dir();

        let temp_artifacts_path = crate_path.join(".univm");
        fs::create_dir_all(&temp_artifacts_path).unwrap();

        {
            let mut ignorefile = File::create(temp_artifacts_path.join(".gitignore")).unwrap();
            writeln!(ignorefile, "*").unwrap();
        }

        {
            let mut cachedir = File::create(temp_artifacts_path.join("CACHEDIR.TAG")).unwrap();
            writeln!(cachedir, "{}", CACHEDIR_TAG_CONTENT).unwrap();
        }

        let vms = self
            .compilers
            .iter()
            .map(|compiler| {
                compiler
                    .compile(crate_path, &temp_artifacts_path, &target_dir)
                    .unwrap()
            })
            .collect::<Vec<_>>();

        let mut generated_methods = File::create(target_dir.join("methods.rs")).unwrap();
        writeln!(
            generated_methods,
            r#"macro_rules! impl_program {{
                ($input: ty, $output: ty, $io: ty, $base_program_name: ident) => {{
                    univm_interface::compiler::paste! {{
                        {concrete_program_impls}
                        
                        enum $base_program_name {{
                            {programs}
                        }};

                        {builder_impls}

                        impl univm_interface::GuestProgram<univm_interface::UniVM> for $base_program_name {{
                            type Input = $input;
                            type Output = $output;

                            fn execute(&self, zkvm: &univm_interface::UniVM, input: Self::Input) -> Result<(Self::Output, T::ExecutionReport), ()> {{
                                {univm_execute}
                            }}

                            fn prove(&self, zkvm: &univm_interface::UniVM, input: Self::Input) -> Result<(Self::Output, T::Proof, T::ExecutionReport), ()> {{
                                {univm_prove}
                            }}

                            fn verify(&self, zkvm: &univm_interface::UniVM, proof: &univm_interface::UniProof) -> bool {{
                                {univm_verify}
                            }}
                        }}

                        impl univm_interface::GuestProgramBuilder<univm_interface::UniVM> for $base_program_name {{
                            fn init() -> impl univm_interface::GuestProgram {{
                                $base_program_name::init()
                            }}
                        }}
                    }};
                }};
            }}"#,
            concrete_program_impls = vms
                .iter()
                .map(|CompilationResult { program_impl, .. }| program_impl.as_str())
                .collect::<String>(),
            programs = vms
                .iter()
                .map(
                    |CompilationResult {
                         vm_name,
                         program_name,
                         ..
                     }| format!("{vm_name}({program_name}),")
                )
                .collect::<String>(),
            builder_impls = vms.iter().map(|CompilationResult {
                         vm_full_name,
                         program_name,
                         ..
                     }| format!(r#"impl univm_interface::GuestProgramBuilder<{vm_full_name}> for $base_program_name {{
                            fn init() -> impl univm_interface::GuestProgram<{vm_full_name}> {{
                                {program_name}
                            }}
                        }}"#))
                    .collect::<String>(),
            univm_execute = format!(
                r#"match self {{
                    {}
                }}"#,
                vms.iter().map(|CompilationResult { vm_name, vm_full_name, .. }| format!(
                    r#"Self::{vm_name}(ref program) => {{
                        let zkvm = zkvm.downcast_ref::<{vm_full_name}>().unwrap();

                        let (output, report) = program.execute(zkvm, input)?;

                        Ok((output, Box::new(report)))
                    }}"#)).collect::<String>()
            ),
            univm_prove = format!(
                r#"match self {{
                    {}
                }}"#,
                vms.iter().map(|CompilationResult { vm_name, vm_full_name, .. }| format!(
                    r#"Self::{vm_name}(ref program) => {{
                        let zkvm = zkvm.downcast_ref::<{vm_full_name}>().unwrap();

                        let (output, proof, report) = program.prove(zkvm, input)?;

                        Ok((output, UniProof::new(proof), Box::new(report)))
                    }}"#)).collect::<String>()
            ),
            univm_verify = format!(
                r#"match self {{
                    {}
                }}"#,
                vms.iter().map(|CompilationResult { vm_name, vm_full_name, .. }| format!(
                    r#"Self::{vm_name}(ref program) => {{
                        let zkvm = zkvm.downcast_ref::<{vm_full_name}>().unwrap();
                        let proof = proof.downcast_ref::<{vm_full_name}::Proof>().unwrap();

                        program.verify(zkvm, proof)
                    }}"#
                )).collect::<String>()
            )
        )
        .unwrap();
    }

    pub fn build(self) {
        if self.compilers.len() == 0 {
            panic!("No compilers installed - please choose at least one zkvm");
        }

        if self.crates.len() == 0 {
            panic!("No crates selected to build - please provide at least one crate to build");
        }

        for c in self.crates.iter() {
            self.build_crate(&c)
        }
    }
}

pub fn new() -> BuildOptions {
    BuildOptions::new()
}
