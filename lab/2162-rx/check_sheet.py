#!/usr/bin/env python3
"""Check pre-sitting file references, the arm names, and the complete three-column manifest."""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".xml", ".json", ".md", ".txt")
GENERATED = {"rates.xml", "restore-plan.txt", "kept-local.txt", "launch-env.json", "client-log.txt"}
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
# Row 2162-RX's arms, in sitting order. Every arm after A0 runs the sheet's arm frame, whose labels use <arm>.
ARMS = ("A0", "C313", "C314", "LVS", "LVT", "W1", "W2")
FRAME_ARMS = ARMS[1:]
# Arms, launches and helpers of row 2162-RW that this row removed; none may be named in RUN-SHEET.md.
REMOVED = re.compile(r"\b(?:A1a|A1b|A1c|A2|A3|A4|L0|L3|L4|run-0|run-3|run-4|data-0|data-3|data-4)\b|clearing rule|quit-and-start")
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
SEND = re.compile(r"`([A-Za-z0-9<>-]+)`: `([^`]+)`")
ANSWER = re.compile(r"`(a-[^`\s]+)`|\b(a-[A-Za-z0-9<>-]+-(?:M-marks|rates|guids)\.xml)\b")


def generated(name):
    return (
        name in GENERATED
        or name.startswith("a-")
        or name == "launch/X-launch-env.json"
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
    actual = {
        path.relative_to(ROOT).as_posix(): path
        for path in ROOT.rglob("*")
        if path.is_file() and path.name != "SHA256.txt"
    }
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
          "no removed arm named; no GUID-shaped text")
    return 0


if __name__ == "__main__":
    sys.exit(main())
