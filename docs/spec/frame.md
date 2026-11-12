# Spesifikasi — Format Frame

Frame adalah unit dasar lalu lintas di lapisan link. Format dirancang untuk
parser zero-copy dan fuzz-friendly.

## Struktur frame (M2)

```
 +--------+--------+--------+--------+--------+--------+--------+--------+
 | magic (2) | ver(1) | type(1) |     length (2)    | seq (2)        |
 +--------+--------+--------+--------+--------+--------+--------+--------+
 |                       payload (length bytes)                          |
 +-----------------------------------------------------------------------+
 |                    crc16 (2, little-endian)                           |
 +-----------------------------------------------------------------------+
```

- `magic`: `0x53 0x49` ("SI") — identifikasi frame sig6g.
- `ver`: versi format (saat ini 1).
- `type`: tipe frame (0=data, 1=sync, 2=handshake, 3=disconnect).
- `length`: panjang payload dalam byte (max 65535).
- `seq`: nomor urut frame.
- `payload`: `length` byte, dibaca sebagai `&[u8]` (zero-copy).
- `crc16`: CRC-16-CCITT (poly 0x1021) atas magic+ver+type+length+seq+payload.

## Kontrak parser

- `parse_frame(buf: &[u8]) -> Result<(Frame, usize), FrameError>` — mengembalikan
  frame dan jumlah byte yang dikonsumsi. Tidak menyalin payload (slice `&[u8]`).
- `serialize_frame(frame: &Frame) -> Vec<u8>` — untuk TX.

## Error parser

- `TooShort` — buffer lebih pendek dari header.
- `BadMagic` — magic tidak cocok.
- `BadVersion` — versi tidak didukung.
- `LengthExceeds` — panjang melebihi sisa buffer.
- `BadCrc` — CRC tidak cocok.

## Properti wajib

1. Round-trip: `serialize_frame(parse_frame(buf).0)` == byte asli.
2. Deterministik: input yang sama -> output yang sama.
3. Fuzz-safe: setiap input byte (termasuk invalid) menghasilkan `Err` atau
   `Ok`, tidak pernah panic.
