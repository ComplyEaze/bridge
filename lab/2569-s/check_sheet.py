# -*- coding: ascii -*-
"""Check pre-sitting file references, the launch names, and the complete three-column manifest.

Row 2162-RX's check_sheet.py, with this row's launches in place of its arms, this row's generated and
cited names, and an ASCII check of every helper. Python 3.10, standard library only.
"""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".xml", ".json", ".md", ".txt", ".ps1", ".cs")
GENERATED = {"restore-plan.txt", "kept-local.txt", "launch-env.json", "client-log.txt",
             "05-post-1.json", "06-post-2.json", "07-post-3.json"}
# Files named in RUN-SHEET.md that live on other rows' branches (cited, not inputs of this row).
CITED = {"006-s8-post-a-1.json", "launch-method.txt"}
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
# Row 2569-S's launches, in sitting order. The launch frame's labels use <X>.
LAUNCHES = ("L1", "L0", "L2")
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
SEND = re.compile(r"`([A-Za-z0-9<>-]+)`: `([^`]+)`")
ANSWER = re.compile(r"`(a-[^`\s]+)`|\b(a-[A-Za-z0-9<>-]+-(?:M-marks|F-0101|K-customer)\.xml)\b")
HELPERS = ("ans.py", "postcall.py", "check_sheet.py", "startinfo.cs", "startinfo.ps1", "boxwatch.ps1", "exes.ps1",
           "ops.ps1")
# What the sitting writes inside the row folder, skipped by the manifest check by name: these folders (the first
# part of the path) and these files at the top level. Nothing else is skipped.
OUTPUT_FOLDERS = ("evidence", "backups")
OUTPUT_FOLDER_PREFIXES = ("run-", "data-", "reads-")
OUTPUT_TOP_FILES = ("restore-plan.txt", "kept-local.txt")
OUTPUT_TOP_PREFIXES = ("a-", "log-")


def sitting_output(rel):
    parts = rel.split("/")
    if len(parts) > 1:
        return parts[0] in OUTPUT_FOLDERS or parts[0].startswith(OUTPUT_FOLDER_PREFIXES)
    return rel in OUTPUT_TOP_FILES or rel.startswith(OUTPUT_TOP_PREFIXES)


def generated(name):
    return name in GENERATED or name.startswith(("a-", "evidence/")) or name in CITED


def references():
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    missing = []
    checked = set()
    for token in re.findall(r"`([^`]+)`", text):
        if any(mark in token for mark in ("*", "<", ">", ":")) or any(c.isspace() for c in token):
            continue
        name = token.replace("\\", "/")
        if name.startswith(("/", "../", "src-tauri/", "lab/")):
            continue
        if not (name.endswith(SUFFIXES) or name == ".gitattributes"):
            continue
        candidates = [name]
        if "/" not in name and (ROOT / "calls" / name).is_file():
            candidates.insert(0, "calls/" + name)
        chosen = next((candidate for candidate in candidates if (ROOT / candidate).is_file()), candidates[0])
        if generated(chosen):
            continue
        checked.add(chosen)
        if not (ROOT / chosen).is_file():
            missing.append(chosen)
    return sorted(checked), sorted(set(missing))


def expand(name):
    return [name.replace("<X>", x) for x in LAUNCHES] if "<X>" in name else [name]


def section(text, title):
    start = text.index(f"\n## {title}\n")
    end = text.find("\n## ", start + 1)
    return text[start:] if end < 0 else text[start:end]


def launches():
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    errors = []
    table = section(text, "The questions and the one variable per launch")
    steps = section(text, "Steps")
    decides = section(text, "Decides")
    for x in LAUNCHES:
        if f"\n| {x} |" not in table:
            errors.append(f"launch {x} missing from the launch table")
        if not re.search(rf"\*\*{x}: ", steps):
            errors.append(f"launch {x} has no step heading")
        if not re.search(rf"\b{x}\b", decides):
            errors.append(f"launch {x} missing from Decides")
        if not (ROOT / "launch" / f"{x}-launch-env.json").is_file():
            errors.append(f"launch {x} has no launch file")
    for match in GUID_SHAPE.finditer(text):
        errors.append(f"GUID-shaped text in RUN-SHEET.md: {match.group(0)[:8]}...")
    sends = set()
    for label, files in SEND.findall(text):
        for expanded in expand(label):
            for name in files.split():
                sends.add(f"a-{expanded}-{name}")
    answers = 0
    for match in ANSWER.finditer(text):
        token = match.group(1) or match.group(2)
        if not token.endswith(".xml") or "*" in token:
            continue
        for name in expand(token):
            answers += 1
            if name not in sends:
                errors.append(f"answer compared but never sent: {name}")
    return answers, errors


