# Spesifikasi — Model Kanal Sintetis

Sumber konsep: AWGN dan Rayleigh fading (parafrase; tidak menyalin teks 3GPP
secara utuh). Referensi bagian ilustratif.

## AWGN

Tambahan noise Gaussian nol-sesetimen dengan variansi `N0/2` per dimensi
(real dan imajiner). SNR dB ditetapkan secara eksplisit, deterministik per seed
(`rand_chacha`).

## Rayleigh

Amplitudo fade kompleks `h = (x + iy) / sqrt(2)` dengan `x, y ~ N(0, 1)`.
Diterapkan per simbol (flat fading), koheren dalam satu simbol, berubah antar
simbol. Gabungan dengan AWGN.

## Kontrak `Channel` (trait)

- `apply(samples: &[Complex32], snr_db: f64, seed: u64) -> Vec<Complex32>`
- Deterministik untuk `(samples, snr_db, seed)` yang sama.

## Kurva BER teoretis (referensi, untuk toleransi)

- BPSK/AWGN: `Pb = Q(sqrt(2 * Eb/N0))`
- QPSK/AWGN: sama dengan BPSK per bit.
- M-QAM: dekatkan dengan ekspresi standar (parafrase).

Toleransi BER simulasi vs teori: selisih dalam `+/- 20%` relatif pada SNR yang
dicatat per skenario (disediakan di `docs/decisions.md` bila perlu disesuaikan).

## Determinisme

RNG hanya melalui `rand_chacha` dengan seed eksplisit. Tidak ada `thread_rng()`
atau entropy sistem pada jalur simulasi.
