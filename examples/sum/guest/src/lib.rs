use univm_io::ssz::SszIo;

pub struct Input {}

struct StateTransitionProgramRisc0(Risc0Program<Input, (), SszIo>);

impl GuestProgram<Risc0> for StateTransitionProgramRisc0 {
    fn execute(...) {
        self.0.execute()
    }
}

struct StateTransitionProgram;

trait StateTransitionProgramBuilder<T: Zkvm> {
    fn init(vm: &T) -> impl GuestProgram<T>;
}

impl StateTransitionProgramBuilder for StateTransitionProgram {}

fn state_transition<T: Zkvm>();

#[univm_platform::function(SszIo)]
pub fn state_transition(input: Input) {
    println!("Hello, world!");
}