PLACED_ROW = re.compile(r"\| `((?:calls|launch)/[^`]+)` \| `([0-9a-f]{64})` \|")


def placed_inputs(entries):
    """Every call and launch file is in the sheet's placed-inputs table once, with its manifest SHA-256."""
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    errors = []
    rows = {}
    for name, digest in PLACED_ROW.findall(text):
        if name in rows:
            errors.append(f"placed-inputs table names {name} twice")
        rows[name] = digest
    wanted = sorted(name for name in entries if name.startswith(("calls/", "launch/")))
    for name in wanted:
        if name not in rows:
            errors.append(f"placed-inputs table lacks {name}")
        elif rows[name] != entries[name][0]:
            errors.append(f"placed-inputs table hash differs from SHA256.txt: {name}")
    for name in sorted(set(rows) - set(wanted)):
        errors.append(f"placed-inputs table names a file not in SHA256.txt: {name}")
    return len(rows), errors


def ascii_helpers():
    errors = []
    for name in HELPERS + ("boxstate.ps1", "client.py", "run.py", "count8.py"):
        raw = (ROOT / name).read_bytes()
        if any(b > 0x7E or (b < 0x20 and b not in (0x09, 0x0A, 0x0D)) for b in raw):
            errors.append(f"non-ASCII byte in {name}")
    for name in ("ans.py", "postcall.py", "check_sheet.py"):
        first = (ROOT / name).read_text(encoding="ascii").splitlines()[0]
        if first != "# -*- coding: ascii -*-":
            errors.append(f"{name} does not name its encoding on line 1")
    return errors


def manifest():
    lines = (ROOT / "SHA256.txt").read_text(encoding="utf-8").splitlines()
    entries = {}
    errors = []
    for line in lines:
        match = ROW.fullmatch(line)
        if not match:
            errors.append(f"bad manifest row: {line!r}")
            continue
        digest, size, name = match.groups()
        if name in entries or pathlib.PurePosixPath(name).is_absolute() or ".." in pathlib.PurePosixPath(name).parts:
            errors.append(f"duplicate or unsafe path: {name}")
            continue
        entries[name] = (digest, int(size))
    actual = {}
    skipped = 0
    for path in ROOT.rglob("*"):
        rel = path.relative_to(ROOT).as_posix()
        if not path.is_file() or rel == "SHA256.txt":
            continue
        if sitting_output(rel):
            skipped += 1
            continue
        actual[rel] = path
    for name in sorted(set(entries) - set(actual)):
        errors.append(f"manifest names missing file: {name}")
    for name in sorted(set(actual) - set(entries)):
        errors.append(f"file absent from manifest: {name}")
    for name, path in actual.items():
        if name not in entries:
            continue
        raw = path.read_bytes()
        expected = (hashlib.sha256(raw).hexdigest(), len(raw))
        if entries[name] != expected:
            errors.append(f"hash or size differs: {name}")
    for name in sorted(entries):
        if sitting_output(name):
            errors.append(f"manifest names a sitting output: {name}")
    return entries, errors, skipped


def main():
    checked, missing = references()
    entries, errors, skipped = manifest()
    answers, launch_errors = launches()
    errors.extend(launch_errors)
    placed, placed_errors = placed_inputs(entries)
    errors.extend(placed_errors)
    errors.extend(ascii_helpers())
    errors.extend(f"RUN-SHEET.md names missing input: {name}" for name in missing)
    if errors:
        for error in errors[:100]:
            print("ERROR", error)
        if len(errors) > 100:
            print(f"... {len(errors) - 100} errors not printed")
        return 1
    print(f"OK: {len(checked)} pre-sitting file references exist; generated names are declared; "
          f"SHA256.txt matches all {len(entries)} other files; {skipped} sitting outputs skipped by name "
          f"(evidence/, backups/, run-*/, data-*/, reads-*/, top-level a-*, log-*, restore-plan.txt, kept-local.txt)")
    print(f"OK: launches {', '.join(LAUNCHES)} each in the table, Steps and Decides, each with a launch file; "
          f"{answers} compared answers each sent; {placed} placed inputs each with its manifest hash; "
          f"no GUID-shaped text")
    print("OK: every helper is ASCII; the new Python helpers name their encoding")
    return 0


if __name__ == "__main__":
    sys.exit(main())
