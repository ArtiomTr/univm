use std::any::Any;

trait Proof {
    fn verify(&self) -> bool;
}

trait ExecutionReport {
    fn cycles(&self) -> u64;
}

trait Zkvm: ZkvmMethods {
    type Proof: Proof;
    type ExecutionReport: ExecutionReport;
}

trait ZkvmMethods: Any {
    fn name(&self) -> &'static str;
}

trait GuestProgram<T: Zkvm> {
    type Output;

    fn execute(&self, vm: &T) -> (Self::Output, T::ExecutionReport);

    fn prove(&self, vm: &T) -> (Self::Output, T::Proof, T::ExecutionReport);
}

#[derive(Clone)]
struct Risc0Zkvm;

#[derive(Clone)]
struct Sp1Zkvm;

struct Sp1Proof {
    b: Vec<u8>,
}

impl Proof for Sp1Proof {
    fn verify(&self) -> bool {
        true
    }
}

struct Sp1ExecutionReport {
    c: u64,
}

impl ExecutionReport for Sp1ExecutionReport {
    fn cycles(&self) -> u64 {
        self.c
    }
}

impl ZkvmMethods for Sp1Zkvm {
    fn name(&self) -> &'static str {
        "sp1"
    }
}

impl Zkvm for Sp1Zkvm {
    type Proof = Sp1Proof;
    type ExecutionReport = Sp1ExecutionReport;
}

impl GuestProgram<Sp1Zkvm> for Program {
    type Output = u32;

    fn execute(&self, value: &Sp1Zkvm) -> (Self::Output, Sp1ExecutionReport) {
        return (4, Sp1ExecutionReport { c: 11 });
    }

    fn prove(&self, value: &Sp1Zkvm) -> (Self::Output, Sp1Proof, Sp1ExecutionReport) {
        todo!()
    }
}

struct Risc0Proof {
    bytes: Vec<u8>,
}

impl Proof for Risc0Proof {
    fn verify(&self) -> bool {
        true
    }
}

struct Risc0ExecutionReport {
    cycles: u64,
}

impl ExecutionReport for Risc0ExecutionReport {
    fn cycles(&self) -> u64 {
        self.cycles
    }
}

impl Zkvm for Risc0Zkvm {
    type Proof = Risc0Proof;
    type ExecutionReport = Risc0ExecutionReport;
}

impl ZkvmMethods for Risc0Zkvm {
    fn name(&self) -> &'static str {
        "risc0"
    }
}

struct Program;

mod risc0 {
    use super::{GuestProgram, Program, Risc0ExecutionReport, Risc0Proof, Risc0Zkvm};

    impl GuestProgram<Risc0Zkvm> for Program {
        type Output = u32;

        fn execute(&self, value: &Risc0Zkvm) -> (Self::Output, Risc0ExecutionReport) {
            return (3, Risc0ExecutionReport { cycles: 0 });
        }

        fn prove(&self, value: &Risc0Zkvm) -> (Self::Output, Risc0Proof, Risc0ExecutionReport) {
            todo!()
        }
    }
}

type DynProof = Box<dyn Proof + 'static>;
type DynExecutionReport = Box<dyn ExecutionReport + 'static>;

struct DynZkvm(Box<dyn ZkvmMethods>);

impl ZkvmMethods for DynZkvm {
    fn name(&self) -> &'static str {
        self.0.as_ref().name()
    }
}

impl Zkvm for DynZkvm {
    type Proof = DynProof;
    type ExecutionReport = DynExecutionReport;
}

impl Proof for DynProof {
    fn verify(&self) -> bool {
        self.as_ref().verify()
    }
}

impl ExecutionReport for DynExecutionReport {
    fn cycles(&self) -> u64 {
        self.as_ref().cycles()
    }
}

impl GuestProgram<DynZkvm> for Program {
    type Output = u32;

    fn execute(&self, value: &DynZkvm) -> (Self::Output, DynExecutionReport) {
        let value: &dyn Any = value.0.as_ref();

        if let Some(zkvm) = value.downcast_ref::<Risc0Zkvm>() {
            let (output, report) = self.execute(zkvm);

            (output, Box::new(report))
        } else if let Some(zkvm) = value.downcast_ref::<Sp1Zkvm>() {
            let (output, report) = self.execute(zkvm);

            (output, Box::new(report))
        } else {
            panic!("unknown zkvm")
        }
    }

    fn prove(&self, value: &DynZkvm) -> (Self::Output, DynProof, DynExecutionReport) {
        todo!()
    }
}

fn main() {
    let zkvm: Box<dyn ZkvmMethods> = Box::new(Risc0Zkvm);
    let zkvm = DynZkvm(zkvm);

    let program = Program;

    let (output, report) = program.execute(&zkvm);

    println!("{} {output} {}", zkvm.name(), report.cycles());

    let zkvm: Box<dyn ZkvmMethods> = Box::new(Sp1Zkvm);
    let zkvm = DynZkvm(zkvm);

    let program = Program;

    let (output, report) = program.execute(&zkvm);

    println!("{} {output} {}", zkvm.name(), report.cycles());
}
