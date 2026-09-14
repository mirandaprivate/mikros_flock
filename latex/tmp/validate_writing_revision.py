"""Read-only static checks for the manuscript writing revision."""
from pathlib import Path
import collections
import difflib
import json
import re

ROOT = Path(__file__).resolve().parent.parent
BEFORE = ROOT / "tmp/writing-revision-2026-09-08-before"


def read(path):
    return path.read_text(encoding="utf-8-sig")


def uncomment(text):
    lines = []
    for line in text.splitlines():
        for i, char in enumerate(line):
            if char == "%":
                slashes = len(line[:i]) - len(line[:i].rstrip("\\"))
                if slashes % 2 == 0:
                    line = line[:i]
                    break
        lines.append(line)
    return "\n".join(lines)


def active_files(base):
    result = []
    seen = set()

    def visit(relative):
        if relative in seen:
            return
        seen.add(relative)
        path = base / relative
        if not path.is_file():
            raise FileNotFoundError(path)
        result.append(relative)
        for name in re.findall(r"\\(?:input|include)\s*\{([^{}]+)\}", uncomment(read(path))):
            if not name.endswith(".tex"):
                name += ".tex"
            visit(name)

    visit("main.tex")
    return result


def inspect(base, relatives):
    issues = []
    labels = collections.defaultdict(list)
    references = []
    citations = set()
    for rel in relatives:
        text = uncomment(read(base / rel))
        stack = []
        depth = 0
        inline = []
        for match in re.finditer(r"\\begin\s*\{([^}]+)\}|\\end\s*\{([^}]+)\}|\\label\s*\{([^}]+)\}|\\(?:eqref|ref|autoref|cref|Cref)\s*\{([^}]+)\}|\\cite\w*\s*(?:\[[^\]]*\]\s*)*\{([^}]+)\}|\\paragraph\b|\\[\(\)\[\]]|[{}]", text):
            line = text.count("\n", 0, match.start()) + 1
            begin, end, label, reference, citation = match.groups()
            token = match.group()
            if begin:
                stack.append(begin)
            elif end:
                if not stack or stack[-1] != end:
                    issues.append(f"{rel}:{line}: environment mismatch {end}, stack={stack}")
                else:
                    stack.pop()
            elif label:
                labels[label].append(f"{rel}:{line}")
            elif reference:
                references.append((reference, f"{rel}:{line}"))
            elif citation:
                citations.update(k.strip() for k in citation.split(","))
            elif token == "\\paragraph" and "proof" in stack:
                issues.append(f"{rel}:{line}: paragraph inside proof")
            elif token in ("\\(", "\\["):
                inline.append(token)
            elif token in ("\\)", "\\]"):
                expected = "\\(" if token == "\\)" else "\\["
                if not inline or inline[-1] != expected:
                    issues.append(f"{rel}:{line}: math delimiter mismatch {token}")
                else:
                    inline.pop()
            elif token in ("{", "}"):
                nslash = len(text[:match.start()]) - len(text[:match.start()].rstrip("\\"))
                if nslash % 2 == 0:
                    depth += 1 if token == "{" else -1
                    if depth < 0:
                        issues.append(f"{rel}:{line}: negative brace depth")
        if stack or depth or inline:
            issues.append(f"{rel}: unclosed env={stack}, braces={depth}, math={inline}")
        if rel.startswith("alg/"):
            if re.search(r"\\\[|\\begin\{(?:equation|align|gather|multline|displaymath)\*?\}|\$\$", text):
                issues.append(f"{rel}: display math in pseudocode")
    for label, locations in labels.items():
        if len(locations) > 1:
            issues.append(f"duplicate label {label}: {locations}")
    for label, location in references:
        if label not in labels:
            issues.append(f"{location}: unresolved label {label}")
    bibkeys = set(re.findall(r"@\w+\s*\{\s*([^,\s]+)\s*,", read(base / "ref.bib")))
    for key in sorted(citations - bibkeys):
        issues.append(f"missing bibliography entry {key}")
    return {
        "issues": issues,
        "labels": dict(labels),
        "citations": sorted(citations),
        "reference_count": len(references),
    }


files = active_files(ROOT)
current = inspect(ROOT, files)
baseline = inspect(BEFORE, active_files(BEFORE))
changed = []
display_changes = {}
pattern = r"\\\[.*?\\\]|\\begin\{(equation\*?|align\*?|gather\*?|multline\*?|displaymath)\}.*?\\end\{\1\}"
for rel in files:
    old, new = read(BEFORE / rel), read(ROOT / rel)
    if old != new:
        diff = list(difflib.unified_diff(old.splitlines(), new.splitlines()))
        changed.append({
            "file": rel,
            "added_lines": sum(x.startswith("+") and not x.startswith("+++") for x in diff),
            "removed_lines": sum(x.startswith("-") and not x.startswith("---") for x in diff),
        })
    old_eq = [re.sub(r"\s+", "", m.group()) for m in re.finditer(pattern, uncomment(old), re.S)]
    new_eq = [re.sub(r"\s+", "", m.group()) for m in re.finditer(pattern, uncomment(new), re.S)]
    if old_eq != new_eq:
        matcher = difflib.SequenceMatcher(None, old_eq, new_eq)
        display_changes[rel] = [
            {"kind": tag, "old": [i, j], "new": [k, l]}
            for tag, i, j, k, l in matcher.get_opcodes() if tag != "equal"
        ]

table = "tab/tab-component.tex"
numeric = lambda text: re.findall(r"\d+(?:\.\d+)?", text)
active_set = set(files)
inactive_changes = [
    str(path.relative_to(BEFORE)).replace("\\", "/")
    for path in BEFORE.rglob("*")
    if path.is_file()
    and str(path.relative_to(BEFORE)).replace("\\", "/") not in active_set
    and (ROOT / path.relative_to(BEFORE)).is_file()
    and path.read_bytes() != (ROOT / path.relative_to(BEFORE)).read_bytes()
]
print(json.dumps({
    "active_files": files,
    "changed": changed,
    "issues_current": current["issues"],
    "issues_baseline": baseline["issues"],
    "labels_removed": sorted(set(baseline["labels"]) - set(current["labels"])),
    "labels_added": sorted(set(current["labels"]) - set(baseline["labels"])),
    "citations_removed": sorted(set(baseline["citations"]) - set(current["citations"])),
    "citations_added": sorted(set(current["citations"]) - set(baseline["citations"])),
    "reference_count": current["reference_count"],
    "label_count": len(current["labels"]),
    "citation_count": len(current["citations"]),
    "display_changes": display_changes,
    "table_numbers_unchanged": numeric(read(BEFORE / table)) == numeric(read(ROOT / table)),
    "bibliography_unchanged": (BEFORE / "ref.bib").read_bytes() == (ROOT / "ref.bib").read_bytes(),
    "inactive_files_changed": inactive_changes,
}, indent=2))
