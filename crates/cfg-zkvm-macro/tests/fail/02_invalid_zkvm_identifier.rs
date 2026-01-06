//! This test checks error message on invalid zkvm identifier.
//!
//! Unlike `04_unknown_shorthand.rs` case, this test case should actually fail,
//! as cfg `zkvm` option has limited accepted value set, and unknown values
//! should not be ignored.

use cfg_zkvm_macro::cfg_zkvm;

#[cfg_zkvm(zkvm = "dummy")]
fn fun() {}

#[cfg_zkvm(any(feature = "cool", zkvm = "some-cool-zkvm"))]
fn fun2() {}

fn main() {}
