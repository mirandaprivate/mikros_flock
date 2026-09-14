# Upstream WHIR

This directory vendors the source of
[`worldfnd/whir`](https://github.com/worldfnd/whir) at commit
`92652ca01e215548c98e11834e110c43994b94c1`.

The upstream source is preserved unchanged. Its manifest pins the dependency
versions recorded by the upstream lockfile and enables the zerocopy derive
feature so the crate can also be resolved from the parent workspace.

The local integration binary now lives at
`poly-commit/src/bin/whir_experiment.rs`. It measures one non-ZK multilinear
PCS commitment and opening using:

- the 192-bit prime field exposed upstream as `Field192`;
- BLAKE3 Merkle hashing;
- rate \(1/2\);
- Johnson decoding;
- 100 bits of target security;
- one multilinear evaluation;
- initial and subsequent folding factors of four.

Input generation, parameter derivation, and evaluation generation are outside
the reported prover time. Proof size is the exact upstream transcript length
`narg_string.len() + hints.len()`.
