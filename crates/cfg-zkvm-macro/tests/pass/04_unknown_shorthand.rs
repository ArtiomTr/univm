//! This test checks that shorthand syntax for unknown zkvms still pass.
//! This test ensures that any user-defined cfg options still work inside proc
//! macro, even if this proc macro doesn't know anything about them.
//!
//! For example, user may expect that this code fails:
//!
//! ```rust
//! #[cfg_zkvm(airbender)]
//! fn some_airbender_function() {}
//! ```
//!
//! Because this macro doesn't support airbender yet. However, failing in such
//! case will also make this code invalid:
//!
//! ```rust
//! #[cfg_zkvm(all(fuzz, risc0))]
//! fn some_risc0_fuzzing_utility() {}
//! ```
//!
//! Because this macro *also* doesn't know anything about `fuzz` cfg value.
//!
//! So to make this work, macro actually should allow unknown cfg options, and
//! simply ignore them.
//! This test ensures described behavior.

use cfg_zkvm_macro::cfg_zkvm;

#[cfg_zkvm(unkown)]
fn some_fun() {}

fn main() {}
