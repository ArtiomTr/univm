pub use univm_platform_macros::entrypoint;

cfg_if::cfg_if! {
    if #[cfg(not(target_os = "zkvm"))] {

    } else if #[cfg(target_vendor = "risc0")] {
        use univm_platform_risc0::Risc0Platform as CurrentPlatform;
    } else {
        compile_error!("univm-platform doesn't support this zkvm");
    }
}

#[cfg(target_os = "zkvm")]
pub fn read<T>(reader: impl univm_io::Io<T>) -> T {
    let bytes = CurrentPlatform::read_bytes();

    reader.deserialize(bytes).unwrap()
}
