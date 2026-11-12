//! OFDM: IFFT/FFT dengan cyclic padding, normalisasi 1/sqrt(N).
//!
//! Implementasi DFT O(N^2) deterministik untuk M1; dapat diganti FFT cepat
//! di M5 tanpa mengubah konstanta antarmuka.
//!
//! Untuk kompatibilitas MSRV 1.82 (tidak mendukung `N + CP` dalam tipe array),
//! buffer output disediakan oleh pemanggil lewat slice.

#![allow(clippy::module_name_repetitions)]

use crate::{math, Complex32, Error};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct C64 {
    re: f64,
    im: f64,
}

impl C64 {
    #[must_use]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn from_c32(c: Complex32) -> Self {
        Self {
            re: f64::from(c.re),
            im: f64::from(c.im),
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn to_c32(self) -> Complex32 {
        Complex32::new(math::round_i32(self.re), math::round_i32(self.im))
    }
}

impl core::ops::MulAssign for C64 {
    fn mul_assign(&mut self, o: Self) {
        let re = self.re * o.re - self.im * o.im;
        let im = self.re * o.im + self.im * o.re;
        self.re = re;
        self.im = im;
    }
}

/// Forward DFT (negatif eksponen), normalisasi 1/sqrt(N).
#[allow(clippy::cast_precision_loss)]
fn dft<const N: usize>(input: &[C64; N]) -> [C64; N] {
    let mut output = [C64::new(0.0, 0.0); N];
    let norm = 1.0 / math::sqrt(N as f64);
    let two_pi = 2.0 * core::f64::consts::PI;
    for (k, slot) in output.iter_mut().enumerate() {
        let mut sum = C64::new(0.0, 0.0);
        let kf = k as f64;
        for (n, &sample) in input.iter().enumerate() {
            let ang = -two_pi * kf * (n as f64) / (N as f64);
            let mut prod = sample;
            let w = C64::new(math::cos(ang), math::sin(ang));
            prod *= w;
            sum.re += prod.re;
            sum.im += prod.im;
        }
        *slot = C64::new(sum.re * norm, sum.im * norm);
    }
    output
}

/// Inverse DFT (positif eksponen), normalisasi 1/sqrt(N).
#[allow(clippy::cast_precision_loss)]
fn idft<const N: usize>(input: &[C64; N]) -> [C64; N] {
    let mut output = [C64::new(0.0, 0.0); N];
    let norm = 1.0 / math::sqrt(N as f64);
    let two_pi = 2.0 * core::f64::consts::PI;
    for (k, slot) in output.iter_mut().enumerate() {
        let mut sum = C64::new(0.0, 0.0);
        let kf = k as f64;
        for (n, &sample) in input.iter().enumerate() {
            let ang = two_pi * kf * (n as f64) / (N as f64);
            let mut prod = sample;
            let w = C64::new(math::cos(ang), math::sin(ang));
            prod *= w;
            sum.re += prod.re;
            sum.im += prod.im;
        }
        *slot = C64::new(sum.re * norm, sum.im * norm);
    }
    output
}

/// Trait OFDM untuk ukuran FFT `N` dan siklus prefix `CP`, diketahui kompilasi.
pub trait Ofdm<const N: usize> {
    /// Panjang siklus prefix dalam sampel.
    const CP: usize;

    /// Simbol domain frekuensi -> sampel domain waktu (IFFT + CP).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidOfdmSize`] if `out` is smaller than
    /// `N + Self::CP`.
    ///
    /// `out` harus memiliki kapasitas minimal `N + Self::CP`.
    /// Mengembalikan jumlah sampel yang ditulis ke `out`.
    fn modulate(&self, symbols: &[Complex32; N], out: &mut [Complex32]) -> Result<usize, Error>;

    /// Sampel domain waktu (dengan CP) -> simbol domain frekuensi (hapus CP + FFT).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidOfdmSize`] if `samples` is smaller than
    /// `N + Self::CP`.
    ///
    /// `samples` harus berisi minimal `N + Self::CP` sampel. `out` berukuran `N`.
    /// Mengembalikan jumlah simbol yang ditulis ke `out`.
    fn demodulate(&self, samples: &[Complex32], out: &mut [Complex32; N]) -> Result<usize, Error>;
}

/// Implementasi OFDM konkret.
pub struct OfdmImpl<const N: usize, const CP: usize> {
    _marker: core::marker::PhantomData<*const ()>,
}

impl<const N: usize, const CP: usize> Default for OfdmImpl<N, CP> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const CP: usize> OfdmImpl<N, CP> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
    }
}

impl<const N: usize, const CP: usize> Ofdm<N> for OfdmImpl<N, CP> {
    const CP: usize = CP;

    fn modulate(&self, symbols: &[Complex32; N], out: &mut [Complex32]) -> Result<usize, Error> {
        if out.len() < N + CP {
            return Err(Error::InvalidOfdmSize);
        }
        let mut freq = [C64::new(0.0, 0.0); N];
        for (i, &s) in symbols.iter().enumerate() {
            freq[i] = C64::from_c32(s);
        }
        let time = idft(&freq);
        // CP = last CP samples of time-domain symbol.
        for i in 0..CP {
            out[i] = time[N - CP + i].to_c32();
        }
        for i in 0..N {
            out[CP + i] = time[i].to_c32();
        }
        Ok(N + CP)
    }

    fn demodulate(&self, samples: &[Complex32], out: &mut [Complex32; N]) -> Result<usize, Error> {
        if samples.len() < N + CP {
            return Err(Error::InvalidOfdmSize);
        }
        let mut time = [C64::new(0.0, 0.0); N];
        for i in 0..N {
            time[i] = C64::from_c32(samples[CP + i]);
        }
        let freq = dft(&time);
        for i in 0..N {
            out[i] = freq[i].to_c32();
        }
        Ok(N)
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
    fn ofdm_roundtrip_no_channel() {
        let ofdm = OfdmImpl::<64, 16>::new();
        let symbols: [Complex32; 64] = core::array::from_fn(|i| {
            if i % 2 == 0 {
                Complex32::new(1, 0)
            } else {
                Complex32::new(-1, 0)
            }
        });
        let mut tx = [Complex32::zero(); 80];
        let n = ofdm.modulate(&symbols, &mut tx).unwrap();
        assert_eq!(n, 64 + 16);
        let mut rx = [Complex32::zero(); 64];
        let m = ofdm.demodulate(&tx, &mut rx).unwrap();
        assert_eq!(m, 64);
        assert_eq!(rx, symbols);
    }

    #[test]
    fn ofdm_insufficient_output_errors() {
        let ofdm = OfdmImpl::<64, 16>::new();
        let symbols: [Complex32; 64] = core::array::from_fn(|_i| Complex32::zero());
        let mut out = [Complex32::zero(); 10];
        assert!(matches!(
            ofdm.modulate(&symbols, &mut out),
            Err(Error::InvalidOfdmSize)
        ));
    }

    #[test]
    fn dft_idft_roundtrip() {
        const N: usize = 16;
        let input: [C64; N] = core::array::from_fn(|i| C64::new(i as f64, (i as f64) * 0.5));
        let f = dft(&input);
        let r = idft(&f);
        for i in 0..N {
            assert!((r[i].re - input[i].re).abs() < 1e-6);
            assert!((r[i].im - input[i].im).abs() < 1e-6);
        }
    }
}
