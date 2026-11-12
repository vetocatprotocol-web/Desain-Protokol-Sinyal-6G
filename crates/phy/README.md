# sig6g-phy — PHY Layer

`sig6g-phy` menyediakan lapisan fisik untuk framework protokol sinyal 6G:
modulasi QAM bertingkat, OFDM/DFT-s-OFDM, dan model kanal sintetis (AWGN,
Rayleigh). Semua implementasi `no_std`, deterministik per seed, dan bebas
`unsafe`.

## Konten

| Modul | Deskripsi |
|-------|-----------|
| `modulation` | BPSK/QPSK/16-QAM/64-QAM dengan Gray coding; round-trip bit→simbol→bit. |
| `ofdm` | DFT O(N²), IFFT/FFT dengan cyclic prefix; round-trip modulasi/demodulasi. |
| `channel` | Awgn dan Rayleigh, deterministik per seed via `ChaCha20Rng`. |
| `math` | Implementasi `no_std` dari `sin`, `cos`, `sqrt`, `ln`, `exp`, `powf`, `round_i32`, `abs`. |

## Persyaratan

- Rust ≥ 1.82
- `#![no_std]` dan `#![forbid(unsafe_code)]` diterapkan.
- Bergantung pada `sig6g-core`, `rand-chacha`, `rand-core`.

## Build & Tes

```bash
# Di root workspace
cargo build -p sig6g-phy
cargo test -p sig6g-phy
cargo test -p sig6g-phy --doc
cargo bench -p sig6g-phy --bench phy_modem
cargo clippy -p sig6g-phy --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery
```

## Benchmark

Baseline tersimpan di `crates/phy/benches/baselines/phy.json`. CI gagal jika
regresi throughput melebihi 3% vs baseline.

## Properti Wajib

- Deterministik: untuk seed yang sama, output kanal/modulasi bit-identik.
- Round-trip modulasi: demodulasi(modulasi(bits)) == bits (100% untuk semua skema).
- Zero-copy: payload dibaca sebagai `&[u8]` atau slice `&[Complex32]`.

## Lisensi

Dual-licensed di bawah MIT OR Apache-2.0.
