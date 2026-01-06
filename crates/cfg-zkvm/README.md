# cfg-zkvm

A Rust crate for detecting zkVM targets at compile time. Write portable code that compiles for multiple zkVM backends.

## Supported zkVMs

- `risc0` - RISC Zero
- `sp1` - Succinct SP1
- `pico` - Brevis Pico
- `ziren` - ZKM Ziren
- `zisk` - Zisk

## Usage

### Build Script (Recommended)

The recommended way to use `cfg-zkvm` is via a build script, which enables standard `#[cfg(...)]` attributes and `cfg!()` macros.

1. Add `cfg-zkvm` as a build dependency in your `Cargo.toml`:

```toml
[build-dependencies]
cfg-zkvm = "0.1"
```

2. Create a `build.rs` file:

```rust
fn main() {
    cfg_zkvm::config_values();
}
```

3. Use standard `#[cfg]` attributes in your code:

```rust
#[cfg(risc0)]
fn risc0_specific() {
    // RISC Zero specific code
}

#[cfg(zkvm = "sp1")]
fn sp1_specific() {
    // SP1 specific code
}

// Works with cfg! macro too
fn example() {
    if cfg!(pico) {
        println!("Running on Pico");
    }

    if cfg!(zkvm = "ziren") {
        println!("Running on Ziren");
    }
}

#[cfg(any(risc0, sp1, pico))]
fn multiple_zkvms() {
    // Code for multiple zkVMs
}

#[cfg(all(zisk, feature = "advanced"))]
fn zisk_advanced() {
    // Zisk with advanced feature
}
```

### Proc Macro

For cases where adding a build script is not desired or not possible, you can use the `#[cfg_zkvm]` attribute macro.

Add to your `Cargo.toml`:

```toml
[dependencies]
cfg-zkvm = "0.1"
```

Then use the attribute in your code:

```rust
use cfg_zkvm::cfg_zkvm;

#[cfg_zkvm(risc0)]
fn risc0_specific() {
    // RISC Zero specific code
}

#[cfg_zkvm(zkvm = "sp1")]
fn sp1_specific() {
    // SP1 specific code
}

#[cfg_zkvm(any(risc0, sp1))]
fn risc0_or_sp1() {
    // Code for either RISC Zero or SP1
}

#[cfg_zkvm(all(feature = "my-feature", pico))]
fn pico_with_feature() {
    // Pico with feature enabled
}
```

## Syntax Reference

| Syntax     | Example                                 | Description                         |
| ---------- | --------------------------------------- | ----------------------------------- |
| Identifier | `risc0`, `sp1`, `pico`, `ziren`, `zisk` | Direct zkVM name                    |
| Key-value  | `zkvm = "risc0"`                        | Explicit zkvm key with string value |
| `any()`    | `any(risc0, sp1)`                       | Match any of the listed zkVMs       |
| `all()`    | `all(risc0, feature = "x")`             | Match all conditions                |

## License

See the repository root for license information.
