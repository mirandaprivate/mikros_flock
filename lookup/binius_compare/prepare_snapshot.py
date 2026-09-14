"""Freeze the current upstream kernels without modifying either live workspace."""
from pathlib import Path

# Reuse the existing, reviewed freeze procedure, but execute its exact source
# with this file's directory. It creates a NEW snapshot and refuses overwrite.
here = Path(__file__).resolve().parent
script = here.parent / "flock_compare" / "prepare_snapshot.py"
exec(compile(script.read_text(), str(script), "exec"), {"__file__": str(here / "prepare_snapshot.py"), "__name__": "__main__"})
