# v5 task state

The local release differential suite passes 17 tests; all-target Clippy passes
for this crate (the frozen Flock dependency retains its existing warning).
Native local compilation lacks the server AVX-512 feature set; server tests
are required before any hardware performance claim.

Frozen package: 415 files, 1,787,805 archive bytes.
SHA256: 21989af1a3db2b3c1e4a62ac6115d3806b8b7a3da1a8650acad01cf89350c249.

Server driver PID 3582859 started under the existing approved scope:
/home/mscong1/mikros-experiments/20260912T-native64/permutation-binary-v4/iteration-v5

Nine cases are planned: smoke, N26 ablation, selected N30 comparison, three
cold and three warm separate processes. The N26 selection is written before
the N30 run. All proof files and explicit cleanup timers are retained.

Complete: server native tests/build, all 9 cases and 71 proof records,
110 evidence files downloaded and hash verified, frozen report generated.
Independent raw-data and code/assembly audits completed; no remaining work.
No interference attempts were excluded. The driver exited; SSH is closed.

Native executable SHA256:
57cc3e617f657fceddfdac55218c1fcae0186d804d51e8a3c75f231020a0e7df.
Evidence archive SHA256:
7d66e2cf2a7b3111fda5296ace64bd5f360c6398c73ce94da60169414dd044e3.

N30 warm core medians old/combined/Flock = 9.370989164 / 6.3835728585 /
12.125665264 seconds. The principal gain is bounded tree storage reuse;
register fusion alone regressed in N26. Preserve the negative result and
the explicit cleanup/retained-memory tradeoff described in RESULTS.md.
Frozen summarizer executed with bundled Python 3.12.14 because Python 3.10
does not parse the UTC Z suffix using datetime.fromisoformat.
Frozen v4 sources/results remain unchanged. Default prove retains v4;
new engines are opt-in while their cold/warm tradeoffs are evaluated.

Subsequent accounting clarification: exclude offline conversion and include
all online conversion. OFFLINE_ONLINE.md records the source audit. Existing
prepare is offline, but core still mixes in additional precomputable work;
the frozen results must not be relabeled as a strictly separated online
benchmark. Such a separately prepared experiment has not been run.
