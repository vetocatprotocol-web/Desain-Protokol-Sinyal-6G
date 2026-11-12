#!/usr/bin/env bash
# ci/run-all — Gerbang CI lokal untuk workspace sig6g (M0 Fondasi).
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

echo
if [ "$status" -eq 0 ]; then
  log "Semua gerbang CI hijau."
else
  log "Satu atau lebih gerbang CI GAGAL."
fi
exit "$status"
