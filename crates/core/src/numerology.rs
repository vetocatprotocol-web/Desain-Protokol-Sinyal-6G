//! Compile-time numerology for the 6G signal protocol.
//!
//! All FFT sizes, subcarrier counts, and slot lengths are known at compile time
//! via const generics. This enables stack allocation and zero heap traffic on
//! the hot path.

use core::fmt;

/// Numerology constants for a numerology index `mu` (subcarrier spacing
/// exponent, cf. 3GPP §5 para. numerology scaling, paraphrased).
///
/// # Type parameters
/// - `SCS_KHZ` — subcarrier spacing in kHz (e.g. 15, 30, 60, 120).
/// - `FFT_SIZE` — FFT/IFFT size (e.g. 256, 512, 1024, 2048).
/// - `CP_SAMPLES` — cyclic prefix length in samples.
/// - `SUBCARRIERS` — number of used subcarriers (<= `FFT_SIZE`).
///
/// # Panics
/// None; all values are compile-time constants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Numerology<
    const SCS_KHZ: u16,
    const FFT_SIZE: usize,
    const CP_SAMPLES: usize,
    const SUBCARRIERS: usize,
>;

impl<
        const SCS_KHZ: u16,
        const FFT_SIZE: usize,
        const CP_SAMPLES: usize,
        const SUBCARRIERS: usize,
    > Numerology<SCS_KHZ, FFT_SIZE, CP_SAMPLES, SUBCARRIERS>
{
    /// Subcarrier spacing in kHz.
    pub const SCS_KHZ: u16 = SCS_KHZ;
    /// FFT/IFFT size.
    pub const FFT_SIZE: usize = FFT_SIZE;
    /// Cyclic prefix length in samples.
    pub const CP_SAMPLES: usize = CP_SAMPLES;
    /// Number of used subcarriers.
    pub const SUBCARRIERS: usize = SUBCARRIERS;
    /// Symbol duration in samples (FFT + CP).
    pub const SYMBOL_SAMPLES: usize = FFT_SIZE + CP_SAMPLES;

    /// Returns subcarrier spacing in Hz.
    #[must_use]
    pub const fn scs_hz() -> u32 {
        SCS_KHZ as u32 * 1_000
    }

    /// Returns the number of data subcarriers per resource block (12, cf.
    /// 3GPP resource block definition, paraphrased).
    #[must_use]
    pub const fn rb_subcarriers() -> usize {
        12
    }
}

/// Numerology μ=0: 15 kHz SCS, 256-point FFT, 16-sample CP, 200 subcarriers.
pub type Mu0 = Numerology<15, 256, 16, 200>;
/// Numerology μ=1: 30 kHz SCS, 512-point FFT, 32-sample CP, 400 subcarriers.
pub type Mu1 = Numerology<30, 512, 32, 400>;
/// Numerology μ=2: 60 kHz SCS, 1024-point FFT, 64-sample CP, 800 subcarriers.
pub type Mu2 = Numerology<60, 1024, 64, 800>;

impl<
        const SCS_KHZ: u16,
        const FFT_SIZE: usize,
        const CP_SAMPLES: usize,
        const SUBCARRIERS: usize,
    > fmt::Display for Numerology<SCS_KHZ, FFT_SIZE, CP_SAMPLES, SUBCARRIERS>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Numerology(scs={SCS_KHZ}kHz, fft={FFT_SIZE}, cp={CP_SAMPLES}, sc={SUBCARRIERS})"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mu0_constants() {
        assert_eq!(Mu0::SCS_KHZ, 15);
        assert_eq!(Mu0::FFT_SIZE, 256);
        assert_eq!(Mu0::CP_SAMPLES, 16);
        assert_eq!(Mu0::SUBCARRIERS, 200);
        assert_eq!(Mu0::SYMBOL_SAMPLES, 272);
        assert_eq!(Mu0::scs_hz(), 15_000);
        assert_eq!(Mu0::rb_subcarriers(), 12);
    }

    #[test]
    fn mu1_constants() {
        assert_eq!(Mu1::SCS_KHZ, 30);
        assert_eq!(Mu1::FFT_SIZE, 512);
        assert_eq!(Mu1::SYMBOL_SAMPLES, 544);
        assert_eq!(Mu1::scs_hz(), 30_000);
    }

    #[test]
    const fn subcarriers_le_fft() {
        const {
            assert!(Mu0::SUBCARRIERS <= Mu0::FFT_SIZE);
        }
        const {
            assert!(Mu2::SUBCARRIERS <= Mu2::FFT_SIZE);
        }
    }

    #[test]
    fn display_mu0() {
        let n = Numerology::<15, 256, 16, 200>;
        assert_eq!(
            alloc::format!("{n}"),
            "Numerology(scs=15kHz, fft=256, cp=16, sc=200)"
        );
    }
}
