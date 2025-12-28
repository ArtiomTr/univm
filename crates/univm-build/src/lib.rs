use std::{
    env,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
};

use univm_interface::compiler::Compiler;

#[derive(Default)]
pub struct BuildOptions {
    compilers: Vec<Box<dyn Compiler>>,
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

impl BuildOptions {
    pub fn new() -> Self {
        BuildOptions {
            compilers: Vec::new(),
        }
    }

    pub fn add(mut self, compiler: impl Compiler + 'static) -> Self {
        self.compilers.push(Box::new(compiler));
        self
    }

    pub fn build(self) {
        if self.compilers.len() == 0 {
            panic!("No compilers installed - please choose at least one zkvm");
        }

        if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "zkvm" {
            // risc0::Risc0BuildOptions::new().build();
            return;
        }

        let target_dir = get_out_dir();
        let mut generated_methods = File::create(target_dir.join("methods.rs")).unwrap();

        let vms = self
            .compilers
            .iter()
            .map(|compiler| {
                compiler
                    .compile(
                        &Path::new(std::env!("CARGO_MANIFEST_DIR")),
                        &target_dir,
                        &mut generated_methods,
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
    }
}

pub fn build() {
    BuildOptions::default().build();
}
