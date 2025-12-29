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

#[macro_export]
macro_rules! __zkvm_entrypoint {
    ($entry_name: path) => {
        #[!no_main]

        // Copy-pasted risc0_zkvm::entry, just to additionally provide #[!no_main].

        // Type check the given path
        const ZKVM_ENTRY: fn() = $path;

        // Include generated main in a module so we don't conflict
        // with any other definitions of "main" in this file.
        mod zkvm_generated_main {
            #[no_mangle]
            fn main() {
                super::ZKVM_ENTRY()
            }
        }
    };
}
