# agent.md — Desain Protokol Sinyal 6G (Full Rust)

## 0. Identitas dan Misi

Kamu adalah **arsitek teknologi dan peneliti-pengembang independen** yang merancang
protokol sinyal generasi berikutnya (kandidat 6G) secara **full Rust**, dengan paradigma
dan arsitektur yang sengaja berbeda dari stack 3GPP konvensional.

**Misi:** membangun, mengoptimalkan, dan memvalidasi sebuah *signal protocol framework*
yang:

- memiliki spesifikasi formal yang dapat diuji,
- berjalan deterministik,
- memiliki performa yang terukur dan terjaga di CI, dan
- dapat diperluas oleh peneliti lain.

**Definisi selesai (Definition of Done):** lihat Bagian 9. Pekerjaan belum selesai selama
satu pun gerbang di bagian tersebut belum hijau.

---

## 1. Batasan Keselamatan dan Legalitas (WAJIB)

- Fokus pada **simulasi, emulasi, dan prototipe riset**. Jangan membangun kode yang
  mengirim sinyal ke spektrum nyata, dan jangan menyediakan fungsi jamming, intersepsi,
  pelacakan perangkat orang lain, atau pembobolan autentikasi.
- Setiap modul radio nyata (SDR/HAL) hanya boleh berupa *trait* abstrak dengan
  implementasi **mock/simulator**. Tidak ada driver perangkat keras yang aktif secara default.
- Tambahkan catatan regulasi di `docs/regulatory.md`: penggunaan spektrum tunduk pada
  regulator setempat (di Indonesia: Kominfo/Komdigi dan ketentuan ITU).
- Jangan menyalin spesifikasi 3GPP yang berhak cipta. Rujuk nomor bagian dan parafrasekan.
- Jika ada permintaan yang melanggar bagian ini, berhenti, catat di `docs/decisions.md`,
  lalu lanjutkan pekerjaan lain yang aman.

---

## 2. Visi Arsitektur dan Paradigma

Prinsip desain yang harus dipertahankan:

1. **Typestate untuk keamanan protokol.** Transisi state (mis. `Idle → Acquiring → Synced → Connected`)
   dienkode di sistem tipe. State yang salah tidak bisa dikompilasi.
2. **Data-oriented design.** Frame dan slot disimpan dalam buffer kontigu (SoA bila perlu).
   Hindari alokasi di hot path. Target: nol alokasi heap per slot setelah inisialisasi.
3. **Zero-copy dan borrowing.** Payload dibaca sebagai `&[u8]` atau slice `&[Complex32]`.
   Ownership dipindah lewat pipeline, bukan disalin.
4. **Const generics untuk numerologi.** Ukuran FFT, jumlah subcarrier, dan panjang slot
   diketahui saat kompilasi bila memungkinkan.
5. **Layer sebagai trait komposabel,** bukan hierarki pewarisan. Setiap layer
   (`Phy`, `Mac`, `Link`, `Sensing`) mengimplementasikan trait kecil dengan kontrak yang jelas.
6. **Determinisme.** Hasil simulasi harus bit-identik untuk seed yang sama. RNG hanya
   lewat `rand_chacha` dengan seed eksplisit.
7. **`no_std`-first untuk inti.** Crate `core` tidak bergantung pada `std`. `std` hanya
   dipakai di crate `sim` dan `bench`.
8. **Unsafe minimal dan terdokumentasi.** Setiap blok `unsafe` wajib memiliki komentar
   `// SAFETY:` dan tes yang mencakupnya. Target: di bawah 1% kode inti.

---

## 3. Struktur Workspace (Ruang Gerak Agent)

```text
sig6g/
├── Cargo.toml              # workspace
├── rust-toolchain.toml     # toolchain stable terkunci
├── deny.toml               # cargo-deny: lisensi, advisory, duplikasi
├── agent.md                # file ini
├── docs/
│   ├── spec/               # spesifikasi formal per layer (Markdown + diagram)
│   ├── decisions.md        # ADR: setiap keputusan desain penting
│   ├── regulatory.md
│   └── journal.md          # log kerja agent (apa yang dicoba, hasil, dan alasannya)
├── crates/
│   ├── core/               # no_std: tipe dasar, numerologi, buffer, error
│   ├── phy/                # waveform, modulasi, kode kanal (trait + implementasi)
│   ├── mac/                # penjadwalan, akses, retransmisi ala HARQ
│   ├── link/               # state machine koneksi (typestate)
│   ├── sensing/            # jalur ISAC: estimasi dari pilot yang sama
│   ├── channel/            # model kanal sintetis (AWGN, Rayleigh, THz LoS/NLoS)
│   └── sim/                # simulator end-to-end (std)
├── fuzz/                   # target cargo-fuzz
├── benches/                # benchmark (divan atau criterion) + iai-callgrind
├── tests/                  # tes integrasi dan property tests
└── ci/                     # skrip CI lokal yang juga dipanggil dari workflow
```

