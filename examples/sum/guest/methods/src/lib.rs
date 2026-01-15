use ssz_grandine::Ssz;
use univm_io::ssz_grandine::SszGrandineIo;

#[derive(Debug, Ssz)]
pub struct Input {
    pub a: u64,
    pub b: u64,
}

#[derive(Debug, Ssz)]
pub struct Output {
    pub sum: u64,
}

#[univm_platform::function(SszGrandineIo)]
pub fn state_transition(input: Input) -> Output {
    println!("Hello, world!");

    Output {
        sum: input.a.checked_add(input.b).unwrap(),
    }
}
