//! Kanal sintetis: AWGN dan Rayleigh, deterministik per seed via `ChaCha20Rng`.
//!
//! RNG hanya `rand_chacha` dengan seed eksplisit. Tidak ada `thread_rng()`.

use crate::{math, Complex32, Error};
use alloc::vec::Vec;
use rand_chacha::ChaCha20Rng;
use rand_core::{RngCore, SeedableRng};

/// Trait kanal: terima sampel, keluarkan sampel yang melalui kanal.
///
/// # Errors
///
/// Mengembalikan [`Error::ChannelError`] jika kanal gagal memproses sampel.
#[allow(clippy::missing_errors_doc)]
pub trait Channel {
    fn apply(
        &self,
        samples: &[Complex32],
        snr_db: f64,
        seed: u64,
    ) -> Result<Vec<Complex32>, Error>;
}

/// Bangkitkan 32-byte seed dari u64 eksplisit (deterministik).
fn seed_from_u64(seed: u64) -> [u8; 32] {
    let mut sb = [0u8; 32];
    sb[..8].copy_from_slice(&seed.to_le_bytes());
    sb
}

/// Bilangan seragam [0, 1) dari `ChaCha20Rng`.
#[allow(clippy::cast_precision_loss)]
fn next_f64(rng: &mut ChaCha20Rng) -> f64 {
    let bits = rng.next_u64() >> 11;
    bits as f64 / 9_007_199_254_740_992.0
}

/// Bilangan Gaussian standar N(0,1) via Box-Muller.
#[allow(clippy::while_float)]
fn gaussian(rng: &mut ChaCha20Rng) -> f64 {
    let mut u1 = next_f64(rng);
    while u1 < 1e-12 {
        u1 = next_f64(rng);
    }
    let u2 = next_f64(rng);
    let r = math::sqrt(-2.0 * math::ln(u1));
    let theta = 2.0 * core::f64::consts::PI * u2;
    r * math::cos(theta)
}

/// AWGN: noise Gaussian nol-sesetimen, variansi N0/2 per dimensi.
pub struct Awgn;

impl Channel for Awgn {
    fn apply(
        &self,
        samples: &[Complex32],
        snr_db: f64,
        seed: u64,
    ) -> Result<Vec<Complex32>, Error> {
        let mut rng = ChaCha20Rng::from_seed(seed_from_u64(seed));
        let snr_linear = math::powf(10.0, snr_db / 10.0);
        let sigma = math::sqrt(1.0 / (2.0 * snr_linear));
        let mut out = Vec::with_capacity(samples.len());
        for &s in samples {
            let n1 = gaussian(&mut rng) * sigma;
            let n2 = gaussian(&mut rng) * sigma;
            out.push(Complex32::new(
                math::round_i32(f64::from(s.re) + n1),
                math::round_i32(f64::from(s.im) + n2),
            ));
        }
        Ok(out)
    }
}

/// Rayleigh flat fading + AWGN, koheren per simbol.
pub struct Rayleigh;

impl Channel for Rayleigh {
    fn apply(
        &self,
        samples: &[Complex32],
        snr_db: f64,
        seed: u64,
    ) -> Result<Vec<Complex32>, Error> {
        let mut rng = ChaCha20Rng::from_seed(seed_from_u64(seed));
        let snr_linear = math::powf(10.0, snr_db / 10.0);
        let sigma = math::sqrt(1.0 / (2.0 * snr_linear));
        let sqrt2 = math::sqrt(2.0);
        let mut out = Vec::with_capacity(samples.len());
        for &s in samples {
            let h_re = gaussian(&mut rng) / sqrt2;
            let h_im = gaussian(&mut rng) / sqrt2;
            let y_re = f64::from(s.re) * h_re - f64::from(s.im) * h_im;
            let y_im = f64::from(s.re) * h_im + f64::from(s.im) * h_re;
            let n1 = gaussian(&mut rng) * sigma;
            let n2 = gaussian(&mut rng) * sigma;
            out.push(Complex32::new(
                math::round_i32(y_re + n1),
                math::round_i32(y_im + n2),
            ));
        }
        Ok(out)
    }
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss, clippy::cast_lossless, clippy::cast_possible_truncation, clippy::uninlined_format_args)]
mod tests {
    use super::*;

    #[test]
    fn awgn_deterministic_per_seed() {
        let samples = [Complex32::new(10, 0); 4];
        let a = Awgn.apply(&samples, 20.0, 42).unwrap();
        let b = Awgn.apply(&samples, 20.0, 42).unwrap();
        assert_eq!(a, b);
        // Pada SNR rendah noise bakal terlihat setelah pembulatan, sehingga
        // beda seed harus menghasilkan beda output.
        let c = Awgn.apply(&samples, -10.0, 43).unwrap();
        assert_ne!(a, c);
    }

    #[test]
    fn rayleigh_deterministic_per_seed() {
        let samples = [Complex32::new(10, 0); 4];
        let a = Rayleigh.apply(&samples, 20.0, 42).unwrap();
        let b = Rayleigh.apply(&samples, 20.0, 42).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn awgn_output_len_matches() {
        let samples = [Complex32::new(1, 1); 8];
        let out = Awgn.apply(&samples, 10.0, 7).unwrap();
        assert_eq!(out.len(), 8);
    }
}
