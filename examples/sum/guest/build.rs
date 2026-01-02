fn main() {
    univm_build::new()
        .add_crate("methods")
        .zkvm(univm_risc0::compiler())
        .build();
}
