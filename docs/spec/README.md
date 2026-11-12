# Spesifikasi Formal

Spesifikasi per layer ada di subdirektori ini (Markdown + diagram).

## Status

- `spec/numerology.md` — konstanta numerologi, ukuran FFT, panjang slot (const generics).
- `spec/frame.md` — format frame dan slot.
- `spec/link-state.md` — state machine koneksi (typestate): Idle → Acquiring → Synced → Connected.
- `spec/modulation.md`, `spec/ofdm.md`, `spec/channel.md`, `spec/phy.md` — PHY (M1).
- `spec/mac.md` — MAC dan penjadwalan: multi-UE, round-robin, HARQ chase combining (M3).

Belum ada spesifikasi yang ditetapkan; ditulis bersama dengan implementasi tiap milestone.
