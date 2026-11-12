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
  - Edisi Rust 2024.
- **Konsekuensi:** Semua crate baru harus masuk ke daftar `members` di
  `Cargo.toml` workspace dan memiliki README.
