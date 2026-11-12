//! Tipe error untuk `sig6g-link`.
//!
//! Manual enum (no `thiserror`) karena crate ini `no_std`.

use core::fmt;

/// Error yang dihasilkan oleh `sig6g-link`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// Buffer terlalu pendek untuk menyimpan header frame.
    TooShort,
    /// Magic frame tidak cocok (`0x53 0x49`).
    BadMagic,
    /// Versi format frame tidak didukung.
    BadVersion,
    /// Panjang payload melebihi sisa buffer.
    LengthExceeds,
    /// CRC-16 tidak cocok.
    BadCrc,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooShort => f.write_str("buffer too short for frame header"),
            Self::BadMagic => f.write_str("bad frame magic"),
            Self::BadVersion => f.write_str("unsupported frame version"),
            Self::LengthExceeds => f.write_str("frame length exceeds buffer"),
            Self::BadCrc => f.write_str("frame crc mismatch"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_variants() {
        assert_eq!(alloc::format!("{}", Error::BadMagic), "bad frame magic");
        assert_eq!(alloc::format!("{}", Error::BadCrc), "frame crc mismatch");
    }
}
