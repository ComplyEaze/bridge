#!/usr/bin/env python3
"""Check pre-sitting file references, the arm names, the four launch files, and the complete three-column manifest.

Row 2162-RX's check_sheet.py with five changes for row 2162-RC: this row's arms and removed names; the folders the
lab itself adds (local/) and the sitting's own output folders are skipped by name, and so are the top-level files the
sitting's commands generate, by exact pattern (row 2162-RX, RESULTS.md deviation 2), while any other file outside the
manifest is still an error; each launch file must hold exactly the five settings with its own data folder; the two
builds are never called old or new; and each launch's exe path is paired with its own build's hash.
"""

import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".xml", ".json", ".md", ".txt")
GENERATED = {"rates.xml", "restore-plan.txt", "kept-local.txt", "launch-env.json", "client-log.txt"}
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
# Row 2162-RC's arms, in sitting order. G and C run the sheet's arm frame, whose labels use <arm>.
ARMS = ("P", "G", "G1", "G2", "C")
FRAME_ARMS = ("G", "C")
# Arms and launches of row 2162-RX that this row does not run; none may be named in RUN-SHEET.md.
REMOVED = re.compile(r"\b(?:C313|C314|LVS|LVT|W2|L1C|L2|run-1c|run-2|data-1c|data-2)\b")
# The two builds are named by commit; other sheets use these words for other builds.
OLD_NEW = re.compile(r"\b(?:old|new)\s+(?:build|exe|rc)\b|\bbuild\s+(?:old|new)\b", re.IGNORECASE)
# What the lab adds (local/) and what the sitting writes, by name; nothing else outside the manifest is admitted.
SKIP_DIRS = {"local", "evidence", "reads", "dl", "exe-6f541ae4", "backup-1", "scratch",
             "run-p", "run-g1", "run-g2", "run-c", "data-p", "data-g1", "data-g2", "data-c"}
SKIP_FILES = re.compile(r"rates\.xml|kept-local\.txt|a-[A-Za-z0-9-]+-(?:M-marks|rates|guids)\.xml|log-[A-Za-z0-9-]+\.txt")
# Each launch: its file, its data folder, and the build it runs, by exe path and hash.
EXE_8C = ("C:/bridge-lab/2569-r/exe-new/mcpb/bin/x86_64-pc-windows-msvc/bridge_mcp.exe",
          "1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59")
EXE_6F = ("C:/bridge-lab/2162-rc/exe-6f541ae4/bin/x86_64-pc-windows-msvc/bridge_mcp.exe",
          "9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc")
LAUNCHES = {"LP": ("p", EXE_6F), "LG1": ("g1", EXE_8C), "LG2": ("g2", EXE_6F), "LC": ("c", EXE_6F)}
SETTINGS = {"BRIDGE_AGENT_ENABLE_WRITES": "true", "BRIDGE_TERMS_ACCEPTED": "true",
            "BRIDGE_TALLY_HOST": "127.0.0.1", "BRIDGE_TALLY_PORT": "9001"}
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
SEND = re.compile(r"`([A-Za-z0-9<>-]+)`: `([^`]+)`")
ANSWER = re.compile(r"`(a-[^`\s]+)`|\b(a-[A-Za-z0-9<>-]+-(?:M-marks|rates|guids)\.xml)\b")


def generated(name):
    return (
        name in GENERATED
        or name.startswith("a-")
        or name.split("/")[0] in SKIP_DIRS
        or bool(SKIP_FILES.fullmatch(name))
    )


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
    return [name.replace("<arm>", arm) for arm in FRAME_ARMS] if "<arm>" in name else [name]


def section(text, title):
    start = text.index(f"\n## {title}\n")
    end = text.find("\n## ", start + 1)
    return text[start:] if end < 0 else text[start:end]


def arms():
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    errors = []
    table = section(text, "The questions and the one variable per arm")
    steps = section(text, "Steps")
    decides = section(text, "Decides")
    for arm in ARMS:
        if f"\n| {arm} |" not in table:
            errors.append(f"arm {arm} missing from the arm table")
        if not re.search(rf"\*\*{arm}: ", steps):
            errors.append(f"arm {arm} has no step heading")
        if not re.search(rf"\b{arm}\b", decides):
            errors.append(f"arm {arm} missing from Decides")
    for match in REMOVED.finditer(text):
        errors.append(f"removed arm, launch or rule named: {match.group(0)!r}")
    for match in OLD_NEW.finditer(text):
        errors.append(f"a build called old or new: {match.group(0)!r}")
    for name, (tag, (exe, sha)) in LAUNCHES.items():
        data = f"data-{tag}"
        # The literal lines of the launch: its exe hash check, its launch file, and its client start command.
        for line in (f'echo "{sha}  {exe}" | sha256sum -c - > evidence/run-{tag}-hashes.txt',
                     f"cp launch/{name}-launch-env.json run-{tag}/launch-env.json && ",
                     f'python -I client.py "C:/bridge-lab/2162-rc/run-{tag}" "{exe}" "{sha}" > '):
            if text.count(line) != 1:
                errors.append(f"launch {name}: the sheet does not hold exactly once the line {line!r}")
        try:
            launch = json.loads((ROOT / "launch" / f"{name}-launch-env.json").read_text(encoding="utf-8"))
        except (OSError, ValueError) as e:
            errors.append(f"launch {name}: launch file unreadable ({e})")
            continue
        want = dict(SETTINGS, BRIDGE_AGENT_DATA_DIR=f"C:\\bridge-lab\\2162-rc\\{data}\\")
        if launch != want:
            errors.append(f"launch {name}: settings differ from the five this row allows")
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
        path = ROOT / name
        if name in entries or pathlib.PurePosixPath(name).is_absolute() or ".." in pathlib.PurePosixPath(name).parts:
            errors.append(f"duplicate or unsafe path: {name}")
            continue
        entries[name] = (digest, int(size))
    actual = {}
    for path in ROOT.rglob("*"):
        rel = path.relative_to(ROOT)
        if not path.is_file() or rel.as_posix() == "SHA256.txt" or rel.parts[0] in SKIP_DIRS:
            continue
        if len(rel.parts) == 1 and SKIP_FILES.fullmatch(rel.name):
            continue
        actual[rel.as_posix()] = path
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
    return entries, errors


def main():
    checked, missing = references()
    entries, errors = manifest()
    answers, arm_errors = arms()
    errors.extend(arm_errors)
    errors.extend(f"RUN-SHEET.md names missing input: {name}" for name in missing)
    if errors:
        for error in errors[:100]:
            print("ERROR", error)
        if len(errors) > 100:
            print(f"... {len(errors) - 100} errors not printed")
        return 1
    print("OK: every pre-sitting file reference exists; generated names are declared; SHA256.txt matches every other file")
    print(f"OK: arms {', '.join(ARMS)} each in the table, Steps and Decides; {answers} compared answers each sent; "
          "no removed arm named; no build called old or new; no GUID-shaped text")
    print(f"OK: launches {', '.join(LAUNCHES)} each with its five settings, its own data folder, and its build's exe and hash in its literal hash check and client start line")
    return 0


if __name__ == "__main__":
    sys.exit(main())
