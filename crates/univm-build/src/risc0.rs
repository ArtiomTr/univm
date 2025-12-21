pub struct Risc0BuildOptions {}

impl Risc0BuildOptions {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn build(self) {
        risc0_build::embed_methods();
    }
}
