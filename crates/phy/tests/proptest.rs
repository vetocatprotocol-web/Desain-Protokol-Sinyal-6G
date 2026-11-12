//! Property tests untuk `sig6g-phy` (10.000 kasus per properti di CI).
//! Properti: modulasi round-trip bit-identik, kanal deterministik per seed.

use proptest::prelude::*;

#[allow(unused_imports)]
use sig6g_phy::channel::{Awgn, Channel};
#[allow(unused_imports)]
use sig6g_phy::{Bpsk, Complex32, Modulation, Qam16, Qam64, Qpsk};

fn bit_strategy() -> impl Strategy<Value = u8> {
    any::<u8>().prop_map(|b| b & 1)
}

proptest! {
    #[test]
    fn bpsk_roundtrip_index(b in bit_strategy()) {
        let sym = Bpsk::map(&[b]);
        let idx = Bpsk::demap(sym);
        prop_assert_eq!(idx, b as usize);
    }

    #[test]
    fn qpsk_roundtrip_index(b0 in bit_strategy(), b1 in bit_strategy()) {
        let sym = Qpsk::map(&[b0, b1]);
        let idx = Qpsk::demap(sym);
        let expect = ((b0 as usize) << 1) | (b1 as usize);
        prop_assert_eq!(idx, expect);
    }

    #[test]
    fn qam16_roundtrip_index(
        b0 in bit_strategy(),
        b1 in bit_strategy(),
        b2 in bit_strategy(),
        b3 in bit_strategy(),
    ) {
        let sym = Qam16::map(&[b0, b1, b2, b3]);
        let idx = Qam16::demap(sym);
        let expect =
            ((b0 as usize) << 3) | ((b1 as usize) << 2) | ((b2 as usize) << 1) | (b3 as usize);
        prop_assert_eq!(idx, expect);
    }

    #[test]
    fn qam64_roundtrip_index(
        b0 in bit_strategy(),
        b1 in bit_strategy(),
        b2 in bit_strategy(),
        b3 in bit_strategy(),
        b4 in bit_strategy(),
        b5 in bit_strategy(),
    ) {
        let sym = Qam64::map(&[b0, b1, b2, b3, b4, b5]);
        let idx = Qam64::demap(sym);
        let expect = ((b0 as usize) << 5)
            | ((b1 as usize) << 4)
            | ((b2 as usize) << 3)
            | ((b3 as usize) << 2)
            | ((b4 as usize) << 1)
            | (b5 as usize);
        prop_assert_eq!(idx, expect);
    }

    #[test]
    fn awgn_deterministic_same_seed(
        samples in proptest::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(re, im)| Complex32::new(re, im)),
            1..16,
        ),
        snr in 0.0f64..40.0,
        seed in 1u64..10000,
    ) {
        let awgn = Awgn;
        let out1 = awgn.apply(&samples, snr, seed).unwrap();
        let out2 = awgn.apply(&samples, snr, seed).unwrap();
        prop_assert_eq!(out1.len(), samples.len());
        prop_assert_eq!(out1, out2);
    }

    #[test]
    fn awgn_output_length_matches_input(
        samples in proptest::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(re, im)| Complex32::new(re, im)),
            1..16,
        ),
        snr in 0.0f64..40.0,
        seed in 1u64..10000,
    ) {
        let awgn = Awgn;
        let out = awgn.apply(&samples, snr, seed).unwrap();
        prop_assert_eq!(out.len(), samples.len());
    }
}

#[test]
fn constellation_len_matches_points() {
    assert_eq!(Bpsk::constellation().len(), Bpsk::POINTS);
    assert_eq!(Qpsk::constellation().len(), Qpsk::POINTS);
    assert_eq!(Qam16::constellation().len(), Qam16::POINTS);
    assert_eq!(Qam64::constellation().len(), Qam64::POINTS);
}
