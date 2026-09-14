# v8 aggregation: machine-code audit index

This is local analysis of the downloaded native executable's actual machine
code, not a server hardware-counter result. All instruction extraction finished
after screening and before the N30 results process started. The executable SHA
is `c26172d4caa709fa3ee43a5b077757e56378702afe992c08d2b360daa9bc287f`.
Function identities, timestamps, command and file SHA values are retained in
[assembly_manifest.json](assembly_manifest.json).

- [Paired interpolation audit](local-pair-assembly-audit.md) compares the same
  executable's retained dynamic-index interpolation and fixed-index pair path.
- [Consumer audit](local-consumer-assembly-audit.md) follows actual loop back
  edges, accumulator registers, private-stack traffic, and H output accesses.

The pair path removes the old two 15-chunk interpolation loops and their
indirect permutation dispatch. It keeps the same compressed table and field
samples. However, the compiler also emits its four-coordinate consumer inline,
while the retained baseline calls a separate consumer. Thus measured timing
belongs to the whole engineering candidate, not a separately measured cost of
indirect branches alone. Source-level `inline(never)` does not substitute for
inspection of the resulting executable.

The eight-coordinate consumers have sixteen live accumulator vectors in their
actual row loops and no accumulator spill inside those loops. H affine has a
different outcome: the compiler hoists thirty-two H accumulators, and twenty-one
components use stack slots. Its complete 64-row case has 432 B/row of logical H
output plus spill operands, with another 32 B/row of transpose temporary
operands. These are instruction-operand counts, not cache misses, DRAM traffic,
or wall-clock predictions. The detailed note separates loop and boundary costs.

H affine executes 36 vector affine instructions per full eight-row group,
including the four preparation transforms. Fewer per-row masks do not imply
an eightfold speedup. A future independent engineering candidate could limit
the H accumulator live set while reusing the prepared subset indices and
weight matrices. That change is not implemented or measured in this snapshot.

The source audit and all proof checks retain the protocol boundary. Native
tests validate matrix order, arbitrary coordinates, all subset values, tails,
nonzero initial sums, and complete proof bytes/claims/transcripts. Read actual
phase and end-to-end measurements in the report; timing does not identify
unmeasured cache, NUMA, frequency, or branch-miss causes.
