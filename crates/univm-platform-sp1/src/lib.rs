use sp1_zkvm::io;
use univm_platform::Platform;

pub struct Sp1Platform;

impl Platform for Sp1Platform {
    fn read_input() -> Vec<u8> {
        io::read_vec()
    }

    fn write_output(bytes: &[u8]) {
        io::commit_slice(bytes);
    }
}

pub use sp1_zkvm::entrypoint as __univm_entrypoint;
