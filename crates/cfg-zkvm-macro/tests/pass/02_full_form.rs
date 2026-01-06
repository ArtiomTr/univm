//! This test demonstrates usage of `cfg_zkvm` proc macro, with standard, full
//! form syntax.

use cfg_zkvm_macro::cfg_zkvm;

#[cfg_zkvm(zkvm = "risc0")]
pub fn sample_code_for_risc0() {}

#[cfg_zkvm(zkvm = "sp1")]
fn do_something() -> i32 {
    return 1;
}

#[cfg_zkvm(zkvm = "pico")]
mod some_impl {
    // pico implementation
}

#[cfg_zkvm(zkvm = "pico")]
pub use some_impl::*;

#[cfg_zkvm(zkvm = "ziren")]
fn function_for_ziren() {}

#[cfg_zkvm(zkvm = "zisk")]
fn function_for_zisk() {}

#[cfg_zkvm(zkvm = "risc0")]
struct Risc0Data {
    value: u32,
}

#[cfg_zkvm(zkvm = "sp1")]
enum Sp1Operation {
    Read,
    Write,
}

#[cfg_zkvm(zkvm = "pico")]
impl Risc0Data {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg_zkvm(zkvm = "ziren")]
trait ZirenTrait {
    fn execute(&self);
}

#[cfg_zkvm(zkvm = "zisk")]
type ZiskResult = Result<u32, &'static str>;

#[cfg_zkvm(zkvm = "risc0")]
const RISC0_CONSTANT: u32 = 42;

#[cfg_zkvm(zkvm = "sp1")]
static SP1_STATIC: u32 = 100;

fn main() {}
