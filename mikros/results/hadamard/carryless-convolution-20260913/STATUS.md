# Experiment stopped; interpolation retained

The user selected the original continuation-point interpolation scheme on
2026-09-13. `Kernel::PairAggregate` / `pairreuse` remains the selected Hadamard
implementation; the E2E example already uses this kernel. The standalone
Hadamard benchmark now also defaults to `pairreuse,flock128`.

The coefficient-convolution kernels remain explicit experimental options.
Their last validated source snapshot is
`mikros/server_experiments/20260913T-carryless/release-v1/source.tar.gz`, SHA256
`a89b4fe4b7cdd7563375f7ae632ef77f00c8c0078a9651b9b76d5828635a4fae`.
It passed 114 local tests with the fallback backend and 115 native server tests.

Server evidence remains under
`/home/mscong1/mikros-experiments/20260912T-native64/v10-carryless/`:

- `screening`: completed N24/N26, 64 threads, four systems, eight formal runs,
  separate profiles, 122 validated proof records including warmups.
- `large`: the N30 ordinary timing process completed; the experiment was
  stopped during the separate profile stage at the user's request. The entire
  stage must not be reported as complete. See `large-user-stop.json` and raw logs.

No further convolution optimization or server measurement is scheduled.
The unfinished proposed GFNI transpose optimization was not adopted.
