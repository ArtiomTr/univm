use univm_io::ssz::SszIo;

pub struct Input {}

#[univm_platform::function(SszIo)]
pub fn state_transition(input: Input) {
    println!("Hello, world!");
}
