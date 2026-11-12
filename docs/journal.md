# Journal — Log Kerja Agent

Format: `YYYY-MM-DD | topik | ringkasan | angka/keterangan`

- 2026-11-12 | M0 Fondasi | Inisialisasi workspace Rust, toolchain stable 1.82, deny.toml,
  docs/ (regulatory, decisions, journal), crates/core no_std, CI run-all.sh.
  Semua gerbang CI hijau: fmt --check, clippy pedantic+nursery (-D warnings),
  cargo test --workspace --all-targets --doc (12 tes lulus). Commit b7e075c.
- 2026-11-12 | M1 PHY sintetis (mulai) | Hipotesis: modulasi QAM bertingkat +
  OFDM over AWGN/Rayleigh menghasilkan kurva BER mendekati teori dalam
  toleransi tercatat. Kriteria: tes golden BER bit-identik, deterministik
  per seed, 0 alokasi heap per simbol pada jalur panas, benchmark baseline
  disimpan di benches/baselines/.