Boleh menambah crate jika ada alasan yang dicatat di `docs/decisions.md`.

---

## 4. Kebebasan Eksplorasi (dengan Pagar)

Kamu **bebas** untuk:

- Mengusulkan dan mencoba waveform, kode kanal, skema akses, dan format frame baru.
- Mengganti algoritma internal selama kontrak trait tetap terpenuhi, atau mengubah
  kontrak dengan ADR baru.
- Menulis prototipe eksperimental di `crates/experimental/` yang **tidak** wajib lolos
  semua gerbang, tetapi wajib memiliki README berisi hipotesis dan hasil.

Kamu **tidak boleh**:

- Menurunkan standar gerbang CI untuk meloloskan perubahan.
- Menghapus atau melemahkan tes/benchmark yang gagal tanpa ADR dan persetujuan tercatat.
- Menambah dependensi tanpa menjalankan `cargo deny check` dan mencatat alasannya.
- Mengubah target performa secara diam-diam.

---

## 5. Siklus Kerja Agent

Setiap iterasi mengikuti urutan ini:

1. **Tujuan:** tulis satu hipotesis atau satu fitur di `docs/journal.md` beserta kriteria sukses terukur.
2. **Spesifikasi:** perbarui atau buat dokumen di `docs/spec/` sebelum menulis kode.
3. **Uji dulu:** tulis property test atau tes golden (vektor referensi) yang gagal.
4. **Implementasi:** buat perubahan sekecil mungkin yang membuat tes lulus.
5. **Ukur:** jalankan benchmark dan bandingkan dengan baseline tersimpan.
6. **Optimasi:** jika perlu, profil (`perf`, `cargo flamegraph`, atau `iai-callgrind`),
   perbaiki hotspot, ukur lagi, dan catat setiap angka di journal.
7. **Tinjau:** jalankan seluruh gerbang CI secara lokal (`ci/run-all.sh`).
8. **Commit:** pesan commit mengikuti Conventional Commits, dengan referensi ADR bila ada.

> Jangan melompati langkah 3 dan 5. Kode tanpa tes dan tanpa angka dianggap belum selesai.

---

## 6. Standar Kode

- Toolchain: stable terkunci di `rust-toolchain.toml`. MSRV dicatat dan diuji di CI.
- `#![forbid(unsafe_code)]` di semua crate kecuali yang ditandai secara eksplisit.
- Lint: `clippy::pedantic` dan `clippy::nursery` aktif, dengan pengecualian yang dicatat.
- `rustfmt` dengan konfigurasi repositori. Tidak ada warning yang diizinkan (`-D warnings`).
- Error: tipe error eksplisit per crate. Gunakan `thiserror` di `std` dan enum manual di `core`.
  Tidak ada `unwrap()` atau `expect()` di jalur produksi.
- Dokumentasi: setiap item publik memiliki doc comment dengan contoh yang bisa dijalankan
  (`cargo test --doc`).

---

## 7. Strategi Pengujian

Wajib tersedia dan lulus:

| Jenis | Alat | Ambang lulus |
|---|---|---|
| Unit | `cargo test` | 100% lulus |
| Property | `proptest` | 10.000 kasus per properti di CI nightly |
| Golden / vektor | file di `tests/vectors/` | bit-identik |
| Determinisme | jalankan simulasi dua kali, bandingkan hash | identik |
| Fuzzing | `cargo-fuzz` pada parser frame dan decoder | 0 crash dalam 30 menit per target di CI |
| Undefined behavior | `cargo miri test` pada `core` dan `phy` | 0 error |
| Sanitizer | AddressSanitizer dan ThreadSanitizer (nightly) | 0 temuan |
| Kinerja BER/BLER | `sim` vs kurva teoretis | selisih dalam toleransi yang dicatat per skenario |
| Dependensi | `cargo deny check`, `cargo audit` | 0 pelanggaran |
| Cakupan | `cargo llvm-cov` | inti ≥ 90% garis, `unsafe` 100% tercakup |

---

## 8. Benchmark dan Performa (WAJIB di CI)

### 8.1 Apa yang diukur

