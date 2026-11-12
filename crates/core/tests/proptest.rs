//! Property tests untuk `sig6g-core` (10.000 kasus per properti di CI).
//! Properti: determinisme, konstanta numerologi, buffer round-trip.
//! Catatan: const generics tidak dapat diparametrikan pada runtime, sehingga
//! tes numerologi menggunakan instantiate konkret (Mu0/Mu1/Mu2).

use proptest::prelude::*;

use sig6g_core::numerology::{Mu0, Mu1, Mu2};
use sig6g_core::{buffer::SlotBuffer, Complex32};

proptest! {
    #[test]
    fn buffer_push_then_slice_roundtrip(
        values in proptest::collection::vec(any::<i32>(), 0..8),
    ) {
        let mut buf: SlotBuffer<8> = SlotBuffer::new();
        for &v in &values {
            buf.push(Complex32::new(v, v)).unwrap_or(());
        }
        let slice = buf.as_slice();
        prop_assert_eq!(slice.len(), values.len());
        for (i, &v) in values.iter().enumerate() {
            prop_assert_eq!(slice[i], Complex32::new(v, v));
        }
    }

    #[test]
    fn buffer_clear_resets_len(values in proptest::collection::vec(any::<i32>(), 0..8)) {
        let mut buf: SlotBuffer<8> = SlotBuffer::new();
        for &v in &values {
            buf.push(Complex32::new(v, 0)).unwrap_or(());
        }
        buf.clear();
        prop_assert!(buf.is_empty());
        prop_assert_eq!(buf.len(), 0);
    }
}

#[test]
fn numerology_mu0_constants() {
    assert_eq!(Mu0::SCS_KHZ, 15);
    assert_eq!(Mu0::FFT_SIZE, 256);
    assert_eq!(Mu0::CP_SAMPLES, 16);
    assert_eq!(Mu0::SUBCARRIERS, 200);
    assert_eq!(Mu0::SYMBOL_SAMPLES, 272);
    assert_eq!(Mu0::scs_hz(), 15_000);
    assert_eq!(Mu0::rb_subcarriers(), 12);
}

#[test]
fn numerology_mu1_constants() {
    assert_eq!(Mu1::SCS_KHZ, 30);
    assert_eq!(Mu1::FFT_SIZE, 512);
    assert_eq!(Mu1::CP_SAMPLES, 32);
    assert_eq!(Mu1::SUBCARRIERS, 400);
    assert_eq!(Mu1::SYMBOL_SAMPLES, 544);
    assert_eq!(Mu1::scs_hz(), 30_000);
}

#[test]
fn numerology_mu2_constants() {
    assert_eq!(Mu2::SCS_KHZ, 60);
    assert_eq!(Mu2::FFT_SIZE, 1024);
    assert_eq!(Mu2::CP_SAMPLES, 64);
    assert_eq!(Mu2::SUBCARRIERS, 800);
    assert_eq!(Mu2::SYMBOL_SAMPLES, 1088);
    assert_eq!(Mu2::scs_hz(), 60_000);
}

#[allow(clippy::missing_const_for_fn)]
#[test]
fn numerology_subcarriers_le_fft() {
    const {
        assert!(Mu0::SUBCARRIERS <= Mu0::FFT_SIZE);
    }
    const {
        assert!(Mu1::SUBCARRIERS <= Mu1::FFT_SIZE);
    }
    const {
        assert!(Mu2::SUBCARRIERS <= Mu2::FFT_SIZE);
    }
}
