#!/usr/bin/env bash
# Serial measurements; keep diagnostic writes on WSL's Linux filesystem.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
mikros_toolchain="${MIKROS_TOOLCHAIN:-nightly-2026-03-21}"
mikros_tmp="$(mktemp -d /tmp/mikros-profile.XXXXXX)"
mkdir -p mikros/results/profile
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
  env -u FLOCK_ZC_TIMING "$mikros_executable" --logs 20,24 --runs 5 \
    --threads "$mikros_threads" --systems lookup,flock \
    --output "mikros/results/profile/control-${mikros_threads}t.json" \
    > "$mikros_tmp/control-${mikros_threads}t.log" 2>&1
  cp -- "$mikros_tmp/control-${mikros_threads}t.log" mikros/results/profile/
  FLOCK_ZC_TIMING=1 "$mikros_executable" --logs 20,24 --runs 5 \
    --threads "$mikros_threads" --systems lookup,flock --profile \
    --output "mikros/results/profile/wsl-phases-${mikros_threads}t.json" \
    > "$mikros_tmp/wsl-phases-${mikros_threads}t.log" 2>&1
  cp -- "$mikros_tmp/wsl-phases-${mikros_threads}t.log" mikros/results/profile/
done
python3 mikros/summarize_profile.py
