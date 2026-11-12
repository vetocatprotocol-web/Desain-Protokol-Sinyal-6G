//! # sig6g-phy
//!
//! PHY layer for the 6G signal protocol framework.
//!
//! Provides:
//! - [`Modulation`] — konstelasi dan pemetaan bit/simbol.
//! - [`Ofdm`] — IFFT/FFT dengan cyclic prefix.
//! - [`Channel`] — model kanal AWGN dan Rayleigh (deterministik per seed).
//!
//! `#![forbid(unsafe_code)]` dan `#![no_std]` dienkode. `std` hanya diperlukan
//! untuk simulasi/bench (crate `sim`), bukan di sini.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

pub mod channel;
pub mod math;
pub mod modulation;
pub mod ofdm;

pub use sig6g_core::Complex32;

use core::fmt;

/// Error PHY.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Error {
    InvalidModulation,
    InvalidOfdmSize,
    ChannelError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModulation => f.write_str("invalid modulation"),
            Self::InvalidOfdmSize => f.write_str("invalid OFDM size"),
            Self::ChannelError => f.write_str("channel error"),
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::uninlined_format_args
)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        assert_eq!(
            alloc::format!("{}", Error::InvalidModulation),
            "invalid modulation"
        );
    }
}
