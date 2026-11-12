# Regulatory — Penggunaan Spektrum

Protokol ini adalah **simulasi, emulasi, dan prototipe riset** untuk sinyal
generasi berikutnya (kandidat 6G). Kode dalam repositori ini **tidak mengirim
sinyal ke spektrum nyata**, dan tidak menyediakan fungsi jamming, intersepensi,
pelacakan perangkat milik orang lain, atau pembobolan autentikasi.

Setiap modul radio nyata (SDR/HAL) hanya boleh berupa *trait* abstrak dengan
implementasi mock/simulator. Tidak ada driver perangkat keras yang aktif
secara default.

Penggunaan spektrum radio di dunia nyata tunduk pada regulator setempat:

- **Indonesia:** Kementerian Komunikasi dan Informatika (Kominfo/Komdigi) serta
  ketentuan ITU yang berlaku.
- **Internasional:** ITU-R Radio Regulations dan batasan spektrum yang
  ditetapkan per wilayah.

Sebelum melakukan uji lapangan dengan perangkat keras, pengguna wajib memperoleh
izin dan alokasi frekuensi yang sesuai dari regulator terkait.
