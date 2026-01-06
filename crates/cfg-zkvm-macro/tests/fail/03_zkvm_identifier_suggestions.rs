//! This test checks error message on invalid zkvm identifier.
//!
//! The error message must contain suggestion, if zkvm identifier is close
//! enough to known ones.

use cfg_zkvm_macro::cfg_zkvm;

#[cfg_zkvm(zkvm = "r0")]
fn risc0() {}

#[cfg_zkvm(zkvm = "RISC0")]
fn risc0() {}

#[cfg_zkvm(zkvm = " riSc0   ")]
fn risc0() {}

#[cfg_zkvm(zkvm = "succinct")]
fn sp1() {}

#[cfg_zkvm(zkvm = "SP1")]
fn sp1() {}

#[cfg_zkvm(zkvm = "brevis")]
fn pico() {}

#[cfg_zkvm(zkvm = "brevis-pico")]
fn pico() {}

#[cfg_zkvm(zkvm = "picovm")]
fn pico() {}

#[cfg_zkvm(zkvm = "pico-vm")]
fn pico() {}

#[cfg_zkvm(zkvm = "PICO")]
fn pico() {}

#[cfg_zkvm(zkvm = "zkm")]
fn ziren() {}

#[cfg_zkvm(zkvm = "ZISK")]
fn zisk() {}

fn main() {}
