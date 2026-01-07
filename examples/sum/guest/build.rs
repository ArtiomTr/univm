fn main() {
    univm_build::new()
        .add_crate("methods")
        .zkvm(univm_risc0::compiler())
        .zkvm(univm_sp1::compiler())
        .build();
}
