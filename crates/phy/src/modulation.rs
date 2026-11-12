//! Modulasi: konstelasi dan pemetaan bit/simbol (Gray).
//!
//! Skema: BPSK, QPSK, 16-QAM, 64-QAM. Semua fungsi murni dan deterministik.
//! Pemetaan Gray: indeks biner `v` dikodekan sebagai `v ^ (v >> 1)`.

use crate::Complex32;

/// Trait modulasi. Setiap skema menentukan konstelasi tetap saat kompilasi.
pub trait Modulation {
    /// Jumlah titik konstelasi.
    const POINTS: usize;
    /// Bit per simbol.
    const BITS_PER_SYMBOL: usize;

    /// Peta `bits` (panjang `BITS_PER_SYMBOL`, masing-masing 0/1, MSB dulu)
    /// ke titik konstelasi.
    fn map(bits: &[u8]) -> Complex32;

    /// Temukan indeks konstelasi terdekat ke `soft` (kembalikan indeks biner).
    fn demap(soft: Complex32) -> usize;

    /// Tabel referensi semua titik konstelasi (urutan indeks biner).
    fn constellation() -> &'static [Complex32];
}

/// Konversi indeks biner ke Gray: `v ^ (v >> 1)`.
#[inline]
const fn bin_to_gray(v: usize) -> usize {
    v ^ (v >> 1)
}

/// Konversi Gray ke indeks biner (invers).
#[inline]
const fn gray_to_bin(mut g: usize) -> usize {
    let mut mask = g >> 1;
    while mask != 0 {
        g ^= mask;
        mask >>= 1;
    }
    g
}

/// Cari indeks terdekat ke `val` dalam `coords`.
fn nearest_coord(val: i32, coords: &[i32]) -> usize {
    let mut best = 0usize;
    let mut best_dist: i64 = i64::MAX;
    for (i, &c) in coords.iter().enumerate() {
        let d = (i64::from(val) - i64::from(c)).abs();
        if d < best_dist {
            best_dist = d;
            best = i;
        }
    }
    best
}

/// Baca `count` bit dari `bits` mulai pada `offset` (MSB dulu) sebagai nilai biner.
fn bits_to_val(bits: &[u8], offset: usize, count: usize) -> usize {
    let mut v: usize = 0;
    for k in 0..count {
        v = (v << 1) | (bits[offset + k] as usize & 1);
    }
    v
}

/// Tulis `count` bit LSB-dari-`val` ke `bits` mulai pada `offset` (MSB dulu).
fn val_to_bits(val: usize, bits: &mut [u8], offset: usize, count: usize) {
    #[allow(clippy::cast_possible_truncation)]
    for k in 0..count {
        let shift = count - 1 - k;
        bits[offset + k] = ((val >> shift) & 1) as u8;
    }
}

macro_rules! impl_modulation {
    ($type:ident, $points:expr, $bps:expr, $constel:expr) => {
        impl Modulation for $type {
            const POINTS: usize = $points;
            const BITS_PER_SYMBOL: usize = $bps;

            fn map(bits: &[u8]) -> Complex32 {
                debug_assert_eq!(bits.len(), $bps);
                let mut idx: usize = 0;
                for &b in bits.iter().take($bps) {
                    idx = (idx << 1) | (b as usize & 1);
                }
                Self::constellation()[idx]
            }

            fn demap(soft: Complex32) -> usize {
                let mut best: usize = 0;
                let mut best_dist: i64 = i64::MAX;
                for (i, &c) in Self::constellation().iter().enumerate() {
                    let dr = i64::from(soft.re) - i64::from(c.re);
                    let di = i64::from(soft.im) - i64::from(c.im);
                    let dist = dr * dr + di * di;
                    if dist < best_dist {
                        best_dist = dist;
                        best = i;
                    }
                }
                best
            }

            fn constellation() -> &'static [Complex32] {
                &$constel
            }
        }
    };
}

/// BPSK: 2 titik, 1 bit/simbol.
///
/// # Example
///
/// ```
/// use sig6g_phy::{Bpsk, Complex32, Modulation};
///
/// let c = Bpsk::map(&[1]);
/// assert_eq!(c, Complex32::new(1, 0));
/// assert_eq!(Bpsk::demap(c), 1);
/// ```
pub struct Bpsk;
const BPSK: [Complex32; 2] = [
    Complex32::new(-1, 0), // 0
    Complex32::new(1, 0),  // 1
];
impl_modulation!(Bpsk, 2, 1, BPSK);

