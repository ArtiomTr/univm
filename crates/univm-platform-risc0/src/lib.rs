use risc0_zkvm::guest::env;
use univm_platform_base::Platform;

pub struct Risc0Platform;

impl Platform for Risc0Platform {
    fn read_input() -> Vec<u8> {
        todo!()
    }

    fn write_output(bytes: &[u8]) {
        todo!()
    }
}
