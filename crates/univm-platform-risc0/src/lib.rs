use risc0_zkvm::guest::env;
use univm_platform::Platform;

pub struct Risc0Platform;

impl Platform for Risc0Platform {
    fn read_input() -> Vec<u8> {
        let mut len = [0u8; 4];
        env::read_slice(&mut len);

        let len = u32::from_be_bytes(len) as usize;
        let mut buffer = vec![0u8; len];
        env::read_slice(&mut buffer);

        buffer
    }

    fn write_output(bytes: &[u8]) {
        env::commit_slice(bytes);
    }
}

pub use risc0_zkvm::entry as __univm_entrypoint;
