# Spesifikasi PHY — Ringkasan

- `crates/phy` (no_std-first): trait `Modulation`, `Ofdm`, `Channel`.
- Numerologi diambil dari `sig6g-core` (const generics).
- Zero-copy di hot path: slice `&[Complex32]` dibawa melalui pipeline.
- RNG: `rand_chacha` dengan seed eksplisit (sim/std saja; core tetap no_std).

Kontrak lintas lapis:
1. Bits → simbol kompleks (modulasi).
2. Simbol → sampel OFDM waktu (IFFT + CP).
3. Sampel → kanal (AWGN/Rayleigh).
4. Sampel → simbol frekuensi (FFT + CP removal + equalisasi).
5. Simbol → bits (demodulasi).

Semua langkah deterministik per seed.
