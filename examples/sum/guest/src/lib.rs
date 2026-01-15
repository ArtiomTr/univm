include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use univm_io::ssz_grandine::SszGrandineIo;
pub use zkvm_guest_methods_lib::{Input, Output};

impl_program!(Input, Output, SszGrandineIo, StateTransition);