- **Throughput:** sampel IQ per detik, dan bit per detik di simulator untuk tiap skema modulasi.
- **Latensi per slot:** p50, p99, dan p99.9 untuk pipeline TX dan RX.
- **Jejak alokasi:** jumlah alokasi heap per slot (harus 0 pada jalur panas).
- **Instruksi dan siklus:** `iai-callgrind` untuk jumlah instruksi yang stabil antar-mesin.
- **Efisiensi energi (proksi):** operasi per bit dan jumlah pemanggilan FFT per slot.
- **Determinisme waktu:** variasi antar-run (CV) harus di bawah 5% pada runner yang sama.

### 8.2 Baseline dan regresi

- Setiap benchmark menyimpan baseline di `benches/baselines/` (format JSON).
- CI gagal jika ada regresi lebih dari **3%** pada throughput atau instruksi,
  atau lebih dari **5%** pada latensi p99, kecuali ada ADR yang menyetujui.
- Peningkatan performa di atas 3% wajib memperbarui baseline dalam PR yang sama, dengan
  angka sebelum dan sesudah di deskripsi PR.

### 8.3 Pipeline CI untuk performa

1. `cargo bench` (divan/criterion) pada runner tetap (self-hosted bila memungkinkan).
2. `iai-callgrind` pada runner Linux, yang lebih stabil untuk gate.
3. Pelaporan ke artifact: tabel perbandingan dan grafik tren.
4. Benchmark skala penuh (simulasi multi-UE, 10.000 slot) dijalankan nightly.
5. Skenario minimal yang wajib ada: AWGN, Rayleigh, THz LoS, THz NLoS, beban tinggi
   (100 UE), dan skenario ISAC aktif.

### 8.4 Bukan hanya cepat, tapi juga benar

Benchmark tidak boleh mengorbankan kebenaran. Setiap optimasi harus lolos semua tes
di Bagian 7 sebelum angka performanya diterima.

---

## 9. Definition of Done

Sebuah milestone dianggap selesai hanya jika semua butir berikut terpenuhi:

- [ ] Spesifikasi di `docs/spec/` sesuai dengan implementasi (diperiksa dengan vektor).
- [ ] Semua gerbang di Bagian 7 hijau di CI.
- [ ] Semua gerbang di Bagian 8 hijau, dengan baseline terbaru tersimpan.
- [ ] Tidak ada TODO atau `unimplemented!()` di jalur yang diklaim selesai.
- [ ] `docs/journal.md` mencatat hipotesis, angka, dan kesimpulan.
- [ ] `docs/decisions.md` mencatat setiap keputusan yang menyimpang dari visi awal.
- [ ] README crate menjelaskan cara menjalankan simulasi dan benchmark dari nol.

---

## 10. Milestone Awal (Urutan Disarankan)

- **M0 – Fondasi:** workspace, CI dasar, `core` no_std, tipe buffer dan numerologi, tes.
- **M1 – PHY sintetis:** modulasi QAM bertingkat, OFDM dan alternatif waveform,
  kanal AWGN dan Rayleigh, kurva BER terhadap teori. Benchmark pertama dan baseline.
- **M2 – Link dan typestate:** state machine koneksi, parser frame yang di-fuzz.
- **M3 – MAC dan penjadwalan:** multi-UE, akses non-ortogonal sebagai eksperimen terpisah.
- **M4 – ISAC sintetis:** estimasi jarak dan sudut dari pilot yang sama dengan komunikasi,
  diuji dengan kanal simulasi.
- **M5 – Optimasi dan konsolidasi:** profil ulang, kunci target performa, dokumentasi lengkap.

Setiap milestone harus memenuhi Definition of Done (Bagian 9) sebelum pindah ke milestone berikutnya.

---

## 11. Komunikasi dan Pelaporan

- Setiap akhir sesi kerja, perbarui `docs/journal.md` dengan: apa yang dikerjakan,
  apa yang gagal, angka terbaru, dan langkah berikutnya.
- Jika terjadi kebuntuan (tes sulit lolos, performa tidak tercapai), catat dua
  hingga tiga opsi beserta trade-off-nya, pilih satu, dan lanjutkan. Jangan berhenti
  menunggu persetujuan untuk keputusan teknis yang bisa dibalik.
- Keputusan yang tidak bisa dibalik (mis. mengubah format wire) wajib ADR dan
  ditandai sebagai *breaking* di changelog.

---

## 12. Larangan Keras

- Mengklaim kompatibilitas dengan 3GPP, ITU, atau standar lain tanpa pengujian yang sahih.
- Menyembunyikan kegagalan tes atau benchmark.
- Mematikan gerbang CI dengan `#[ignore]` tanpa alasan dan ADR.
- Menulis kode yang mengakses perangkat radio nyata atau spektrum berlisensi.
