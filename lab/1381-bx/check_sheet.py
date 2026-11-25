#!/usr/bin/env python3
"""Check the sheet's file references, its arms and start lines, the lines that hold the box to a passed check, and the
complete three-column manifest. Python 3.10, standard library only; reads only, writes nothing.

Row 1381-BX's check, in the shape of row 2162-PA's check_sheet.py (lab/2162-pa at f682a6aa), with this row's own
checks: its five arms (LN, SN, SC, SY, ST) each in the arm table, Steps and Decides; each arm's start line exactly once;
the lines of box.ps1, boxcheck.py and arm.py that keep the box from being pressed but by a passed check, each exactly
once, among them arm.py's line that gives ST no guard, no check and no answer file; box.ps1's old default No absent; arm.py's arms equal to the table's; no GUID-shaped text in the two rule files; and
no value that names the build (BUILD.txt's hex fields but CODE COMMIT) in any file but BUILD.txt. Only local/ is skipped:
any other file outside SHA256.txt is an error. While BUILD.txt holds <TO FILL> fields it says how many and still exits 0
(the sheet is whole; build.py check is what stops the sitting).
"""

import hashlib
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent
SUFFIXES = (".py", ".ps1", ".md", ".txt")
ROW = re.compile(r"([0-9a-f]{64})  ([0-9]+)  (.+)")
ARMS = ("LN", "SN", "SC", "SY", "ST")
ARM_TABLE = {"LN": ("masters", "No", "in/ledger-input.txt"), "SN": ("stock", "No", "in/stock-input.txt"),
             "SC": ("stock", "Cancel", "in/stock-input.txt"), "SY": ("stock", "Yes", "in/stock-input.txt"),
             "ST": ("stock", "none", "in/stock-large-input.txt")}
SKIP_DIRS = {"local"}
# Names the sheet cites that are other rows' files or files the sitting writes, never this folder's.
FOREIGN = {"pa.py", "starth.ps1", "requests/confirm-masters-input.txt", "copies.txt", "s3-listing.txt"}
START = ('( cd /c/bridge-lab/1381-bx && if [ -e local/evidence/{a}-out.txt ]; then echo "STOP: local/evidence/{a}-out.txt '
         'exists"; else python -I -X utf8 arm.py {a} < /dev/null > local/evidence/{a}-out.txt 2>&1; echo "arm exit $?" >> '
         'local/evidence/{a}-out.txt; fi )')
HELD = (
    ("box.ps1", "if ($choice -ne 'Yes' -and $choice -ne 'No' -and $choice -ne 'Cancel') { Log \"NO VALID ANSWER by "
                "$AnswerBy s (read '$choice'): nothing pressed\"; Shot 'unanswered'; exit 6 }  # 1381-BX"),
    ("boxcheck.py", "    if ans in ANSWERS:\n        with open(f'{D}/{tag}.answer.tmp', 'w', encoding='utf-8') as f:\n"
                    "            f.write(ans)\n        os.replace(f'{D}/{tag}.answer.tmp', f'{D}/{tag}.answer')\n"),
    ("boxcheck.py", "    answer(tag, ans if ok else None, out)\n"),
    ("arm.py", "'-SeeWithin', '30', '-AnswerBy', '90'"),
    ("arm.py", "g.append(f'{utc()} GUARD PASSED: boxcheck.py started')"),
    ("arm.py", "ARMS = {'LN': ('masters', 'No', 'in/ledger-input.txt'), 'SN': ('stock', 'No', 'in/stock-input.txt'),\n"
               "        'SC': ('stock', 'Cancel', 'in/stock-input.txt'), 'SY': ('stock', 'Yes', 'in/stock-input.txt'),\n"
               "        'ST': ('stock', None, 'in/stock-large-input.txt')}\n"),
    ("arm.py", "    if ans is None:  # ST: no guard, no check, no answer file, whatever the box shows\n"),
)
# box.ps1's default No at its deadline, as row 2162-SALES-Q's copy has it; this row's copy must not.
GONE = (("box.ps1", "$choice = 'No' }"),)
GUID_SHAPE = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
BUILD_HEX = re.compile(r"([A-Z0-9 ]+): ([0-9a-f]{40}|[0-9a-f]{64})")


def section(text, title):
    start = text.index(f"\n## {title}\n")
    end = text.find("\n## ", start + 1)
    return text[start:] if end < 0 else text[start:end]


def references():
    missing, checked = [], set()
    for doc in ("RUN-SHEET.md", "LAB-COMMON-BX.md"):
        # Inline code only, read line by line, with fenced blocks left out: a fence's three backticks would
        # otherwise pair with the next inline one and hide every name after it.
        text = re.sub(r"(?ms)^\s*```.*?^\s*```[ \t]*$", "", (ROOT / doc).read_text(encoding="utf-8"))
        for token in (t for line in text.splitlines() for t in re.findall(r"`([^`]+)`", line)):
            if any(mark in token for mark in ("*", "<", ">", ":", "…")) or any(c.isspace() for c in token):
                continue
            name = token.replace("\\", "/")
            if name.startswith(("/", "../", "src-tauri/", "lab/", "local/")) or name in FOREIGN:
                continue
            if not (name.endswith(SUFFIXES) or name == ".gitattributes"):
                continue
            checked.add(name)
            if not (ROOT / name).is_file():
                missing.append(name)
    return sorted(checked), sorted(set(missing))


