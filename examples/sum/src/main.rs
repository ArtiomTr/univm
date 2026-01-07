use univm_interface::{GuestProgram, GuestProgramBuilder, UniVM, Zkvm as _, ZkvmMethods as _};
use univm_risc0::Risc0;
use zkvm_guest::{Input, Output, StateTransition, StateTransitionRisc0, StateTransitionSp1};

use univm_sp1::Sp1;

fn main() {
    let vm = Risc0::default();
    let program: StateTransitionRisc0 = StateTransition::init(&vm);
    let (value, report) = program.execute(&vm, Input { a: 1, b: 2 }).unwrap();
    println!("{:?}", value);

    let vm = Sp1::default();
    let program: StateTransitionSp1 = StateTransition::init(&vm);
    let (value, report) = program.execute(&vm, Input { a: 1, b: 2 }).unwrap();
    println!("{:?}", value);
}
