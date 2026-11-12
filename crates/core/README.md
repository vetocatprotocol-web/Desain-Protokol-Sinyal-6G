# sig6g-core — no_std Foundation

`sig6g-core` adalah fondasi `no_std` untuk framework protokol sinyal 6G.
Tidak bergantung pada `std`; alokasi heap hanya melalui `alloc` di ujung
pipeline, bukan pada jalur panas.

## Konten

| Modul | Deskripsi |
|-------|-----------|
| `numerology` | Konstanta FFT, subcarrier, dan panjang slot melalui const generics. |
| `buffer` | `Complex32`, `FrameBuffer`, `SlotBuffer` — buffer kontigu zero-copy. |
| `error` | Tipe error eksplisit per crate (manual enum, tanpa `thiserror`). |

## Persyaratan

- Rust ≥ 1.82 (`rust-toolchain.toml` mengunci ke 1.82)
- `#![no_std]` dan `#![forbid(unsafe_code)]` diterapkan.

## Build & Tes

```bash
# Di root workspace
cargo build -p sig6g-core
cargo test -p sig6g-core
cargo test -p sig6g-core --doc
cargo clippy -p sig6g-core --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery
```

## Properti Wajib

- Deterministik: untuk input yang sama, output selalu bit-identik.
- Nol alokasi heap per slot pada jalur panas (buffer dialokasikan sekali saat inisialisasi).
- State yang salah tidak bisa dikompilasi (typestate diterapkan di layer atas).

## Lisensi

Dual-licensed di bawah MIT OR Apache-2.0.
