use univm_io::RawIo;

/// Entrypoint for the zkvm guest program.
/// When compiled for zkvm: executes the computation.
/// When compiled for host: exposes ELF, ImageID, and Input type.
#[univm_platform::entrypoint(RawIo)]
fn guest_main(_input: ()) {
    // Simple computation for testing
    let a = 5u32;
    let b = 3u32;
    let sum = a + b;
    // In a real implementation, we would commit the result
    let _ = sum;
}
