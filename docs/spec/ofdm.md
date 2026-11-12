# Spesifikasi — OFDM

Sumber konsep: transformasi Fourier invers/FFT untuk multiplexing subcarrier
(parafrase; tidak menyalin teks 3GPP secara utuh). Referensi bagian ilustratif.

## Kontrak OFDM

- Ukuran FFT `N` diketahui saat kompilasi (const generics, dari `Numerology`).
- IFFT pada TX, FFT pada RX, dengan normalisasi `1/sqrt(N)` agar energi terjaga
  (Parsekal).
- Siklus prefix (CP) diambil dari akhir simbol waktu dan ditempel di awal.
- Pilot terdistribusi (subcarnier tertentu) untuk estimasi kanal (ISAC, M4).

## Alur TX

1. Simbol kompleks per subcarrier dari modulasi.
2. IFFT-N → domain waktu.
3. Tambahkan CP.
4. Keluarkan sampel IQ ke buffer slot.

## Alur RX

1. Terima sampel IQ, hilangkan CP.
2. FFT-N → domain frekuensi.
3. Estimasi kanal dari pilot, samakan (equalize).
4. Demodulasi subcarrier ke bit.

## Properti wajib

1. OFDM round-trip (tanpa kanal/noise): bit keluaran = bit masukan.
2. Energi domain waktu ≈ energi domain frekuensi (within tolerance).
3. CP tidak mempengaruhi data simbol setelah dihilangkan.

## Determinisme

IFFT/FFT implemented secara deterministik (tidak bergantung pada library FFT
ektoral yang non-deterministik pada awalnya; implementasi sederhana O(N^2) DFT
untuk M1, dapat diganti FFT cepat di M5 tanpa mengubah konstanta).
