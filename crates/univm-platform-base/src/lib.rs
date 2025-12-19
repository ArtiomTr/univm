pub trait Platform {
    fn read_input();

    fn write_output(bytes: &[u8]);
}
