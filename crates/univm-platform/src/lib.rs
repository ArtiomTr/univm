pub use univm_platform_macros::function;

#[cfg(target_os = "zkvm")]
pub fn read<T>(reader: impl univm_io::Io<T>) -> T {
    todo!()
}

#[macro_export]
macro_rules! entrypoint {
    ($curr: ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), ".univm", "platform.rs"))
    };
}
