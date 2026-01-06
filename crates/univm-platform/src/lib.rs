pub use cfg_zkvm::cfg_zkvm;
use univm_io::Io;
pub use univm_platform_macros::function;

#[macro_export]
macro_rules! entrypoint {
    ($curr: path) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/.univm/platform.rs"));

        __univm_entrypoint!($curr);
    };
}

pub trait Platform {
    fn read_input() -> Vec<u8>;

    fn write_output(bytes: &[u8]);
}

pub fn read<P: Platform, T>(io: impl Io<T>) -> T {
    let content = P::read_input();

    let result = io.deserialize(&content).unwrap();

    result
}

pub fn commit<P: Platform, T>(io: impl Io<T>, value: T) {
    let bytes = io.serialize(value).unwrap();
    P::write_output(&bytes);
}
