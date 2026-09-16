# 128 independent SHA-256 compression proofs

Use the current committed native FLOCK PCS. This experiment does not use WHIR.
The workload is exactly 128 independent fixed-IV SHA-256 compression calls on
512-bit blocks, with all inputs and outputs public and wired into the proof.
It is not 128 leaves of a binary tree (which would require 255 compressions),
nor a padded full hash of 128 separate 64-byte messages.

The primary comparison reuses `e2e_bit_compare`: identical bit wiring,
Hadamard compression 128 / FLOCK skip7, dense witness commitment and native
Ligerito Fast parameters. Mikros uses packed permutation and lockstep gathers.
The native FLOCK circuit interface is reported as a separate implementation
reference: it uses word wiring and includes assembly/compaction in its timer.

Following the established measurement boundary, witness generation and
challenge-independent preparation/conversion are offline. Online includes
commitment, challenge-dependent conversions, all PIOPs, gather, configured
grinding and PCS opening. Boolean and wiring execute in parallel: their times
must not be added together. Serialization is outside the prover timer.

The runner pins both 64-thread pools to `0-47,48-55,72-79`, builds with
`-C target-cpu=native`, checks GFNI/AVX-512/VPCLMUL availability and samples
three fresh processes per interface. Each has one warmup and six formal
repetitions. The common comparison alternates the two systems. All repetitions
use one deterministic public input and transcript; reported ranges measure
runtime noise, not random-input/PoW variability.

```sh
python3 mikros/server_experiments/20260916-sha128/run_server.py \
  --target /home/mscong1/mikros-experiments/20260912T-native64/v14-packed-aggregate-20260914/source/target
```

Only execute from a frozen source snapshot produced by `package_e2e.py`.
The runner requires successful complete verification, matching transcript
states, identical commitments and PCS parameters across both comparisons,
stable proof bytes across repetitions, and rejected tamper cases. Raw JSONL,
command lines, environment, resource usage and SHA-256 manifests are retained.