def arms():
    text = (ROOT / "RUN-SHEET.md").read_text(encoding="utf-8")
    errors = []
    table = section(text, "The questions and the arms")
    steps = section(text, "Steps")
    decides = section(text, "Decides")
    for arm in ARMS:
        mode, answer, rel = ARM_TABLE[arm]
        row = re.search(rf"\n\| {arm} \| `{mode}`[^|]*\| `{re.escape(rel)}` \| {answer} \|", table)
        if not row:
            errors.append(f"arm {arm} missing from the arm table, or not with mode {mode}, input {rel}, answer {answer}")
        if not re.search(rf"\*\*{arm}: ", steps):
            errors.append(f"arm {arm} has no step heading")
        if not re.search(rf"\b{arm}\b", decides):
            errors.append(f"arm {arm} missing from Decides")
        if text.count(START.format(a=arm)) != 1:
            errors.append(f"arm {arm}: the sheet does not hold its start line exactly once")
    for name, line in HELD:
        try:
            held = (ROOT / name).read_text(encoding="utf-8").count(line)
        except OSError as e:
            held = f"unreadable ({e})"
        if held != 1:
            errors.append(f"{name} does not hold exactly once {line.strip()[:60]!r} (found {held})")
    for name, line in GONE:
        if line in (ROOT / name).read_text(encoding="utf-8"):
            errors.append(f"{name} still holds {line!r}")
    for doc in ("RUN-SHEET.md", "LAB-COMMON-BX.md"):
        for match in GUID_SHAPE.finditer((ROOT / doc).read_text(encoding="utf-8")):
            errors.append(f"GUID-shaped text in {doc}: {match.group(0)[:8]}...")
    return errors


def build_values(files):
    text = (ROOT / "BUILD.txt").read_text(encoding="utf-8")
    values = [(k, v) for k, v in BUILD_HEX.findall(text) if k != "CODE COMMIT"]
    errors = []
    for name in files:
        if name == "BUILD.txt":
            continue
        body = (ROOT / name).read_bytes().decode("utf-8", "replace")
        for key, value in values:
            if value in body:
                errors.append(f"{name} carries BUILD.txt's {key}; only BUILD.txt names the build")
    return len(re.findall(r"^[A-Z0-9 ]+: <TO FILL", text, re.M)), errors


def manifest():
    entries, errors = {}, []
    for line in (ROOT / "SHA256.txt").read_text(encoding="utf-8").splitlines():
        match = ROW.fullmatch(line)
        if not match:
            errors.append(f"bad manifest row: {line!r}")
            continue
        digest, size, name = match.groups()
        parts = pathlib.PurePosixPath(name)
        if name in entries or parts.is_absolute() or ".." in parts.parts or parts.parts[0] in SKIP_DIRS:
            errors.append(f"duplicate or unsafe path: {name}")
            continue
        entries[name] = (digest, int(size))
    actual = {}
    for path in ROOT.rglob("*"):
        rel = path.relative_to(ROOT)
        if not path.is_file() or rel.as_posix() == "SHA256.txt" or rel.parts[0] in SKIP_DIRS:
            continue
        actual[rel.as_posix()] = path
    for name in sorted(set(entries) - set(actual)):
        errors.append(f"manifest names missing file: {name}")
    for name in sorted(set(actual) - set(entries)):
        errors.append(f"file absent from manifest: {name}")
    for name, path in actual.items():
        if name in entries:
            raw = path.read_bytes()
            if entries[name] != (hashlib.sha256(raw).hexdigest(), len(raw)):
                errors.append(f"hash or size differs: {name}")
    return sorted(actual), errors


def main():
    try:
        checked, missing = references()
        files, errors = manifest()
        errors.extend(arms())
        todo, build_errors = build_values(files)
        errors.extend(build_errors)
    except (OSError, ValueError) as e:
        print(f"FAILED READ: {type(e).__name__}: {e}")
        return 2
    errors.extend(f"a rule file names a missing file: {name}" for name in missing)
    if errors:
        for error in errors[:100]:
            print("ERROR", error)
        if len(errors) > 100:
            print(f"... {len(errors) - 100} errors not printed")
        return 1
    print(f"OK: {len(checked)} file references exist; SHA256.txt matches all {len(files)} other files; only local/ skipped")
    print(f"OK: arms {', '.join(ARMS)} each in the table (as arm.py runs them), Steps and Decides, each start line once; "
          f"the box held to a passed check ({len(HELD)} lines once each, box.ps1's default No gone); no GUID-shaped text")
    print(f"OK: no file but BUILD.txt names the build; BUILD.txt holds {todo} field(s) <TO FILL>"
          + (" (build.py check stops the sitting until they are filled)" if todo else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
