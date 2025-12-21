use univm_io::ssz::SszIo;

pub struct Input {}

#[univm_platform::entrypoint(SszIo)]
fn main(input: Input) {
    println!("Hello, world!");
}
