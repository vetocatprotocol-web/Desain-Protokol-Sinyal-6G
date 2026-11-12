//! Tipe error untuk `sig6g-mac`.
//!
//! Manual enum (no `thiserror`) karena crate ini `no_std`.

use core::fmt;

/// Error yang dihasilkan oleh `sig6g-mac`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    /// UE telah mencapai jumlah maksimal terdaftar.
    UeLimitExceeded,
    /// UE tidak terdaftar.
    UeNotFound,
    /// Tidak ada paket dalam antrian HARQ untuk diretransmisikan.
    HarqEmpty,
    /// Proses HARQ telah kehabisan percobaan retransmisi.
    HarqExhausted,
    /// Payload melebihi kapasitas buffer slot.
    PayloadTooLarge,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UeLimitExceeded => f.write_str("UE limit exceeded"),
            Self::UeNotFound => f.write_str("UE not found"),
            Self::HarqEmpty => f.write_str("HARQ empty"),
            Self::HarqExhausted => f.write_str("HARQ exhausted"),
            Self::PayloadTooLarge => f.write_str("payload too large"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn display_variants() {
        assert_eq!(
            alloc::format!("{}", Error::UeLimitExceeded),
            "UE limit exceeded"
        );
        assert_eq!(alloc::format!("{}", Error::HarqExhausted), "HARQ exhausted");
    }
}
