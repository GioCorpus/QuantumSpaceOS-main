# Quantum Core Documentation

## Introduction

The quantum core (`src/quantum-core`) is a Rust library implementing a quantum computing simulator optimized for spaceflight calculations. It interfaces with Q# when available for high-fidelity simulation.

## Modules

### `qubit.rs` — Qubit and QubitState

```rust
use quantum_core::{Qubit, QubitState};

// Create |0⟩ qubit
let mut q = Qubit::zero();

// Apply Hadamard to create superposition
apply_hadamard(&mut q);

// Measure (collapses state)
let bit = q.measure();
```

**`QubitState`** stores complex amplitudes α and β for basis states |0⟩ and |1⟩. It enforces normalization on creation.

**`Qubit`** includes metadata:
- `coherence_time` — simulated decoherence countdown
- `id` — UUID for tracking

### `gates.rs` — Quantum Gates

Built as `SingleQubitGate` and `TwoQubitGate` structs with `apply` methods. Common gates provided as constructors:

| Gate | Function | Matrix |
|------|----------|--------|
| Pauli-X | `x()` | `[[0,1],[1,0]]` |
| Pauli-Y | `y()` | `[[0,-i],[i,0]]` |
| Pauli-Z | `z()` | `[[1,0],[0,-1]]` |
| Hadamard | `h()` | `1/√2[[1,1],[1,-1]]` |
| Phase (S) | `s()` | `[[1,0],[0,i]]` |
| T (π/8) | `t()` | `[[1,0],[0,e^{iπ/4}]]` |
| CNOT | `cnot()` | controlled-NOT |
| Toffoli | `toffoli()` | CCNOT |

Convenience modules: `single_q` and `multi_q`.

### `algorithms.rs` — Quantum Algorithms

Space-optimized implementations:

- **GroverSearch** — Unstructured search; optimal iterations `≈ π√N/4`
- **QuantumFourierTransform** — Period finding; basis of Shor's
- **ShorAlgorithm** — Integer factorization (stub)
- **VQE** (Variational Quantum Eigensolver) — for orbital Hamiltonian minimization

Example: orbital optimization

```rust
let hamiltonian = OrbitalHamiltonian {
    radial_energy: -13.6,  // eV
    angular_momentum: 0.0,
    potential_well: -27.2,
};
let ground_energy = VQE::new(3).minimize(&hamiltonian);
```

### `state_vector.rs` — Full State Vector

`StateVector` uses `ndarray` for n-qubit simulation. Enabled for algorithms requiring full state evolution.

### `error.rs` — Error Types

`QuantumError` enum covers:
- `InvalidState`
- `GateError`
- `NonUnitaryGate`
- `DecoherenceError`
- `FeatureNotAvailable`

## Integration with Q#

When compiled with `--features qsharp`, the core loads Microsoft's Q# runtime. Set up:

```bash
# Install .NET SDK
wget https://dot.net/v1/dotnet-install.sh -O /tmp/dotnet-install.sh
bash /tmp/dotnet-install.sh --channel 8.0
dotnet new -i Microsoft.Quantum.ProjectTemplates
```

In Rust:

```rust
#[cfg(feature = "qsharp")]
use quantum_core::init_qsharp;

fn main() -> Result<(), QuantumError> {
    init_qsharp()?;
    // Now Q# operations available
    Ok(())
}
```

## Performance Notes

- Single-qubit operations: ~10 ns (release build)
- Two-qubit gates: ~40 ns (controlled-NOT)
- Memory: 16 bytes per qubit (state only); StateVector doubles that

## Future Work

- Support for 3+ qubit interactions
- Noise models (amplitude damping, depolarizing)
- GPU acceleration using ` cust`/`wgpu`
- Integration with actual QPUs via `qsharp` or `qir`

---

See also: `ARCHITECTURE.md`
