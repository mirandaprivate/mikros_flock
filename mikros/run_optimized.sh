#!/usr/bin/env bash
# Six permutations balance all three systems' execution positions.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
mikros_toolchain="${MIKROS_TOOLCHAIN:-nightly-2026-03-21}"
mikros_tmp="$(mktemp -d /tmp/mikros-optimized.XXXXXX)"
mkdir -p mikros/results/optimized
cargo "+$mikros_toolchain" test -p mikros --release
cargo "+$mikros_toolchain" bench -p mikros --bench hadamard_vs_flock \
  --no-run --message-format=json > "$mikros_tmp/build.jsonl"
mikros_executable="$(python3 - "$mikros_tmp/build.jsonl" <<'PY'
import json, sys
paths = []
for line in open(sys.argv[1]):
    message = json.loads(line)
    if (message.get('reason') == 'compiler-artifact'
            and message.get('target', {}).get('name') == 'hadamard_vs_flock'
            and message.get('executable')):
        paths.append(message['executable'])
assert len(paths) == 1, paths
print(paths[0])
PY
)"
for mikros_threads in 1 6; do
  env -u FLOCK_ZC_TIMING "$mikros_executable" --logs 16,18,20,22,24 --runs 6 \
    --threads "$mikros_threads" --systems lookup,packed,flock \
    --output "mikros/results/optimized/comparison-${mikros_threads}t.json" \
    > "$mikros_tmp/comparison-${mikros_threads}t.log" 2>&1
  cp -- "$mikros_tmp/comparison-${mikros_threads}t.log" mikros/results/optimized/
  env -u FLOCK_ZC_TIMING "$mikros_executable" --logs 20,24 --runs 6 \
    --threads "$mikros_threads" --systems lookup,packed --profile \
    --output "mikros/results/optimized/phases-${mikros_threads}t.json" \
    > "$mikros_tmp/phases-${mikros_threads}t.log" 2>&1
  cp -- "$mikros_tmp/phases-${mikros_threads}t.log" mikros/results/optimized/
  for mikros_system in lookup packed flock; do
    env -u FLOCK_ZC_TIMING "$mikros_executable" --logs 24 --rss \
      --threads "$mikros_threads" --systems "$mikros_system" \
      --output "mikros/results/optimized/rss-${mikros_system}-${mikros_threads}t.json" \
      > "$mikros_tmp/rss-${mikros_system}-${mikros_threads}t.log" 2>&1
    cp -- "$mikros_tmp/rss-${mikros_system}-${mikros_threads}t.log" mikros/results/optimized/
  done
done
python3 mikros/summarize_optimized.py
