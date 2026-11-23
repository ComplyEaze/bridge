#!/usr/bin/env python3
"""Check pre-sitting file references and the complete three-column manifest."""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".xml", ".json", ".md", ".txt")
GENERATED = {"rates.xml", "restore-plan.txt", "kept-local.txt", "launch-env.json", "client-log.txt"}
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")


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
    errors.extend(f"RUN-SHEET.md names missing input: {name}" for name in missing)
    if errors:
        for error in errors[:100]:
            print("ERROR", error)
        if len(errors) > 100:
            print(f"... {len(errors) - 100} errors not printed")
        return 1
    print("OK: every pre-sitting file reference exists; generated names are declared; SHA256.txt matches every other file")
    return 0


if __name__ == "__main__":
    sys.exit(main())
