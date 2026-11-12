# Spesifikasi — State Machine Koneksi (Typestate)

State koneksi dienkode di sistem tipe Rust. Transisi hanya melalui metode
yang mengembalikan tipe state baru; state ilegal tidak bisa dikompilasi.

## State (typestate)

| State | Arti | Transisi yang diizinkan |
|-------|------|-------------------------|
| `Idle` | Tidak ada koneksi. | `Idle -> Acquiring` (mulai akuisisi) |
| `Acquiring` | Mencari sinkronisasi frame/pilot. | `Acquiring -> Synced` (sinkron ditemukan) |
| `Synced` | Sinkronisasi frame telah tercapai. | `Synced -> Connected` (handshake selesai) |
| `Connected` | Koneksi siap lalu lintas data. | `Connected -> Idle` (putus koneksi) |

## Kontrak `LinkState` (trait)

Setiap state adalah tipe unit yang mengimplementasikan `LinkState` dengan
metode transisi:

- `acquire(self) -> Acquiring` — hanya dari `Idle`.
- `sync(self) -> Synced` — hanya dari `Acquiring`.
- `connect(self) -> Connected` — hanya dari `Synced`.
- `disconnect(self) -> Idle` — hanya dari `Connected`.

## Properti wajib (property test / compile-fail)

1. Tidak ada metode transisi yang salah state (mis. `connect()` dari `Idle`
   tidak ada) — tidak terkompilasi.
2. `Idle -> Acquiring -> Synced -> Connected -> Idle` lengkap dan deterministik.
3. Setiap state mengekspos `state_id() -> u8` untuk debugging (tidak dipakai
   dalam logika transisi).

## Determinisme

Semua transisi adalah fungsi murni tanpa RNG. Tidak ada state tersembunyi
selain tipe itu sendiri.
