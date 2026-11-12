# sig6g-mac — MAC Layer

`sig6g-mac` menyediakan penjadwal round-robin multi-UE dan proses HARQ chase
combining untuk framework protokol sinyal 6G.

## Konten

| Modul | Deskripsi |
|-------|-----------|
| `scheduler` | `MacScheduler` — round-robin deterministik, satu UE per slot, MAX_UE=64. |
| `harq` | `HarqProcess` — chase combining, batas 3 retransmisi (4 total). |
| `error` | Tipe error eksplisit (`UeLimitExceeded`, `HarqExhausted`, dst). |

## Persyaratan

- Rust ≥ 1.82
- `#![no_std]` dan `#![forbid(unsafe_code)]` diterapkan.
- Bergantung pada `sig6g-core`, `rand-chacha`, `rand-core`.

## Build & Tes

```bash
# Di root workspace
cargo build -p sig6g-mac
cargo test -p sig6g-mac
cargo test -p sig6g-mac --doc
cargo clippy -p sig6g-mac --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery
```

## Properti Wajib

- Eksklusivitas slot: tiap slot paling banyak satu UE terpilih.
- Fairness round-robin: setiap UE aktif terpilih paling sedikit sekali dalam
  `N` slot berturut-turut (N = jumlah UE aktif).
- HARQ batas: paket dibuang setelah 4 transmisi tanpa ACK.
- Determinisme: `schedule(slot, active_ues)` dengan input yang sama
  selalu mengembalikan UE yang sama.
- Zero-copy: payload dibawa sebagai `&[u8]`, tidak disalin pada jalur panas.

## Lisensi

Dual-licensed di bawah MIT OR Apache-2.0.
