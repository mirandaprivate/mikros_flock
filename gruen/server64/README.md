# Goldilocks / KoalaBear 64-thread server comparison

Status: the 2026-09-13 server run is complete. See
[N30 results](results/server-20260913-n30/RESULTS.md): 64 threads, eight repeats
per method/field, 35 native tests and 144 verified proofs across N20/N30.
Mikros optimized takes 6.701 s on Goldilocks and 4.770 s on KoalaBear;
the faster Gruen baseline takes 7.408 s and 6.139 s respectively.
Local release validation: 35 tests passed, including full-proof differential
checks at 1/4/64 worker threads. See `results/local-validation/tests.log`.
The preceding `../MIKROS_COMPARISON.md` reports local single-thread results.
The repository's existing binary F128 server reports concern a different field
and protocol and must not be substituted for this experiment.

This independent workspace parallelizes the same four PIOP reductions:
Gruen ordinary two-evaluation recovery, Gruen skip-kappa, Mikros reference
Lagrange, and Mikros optimized Lagrange/quotient/packing. Goldilocks uses
X^2-7 (kappa=2), KoalaBear uses X^4-3 (kappa=4). The common scalar field
arithmetic is unchanged; this is not native Plonky3 SIMD or a PCS benchmark.
The statement is Hadamard `a*b-c=0`, not the separate binary permutation task.

All modifications stay in this directory. `reference/` freezes the preceding
scalar source and `reference_manifest.json` records its hashes. Tests compare
every ordered proof coordinate and source-oracle result against that reference
at 1, 4 and 64 Rayon threads, including node/zero/one challenges. Local 64-worker
tests oversubscribe the workstation; they test correctness, not server speed.

Parallel work includes shared split equality tables, block quotient/helper
reductions, packing, round messages, folding, terminal columns and source checks.
Field addition is exact, so different reduction orders preserve messages.
Each parallel fold uses a distinct output allocation to avoid reader/writer
races. Allocation and old-buffer release stay inside each prover timer for all
methods. Public setup and C are included. Witness generation, independent dense
source authentication and writing explicit proof coordinates are outside it.
The deterministic verifier tape is experimental, not Fiat-Shamir.

The runtime explicitly constructs and checks a 64-worker Rayon pool; setting
an environment variable alone is insufficient. Raw samples record process CPU
time as well as wall time. Their ratio estimates average CPU use and must not
be described as guaranteed 64-core utilization.

On the recorded swiftlab topology the proposed affinity is
`0-47,48-55,72-79`: 48 physical cores and 16 additional SMT threads, across two
NUMA nodes. It is not 64 physical cores. The driver verifies CPU availability
and records topology, cgroup limits, first-touch placement, build flags and
source/binary hashes. It waits for competing known benchmark/build processes;
it does not stop them. A shared server cannot be assumed exclusive.

Completed main run: N=2^20,2^30; both fields; four systems; one warmup and
eight repeats, cycling execution positions. Separate optional single-system
processes record whole-process RSS (including input/source checks, not solely
prover workspace). N30 needs an available 160 GiB conservative reservation;
the driver also checks visible memory cgroup limits. The actual peak is to be
measured, not inferred from this reservation. Existing results are never erased.

```sh
# On the authorized server after extracting the new source archive:
cd /absolute/run/source
export PATH="$HOME/.cargo/bin:$PATH"
# If the locked Rayon dependencies are not already cached:
cargo +nightly-2026-03-21 fetch --locked
python3 run_server.py --out /absolute/run/results --n 20,30 --threads 64 \
  --cpus 0-47,48-55,72-79 --reps 8
```

The driver builds/tests natively and offline against Cargo.lock, then validates
every measured proof and all source claims, exact Mikros paired messages and
balanced run positions. It retains raw/summary CSV, explicit proof coordinates,
commands, environment, logs and acceptance records. `--skip-build` explicitly
records that compilation/tests were not performed by that invocation; use the
default for a first server run.

To package only authorized code (no credentials, `.git` or target directories):

```sh
python3 package_source.py --out results/NEW_PACKAGE
```
