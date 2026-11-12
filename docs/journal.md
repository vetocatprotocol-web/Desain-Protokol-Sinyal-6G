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
- 2026-11-12 | M1 PHY sintetis (selesai) | crates/phy no_std: Modulation
  (BPSK/QPSK/16-QAM/64-QAM Gray, round-trip 100%), Ofdm (DFT O(N^2),
  IFFT/FFT + CP round-trip), Channel (Awgn/Rayleigh deterministik per seed
  via ChaCha20Rng), math no_std (sin/cos/sqrt/ln/exp/powf/round_i32/abs).
  31 tes lulus, clippy pedantic+nursery -D warnings, fmt bersih.
  Benchmark phy_modem + baseline benches/baselines/phy.json; ci/run-all.sh
  gate regresi <=3% hijau.
- 2026-11-12 | M2 Link dan typestate (selesai) | crates/link no_std: state machine
  typestate (Idle -> Acquiring -> Synced -> Connected, transisi ilegal tidak
  terkompilasi), parser frame zero-copy (magic/ver/type/length/seq/payload/crc16,
  CRC-16-CCITT), fuzz target frame_parser. 43 tes lulus (core 12 + link 12 +
  phy 19), clippy pedantic+nursery -D warnings, fmt bersih, semua gerbang
  CI (termasuk benchmark regresi <=3% dengan 3x pengukuran + max) hijau.
  Doc examples (runnable) ditambahkan ke semua item publik M1 (Modulation,
  Ofdm, Awgn, Rayleigh, math) dan M2 (Idle/Acquiring/Synced/Connected,
  Frame, parse_frame, serialize_frame) — divalidasi via cargo test --doc.
- 2026-11-12 | M3 MAC dan penjadwalan (selesai) | crates/mac no_std ditambahkan ke
  workspace: MacScheduler round-robin multi-UE (MAX_UE=64, eksklusivitas slot,
  fairness, deterministik per seed, 0 alokasi heap per slot), HarqProcess chase
  combining (MAX_HARQ_RETRANSMISSIONS=3, state Idle/Pending/Exhausted,
  on_ack/on_nack/retransmit/reset), Error enum spesifik. Spesifikasi
  docs/spec/mac.md ditulis. 8 tes unit + 17 doc test lulus. Semua gerbang
  ci/run-all.sh hijau (fmt, clippy pedantic+nursery -D warnings, test,
  doc-test, benchmark regresi <=3% semua dalam toleransi). Doc example runnable
  ditambahkan ke setiap item publik per AGENTS.md §6. Deny/audit/miri/fuzz
  dilewati (tool tidak terpasang).
- 2026-11-12 | Gap fix M0-M3 (selesai) | Perbaikan gap terhadap Definisi Selesai
  AGENTS.md §9:
  * README per crate ditambahkan (core, phy, link, mac) — wajib §9.
  * proptest =1.9.0 ditambahkan ke dev-dependencies semua crate — wajib §7
    (properti: 10.000 kasus). getrandom dikunci ke 0.3 (cargo 1.82 tidak
    mendukung edition2024 yang dibutuhkan getrandom 0.4).
  * Golden/vektor test di tests/vectors.rs (CRC-16-CCITT, konstelasi QPSK/16-QAM,
    numerologi) — wajib §7.
  * Tes determinisme: awgn_deterministic_same_seed (phy proptest) — wajib §7.
  * decisions.md: fix edisi (2021 bukan 2024), ADR-001 (proptest), ADR-002
    (README per crate).
  Semua gerbang ci/run-all.sh hijau setelah gap fix.
