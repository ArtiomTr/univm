pub mod compiler;

use std::marker::PhantomData;

use risc0_zkvm::{Digest, ExecutorEnv, Receipt, SessionInfo, default_executor, default_prover};
use univm_interface::{ExecutionReport, GuestProgram, Proof, Zkvm, ZkvmMethods};
use univm_io::Io;

pub struct Risc0;

impl Risc0 {
    fn new() -> Self {
        Self
    }
}

impl ZkvmMethods for Risc0 {
    fn name(&self) -> &'static str {
        "risc0"
    }
}

impl Zkvm for Risc0 {
    type Proof = Risc0Proof;
    type ExecutionReport = Risc0ExecutionReport;
}

pub struct Risc0Proof(Receipt);

impl Proof for Risc0Proof {}

pub struct Risc0ExecutionReport(SessionInfo);

impl ExecutionReport for Risc0ExecutionReport {}

pub struct Risc0Program<In, Out, Io> {
    elf: Vec<u8>,
    image_id: Digest,
    io: Io,

    _phantom: PhantomData<(In, Out)>,
}

impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> Risc0Program<TInput, TOutput, TIo> {
    fn new(elf: &[u8], image_id: [u32; 8], io: TIo) -> Self {
        Self {
            elf: elf.to_vec(),
            image_id: image_id.into(),
            io,
            _phantom: PhantomData,
        }
    }
}

impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> GuestProgram<Risc0>
    for Risc0Program<TInput, TOutput, TIo>
{
    type Input = TInput;
    type Output = TOutput;

    fn execute(&self, input: Self::Input) -> Result<(Self::Output, Risc0ExecutionReport), ()> {
        let bytes = self.io.serialize(input).unwrap();
        let env = ExecutorEnv::builder().write_slice(&bytes).build().unwrap();
        let executor = default_executor();

        let info = executor.execute(env, &self.elf).unwrap();

        let output = <TIo as Io<Self::Output>>::deserialize(&self.io, &info.journal.bytes).unwrap();
        let report = Risc0ExecutionReport(info);

        Ok((output, report))
    }

    fn prove(
        &self,
        input: Self::Input,
    ) -> Result<(Self::Output, Risc0Proof, Risc0ExecutionReport), ()> {
        let bytes = self.io.serialize(input).unwrap();
        let env = ExecutorEnv::builder().write_slice(&bytes).build().unwrap();
        let prover = default_prover();

        let info = prover.prove(env, &self.elf).unwrap();

        let output =
            <TIo as Io<Self::Output>>::deserialize(&self.io, &info.receipt.journal.bytes).unwrap();
        let proof = Risc0Proof(info.receipt);

        todo!()
        // Ok((output, ))
    }

    fn verify(&self, proof: &Risc0Proof) -> bool {
        proof.0.verify(self.image_id).is_ok()
    }
}

pub fn compiler() -> compiler::Risc0Compiler {
    compiler::Risc0Compiler::default()
}
