# Spesifikasi — Modulasi

Sumber konsep: konstelasi dan pemetaan bit-simbol diubah dari referensi umum
(parafrase; tidak menyalin teks 3GPP secara utuh). Referensi bagian digunakan
secara ilustratif saja.

## Skema yang didukung (M1)

| Skema | Titik konstelasi | Bit/simbol |
|-------|------------------|------------|
| BPSK  | 2                | 1          |
| QPSK  | 4                | 2          |
| 16-QAM| 16               | 4          |
| 64-QAM| 64               | 6          |

## Kontrak `Modulation` (trait)

Setiap skema mengimplementasikan:

- `const POINTS: usize` — jumlah titik konstelasi.
- `const BITS_PER_SYMBOL: usize` — bit per simbol.
- `map(bits: &[u8]) -> Complex32` — pemetaan bit ke titik konstelasi (Gray).
- `demap(soft: Complex32) -> usize` — indeks konstelasi terdekat.
- `constellation() -> &'static [Complex32]` — tabel referensi titik.

## Properti wajib (property test)

1. Round-trip: `demap(map(bits))` konsisten untuk semua bit yang valid.
2. Simetrisi konstelasi: titik berpasang saling conjugasi untuk skema genap.
3. Jarak minimum antar titik > 0 (tidak ada titik tumpang tindih).
4. Energi rata-rata per simbol terukur dan stabil (dapat dibandingkan teoretis).

## Determinisme

Semua pemetaan/pemetaan invers adalah fungsi murni tanpa RNG. Hasil bit-identik
per input yang sama.
