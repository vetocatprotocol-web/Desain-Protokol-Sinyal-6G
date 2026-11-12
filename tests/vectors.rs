//! Golden/vektor referensi untuk tes bit-identik.
//! File-file ini adalah output referensi yang dihasilkan dari implementasi
//! yang diketahui benar. Tes membandingkan output implementasi terhadap
//! vektor ini; perubahan sengaja harus diikuti pembaruan vektor + ADR.

/// Vektor konstelasi QPSK: (bit_pair, (re, im)) dengan Gray coding.
pub const QPSK_CONSTELLATION: [((u8, u8), (i32, i32)); 4] = [
    ((0b00, 0), (1, 1)),
    ((0b01, 0), (-1, 1)),
    ((0b11, 0), (-1, -1)),
    ((0b10, 0), (1, -1)),
];

/// Vektor konstelasi 16-QAM: (nibble, (re, im)) dengan Gray coding.
pub const QAM16_CONSTELLATION: [((u8, u8), (i32, i32)); 16] = [
    ((0b0000, 0), (-3, -3)),
    ((0b0001, 0), (-1, -3)),
    ((0b0011, 0), (1, -3)),
    ((0b0010, 0), (3, -3)),
    ((0b0100, 0), (-3, -1)),
    ((0b0101, 0), (-1, -1)),
    ((0b0111, 0), (1, -1)),
    ((0b0110, 0), (3, -1)),
    ((0b1100, 0), (-3, 1)),
    ((0b1101, 0), (-1, 1)),
    ((0b1111, 0), (1, 1)),
    ((0b1110, 0), (3, 1)),
    ((0b1000, 0), (-3, 3)),
    ((0b1001, 0), (-1, 3)),
    ((0b1011, 0), (1, 3)),
    ((0b1010, 0), (3, 3)),
];

/// Vektor CRC-16-CCITT: (data, expected_crc).
/// Polinomial 0x1021, init 0xFFFF, refin=false, refout=false, xorout=0.
pub const CRC16_CCITT_VECTORS: [(&[u8], u16); 4] = [
    (b"123456789", 0x29B1),
    (b"", 0xFFFF),
    (b"A", 0xBB3D),
    (b"\x01\x02", 0x5B7A),
];

/// Vektor numerologi: (mu, scs_khz, fft_size, cp_samples, subcarriers).
pub const NUMEROLOGY_VECTORS: [(u8, u16, usize, usize, usize); 3] = [
    (0, 15, 256, 16, 200),
    (1, 30, 512, 32, 400),
    (2, 60, 1024, 64, 800),
];
