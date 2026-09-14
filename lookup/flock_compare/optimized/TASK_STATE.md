# Completed experiment (2026-09-12 09:52:12 UTC)

All ten server cases completed and all commands exited successfully. No
monitored interference was recorded. The remote driver has exited.

Downloaded results to server_experiments/20260912T091817-permutation-v1.
Archive SHA256: 0a722d1ade0fbbc2b0a4c60911b4c0957addbc4efce4befd28985b65b009bc66.
All 134 bundled files verified. summarize_experiment.py validated all 180
local/server proof files and 42 distinct statement/protocol pairs; REPORT.md,
summary.csv, validation.json and download_validation.json exist in the mirror.

N30 core/total seconds (median, six measured samples):
- 24t compact 16.497/19.630, full 12.285/15.397, Flock 12.612/13.592.
- 48t compact 14.301/16.171, full 11.021/12.872, Flock 11.489/12.649.
Cold RSS: both Mikros strategies about 97.5 GiB; Flock about 158.0 GiB.
N26 48t compact core speedup: 47.568 s -> 0.902 s (52.73x).

ENGINEERING.md explains optimizations and remaining materialization costs.
Benchmark Rust sources match the uploaded frozen package. Only reporting
scripts and documentation were added/updated after packaging. Tests and clippy
passed; native AVX-512 differential tests passed remotely. No further server
runs or benchmark changes are required for this completed experiment.

## Historical checkpoint below (superseded; do not resume)

User asked: identify Mikros permutation bottlenecks, implement engineering
optimizations, run N=2^30 multithreaded samples on the server. This is the side
conversation; do not interact with any subagents or other tasks' jobs.

Implementation is isolated in this optimized/ crate. Frozen baseline at
../snapshot is unchanged; parent lookup/ WHIR and mikros/ Hadamard work untouched.
Local tests and clippy passed, local N20 1t/6t ablation complete (6 runs each).
All candidates preserve reference proof bytes and claims. Local six-thread
medians: reference 827.341ms, parallel 313.949, lookup 233.977, optimized 89.128,
full 67.849, Flock 58.027. Native input conversion reference 307.256ms vs optimized
11.246ms. Old dominant core phases: compact build 202.941ms, decode 191.536,
layer materialization 171.216, initial coefficient eval 77.921.

Server source package uploaded and SHA256 verified:
4e7ceb81e0f8d65ba82f42145a3cfc6a2dc8a55a9f60377592681539155e67ae
Remote root:
/home/mscong1/mikros-permutation-experiments/20260912T091817-permutation-v1
Local mirror directory: server_experiments/20260912T091817-permutation-v1
Only archive and manifests locally so far; results still need downloading.

Authenticated persistent SSH exec session: 3840. It is a bash prompt with cwd
set to the remote root, PATH includes /home/mscong1/.cargo/bin. Do not echo or
write credentials to files. Existing default-key authentication failed; password
authentication was successful. The session is this task's own connection.

Remote background driver PID 3511696, driver.log and driver.pid in root.
Native AVX-512 differential tests passed. At 09:24 UTC, driver was in
ablation-n26-48t-attempt1, compare PID 3513799. It runs one warmup + six cyclic
orders of reference/parallel/lookup/optimized/full/flock. Reference is slow
(~48s core +25s conversion per sample); optimized warmup ~0.899s and full ~0.687s,
Flock ~0.869s. Then driver automatically runs N30, 24 and 48 physical cores,
three systems optimized/full/flock, warmup + six paired balanced samples, plus
one fresh-process cold RSS sample per system/thread. Wait for complete.

Server: Xeon Gold 6342, 48 physical /96 logical CPUs. Physical cores 0-23 on
NUMA0, 24-47 on NUMA1. ~390GB available. Cgroup memory.max is max at every
ancestor; cpu.max max; allowed CPUs0-95/memory nodes0-1. Another Hadamard job
was observed earlier; never stop it. Driver waits for monitored competing
processes, monitors active cases, and retries whole contaminated cases.

Remaining work:
1. Monitor own driver via session3840, concise user updates <=60s.
2. If native runs fail, inspect logs and fix isolated candidate only; preserve
   v1 package/results and create v2 for source changes, no silent source drift.
3. When complete, create a results tarball + SHA256 file manifest remotely,
   scp to the local mirror, verify archive and every downloaded file.
4. Run summarize_experiment.py --root <local mirror>. It checks all proof files,
   paired inputs, message size formulas, same-message variants and cross-thread /
   CPU proof identity. Server accepted cases expected10. Local proof data84;
   server data96 (= smoke6 + ablation42 + main42 + cold6), total180.
5. Inspect full report and phase/RSS differences. Do not infer retained tree
   capacity equals RSS: compact leaves are still materialized for GKR.
6. Report completed measured N30 outcomes with code/report links. Current PIOP,
   no PCS/grinding, unchanged original two-chain cubic wire protocol.

Python executable:
C:/Users/miranda/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe
No matplotlib/blake3 Python packages installed; not needed for validator.
Memory used for toolchain guidance; final append memory citation MEMORY.md:200
with rollout01a05ad1-21d6-76f3-9084-0bf054d90c8e. User forbids subagents.
