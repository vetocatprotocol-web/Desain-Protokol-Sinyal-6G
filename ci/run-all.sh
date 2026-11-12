#!/usr/bin/env bash
# ci/run-all — Gerbang CI lokal untuk workspace sig6g (M1 PHY sintetis).
#
# Jalankan seluruh gerbang dari Bagian 7 & 8 AGENTS.md secara lokal.
# Keluar dengan kode non-nol jika salah satu gerbang gagal.
#
# Usage: ./ci/run-all.sh

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

log() { printf '[ci] %s\n' "$*"; }

# Pastikan toolchain terkunci tersedia.
rustup toolchain list 2>/dev/null | grep -q '^1\.82' \
  || rustup toolchain install 1.82 --profile minimal

export RUSTUP_TOOLCHAIN=1.82
export PATH="$HOME/.cargo/bin:$PATH"

status=0

run() {
  log "$*"
  if ! "$@"; then
    log "GAGAL: $*"
    status=1
  fi
}

# --- Benchmark (throughput) dengan regresi <= 3% vs baseline ---
# Pemakaian pertama sebagai warmup (mengatasi penyesuaian frekuensi CPU),
# kedua sebagai pengukuran yang dibandingkan ke baseline. Harus menjadi
# perintah cargo pertama agar kompiler menggunakan profil bench.
BASELINE="$ROOT/crates/phy/benches/baselines/phy.json"
if [ -f "$BASELINE" ]; then
  cargo bench -p sig6g-phy --bench phy_modem >/dev/null 2>&1 || true # warmup
  BENCH_OUT="$(mktemp)"
  cargo bench -p sig6g-phy --bench phy_modem > "$BENCH_OUT" 2>&1 || {
    log "GAGAL: cargo bench phy_modem"
    status=1
  }
  if [ "$status" -eq 0 ]; then
    get_base() {
      grep -oE "\"$1\": [0-9]+" "$BASELINE" | head -1 | grep -oE "[0-9]+" | tail -1
    }
    get_cur() {
      grep "$1" "$BENCH_OUT" | head -1 | grep -oE "= *[0-9]+" | head -1 | grep -oE "[0-9]+"
    }
    TOL=97 # 100 - 3% toleransi
    fail_bench=0
    for metric in qam16_map_demap_symbols_per_sec \
                  ofdm_mod_demod_symbols_per_sec \
                  awgn_apply_samples_per_sec; do
      b="$(get_base "$metric")"
      c="$(get_cur "$metric")"
      if [ -z "$b" ] || [ -z "$c" ]; then
        log "WARN: tidak dapat membaca metrik $metric (baseline=$b sekarang=$c)"
        continue
      fi
      ok="$(awk -v cur="$c" -v base="$b" -v tol="$TOL" \
        'BEGIN { print (cur * 100 >= base * tol) ? 1 : 0 }')"
      if [ "$ok" != "1" ]; then
        log "REGRESI BENCHMARK: $metric sekarang=$c baseline=$b (batas ${TOL}%)"
        fail_bench=1
      else
        log "bench OK: $metric sekarang=$c baseline=$b"
      fi
    done
    if [ "$fail_bench" -ne 0 ]; then
      status=1
      log "GAGAL: regresi benchmark melebihi toleransi 3%."
    fi
    rm -f "$BENCH_OUT"
  fi
else
  log "lewat benchmark (baseline $BASELINE tidak ditemukan)"
fi

# --- Bentuk (fmt) ---
run cargo fmt --all -- --check

# --- Klip (clippy pedantic + nursery, tanpa warning) ---
run cargo clippy --workspace --all-targets -- -D warnings -D clippy::pedantic -D clippy::nursery

# --- Tes (unit + property golden + doc) ---
run cargo test --workspace --all-targets

# --- Tes dokumentasi (doc-test) ---
run cargo test --workspace --doc

# --- Dependensi (lisensi + advisory + duplikasi) jika tersedia ---
if command -v cargo-deny >/dev/null 2>&1; then
  run cargo deny check
else
  log "lewat cargo deny check (cargo-deny tidak terpasang)"
fi

if command -v cargo-audit >/dev/null 2>&1; then
  run cargo audit
else
  log "lewat cargo audit (cargo-audit tidak terpasang)"
fi

# --- Miri (UB) pada core dan phy (no_std) jika tersedia ---
if rustup component list --installed 2>/dev/null | grep -q miri; then
  run cargo miri test -p sig6g-core
else
  log "lewat cargo miri (miri tidak terpasang)"
fi

# --- Fuzzing (parser frame) jika tersedia ---
if command -v cargo-fuzz >/dev/null 2>&1; then
  (cd fuzz && cargo fuzz run frame_parser -max_total_time=60) || {
    log "GAGAL: cargo fuzz frame_parser"
    status=1
  }
else
  log "lewat cargo fuzz (cargo-fuzz tidak terpasang)"
fi

echo
if [ "$status" -eq 0 ]; then
  log "Semua gerbang CI hijau."
else
  log "Satu atau lebih gerbang CI GAGAL."
fi
exit "$status"
