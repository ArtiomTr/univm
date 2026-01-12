use std::marker::PhantomData;

use sp1_prover::components::CpuProverComponents;
use sp1_sdk::{Prover, ProverClient, SP1ProvingKey, SP1Stdin, SP1VerifyingKey};
use univm_interface::{ExecutionReport, GuestProgram, Proof, ProvingReport, Zkvm, ZkvmMethods};
use univm_io::Io;

pub struct Sp1 {
    prover: Box<dyn Prover<CpuProverComponents>>,
}

impl Default for Sp1 {
    fn default() -> Self {
        Self {
            prover: Box::new(ProverClient::from_env()),
        }
    }
}

impl ZkvmMethods for Sp1 {
    fn name(&self) -> &'static str {
        "sp1"
    }
}

impl Zkvm for Sp1 {
    type Proof = Sp1Proof;
    type ExecutionReport = Sp1ExecutionReport;
    type ProvingReport = Sp1ProvingReport;
}

pub struct Sp1ExecutionReport(sp1_sdk::ExecutionReport);

impl ExecutionReport for Sp1ExecutionReport {
    fn cycles(&self) -> u64 {
        self.0.total_instruction_count()
    }
}

pub struct Sp1ProvingReport {}

impl ProvingReport for Sp1ProvingReport {}

pub struct Sp1Proof(sp1_sdk::SP1ProofWithPublicValues);

impl Proof for Sp1Proof {
    fn claim(&self) -> &[u8] {
        self.0.public_values.as_slice()
    }
}

pub struct Sp1Program<In, Out, TIo: Io<In> + Io<Out>> {
    elf: Vec<u8>,
    io: TIo,
    pk: SP1ProvingKey,
    vk: SP1VerifyingKey,

    _phantom: PhantomData<(In, Out)>,
}

impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> Sp1Program<TInput, TOutput, TIo> {
    pub fn new(vm: &Sp1, elf: &[u8], io: TIo) -> Self {
        let (pk, vk) = vm.prover.setup(elf);

        Self {
            elf: elf.to_vec(),
            io,
            pk,
            vk,
            _phantom: PhantomData,
        }
    }
}

impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> GuestProgram<Sp1>
    for Sp1Program<TInput, TOutput, TIo>
{
    type Input = TInput;
    type Output = TOutput;

    fn execute(
        &self,
        zkvm: &Sp1,
        input: Self::Input,
    ) -> Result<(Self::Output, Sp1ExecutionReport), ()> {
        let bytes = self.io.serialize(input).unwrap();
        let mut stdin = SP1Stdin::new();

        stdin.write_slice(&bytes);

        let (values, report) = zkvm.prover.execute(&self.elf, &stdin).unwrap();

        let output = self.io.deserialize(values.as_slice()).unwrap();
        let report = Sp1ExecutionReport(report);

        Ok((output, report))
    }

    fn prove(
        &self,
        zkvm: &Sp1,
        input: Self::Input,
    ) -> Result<(Self::Output, Sp1Proof, Sp1ProvingReport), ()> {
        let bytes = self.io.serialize(input).unwrap();
        let mut stdin = SP1Stdin::new();
        stdin.write_slice(&bytes);

        let proof = zkvm
            .prover
            .prove(&self.pk, &stdin, sp1_sdk::SP1ProofMode::Core)
            .unwrap();

        let output = self.io.deserialize(proof.public_values.as_slice()).unwrap();

        Ok((output, Sp1Proof(proof), Sp1ProvingReport {}))
    }

    fn verify(&self, zkvm: &Sp1, proof: &Sp1Proof) -> Result<Self::Output, ()> {
        zkvm.prover.verify(&proof.0, &self.vk).map_err(|_| ())?;

        let values = proof.0.public_values.as_slice();
        self.io.deserialize(values).map_err(|_| ())
    }
}
