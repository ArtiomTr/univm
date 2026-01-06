//! This test demonstrates usage of `cfg_zkvm` proc macro with function calls.

use cfg_zkvm_macro::cfg_zkvm;

#[cfg_zkvm(any(risc0, sp1))]
fn available_on_risc0_or_sp1() {}

#[cfg_zkvm(any(zkvm = "risc0", zkvm = "pico"))]
fn available_on_risc0_or_pico() {}

#[cfg_zkvm(any(sp1, zkvm = "ziren"))]
fn available_on_sp1_or_ziren() {}

#[cfg_zkvm(all(risc0, feature = "special"))]
fn risc0_with_special_feature() {}

#[cfg_zkvm(all(zkvm = "sp1", feature = "experimental"))]
fn sp1_with_experimental_feature() {}

#[cfg_zkvm(not(risc0))]
fn not_on_risc0() {}

#[cfg_zkvm(not(zkvm = "pico"))]
fn not_on_pico() {}

#[cfg_zkvm(all(feature = "std", any(risc0, sp1, pico)))]
fn std_feature_on_multiple_zkvms() {}

#[cfg_zkvm(any(all(risc0, feature = "foo"), all(sp1, feature = "bar")))]
fn risc0_foo_or_sp1_bar() {}

#[cfg_zkvm(all(not(zisk), any(zkvm = "risc0", zkvm = "ziren")))]
fn not_zisk_but_risc0_or_ziren() {}

#[cfg_zkvm(any(risc0, sp1, pico, ziren, zisk))]
fn available_on_any_zkvm() {}

#[cfg_zkvm(all(
    feature = "default",
    any(
        all(risc0, not(feature = "no-risc0")),
        all(zkvm = "sp1", feature = "sp1-extras")
    )
))]
fn complex_conditional_availability() {}

#[cfg_zkvm(any(risc0, pico))]
struct ZkvmSpecificData {
    value: u32,
}

#[cfg_zkvm(all(sp1, feature = "enums"))]
enum Sp1Operation {
    Read,
    Write,
}

#[cfg_zkvm(any(ziren, zisk))]
impl ZkvmSpecificData {
    fn new(value: u32) -> Self {
        Self { value }
    }
}

#[cfg_zkvm(all(any(risc0, sp1), not(feature = "no-module")))]
mod zkvm_module {
    pub fn inner_function() {}
}

#[cfg_zkvm(any(pico, ziren))]
pub use zkvm_module::inner_function;

#[cfg_zkvm(all(risc0, feature = "traits"))]
trait ZkvmTrait {
    fn execute(&self);
}

#[cfg_zkvm(any(sp1, zkvm = "zisk"))]
type ZkvmResult = Result<u32, &'static str>;

#[cfg_zkvm(all(any(risc0, pico), feature = "consts"))]
const ZKVM_CONSTANT: u32 = 42;

#[cfg_zkvm(not(any(ziren, zisk)))]
static ZKVM_STATIC: u32 = 100;

fn main() {}
