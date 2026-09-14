#!/usr/bin/env bash
# Run from Linux/WSL. Benchmarks are deliberately serial.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

mikros_toolchain="${MIKROS_TOOLCHAIN:-nightly-2026-03-21}"
mikros_logs="${MIKROS_LOGS:-16,18,20,22,24}"
mikros_runs="${MIKROS_RUNS:-5}"
mikros_mt="${MIKROS_MT_THREADS:-6}"
mikros_rss_log="${MIKROS_RSS_LOG:-24}"

cargo "+$mikros_toolchain" test -p mikros --release
cargo "+$mikros_toolchain" bench -p mikros --bench hadamard_vs_flock -- \
  --logs 13,16,18 --runs "$mikros_runs" --threads 1 \
  --output mikros/results/smoke-1t.json

for mikros_threads in 1 "$mikros_mt"; do
  cargo "+$mikros_toolchain" bench -p mikros --bench hadamard_vs_flock -- \
    --logs "$mikros_logs" --runs "$mikros_runs" --threads "$mikros_threads" \
    --systems lookup,flock \
    --output "mikros/results/comparison-${mikros_threads}t.json"
  for mikros_system in lookup flock; do
    cargo "+$mikros_toolchain" bench -p mikros --bench hadamard_vs_flock -- \
      --logs "$mikros_rss_log" --threads "$mikros_threads" \
      --systems "$mikros_system" --rss \
      --output "mikros/results/rss-${mikros_system}-${mikros_threads}t-n${mikros_rss_log}.json"
  done
done
