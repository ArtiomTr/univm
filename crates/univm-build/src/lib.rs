mod risc0;

pub struct BuildOptions {}

impl BuildOptions {
    pub fn new() -> Self {
        BuildOptions {  }
    }

    pub fn build(self) {
        risc0::Risc0BuildOptions::new()
            .build();
    }
}
