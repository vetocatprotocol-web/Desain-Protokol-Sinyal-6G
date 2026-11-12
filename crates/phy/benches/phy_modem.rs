#![allow(
    clippy::wildcard_imports,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use sig6g_core::Complex32;
use sig6g_phy::{
    channel::{Awgn, Channel},
    modulation::*,
    ofdm::{Ofdm, OfdmImpl},
};
use std::hint::black_box;
use std::time::Instant;

const N: usize = 64;
const CP: usize = 16;
const ITER: usize = 20_000;
const BATCH: usize = 64; // simbol per iterasi untuk mengamortisasi overhead pengukuran

fn main() {
    // 1) Modulasi/demodulasi throughput (simbol per detik).
    // Setiap iterasi memproses BATCH simbol agar ukuran pengukuran stabil.
    let t0 = Instant::now();
    let mut acc: u64 = 0;
    for i in 0..ITER {
        for b in 0..BATCH {
            let bits: [u8; 4] = [
                (((i * BATCH + b) >> 3) & 1) as u8,
                (((i * BATCH + b) >> 2) & 1) as u8,
                (((i * BATCH + b) >> 1) & 1) as u8,
                ((i * BATCH + b) & 1) as u8,
            ];
            let c = Qam16::map(&bits);
            let d = Qam16::demap(c);
            acc = acc.wrapping_add(d as u64);
        }
    }
    let mod_ns = t0.elapsed().as_nanos();
    let total_syms = (ITER * BATCH) as f64;
    let mod_sps = total_syms / (mod_ns as f64 / 1_000_000_000.0);
    black_box(acc);

    // 2) OFDM modulate/demodulate throughput (simbol OFDM per detik).
    let ofdm = OfdmImpl::<N, CP>::new();
    let mut tx = [Complex32::zero(); N + CP];
    let mut rx = [Complex32::zero(); N];
    let t0 = Instant::now();
    let mut ofdm_acc: u64 = 0;
    for i in 0..ITER {
        let symbols: [Complex32; N] = std::array::from_fn(|k| {
            if ((i + k) & 1) == 0 {
                Complex32::new(1, 0)
            } else {
                Complex32::new(-1, 0)
            }
        });
        ofdm.modulate(&symbols, &mut tx).unwrap();
        ofdm.demodulate(&tx, &mut rx).unwrap();
        ofdm_acc = ofdm_acc.wrapping_add(rx[0].re as u64);
    }
    let ofdm_ns = t0.elapsed().as_nanos();
    let ofdm_sps = (ITER as f64) / (ofdm_ns as f64 / 1_000_000_000.0);
    black_box(ofdm_acc);
    black_box(tx);
    black_box(rx);

    // 3) Kanal AWGN throughput (sampel per detik).
    let samples: Vec<Complex32> = std::iter::repeat(Complex32::new(10, 0))
        .take(N + CP)
        .collect();
    let mut out = Vec::with_capacity(samples.len());
    let t0 = Instant::now();
    let mut awgn_acc: u64 = 0;
    for i in 0..ITER {
        out = Awgn
            .apply(&samples, 20.0, 7u64.wrapping_add(i as u64))
            .unwrap();
        awgn_acc = awgn_acc.wrapping_add(out[0].re as u64);
    }
    let awgn_ns = t0.elapsed().as_nanos();
    let awgn_sps = ((ITER * samples.len()) as f64) / (awgn_ns as f64 / 1_000_000_000.0);
    black_box(awgn_acc);
    black_box(out);

    println!("phy_modem bench (iter={ITER}, batch={BATCH}):");
    println!("  qam16_map_demap_symbols_per_sec = {mod_sps:.0}");
    println!("  ofdm_mod_demod_symbols_per_sec  = {ofdm_sps:.0}");
    println!("  awgn_apply_samples_per_sec      = {awgn_sps:.0}");
}
