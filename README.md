# sig6g — 6G Signal Protocol Framework

[![CI](https://github.com/your-org/sig6g/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/sig6g/actions/workflows/ci.yml)
[![Coverage](https://codecov.io/gh/your-org/sig6g/branch/main/graph/badge.svg)](https://codecov.io/gh/your-org/sig6g)
[![MSRV](https://img.shields.io/badge/MSRV-1.82-orange.svg)](https://github.com/your-org/sig6g)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

> **Research prototype** — A from-scratch, full-Rust signal protocol framework for 6G candidate waveforms, designed for simulation, emulation, and formal verification. Not a 3GPP implementation.

---

## Overview

`sig6g` is an independent research project exploring next-generation signal protocols beyond 5G/3GPP. It provides:

- **Typestate-enforced protocol state machines** — Invalid transitions are compile-time errors
- **Zero-copy, allocation-free hot paths** — Contiguous buffers, `no_std` core, deterministic execution
- **Const-generic numerology** — FFT size, subcarriers, slot length known at compile time
- **Composable layer traits** — PHY, MAC, Link, Sensing as independent, replaceable traits
- **First-class ISAC** — Integrated Sensing and Communications using shared pilot structure
- **Reproducible simulation** — Bit-identical output for fixed seeds via `rand_chacha`
- **Formal verification readiness** — Property tests, golden vectors, fuzzing, MIRI, sanitizers in CI

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        sim (std)                                │
│  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────────────────┐ │
│  │   MAC    │ │  Link    │ │ Sensing  │ │    Channel         │ │
│  └────┬─────┘ └────┬─────┘ └────┬─────┘ └─────────┬──────────┘ │
│       │            │            │                  │            │
│       ▼            ▼            ▼                  ▼            │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                      PHY (waveform, modulation, coding)  │  │
│  └──────────────────────────────────────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                     core (no_std)                         │  │
│  │  • Numerology • Buffer/Pool • Error types • Complex32    │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### Crate Structure

| Crate | `std` | Purpose |
|-------|-------|---------|
| `core` | ❌ | Numerology, buffers, error types, `Complex32`, traits |
| `phy` | ❌ | Waveforms (OFDM, DFT-s-OFDM), modulation (QAM/PSK), channel codes (LDPC, Polar) |
| `mac` | ❌ | Scheduling, HARQ, random access, grant-free access |
| `link` | ❌ | Typestate connection state machine (`Idle → Acquiring → Synced → Connected`) |
| `sensing` | ❌ | ISAC: range/angle estimation from communication pilots |
| `channel` | ❌ | Synthetic models: AWGN, Rayleigh, THz LoS/NLoS, clustered delay line |
| `sim` | ✅ | End-to-end simulator, multi-UE scenarios, benchmark harness |

---

## Quick Start

### Prerequisites

- Rust ≥ 1.82 (see `rust-toolchain.toml`)
- `cargo-deny`, `cargo-audit`, `cargo-fuzz`, `cargo-llvm-cov`, `iai-callgrind` (installed by CI)

### Build & Test

```bash
# Format, lint, deny, audit
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D clippy::pedantic -D clippy::nursery
cargo deny check
cargo audit

# Unit + doc tests
cargo test --workspace --all-features --lib --doc

# Integration tests
cargo test --workspace --all-features --tests

# Property tests (proptest)
cargo test --workspace --all-features proptest -- --test-threads=4

# Golden vector tests
cargo test --workspace --all-features golden -- --test-threads=1

# Determinism check
cargo build --release --workspace --all-features -p sim
./target/release/sim --seed 12345 --slots 1000 > run1.bin
./target/release/sim --seed 12345 --slots 1000 > run2.bin
cmp run1.bin run2.bin && echo "Deterministic: OK"
```

### Run Simulation

```bash
# Single UE, AWGN, 1000 slots
cargo run --release -p sim -- --ue 1 --channel awgn --slots 1000 --snr 10

# Multi-UE, Rayleigh fading, ISAC enabled
cargo run --release -p sim -- --ue 10 --channel rayleigh --slots 5000 --isac

# THz LoS channel, 100 UE, high load
cargo run --release -p sim -- --ue 100 --channel thz_los --slots 10000
```

### Benchmarks

```bash
# Criterion/divan benchmarks
cargo bench --workspace --all-features

# iai-callgrind (instruction counts, stable across machines)
cargo iai-callgrind --workspace --all-features
```

---

## CI Gates (All Must Pass)

| Gate | Tool | Threshold |
|------|------|-----------|
| Format/Lint | `rustfmt`, `clippy` (pedantic+nursery) | 0 warnings |
| Dependencies | `cargo-deny`, `cargo-audit` | 0 violations |
| Unit/Integration | `cargo test` | 100% pass |
| Property Tests | `proptest` | 10k cases/property |
| Golden Vectors | Custom | Bit-identical |
| Determinism | Binary compare | Identical hash |
| Fuzzing | `cargo-fuzz` | 0 crashes in 30 min/target |
| UB Detection | `cargo miri` | 0 errors (`core`, `phy`) |
| Sanitizers | ASan + TSan | 0 findings |
| Benchmarks | `iai-callgrind` | ≤3% throughput regression, ≤5% p99 latency |
| Coverage | `cargo-llvm-cov` | Core ≥90% lines, unsafe 100% |
| MSRV | `rust-toolchain@1.75` | All tests pass |
| Documentation | `cargo doc` + doc tests | 0 broken links |

---

## Documentation

- **Specification**: [`docs/spec/`](docs/spec/) — Formal per-layer specs with diagrams
- **Decisions**: [`docs/decisions.md`](docs/decisions.md) — Architecture Decision Records
- **Regulatory**: [`docs/regulatory.md`](docs/regulatory.md) — Spectrum usage notes (ITU, Kominfo)
- **Journal**: [`docs/journal.md`](docs/journal.md) — Work log, hypotheses, results

Generate API docs:

```bash
cargo doc --workspace --all-features --no-deps --open
```

---

## Safety & Legal

> **This is a research prototype for simulation/emulation only.**

- No real-radio transmission, no SDR drivers enabled by default
- Radio HAL exists only as abstract traits with mock/simulator implementations
- No jamming, interception, tracking, or auth bypass functionality
- Spectrum usage subject to local regulation (Indonesia: Kominfo/Komdigi, ITU-R)
- No copyrighted 3GPP specifications copied — references are paraphrased with section numbers

See [`docs/regulatory.md`](docs/regulatory.md) and [`docs/decisions.md`](docs/decisions.md).

---

## Contributing

1. Read [`AGENTS.md`](AGENTS.md) — the project constitution
2. Propose changes via ADR in [`docs/decisions.md`](docs/decisions.md)
3. Write failing test first (property/golden/fuzz)
4. Implement minimal change
5. Run full CI locally: `ci/run-all.sh`
6. Submit PR with Conventional Commits message

---

## Roadmap (Milestones)

| Milestone | Focus | Status |
|-----------|-------|--------|
| **M0** | Foundation: workspace, CI, `core` no_std, numerology, buffers | 🟡 In Progress |
| **M1** | PHY: QAM, OFDM/DFT-s-OFDM, AWGN/Rayleigh, BER vs theory | ⬜ Planned |
| **M2** | Link: Typestate state machine, fuzzed frame parser | ⬜ Planned |
| **M3** | MAC: Multi-UE scheduling, HARQ, grant-free access experiments | ⬜ Planned |
| **M4** | ISAC: Range/angle from shared pilots, synthetic channel validation | ⬜ Planned |
| **M5** | Optimization: Profile, lock targets, complete docs | ⬜ Planned |

Each milestone must satisfy [Definition of Done](AGENTS.md#9-definition-of-done).

---

## License

Dual-licensed under **MIT OR Apache-2.0** at your option.

```
SPDX-License-Identifier: MIT OR Apache-2.0
```

---

## Citation

If you use this work in research, please cite:

```bibtex
@software{sig6g,
  title = {sig6g: 6G Signal Protocol Framework},
  author = {sig6g Contributors},
  year = {2026},
  url = {https://github.com/your-org/sig6g}
}
```

---

## Acknowledgments

- Inspired by academic work on 6G waveforms (OTFS, OTFS-OFDM, GFDM, UFMC)
- Channel models based on 3GPP TR 38.901 / ITU-R M.2412 (paraphrased)
- Rust ecosystem: `num-complex`, `rand_chacha`, `proptest`, `iai-callgrind`, `cargo-fuzz`