/// QPSK: 4 titik, 2 bit/simbol (Gray).
///
/// # Example
///
/// ```
/// use sig6g_phy::{Qpsk, Complex32, Modulation};
///
/// let c = Qpsk::map(&[1, 0]); // binary 10
/// assert_eq!(c, Complex32::new(1, -1));
/// assert_eq!(Qpsk::demap(c), 0b10);
/// ```
pub struct Qpsk;
const QPSK: [Complex32; 4] = [
    Complex32::new(-1, -1), // binary 00
    Complex32::new(-1, 1),  // binary 01
    Complex32::new(1, -1),  // binary 10
    Complex32::new(1, 1),   // binary 11
];
impl_modulation!(Qpsk, 4, 2, QPSK);

/// 16-QAM: 16 titik, 4 bit/simbol (Gray per sumbu).
///
/// # Example
///
/// ```
/// use sig6g_phy::{Qam16, Complex32, Modulation};
///
/// // binary 1010 -> simbol 10
/// let c = Qam16::map(&[1, 0, 1, 0]);
/// assert_eq!(Qam16::demap(c), 0b1010);
/// assert_eq!(Qam16::constellation().len(), Qam16::POINTS);
/// ```
pub struct Qam16;
const QAM16_COORDS: [i32; 4] = [-3, -1, 1, 3];

impl Modulation for Qam16 {
    const POINTS: usize = 16;
    const BITS_PER_SYMBOL: usize = 4;

    fn map(bits: &[u8]) -> Complex32 {
        debug_assert_eq!(bits.len(), 4);
        let vr = bits_to_val(bits, 0, 2);
        let vi = bits_to_val(bits, 2, 2);
        let gr = bin_to_gray(vr);
        let gi = bin_to_gray(vi);
        let c = QAM16_COORDS;
        Complex32::new(c[gr], c[gi])
    }

    fn demap(soft: Complex32) -> usize {
        let c = QAM16_COORDS;
        let gr = nearest_coord(soft.re, &c);
        let gi = nearest_coord(soft.im, &c);
        let vr = gray_to_bin(gr);
        let vi = gray_to_bin(gi);
        let mut bits = [0u8; 4];
        val_to_bits(vr, &mut bits, 0, 2);
        val_to_bits(vi, &mut bits, 2, 2);
        (bits[0] as usize) << 3
            | (bits[1] as usize) << 2
            | (bits[2] as usize) << 1
            | (bits[3] as usize)
    }

    fn constellation() -> &'static [Complex32] {
        static CONST: [Complex32; 16] = [
            Complex32::new(-3, -3),
            Complex32::new(-3, -1),
            Complex32::new(-3, 1),
            Complex32::new(-3, 3),
            Complex32::new(-1, -3),
            Complex32::new(-1, -1),
            Complex32::new(-1, 1),
            Complex32::new(-1, 3),
            Complex32::new(1, -3),
            Complex32::new(1, -1),
            Complex32::new(1, 1),
            Complex32::new(1, 3),
            Complex32::new(3, -3),
            Complex32::new(3, -1),
            Complex32::new(3, 1),
            Complex32::new(3, 3),
        ];
        &CONST
    }
}

/// 64-QAM: 64 titik, 6 bit/simbol (Gray per sumbu).
///
/// # Example
///
/// ```
/// use sig6g_phy::{Qam64, Complex32, Modulation};
///
/// let c = Qam64::map(&[1, 0, 0, 0, 1, 1]); // binary 100011 = 35
/// assert_eq!(Qam64::demap(c), 0b100011);
/// assert_eq!(Qam64::POINTS, 64);
/// ```
pub struct Qam64;
const QAM64_COORDS: [i32; 8] = [-7, -5, -3, -1, 1, 3, 5, 7];

impl Modulation for Qam64 {
    const POINTS: usize = 64;
    const BITS_PER_SYMBOL: usize = 6;

    fn map(bits: &[u8]) -> Complex32 {
        debug_assert_eq!(bits.len(), 6);
        let vr = bits_to_val(bits, 0, 3);
        let vi = bits_to_val(bits, 3, 3);
        let gr = bin_to_gray(vr);
        let gi = bin_to_gray(vi);
        let c = QAM64_COORDS;
        Complex32::new(c[gr], c[gi])
    }

