include!(concat!(env!("OUT_DIR"), "/methods.rs"));

use univm_io::ssz::SszIo;
pub use zkvm_guest_methods_lib::{Input, Output};

impl_program!(Input, Output, SszIo, StateTransition);
