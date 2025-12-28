pub mod compiler;

use std::{fs::File, path::Path};

pub trait ZkvmMethods {
    fn name(&self) -> &'static str;
}

pub trait Proof {}

impl Proof for Box<dyn Proof> {}

pub trait ExecutionReport {}

impl ExecutionReport for Box<dyn ExecutionReport> {}

pub trait Zkvm: ZkvmMethods {
    type Proof: Proof;
    type ExecutionReport: ExecutionReport;
}

pub trait GuestProgramBuilder<T: Zkvm> {
    fn init(zkvm: &T) -> impl GuestProgram<T>;
}

pub trait GuestProgram<T: Zkvm> {
    type Input;
    type Output;

    fn execute(&self, input: Self::Input) -> Result<(Self::Output, T::ExecutionReport), ()>;

    fn prove(&self, input: Self::Input)
    -> Result<(Self::Output, T::Proof, T::ExecutionReport), ()>;

    fn verify(&self, proof: &T::Proof) -> bool;
}

pub struct UniVM(Box<dyn ZkvmMethods>);

impl ZkvmMethods for UniVM {
    fn name(&self) -> &'static str {
        self.0.as_ref().name()
    }
}

impl Zkvm for UniVM {
    type Proof = Box<dyn Proof>;

    type ExecutionReport = Box<dyn ExecutionReport>;
}
