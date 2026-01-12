use std::any::Any;

pub mod compiler;

pub trait ZkvmMethods {
    fn name(&self) -> &'static str;
}

/// Enum, describing what proof kind
pub enum ProofKind {
    Core,
    Compressed,
    Groth16,
    Plonk,
}

#[auto_impl::auto_impl(&, Box)]
pub trait Proof {
    fn claim(&self) -> &[u8];
}

#[auto_impl::auto_impl(&, Box)]
pub trait ExecutionReport {
    fn cycles(&self) -> u64;
}

#[auto_impl::auto_impl(&, Box)]
pub trait ProvingReport {}

pub trait Zkvm: ZkvmMethods {
    type Proof: Proof;
    type ExecutionReport: ExecutionReport;
    type ProvingReport: ProvingReport;
}

pub trait GuestProgramBuilder<V: Zkvm> {
    type Program: GuestProgram<V>;

    fn init(zkvm: &V) -> Self::Program;
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
    ) -> Result<(Self::Output, T::Proof, T::ProvingReport), ()>;

    #[must_use]
    fn verify(&self, zkvm: &T, proof: &T::Proof) -> Result<Self::Output, ()>;
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

impl Proof for UniProof {
    fn claim(&self) -> &[u8] {
        self.0.claim()
    }
}

pub type UniExecutionReport = Box<dyn ExecutionReport>;

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
    type ProvingReport = Box<dyn ProvingReport>;
}
