# CRYPTO / EUROCRYPT SNARK and PCS literature (2024--2026)

This directory contains a curated, high-recall collection of papers formally published at CRYPTO or EUROCRYPT in 2024--2026. The selection includes both papers directly about SNARKs, SNARGs, or polynomial commitment schemes (`core`) and papers on central components used by these systems, such as folding, IOPPs, proximity testing, accumulation, lookup, and memory checking (`component`).

| Venue | 2024 | 2025 | 2026 | Total |
|---|---:|---:|---:|---:|
| CRYPTO | 14 | 8 | 15 | 37 |
| EUROCRYPT | 6 | 5 | 9 | 20 |
| **Total** | **20** | **13** | **24** | **57** |

The directory hierarchy uses the conference publication year. An ePrint identifier in a filename may therefore have an earlier year than the conference version. `manifest.csv` records the paper title, topic category, publication record, public PDF source, and local filename.

Public IACR ePrint versions are preferred. A Springer proceedings PDF is used only when it is publicly retrievable. Two additional relevant CRYPTO 2025 papers were identified but had no lawful public full-text version available during collection; their metadata is recorded in `unavailable.csv` rather than saving a paywall or error page as a PDF.

All 57 stored files were checked to be larger than 10 KiB, to begin with the `%PDF-` signature, and to expose the expected title on the first page through `pdftotext`.

To re-download missing files or reproduce the collection, run:

```powershell
& .\download-papers.ps1
```
