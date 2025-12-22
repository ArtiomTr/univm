# UniVM Development Guide

## Project Structure

UniVM is a universal zero-knowledge virtual machine abstraction layer that provides a unified interface for building zk applications across different backends.

### Workspace Organization
```
univm/
├── crates/                    # Core library crates
│   ├── univm-interface/       # Zkvm trait abstraction
│   ├── univm-platform/        # Main platform abstraction layer
│   ├── univm-platform-base/   # Base Platform trait
│   ├── univm-platform-risc0/  # Risc0-specific platform implementation
│   ├── univm-platform-macros/ # Procedural macros for entrypoints
│   ├── univm-io/              # I/O abstraction and serialization
│   └── univm-build/           # Build system configuration
├── examples/                  # Example applications
│   └── sum/                   # Basic sum example with guest/host
└── Cargo.toml                 # Workspace configuration
```

### Key Components

**univm-interface**: Defines the core `Zkvm` trait for host-side backend abstraction
**univm-platform**: Main abstraction layer for guest code with conditional compilation for different zkVMs
**univm-platform-base**: Base `Platform` trait with I/O method definitions for guest environments
**univm-platform-risc0**: Risc0-specific platform implementation for guest code
**univm-io**: Serialization/deserialization abstraction with SSZ support
**univm-build**: Build system integration for different zkVM backends
**univm-platform-macros**: Procedural macros including the `entrypoint` macro

### Platform vs Interface Distinction

**Interface (Host-side)**: 
- Provides abstraction for host applications to interact with different zkVM backends
- Defines `Zkvm` trait for backend selection and configuration
- Used by host code to execute, verify, and manage zkVM operations

**Platform (Guest-side)**:
- Provides abstraction for guest code running inside zkVM environments
- Defines `Platform` trait for I/O operations within the zkVM
- Used by guest programs to read inputs and write outputs in a backend-agnostic manner

## Code Standards

### Safety Requirements
- **No unsafe code**: All code must be written in safe Rust
- **Memory safety**: Rely on Rust's ownership system and borrow checker
- **Thread safety**: Use Rust's concurrency primitives and Send/Sync traits

### Code Quality
- **Error handling**: Use `Result` types for fallible operations
- **Type safety**: Leverage Rust's type system for compile-time guarantees
- **Documentation**: Public APIs must have comprehensive rustdoc comments
- **Testing**: All public functions should have unit tests

### Style Guidelines
- **Edition**: Use Rust 2024 edition features where appropriate
- **Dependencies**: Prefer workspace dependencies over external crates
- **Conditional compilation**: Use `cfg_if` for platform-specific code
- **Trait design**: Keep traits minimal and focused on single responsibilities

### Architecture Principles
- **Abstraction over implementation**: Define traits before concrete types
- **Platform agnosticism**: Support multiple zkVM backends through abstraction
- **Pluggable components**: Allow different serialization formats and I/O methods
- **Macro-driven development**: Use procedural macros to reduce boilerplate

### Development Workflow
- **Feature flags**: Use Cargo features for optional functionality
- **Workspace management**: Maintain consistent versions across workspace
- **Build integration**: Ensure proper integration with zkVM build systems
- **Example driven**: Maintain examples that demonstrate core functionality