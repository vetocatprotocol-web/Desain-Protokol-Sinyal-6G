# Design Decisions (ADR)

Setiap keputusan desain yang menyimpang dari visi awal di AGENTS.md wajib
dicatat di sini sebagai Architecture Decision Record (ADR).

## ADR-000 — Workspace dan Toolchain (M0)

- **Status:** diterima
- **Tanggal:** 2026-11-12
- **Konteks:** Repositori dimulai kosong; diperlukan fondasi workspace Rust.
- **Keputusan:**
  - Workspace Rust dengan crate `core` (no_std), `phy`, `mac`, `link`,
    `sensing`, `channel`, dan `sim` (std).
  - Toolchain stable terkunci di `rust-toolchain.toml` (1.82).
  - Lint: `clippy::pedantic` + `clippy::nursery`, `-D warnings`, rustfmt.
  - `#![forbid(unsafe_code)]` di semua crate kecuali yang ditandai eksplisit.
  - Edisi Rust 2021.
- **Konsekuensi:** Semua crate baru harus masuk ke daftar `members` di
  `Cargo.toml` workspace dan memiliki README.

## ADR-001 — Property Testing dengan proptest (M3 gap fix)

- **Status:** diterima
- **Tanggal:** 2026-11-12
- **Konteks:** AGENTS.md §7 menuntut property test (10.000 kasus per properti
  di CI nightly). Crate `core` adalah `no_std`, sehingga proptest harus
  dikonfigurasi dengan fitur `alloc` + `no_std` dan tanpa default features
  yang menarik `getrandom` 0.4 (butuh edition2024, tidak didukung cargo 1.82).
- **Keputusan:**
  - proptest `=1.9.0` ditambahkan ke dev-dependencies semua crate.
  - `core`: `default-features = false, features = ["alloc", "no_std"]`.
  - phy/link/mac: `default-features = false, features = ["std"]`.
  - getrandom dikunci ke `0.3` agar kompatibel dengan cargo 1.82.
- **Konsekuensi:** Tes properti ditulis sebagai integration test di
  `crates/*/tests/proptest.rs`. `getrandom` 0.4 tidak pernah terpilih.

## ADR-002 — README per crate (M3 gap fix)

- **Status:** diterima
- **Tanggal:** 2026-11-12
- **Konteks:** AGENTS.md §9 Definition of Done menuntut setiap crate memiliki
  README yang menjelaskan cara menjalankan simulasi dan benchmark dari nol.
- **Keputusan:** README ditambahkan untuk `core`, `phy`, `link`, dan `mac`.
- **Konsekuensi:** Setiap crate memiliki `README.md` dengan Quick Start,
  build/test perintah, dan properti wajib.

