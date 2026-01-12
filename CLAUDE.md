# UniVM Project Guide

## Overview

UniVM is a unified zkVM abstraction layer. This guide explains how to add a new zkVM backend.

## Adding a New zkVM Backend

To add a new zkVM (e.g., `myzkvm`), you need to create **two crates** and update the **conditional compilation system**.

---

### Step 1: Create the Main Backend Crate (`univm-myzkvm`)

Create `crates/univm-myzkvm/` with the following structure:

```
crates/univm-myzkvm/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── zkvm.rs
    └── compiler.rs
```

#### 1.1 Cargo.toml

```toml
[package]
name = "univm-myzkvm"
version = "0.1.0"
edition.workspace = true

[dependencies]
univm-interface = { workspace = true }
univm-io = { workspace = true }
myzkvm-sdk = { workspace = true }      # Your zkVM's SDK
myzkvm-build = { workspace = true }    # Your zkVM's build tools
```

#### 1.2 src/lib.rs

```rust
use univm_interface::compiler::Compiler;

mod zkvm;
pub use zkvm::*;

mod compiler;

pub fn compiler() -> impl Compiler {
    compiler::MyzkvmCompiler::default()
}
```

#### 1.3 src/zkvm.rs - Implement Core Traits

```rust
use std::marker::PhantomData;
use univm_interface::{ExecutionReport, GuestProgram, Proof, ProvingReport, Zkvm, ZkvmMethods};
use univm_io::Io;

// 1. Main zkVM struct
pub struct Myzkvm {
    // SDK clients, provers, etc.
}

impl Default for Myzkvm {
    fn default() -> Self {
        Self { /* initialize SDK */ }
    }
}

impl ZkvmMethods for Myzkvm {
    fn name(&self) -> &'static str {
        "myzkvm"
    }
}

impl Zkvm for Myzkvm {
    type Proof = MyzkvmProof;
    type ExecutionReport = MyzkvmExecutionReport;
    type ProvingReport = MyzkvmProvingReport;
}

// 2. Proof wrapper - wraps your zkVM's native proof type
pub struct MyzkvmProof(/* native proof type */);

impl Proof for MyzkvmProof {
    fn claim(&self) -> &[u8] {
        // Return the public outputs/journal bytes
    }
}

// 3. Execution report - provides cycle count
pub struct MyzkvmExecutionReport(/* native execution info */);

impl ExecutionReport for MyzkvmExecutionReport {
    fn cycles(&self) -> u64 {
        // Return cycle count from execution
    }
}

// 4. Proving report (can be empty)
pub struct MyzkvmProvingReport;
impl ProvingReport for MyzkvmProvingReport {}

// 5. Program struct - holds ELF and verification data
pub struct MyzkvmProgram<In, Out, TIo: Io<In> + Io<Out>> {
    elf: Vec<u8>,
    // verification_key, image_id, etc.
    io: TIo,
    _phantom: PhantomData<(In, Out)>,
}

impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> MyzkvmProgram<TInput, TOutput, TIo> {
    pub fn new(/* params like elf, keys */) -> Self { ... }
}

// 6. Implement GuestProgram trait
impl<TInput, TOutput, TIo: Io<TInput> + Io<TOutput>> GuestProgram<Myzkvm>
    for MyzkvmProgram<TInput, TOutput, TIo>
{
    type Input = TInput;
    type Output = TOutput;

    fn execute(&self, zkvm: &Myzkvm, input: Self::Input)
        -> Result<(Self::Output, MyzkvmExecutionReport), ()>
    {
        // 1. Serialize input using self.io
        // 2. Call zkVM executor
        // 3. Deserialize output from journal
        // 4. Return (output, report)
    }

    fn prove(&self, zkvm: &Myzkvm, input: Self::Input)
        -> Result<(Self::Output, MyzkvmProof, MyzkvmProvingReport), ()>
    {
        // 1. Serialize input
        // 2. Call zkVM prover
        // 3. Wrap native proof in MyzkvmProof
        // 4. Deserialize output
        // 5. Return (output, proof, report)
    }

    fn verify(&self, zkvm: &Myzkvm, proof: &MyzkvmProof) -> Result<Self::Output, ()> {
        // 1. Verify proof using verification key
        // 2. Deserialize and return output from proof claim
    }
}
```

#### 1.4 src/compiler.rs - Build System Integration

