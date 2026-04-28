# Contributing to QuantumSpaceOS

Thank you for your interest! Please read this guide before contributing.

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Accept responsibility and apologize for mistakes

## How to Contribute

### Reporting Bugs

Open an issue with:
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, etc.)
- Logs or screenshots

### Submitting Features

1. Open an issue to discuss the feature
2. Fork the repository
3. Create a feature branch (`git checkout -b feature/foo`)
4. Write code + unit tests
5. Ensure `cargo fmt`, `cargo clippy`, and `cargo test` pass
6. Commit and push
7. Open a Pull Request

### Development Workflow

#### Prerequisites

Follow `scripts/setup_dev.sh` to install:
- Rust toolchain
- .NET SDK (optional Q#)
- Python venv
- System packages

#### Testing

```bash
# Rust crates
cargo test --all

# Python API
source .venv/bin/activate
pytest src/api/tests/

# Integration tests (requires QEMU)
./scripts/run_qemu.sh
# Inside VM:
sudo flight-sim --mode orbital
```

#### Linting

```bash
cargo fmt --all
cargo clippy -- -D warnings
pre-commit run --all-files
```

#### Building ISO Locally

```bash
# Linux
sudo ./scripts/build_iso.sh

# Windows (WSL2)
.\scripts\build_iso.ps1
```

## Project Structure

See README.md for full tree. Key paths:

- `src/quantum-core/` — core quantum library (crate)
- `src/photonicq-bridge/` — photonic communication layer
- `src/flight-sim/` — flight dynamics binary
- `src/wayland-gui/` — Wayland GUI binary
- `src/api/` — FastAPI Python service
- `scripts/` — build and run helpers
- `config/` — Arch ISO configs
- `docs/` — detailed docs

## Design Principles

1. **Minimalism** — avoid bloat; favor small, composable components
2. **Reliability** — fail safely; prefer correct over fast
3. **Quantum-readiness** — design interfaces that could map to real QPUs
4. **Space-hardened thinking** — anticipate radiation errors, power limits
5. **Open source** — MIT license; all contributions licensed similarly

## Review Process

Pull requests are reviewed by maintainers. Expect:

- Build verification (CI will run `cargo build`, `pytest`, etc.)
- Code style compliance (rustfmt + clippy)
- Test coverage requirement (>70% for new code)
- Documentation updates when public API changes

## Questions?

Open an issue or discussion thread. We're friendly!

---

*Ad astra per aspera.*
