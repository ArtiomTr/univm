mod risc0;

pub use risc0::Risc0BuildOptions;

pub struct BuildOptions {}

impl BuildOptions {
    pub fn new() -> Self {
        BuildOptions {}
    }

    pub fn build(self) {
        Risc0BuildOptions::new().build();
    }
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Build the guest package for zkvm and generate host constants.
/// 
/// This function should be called from the guest package's build.rs file.
/// It will:
/// 1. Build the guest code for the risc0 zkvm target
/// 2. Generate a `guest_methods.rs` file with ELF and ImageID constants
/// 3. The constants can then be included via the `entrypoint` macro
pub fn build() {
    BuildOptions::new().build();
}
