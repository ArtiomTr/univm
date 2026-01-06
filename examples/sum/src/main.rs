use univm_interface::{GuestProgram, GuestProgramBuilder, UniVM, Zkvm as _, ZkvmMethods as _};
use univm_risc0::Risc0;
use zkvm_guest::{Input, Output, StateTransition, StateTransitionRisc0};

fn main() {
    let program: StateTransitionRisc0 = StateTransition::init();
    let vm = Risc0::default();
    let (value, report) = program.execute(&vm, Input { value: 0 }).unwrap();
    println!("{:?}", value);
}
