# sig6g-link — Link Layer

`sig6g-link` menyediakan state machine koneksi dengan typestate dan parser
frame zero-copy untuk framework protokol sinyal 6G.

## Konten

| Modul | Deskripsi |
|-------|-----------|
| `state` | Typestate `Idle → Acquiring → Synced → Connected`; transisi ilegal tidak terkompilasi. |
| `frame` | Parser/serialisasi frame zero-copy (magic/ver/type/length/seq/payload/crc16, CRC-16-CCITT). |

## Persyaratan

- Rust ≥ 1.82
- `#![no_std]` dan `#![forbid(unsafe_code)]` diterapkan.
- Bergantung pada `sig6g-core`.

## Build & Tes

```bash
# Di root workspace
cargo build -p sig6g-link
cargo test -p sig6g-link
cargo test -p sig6g-link --doc
cargo clippy -p sig6g-link --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery
```

## Properti Wajib

- Typestate: `Idle.acquire()` → `Acquiring`, `Acquiring.sync()` → `Synced`,
  `Synced.connect()` → `Connected`, `Connected.disconnect()` → `Idle`.
- Transisi ilegal (mis. `Idle.connect()`) menghasilkan error kompilasi.
- Frame: `serialize` lalu `parse` mengembalikan payload yang sama (round-trip bit-identik).
- CRC-16-CCITT: frame dengan CRC rusak ditolak oleh parser.

## Lisensi

Dual-licensed di bawah MIT OR Apache-2.0.
