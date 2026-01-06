//! This test ensures that `cfg_zkvm` fails on expressions and statements.
//!
//! Proc macros have [limitations] -- they cannot be used in statements &
//! expressions. This test exists to catch when Rust eventually supports these
//! positions, so we can update the macro accordingly.
//!
//! [limitations]: https://github.com/rust-lang/rust/issues/54727

use cfg_zkvm_macro::cfg_zkvm;

fn main() {
    // Proc macro on a statement (let binding) - should fail
    #[cfg_zkvm(risc0)]
    let x = 42;

    // Proc macro on an expression - should fail
    let y = #[cfg_zkvm(sp1)]
    123;
}