```rust
use std::path::Path;
use univm_interface::compiler::{CompilationResult, Compiler};

#[derive(Default)]
pub struct MyzkvmCompiler;

impl Compiler for MyzkvmCompiler {
    fn compile(&self, crate_path: &Path, target_path: &Path) -> Result<CompilationResult, ()> {
        // 1. Call your zkVM's build system to compile guest code
        // 2. Get path to compiled ELF and any verification keys/image IDs

        let elf_path = /* path to compiled ELF */;
        let verification_data = /* image_id, vk, etc. */;

        Ok(CompilationResult {
            vm_name: "Myzkvm".to_owned(),
            vm_full_name: "univm_myzkvm::Myzkvm".to_owned(),
            program_name: "[<$base_program_name Myzkvm>]".to_owned(),
            program_impl: format!(
                r#"pub struct [<$base_program_name Myzkvm>](univm_myzkvm::MyzkvmProgram<$input, $output, $io>);

                impl [<$base_program_name Myzkvm>] {{
                    fn init(vm: &univm_myzkvm::Myzkvm) -> Self {{
                        const ELF: &[u8] = include_bytes!({elf_path:?});
                        // Include any verification data constants
                        Self(univm_myzkvm::MyzkvmProgram::new(/* params */))
                    }}
                }}

                impl univm_interface::GuestProgram<univm_myzkvm::Myzkvm> for [<$base_program_name Myzkvm>] {{
                    type Input = $input;
                    type Output = $output;

                    fn execute(&self, zkvm: &univm_myzkvm::Myzkvm, input: Self::Input)
                        -> Result<(Self::Output, univm_myzkvm::MyzkvmExecutionReport), ()> {{
                        self.0.execute(zkvm, input)
                    }}

                    fn prove(&self, zkvm: &univm_myzkvm::Myzkvm, input: Self::Input)
                        -> Result<(Self::Output, univm_myzkvm::MyzkvmProof, univm_myzkvm::MyzkvmProvingReport), ()> {{
                        self.0.prove(zkvm, input)
                    }}

                    fn verify(&self, zkvm: &univm_myzkvm::Myzkvm, proof: &univm_myzkvm::MyzkvmProof)
                        -> Result<Self::Output, ()> {{
                        self.0.verify(zkvm, proof)
                    }}
                }}"#
            ),
        })
    }

    fn emit_platform(&self) -> Result<String, ()> {
        Ok(r#"
#[cfg_zkvm(myzkvm)]
#[allow(unused)]
pub type UniVMCurrentPlatform = univm_platform_myzkvm::MyzkvmPlatform;

#[cfg_zkvm(myzkvm)]
#[allow(unused)]
pub use univm_platform_myzkvm::__univm_entrypoint;
"#.to_owned())
    }
}
```

---

### Step 2: Create the Platform Crate (`univm-platform-myzkvm`)

This crate runs **inside the guest VM** and handles I/O.

```
crates/univm-platform-myzkvm/
├── Cargo.toml
└── src/
    └── lib.rs
```

#### 2.1 Cargo.toml

```toml
[package]
name = "univm-platform-myzkvm"
version = "0.1.0"
edition.workspace = true

[dependencies]
univm-platform = { workspace = true }
myzkvm-guest = { workspace = true }  # Guest-side SDK
```

#### 2.2 src/lib.rs

```rust
use myzkvm_guest::io;  // Your zkVM's guest I/O module
use univm_platform::Platform;

pub struct MyzkvmPlatform;

impl Platform for MyzkvmPlatform {
    fn read_input() -> Vec<u8> {
        // Read input bytes from host
        // Some VMs use length-prefixed format, others have direct read_vec()
        io::read_vec()
    }

    fn write_output(bytes: &[u8]) {
        // Commit output bytes (journal/public values)
        io::commit_slice(bytes)
    }
}

// Re-export the entrypoint macro
pub use myzkvm_guest::entrypoint as __univm_entrypoint;
```

---

### Step 3: Update Conditional Compilation

#### 3.1 Update `crates/cfg-zkvm-macro/src/lib.rs`

Add your zkVM to the `ZkvmIdent` enum:

```rust
enum ZkvmIdent {
    Risc0,
    Sp1,
    Pico,
    Ziren,
    Zisk,
    Myzkvm,  // Add this
}
```

Add parsing in `FromStr`:

```rust
("myzkvm", _) => Ok(ZkvmIdent::Myzkvm),
```

Add cfg transformation in `cfg_attr()`:

```rust
Self::Myzkvm => quote!(all(target_os = "zkvm", target_vendor = "myvendor")),
```

#### 3.2 Update `crates/cfg-zkvm/src/lib.rs`

Add cfg check:

```rust
println!("cargo::rustc-check-cfg=cfg(myzkvm, values(none()))");
```

Add detection in the match:

```rust
("zkvm", "myvendor", false) => "myzkvm",
```

---

### Step 4: Register in Workspace

Add to `Cargo.toml` workspace members and dependencies:

```toml
[workspace]
members = [
    # ...
    "crates/univm-myzkvm",
    "crates/univm-platform-myzkvm",
]

[workspace.dependencies]
myzkvm-sdk = "x.y.z"
myzkvm-build = "x.y.z"
myzkvm-guest = "x.y.z"
```

---

### Step 5: Usage

Users can now use your backend:

```rust
// build.rs
fn main() {
    univm_build::new()
        .add_crate("methods")
        .zkvm(univm_myzkvm::compiler())
        .build();
}

// main.rs
let vm = univm_myzkvm::Myzkvm::default();
let program = MyProgram::init(&vm);
let (output, report) = program.execute(&vm, input).unwrap();
```

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `crates/univm-interface/src/lib.rs` | Core traits: `Zkvm`, `GuestProgram`, `Proof`, `ExecutionReport` |
| `crates/univm-interface/src/compiler.rs` | `Compiler` trait and `CompilationResult` |
| `crates/univm-platform/src/lib.rs` | `Platform` trait for guest I/O |
| `crates/cfg-zkvm-macro/src/lib.rs` | `#[cfg_zkvm(...)]` macro implementation |
| `crates/cfg-zkvm/src/lib.rs` | Build-time zkVM detection |
| `crates/univm-build/src/lib.rs` | Build system orchestration |

## Existing Implementations

- **RISC0**: `crates/univm-risc0/`, `crates/univm-platform-risc0/`
- **SP1**: `crates/univm-sp1/`, `crates/univm-platform-sp1/`
