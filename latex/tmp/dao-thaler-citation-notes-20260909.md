# Dao--Thaler citation additions

## Scope and evidence

Source: `C:/Users/miranda/Documents/paper/ThalerBinary.pdf`, all 11 pages.
The source is Dao and Thaler, *Constraint-Packing and the Sum-Check Protocol over Binary Tower Fields*, ePrint 2024/1038.
The targeted backward-citation pass covered the paper's reference list, with additions selected for small-field sumcheck, binary-tower arithmetic, and tuple encoding. This was not a systematic literature review.
Only `1-intro.tex` and `ref.bib` were changed. The pre-edit copies are in `tmp/dao-thaler-citations-before-20260909-013036/`.

## Source-to-claim map

| Citation key | Use in Introduction | Verification |
|---|---|---|
| `iacr/DaoT24` (existing) | Fixed tower-generator coordinates for zerocheck; equality-table preprocessing; injective tuple encoding | Supplied PDF Sections 2--3, 5, 7--9; https://eprint.iacr.org/2024/1038 |
| `iacr/Gruen24` (new) | Fewer constraint evaluations and more base-field work, with verifier interpolation tradeoffs | https://eprint.iacr.org/2024/108 |
| `fq/Wiedemann88` (new) | Iterated quadratic binary-field extensions | Original journal scan https://www.fq.math.ca/Scanned/26-4/wiedemann.pdf; publisher-deposited Crossref metadata for DOI 10.1080/00150517.1988.12429608 |
| `isit/FanP97` (new) | Recursive tower-field arithmetic underlying Dao--Thaler's generator multiplication | Supplied PDF Sections 2 and 8; publisher-deposited Crossref metadata for DOI 10.1109/ISIT.1997.612935 |
| `osdi/SettyAGL18` (new) | Spice as an offline-memory-checking application context | Supplied PDF Section 9; official BibTeX at https://www.usenix.org/conference/osdi18/presentation/setty |
| `eurocrypt/SettyTW24` (existing) | Lasso as a lookup-argument application context | Supplied PDF Section 9; https://link.springer.com/chapter/10.1007/978-3-031-58751-1_7 |
| `innovations/CormodeMT12` (existing) | Efficient interactive proofs as the context for later small-field prover work | Author preprint https://arxiv.org/abs/1105.2003; publisher-deposited Crossref metadata for DOI 10.1145/2090236.2090245 |
| `crypto/Thaler13` (existing) | Efficient circuit-evaluation protocols | Author preprint https://arxiv.org/abs/1304.3812; publisher-deposited Crossref metadata for DOI 10.1007/978-3-642-40084-1_5 |

## Attribution and bibliographic decisions

- The equality-table saving is described as a reduction in general extension-field multiplications, not an equal-factor improvement in total prover time or memory.
- The deterministic tuple-encoding optimization is attributed to Dao--Thaler; Lasso and Spice are the application context.
- No unconditional claim about removing sumcheck rounds was added: Dao--Thaler's degree-one optimization has additional hypotheses.
- The comparison with Mikros refers to base-field row products, not to all auxiliary polynomial coefficients being in the base field.
- Four bibliography entries were added; seven previously uncited works are now cited in Introduction. All prior citation keys are preserved.
- Existing entries for Cormode--Mitzenmacher--Thaler, Thaler, and Lasso were reused, with their verified DOIs added. No duplicate preprint versions were created.
- Wiedemann's publication year is 1988, not the publisher website's later digitization date. Spice's official citation does not supply a DOI.
- No URL fields were added to `ref.bib`.

## Validation

- The active-manuscript static check resolves all 47 unique citation keys, 112 labels, and 196 cross-references without an issue.
- Hash comparison confirms that the other 32 inspected TeX/bibliography files are unchanged.
- No TeX compilation or PDF layout validation was performed.
