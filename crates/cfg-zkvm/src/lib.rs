use std::env;

pub use cfg_zkvm_macro::cfg_zkvm;

pub fn config_values() {
    println!("cargo::rustc-check-cfg=cfg(risc0, values(none()))");
    println!("cargo::rustc-check-cfg=cfg(sp1, values(none()))");
    println!("cargo::rustc-check-cfg=cfg(pico, values(none()))");
    println!("cargo::rustc-check-cfg=cfg(ziren, values(none()))");
    println!("cargo::rustc-check-cfg=cfg(zisk, values(none()))");
    println!("cargo::rustc-check-cfg=cfg(zkvm_pico, values(none()))");
    println!(
        r#"cargo::rustc-check-cfg=cfg(zkvm, values("risc0", "sp1", "pico", "ziren", "zisk"))"#
    );

    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect("config_values() function is expected to be executed in build.rs file");
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR")
        .expect("config_values() function is expected to be executed in build.rs file");
    let cfg_zkvm_pico = env::var("CARGO_CFG_ZKVM_PICO").is_ok();

    let zkvm = match (target_os.as_str(), target_vendor.as_str(), cfg_zkvm_pico) {
        ("zkvm", "risc0", false) => "risc0",
        ("zkvm", "succinct", false) => "sp1",
        ("zkvm", "risc0", true) => "pico",
        ("zkvm", "zkm", false) => "ziren",
        ("zkvm", "zisk", false) => "zisk",

        (_, _, true) => panic!(
            "invalid configuration - configuration option `zkvm_pico` may be set only when target_vendor is `risc0`."
        ),
        ("zkvm", vendor, _) => {
            println!(
                r#"cargo::warning=Unknown zkvm vendor found ("{vendor}") - cfg_zkvm doesn't support this zkvm."#
            );
            return;
        }
        (_, _, _) => return,
    };

    println!("cargo::rustc-cfg={}", zkvm);
    println!("cargo::rustc-cfg=zkvm={:?}", zkvm);
}
