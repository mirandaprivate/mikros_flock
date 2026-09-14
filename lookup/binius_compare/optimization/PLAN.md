# Ordered optimization and validation

Scope: independent permutation + Binius/Ligerito experiment in this directory.
Do not edit the frozen baseline, live lookup/Flock sources or other optimized workspace.

Status: all five steps implemented and validated. The completed 11-arm ablation
selected FullOptimized as the speed default; compact and quotient-packed modes
remain selectable. The 10-case scaling suite and cross-suite source/proof checks
are complete. See RESULTS.md and results/validation-all.json.

1. Preserve Reference and FullExtension controls, split PIOP timers, use direct
   integer row views, and construct commitment bits with block transposition.
2. Import and check same-modulus PCLMUL, byte-linear embeddings/evaluation;
   implement delayed reduction for coefficient convolution.
3. Import exact-message fused/split-eq GKR, then add a versioned batched
   quadratic-core GKR with verifier and rejection tests.
4. Test one and six physical cores; accelerate source column/tag evaluation
   with subset-sum tables. Apply common adapter improvements to Flock too.
5. Add coefficient-domain Hadamard quotient reductions for compact layers,
   with explicit kappa, transcript ordering and terminal authentication.

Validation: independent baseline equality for unchanged transcripts; arithmetic
differentials, malformed/tampered PIOP and PCS tests for new transcripts; paired
same-input commitments and PCS parameters, serialized proof replay; sequential
benchmarks with saved raw samples, source hashes and interference detection.
Choose the measured fast mode without discarding slower experimental modes.
