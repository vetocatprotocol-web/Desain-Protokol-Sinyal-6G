use sig6g_core::Complex32;
use sig6g_phy::{modulation::*, ofdm::OfdmImpl, Channel, Awgn};
use std::time::Instant;

const N: usize = 64;
const CP: usize = 16;
const ITER: usize = 20_000;

fn main() {
    // 1) Modulasi/demodulasi throughput (simbol per detik).
    let bits = [1u8, 0, 1, 1];
    let t0 = Instant::now();
    let mut sink = Complex32::zero();
    for _ in 0..ITER {
        let c = Qam16::map(&bits);
        let _d = Qam16::demap(c);
        sink = c;
    }
    let mod_ns = t0.elapsed().as_nanos();
    let mod_sps = (ITER as f64) / (mod_ns as f64 / 1_000_000_000.0);

    // 2) OFDM modulate/demodulate throughput (simbol OFDM per detik).
    let ofdm = OfdmImpl::<N, CP>::new();
    let symbols: [Complex32; N] = std::array::from_fn(|i| {
        if i % 2 == 0 {
            Complex32::new(1, 0)
        } else {
            Complex32::new(-1, 0)
        }
    });
    let mut tx = [Complex32::zero(); N + CP];
    let mut rx = [Complex32::zero(); N];
    let t0 = Instant::now();
    for _ in 0..ITER {
        ofdm.modulate(&symbols, &mut tx).unwrap();
        ofdm.demodulate(&tx, &mut rx).unwrap();
    }
    let ofdm_ns = t0.elapsed().as_nanos();
    let ofdm_sps = (ITER as f64) / (ofdm_ns as f64 / 1_000_000_000.0);

    // 3) Kanal AWGN throughput (sampel per detik).
    let samples: Vec<Complex32> = std::iter::repeat(Complex32::new(10, 0))
        .take(N + CP)
        .collect();
    let mut out = Vec::with_capacity(samples.len());
    let t0 = Instant::now();
    for _ in 0..ITER {
        out = Awgn.apply(&samples, 20.0, 7).unwrap();
    }
    let awgn_ns = t0.elapsed().as_nanos();
    let awgn_sps = ((ITER * samples.len()) as f64) / (awgn_ns as f64 / 1_000_000_000.0);

    println!("phy_modem bench (iter={ITER}):");
    println!("  qam16_map_demap_symbols_per_sec = {mod_sps:.0}");
    println!("  ofdm_mod_demod_symbols_per_sec  = {ofdm_sps:.0}");
    println!("  awgn_apply_samples_per_sec      = {awgn_sps:.0}");
    println!("  sink = {sink:?}, out_len = {}", out.len());

    // Baseline tulisan: simpan ke benches/baselines/phy.json oleh skrip CI.
    let _ = sink;
    let _ = out;
}
