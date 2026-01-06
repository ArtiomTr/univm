use ssz::Ssz;
use univm_io::ssz::SszIo;

#[derive(Debug, Ssz)]
pub struct Input {
    pub value: u64,
}

#[derive(Debug, Ssz)]
pub struct Output {
    pub value2: u64,
}

#[univm_platform::function(SszIo)]
pub fn state_transition(input: Input) -> Output {
    println!("Hello, world!");

    Output {
        value2: input.value + 3,
    }
}
