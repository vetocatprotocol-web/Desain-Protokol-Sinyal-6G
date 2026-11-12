# Spesifikasi — MAC dan Penjadwalan

Sumber konsep: penjadwalan downlink/uplink per slot, HARQ chase combining,
dan identifikasi UE per slot (parafrase; tidak menyalin teks 3GPP secara utuh).

## Lingkup M3

- Multi-UE: tiap slot dijadwalkan paling banyak satu UE untuk downlink.
- Scheduler round-robin: urutan UE tetap dan deterministik.
- HARQ chase combining: transmisikan ulang paket yang gagal hingga batas
  maksimal, kemudian buang.
- Tidak ada alokasi heap per slot pada jalur panas (buffer slot dari
  `sig6g-core` dialokasikan sekali saat inisialisasi).

## Kontrak `MacScheduler` (trait)

- `schedule(slot: usize, active_ues: &[u16]) -> Option<u16>` — pilih UE yang
  akan melayani pada slot ini; `None` jika tidak ada UE aktif.
- `harq_status(ue: u16) -> HarqState` — status proses HARQ per UE.

## State HARQ (`HarqState`)

| State | Arti |
|-------|------|
| `Idle` | Tidak ada transmisi aktif. |
| `Pending(u8)` | Tunggu ACK/NACK, percobaan ke-`u8`. |
| `Exhausted` | Melebihi batas retransmisi, paket dibuang. |

## Kontrak `HarqProcess` (per UE)

- `transmit(packet: &[u8]) -> HarqResult` — mulai transmisi baru.
- `retransmit() -> HarqResult` — kirim ulang paket terakhir (chase combining).
- `on_ack()` — paket diterima, bersihkan proses.
- `on_nack()` — paket gagal, pindah ke percobaan berikutnya atau `Exhausted`.

## Batasan (hard limits)

- `MAX_HARQ_RETRANSMISSIONS = 3` (maksimal 4 transmisi total per paket).
- `MAX_UE = 64` (jumlah UE terdaftar maksimum).
- Slot scheduling deterministik: untuk `slot` dan daftar UE yang sama, hasil
  `schedule()` selalu sama.

## Properti wajib (property test)

1. Eksklusivitas slot: tiap slot paling banyak satu UE terpilih.
2. Fairness round-robin: setiap UE aktif terpilih paling sedikit sekali dalam
   `N` slot berturut-turut (N = jumlah UE aktif).
3. HARQ batas: paket dibuang setelah `MAX_HARQ_RETRANSMISSIONS + 1` percobaan
   tanpa ACK.
4. Determinisme: `schedule()` dengan input yang sama selalu mengembalikan UE
   yang sama.
5. Zero-copy: payload dibawa sebagai `&[u8]`, tidak disalin pada jalur panas.

## Determinisme

Semua keputusan jadwal adalah fungsi murni dari `(slot, active_ues)`. Tidak
ada RNG pada jalur penjadwalan. HARQ RNG (untuk simulasi ACK/NACK acak)
hanya melalui `rand_chacha` dengan seed eksplisit.
