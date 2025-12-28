use std::{env, ffi::OsStr, fs::File, io::Write, path::PathBuf};

use risc0_build::GuestOptions;

pub struct Risc0BuildOptions {}

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

impl Risc0BuildOptions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self) {
        let package = risc0_build::get_package(env::var("CARGO_MANIFEST_DIR").unwrap());
        let out_dir = get_out_dir();
        let mut methods_file = File::create(out_dir.join("methods.rs")).unwrap();

        let methods = risc0_build::build_package(&package, &out_dir, GuestOptions::default())
            .expect("Should not fail");

        for m in methods {
            writeln!(
                methods_file,
                "pub const ELF: &[u8] = include_bytes!({:?});",
                m.path
            )
            .unwrap();
            writeln!(methods_file, "pub const PATH: &str = {:?};", m.path).unwrap();
            writeln!(
                methods_file,
                "pub const ID: [u32; 8] = {:?};",
                m.image_id.as_words()
            )
            .unwrap();
        }
    }
}