    fn demap(soft: Complex32) -> usize {
        let c = QAM64_COORDS;
        let gr = nearest_coord(soft.re, &c);
        let gi = nearest_coord(soft.im, &c);
        let vr = gray_to_bin(gr);
        let vi = gray_to_bin(gi);
        let mut bits = [0u8; 6];
        val_to_bits(vr, &mut bits, 0, 3);
        val_to_bits(vi, &mut bits, 3, 3);
        ((bits[0] as usize) << 5)
            | ((bits[1] as usize) << 4)
            | ((bits[2] as usize) << 3)
            | ((bits[3] as usize) << 2)
            | ((bits[4] as usize) << 1)
            | (bits[5] as usize)
    }

    fn constellation() -> &'static [Complex32] {
        static CONST: [Complex32; 64] = make_qam64_const();
        &CONST
    }
}

const fn make_qam64_const() -> [Complex32; 64] {
    let mut arr = [Complex32::zero(); 64];
    let mut i = 0;
    while i < 8 {
        let mut j = 0;
        while j < 8 {
            arr[i * 8 + j] = Complex32::new(QAM64_COORDS[i], QAM64_COORDS[j]);
            j += 1;
        }
        i += 1;
    }
    arr
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
    fn bpsk_map_demap_roundtrip() {
        for &b in &[0u8, 1] {
            let c = Bpsk::map(&[b]);
            let d = Bpsk::demap(c);
            assert_eq!(d, b as usize);
        }
    }

    #[test]
    fn qpsk_map_demap_roundtrip() {
        for b0 in 0..2 {
            for b1 in 0..2 {
                let c = Qpsk::map(&[b0, b1]);
                let d = Qpsk::demap(c);
                let expect = ((b0 as usize) << 1) | (b1 as usize);
                assert_eq!(d, expect, "qpsk bits {} {}", b0, b1);
            }
        }
    }

    #[test]
    fn qam16_map_demap_roundtrip() {
        for idx in 0..16u16 {
            let bits = [
                ((idx >> 3) & 1) as u8,
                ((idx >> 2) & 1) as u8,
                ((idx >> 1) & 1) as u8,
                (idx & 1) as u8,
            ];
            let c = Qam16::map(&bits);
            let d = Qam16::demap(c);
            assert_eq!(d, idx as usize, "qam16 idx {}", idx);
        }
    }

    #[test]
    fn qam64_map_demap_roundtrip() {
        for idx in 0..64u16 {
            let bits = [
                ((idx >> 5) & 1) as u8,
                ((idx >> 4) & 1) as u8,
                ((idx >> 3) & 1) as u8,
                ((idx >> 2) & 1) as u8,
                ((idx >> 1) & 1) as u8,
                (idx & 1) as u8,
            ];
            let c = Qam64::map(&bits);
            let d = Qam64::demap(c);
            assert_eq!(d, idx as usize, "qam64 idx {}", idx);
        }
    }

    #[test]
    fn constellation_len_matches_points() {
        assert_eq!(Bpsk::constellation().len(), Bpsk::POINTS);
        assert_eq!(Qpsk::constellation().len(), Qpsk::POINTS);
        assert_eq!(Qam16::constellation().len(), Qam16::POINTS);
        assert_eq!(Qam64::constellation().len(), Qam64::POINTS);
    }

    #[test]
    fn gray_formula_check() {
        // 2-bit: binary 0,1,2,3 -> gray 0,1,3,2
        assert_eq!(bin_to_gray(0), 0);
        assert_eq!(bin_to_gray(1), 1);
        assert_eq!(bin_to_gray(2), 3);
        assert_eq!(bin_to_gray(3), 2);
        // invers
        assert_eq!(gray_to_bin(0), 0);
        assert_eq!(gray_to_bin(1), 1);
        assert_eq!(gray_to_bin(3), 2);
        assert_eq!(gray_to_bin(2), 3);
    }

    #[test]
    fn no_overlapping_points() {
        for c in Qam16::constellation() {
            for d in Qam16::constellation() {
                if c != d {
                    let dr = c.re as i64 - d.re as i64;
                    let di = c.im as i64 - d.im as i64;
                    assert!(dr * dr + di * di > 0);
                }
            }
        }
    }
}
