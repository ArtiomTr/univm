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

/// Computes the sum of `input.a` and `input.b` and returns it in an `Output`.
///
/// The function prints "Hello, world!" to standard output.
///
/// # Panics
/// Panics if adding `input.a` and `input.b` would overflow a `u64`.
///
/// # Examples
///
/// ```
/// let input = Input { a: 2, b: 3 };
/// let out = state_transition(input);
/// assert_eq!(out.sum, 5);
/// ```
#[univm_platform::function(SszGrandineIo)]
pub fn state_transition(input: Input) -> Output {
    println!("Hello, world!");

    Output {
        sum: input.a.checked_add(input.b).unwrap(),
    }
}