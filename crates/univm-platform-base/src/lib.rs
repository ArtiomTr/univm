pub trait Platform {
    fn read_input() -> Vec<u8>;

    fn write_output(bytes: &[u8]);
}
