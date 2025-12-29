use std::any::Any;

pub mod compiler;

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
    fn init() -> impl GuestProgram<T>;
}

pub trait GuestProgram<T: Zkvm> {
    type Input;
    type Output;

    fn execute(
        &self,
        zkvm: &T,
        input: Self::Input,
    ) -> Result<(Self::Output, T::ExecutionReport), ()>;

    fn prove(
        &self,
        zkvm: &T,
        input: Self::Input,
    ) -> Result<(Self::Output, T::Proof, T::ExecutionReport), ()>;

    fn verify(&self, zkvm: &T, proof: &T::Proof) -> bool;
}

pub struct UniProof(Box<dyn Proof>);

impl UniProof {
    pub fn new(proof: impl Proof + 'static) -> Self {
        Self(Box::new(proof))
    }

    pub fn downcast_ref<T: Proof + 'static>(&self) -> Option<&T> {
        let anyproof: &dyn Any = &self.0;

        anyproof.downcast_ref()
    }
}

impl Proof for UniProof {}

pub struct UniVM(Box<dyn ZkvmMethods>);

impl UniVM {
    pub fn new(zkvm: impl Zkvm + 'static) -> Self {
        Self(Box::new(zkvm))
    }

    pub fn downcast_ref<T: Zkvm + 'static>(&self) -> Option<&T> {
        let anyvm: &dyn Any = &self.0;

        anyvm.downcast_ref()
    }
}

impl ZkvmMethods for UniVM {
    fn name(&self) -> &'static str {
        self.0.as_ref().name()
    }
}

impl Zkvm for UniVM {
    type Proof = UniProof;

    type ExecutionReport = Box<dyn ExecutionReport>;
}
