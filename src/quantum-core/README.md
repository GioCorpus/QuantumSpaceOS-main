# Quantum Core

## Overview

The quantum core provides Rust-based quantum state simulation with Q# integration support. It handles qubit lifecycle, gate operations, and quantum algorithms optimized for space applications.

## Components

- `qubit.rs` – Qubit and QubitState definitions
- `gates.rs` – Single-qubit and multi-qubit gate implementations
- `algorithms.rs` – Quantum algorithms for navigation and cryptography
- `state_vector.rs` – Full state vector simulation
- `error.rs` – QuantumError type

## Example

```rust
use quantum_core::{Qubit, gates::*, run_circuit};

let mut q = Qubit::zero();
apply_hadamard(&mut q);
let result = run_circuit(|q| {
    apply_x(q);
    apply_hadamard(q);
}, 1000);
```

## Integration with Q#

When compiled with `--features qsharp`, the core loads Microsoft's Q# runtime for high-fidelity simulation.
