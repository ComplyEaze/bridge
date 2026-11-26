#!/usr/bin/env python3
"""Row 1381-RR's records around the one restore (LAB-COMMON v3 C6's before and after), made from files; never a key in Tally.

Written for row 1381-RR, in the shape of row 1108-R3's c6prep.py (lab/1108-r3-equal-voucher, placeholder-helpers/):
the same checks and records, with the company number taken from the backup's own file name (so no copy with machine
strings replaced is needed), Tally's data folder given on the command line, and an after-record that lists the data
folder again and prints the difference. Not product code. Python 3.10, standard library only. Reads Tally's files
only; changes none. Writes only the record files named below (and, for the self-test, its SCRATCH).

  restore.py precheck DATA before the shut, with the book still loaded: BK-S as `plan` checks it; the fallback
                           folder (row 1381-O's backup-1) holds exactly one file TDBK1800_<n>.001 of its recorded
                           bytes and sha256, with the same company number as BK-S; DATA/<company number> exists and
                           can be listed. Writes local/precheck.txt. Prints `PRECHECK MET: ...` (exit 0).
  restore.py plan DATA     before the restore, after the shut, and only after a precheck that met (local/precheck.txt
                           present, local/evidence/r-precheck.txt holding `PRECHECK MET:` and ending `exit 0`, the
                           same company number as now): the source folder holds exactly one file, named
                           TDBK1800_<company number>.001, of the recorded bytes and sha256 (BK-S, row 1108-R3);
                           the company's data folder DATA/<company number> exists. Writes local/restore-plan.txt and
                           local/r-before-listing.txt. Prints `PLAN WRITTEN: ...` (exit 0).
  restore.py after DATA    after the restore, back at Select Company, before the load: the source unchanged; the
                           data folder listed again into local/r-after-listing.txt; the difference printed by name.
                           Prints `AFTER RECORDED: ...` (exit 0), with `NO DIFFERENCE` on its own line when the two
                           listings are equal (a measurement, not a STOP).
  restore.py --selftest SCRATCH   the rules on synthetic folders in SCRATCH (under local/, or outside the row folder).

DATA is Tally's data folder, the Restore Data Path, with forward slashes. The listing is flat: one line per entry of
the company's data folder, `<file|dir> <bytes> <modified, local, to the second> <name>`.

Exit 0: written. Exit 1: STOP-RESTORE-SOURCE (a backup folder absent or not as recorded, the two numbers differing,
or the data folder absent before the restore) or STOP-RESTORE (the company's data folder absent after the restore).
"Absent" is only FileNotFoundError or NotADirectoryError on the path itself. Exit 2: REFUSED (a record exists, or the
precheck or the plan is missing), or FAILED READ (any other error reading a folder or file, or writing a record, or a data path that
is not a directory): never reported as "nothing found" or as a restore that did not complete. Printed text never holds the company
number or DATA: each is replaced by `<company number>` and `<DATA>`; the plan and the two listings stay on the lab PC.
"""

import hashlib
import os
import pathlib
import re
import stat
import sys
from datetime import datetime

ROOT = pathlib.Path(__file__).resolve().parent
SOURCE = "C:/bridge-lab/1108r3/backups/bks"
WANT_SHA = "ecc8d4028b88b763e5a8a22014bc699b07db68c3eb32cff3c6c92052b373b0f4"
WANT_BYTES = 24219
FALLBACK = "C:/lab/1381-o/backup-1"
FALLBACK_SHA = "d23f881a6bac8e1f9bf53035e7f21a72234f9f57c3672a965bc23a3dd76d44eb"
FALLBACK_BYTES = 24219
NAME = re.compile(r"TDBK1800_([0-9]+)\.001")
LISTED = "the Restore list must show BRIDGE STOCK CREATE LAB with its number and 19-Nov-26 (7:40 pm) (lab 1108-R3, step 9)"


class Stop(Exception):
    def __init__(self, name, why, code):
        super().__init__(why)
        self.name, self.why, self.code = name, why, code


def source_file(source, want_sha, want_bytes, what="the source"):
    """(file name, company number, bytes, modified) of the one backup in SOURCE, or Stop. WHAT names it."""
    try:
        names = sorted(os.listdir(source))
    except (FileNotFoundError, NotADirectoryError):
        raise Stop("STOP-RESTORE-SOURCE", f"{what} folder is absent", 1)
    except OSError as error:
        raise Stop("FAILED READ", f"{what} folder could not be read ({type(error).__name__})", 2)
    if len(names) != 1:
        raise Stop("STOP-RESTORE-SOURCE", f"{what} folder holds {len(names)} entries, not 1", 1)
    match = NAME.fullmatch(names[0])
    if not match:
        raise Stop("STOP-RESTORE-SOURCE", f"{what} folder's one file is not named TDBK1800_<company number>.001", 1)
    path = os.path.join(source, names[0])
    try:
        raw = open(path, "rb").read()
        modified = datetime.fromtimestamp(os.path.getmtime(path)).strftime("%Y-%m-%d %H:%M:%S")
    except OSError as error:
        raise Stop("FAILED READ", f"{what} backup file could not be read ({type(error).__name__})", 2)
    digest = hashlib.sha256(raw).hexdigest()
    if (digest, len(raw)) != (want_sha, want_bytes):
        raise Stop("STOP-RESTORE-SOURCE", f"{what} backup is {len(raw)} B sha256 {digest}, not the recorded "
                   f"{want_bytes} B {want_sha}", 1)
    return names[0], match.group(1), len(raw), modified


def listing(folder, when):
    """The flat listing of FOLDER as lines, or Stop: absent is STOP-RESTORE(-SOURCE), anything else FAILED READ."""
    try:
        mode = os.lstat(folder).st_mode
    except (FileNotFoundError, NotADirectoryError):
        if when == "before":
            raise Stop("STOP-RESTORE-SOURCE", "the company's data folder is absent before the restore", 1)
        raise Stop("STOP-RESTORE", "the restore of BK-S did not complete: the company's data folder is absent", 1)
    except OSError as error:
        raise Stop("FAILED READ", f"the company's data folder could not be read ({type(error).__name__})", 2)
    if not stat.S_ISDIR(mode):
        raise Stop("FAILED READ", "the company's data folder path is not a directory", 2)
    try:
        lines = []
        for entry in sorted(os.scandir(folder), key=lambda e: e.name):
            st = entry.stat(follow_symlinks=False)
            kind = "dir" if entry.is_dir(follow_symlinks=False) else "file"
            size = 0 if kind == "dir" else st.st_size
            lines.append(f"{kind} {size} {datetime.fromtimestamp(st.st_mtime).strftime('%Y-%m-%dT%H:%M:%S')} {entry.name}")
    except OSError as error:
        raise Stop("FAILED READ", f"the company's data folder could not be read ({type(error).__name__})", 2)
    return lines


def difference(before, after):
    """(added, removed, changed, unchanged) names between two listings."""
    split = lambda lines: {line.split(" ", 3)[3]: line for line in lines}
    b, a = split(before), split(after)
    added = sorted(set(a) - set(b))
    removed = sorted(set(b) - set(a))
    changed = sorted(n for n in set(a) & set(b) if a[n] != b[n])
    unchanged = sorted(n for n in set(a) & set(b) if a[n] == b[n])
    return added, removed, changed, unchanged, b, a


def precheck(data, local, source=SOURCE, want_sha=WANT_SHA, want_bytes=WANT_BYTES, fallback=FALLBACK,
             fallback_sha=FALLBACK_SHA, fallback_bytes=FALLBACK_BYTES):
    record = local / "precheck.txt"
    if record.exists():
        raise Stop("REFUSED", "local/precheck.txt exists: never overwritten", 2)
    name, number, size, _ = source_file(source, want_sha, want_bytes, "the source")
    fname, fnumber, fsize, _ = source_file(fallback, fallback_sha, fallback_bytes, "the fallback")
    if fnumber != number:
        raise Stop("STOP-RESTORE-SOURCE", "the fallback's company number is not BK-S's", 1)
    folder = f"{data}/{number}"
    lines = listing(folder, "before")
    local.mkdir(parents=True, exist_ok=True)
    record.write_text("\n".join([
        "Row 1381-RR precheck, before the shut", f"BK-S: {source}/{name}, {size} B, sha256 {want_sha}",
        f"Fallback: {fallback}/{fname}, {fsize} B, sha256 {fallback_sha}",
        f"Company: BRIDGE STOCK CREATE LAB, number {number} (both files)",
        f"The company's data folder: {folder}, {len(lines)} entries, listed"]) + "\n", encoding="utf-8", newline="\n")
    return (f"PRECHECK MET: the source folder holds one file, {size} B, sha256 equal to recorded; the fallback folder "
            f"holds one file, {fsize} B, sha256 equal to recorded; the same company number in both; the company's "
            f"data folder present and listed, {len(lines)} entries; local/precheck.txt"), number


def plan(data, local, source=SOURCE, want_sha=WANT_SHA, want_bytes=WANT_BYTES):
    plan_file, before = local / "restore-plan.txt", local / "r-before-listing.txt"
    for path in (plan_file, before):
        if path.exists():
            raise Stop("REFUSED", f"local/{path.name} exists: never overwritten", 2)
    record, evidence = local / "precheck.txt", local / "evidence" / "r-precheck.txt"
    if not record.exists():
        raise Stop("REFUSED", "local/precheck.txt is missing: restore.py precheck first", 2)
    if not evidence.exists():
        raise Stop("REFUSED", "local/evidence/r-precheck.txt is missing: restore.py precheck first", 2)
    shown = [line.strip() for line in evidence.read_text(encoding="utf-8").splitlines() if line.strip()]
    if not any(line.startswith("PRECHECK MET:") for line in shown) or shown[-1] != "exit 0":
        raise Stop("REFUSED", "local/evidence/r-precheck.txt holds no PRECHECK MET line ending exit 0: "
                   "restore.py precheck first", 2)
    name, number, size, modified = source_file(source, want_sha, want_bytes)
    checked = re.search(r"(?m)^Company: BRIDGE STOCK CREATE LAB, number ([0-9]+) \(both files\)$",
                        record.read_text(encoding="utf-8"))
    if not checked or checked.group(1) != number:
        raise Stop("STOP-RESTORE-SOURCE", "the source's company number is not the precheck's", 1)
    folder = f"{data}/{number}"
    lines = listing(folder, "before")
    local.mkdir(parents=True, exist_ok=True)
    before.write_text("".join(line + "\n" for line in lines), encoding="utf-8", newline="\n")
    plan_file.write_text("\n".join([
        "Row 1381-RR, LAB-COMMON v3 C6 restore plan: BK-S (row 1108-R3)",
        f"Source folder: {source} (one file)", f"File: {name}", f"Bytes: {size}", f"sha256: {want_sha}",
        f"File modified: {modified} local", f"Source identity: {LISTED}",
        f"Company: BRIDGE STOCK CREATE LAB, number {number}",
        f"Restore Data Path: {data}", f"The company's data folder: {folder} ({len(lines)} entries before)",
        "Source: Specify Path = the source folder above; select BRIDGE STOCK CREATE LAB only; End of List",
        f'Prompts expected, the only ones answered: "Accept ?" = Yes; "Company data folder ({number}) already exists '
        '... Selecting Replace will overwrite the existing folder." = Replace',
        "After: restore.py after; load THE COMPANY alone; screenshots; then M2, I, G, U"]) + "\n",
        encoding="utf-8", newline="\n")
    return (f"PLAN WRITTEN: the source folder holds one file, {size} B, sha256 {want_sha} (equal to recorded), "
            f"modified {modified} local; the company's data folder {len(lines)} entries (local/r-before-listing.txt); "
            f"local/restore-plan.txt"), number


def after(data, local, source=SOURCE, want_sha=WANT_SHA, want_bytes=WANT_BYTES):
    plan_file, before, after_file = local / "restore-plan.txt", local / "r-before-listing.txt", local / "r-after-listing.txt"
    if not plan_file.exists() or not before.exists():
        raise Stop("REFUSED", "local/restore-plan.txt or local/r-before-listing.txt is missing: restore.py plan first", 2)
    if after_file.exists():
        raise Stop("REFUSED", "local/r-after-listing.txt exists: never overwritten", 2)
    name, number, size, _ = source_file(source, want_sha, want_bytes)
    planned = re.search(r"(?m)^Company: BRIDGE STOCK CREATE LAB, number ([0-9]+)$", plan_file.read_text(encoding="utf-8"))
    if not planned or planned.group(1) != number:
        raise Stop("STOP-RESTORE-SOURCE", "the source's company number is not the plan's", 1)
    lines = listing(f"{data}/{number}", "after")
    after_file.write_text("".join(line + "\n" for line in lines), encoding="utf-8", newline="\n")
    added, removed, changed, unchanged, b, a = difference(before.read_text(encoding="utf-8").splitlines(), lines)
    out = [f"AFTER RECORDED: the source unchanged ({size} B, sha256 equal to recorded); the company's data folder "
           f"{len(b)} entries before, {len(a)} after; {len(added)} added, {len(removed)} removed, {len(changed)} "
           f"changed, {len(unchanged)} unchanged (local/r-after-listing.txt)"]
    out += [f"  added: {a[n]}" for n in added] + [f"  removed: {b[n]}" for n in removed]
    out += [f"  changed: {b[n]}  ->  {a[n]}" for n in changed]
    if not (added or removed or changed):
        out.append("NO DIFFERENCE")
    return "\n".join(out), number


def run(command, data, local, **kw):
    """Print one command's lines with the number and DATA masked; return its exit code."""
    number = None
    try:
        text, number = {"precheck": precheck, "plan": plan, "after": after}[command](data, local, **kw)
        code = 0
    except Stop as stop:
        text, code = f"{stop.name}: {stop.why}", stop.code
    except (OSError, UnicodeDecodeError) as error:  # a record or listing that fails is never a STOP-RESTORE
        text, code = f"FAILED READ: restore.py {command} could not read or write a record ({type(error).__name__})", 2
    if number:  # the number as a whole token only, so a sha256 holding its digits is left as it is
        text = re.sub(rf"(?<![0-9A-Za-z]){number}(?![0-9A-Za-z])", "<company number>", text)
    print(text.replace(str(data), "<DATA>"))
    return code


def selftest(scratch):
    scratch = pathlib.Path(scratch).resolve()
    if (scratch == ROOT or ROOT in scratch.parents) and (ROOT / "local") not in scratch.parents:
        print(f"FAILED READ: {scratch} is in the row folder but not under local/; the self-test writes nowhere else")
        return 2
    number, body, fbody = "73519", b"synthetic backup bytes\x00\x01", b"synthetic fallback bytes\x02"
    sha, size = hashlib.sha256(body).hexdigest(), len(body)
    fsha, fsize = hashlib.sha256(fbody).hexdigest(), len(fbody)
    right = total = 0

    def lay(n, prechecked=True):
        base = scratch / f"case{n}"
        src, data, local = base / "src", base / "data", base / "local"
        for folder in (src, data / number):
            folder.mkdir(parents=True, exist_ok=True)
        (src / f"TDBK1800_{number}.001").write_bytes(body)
        (base / "fb").mkdir(parents=True, exist_ok=True)
        (base / "fb" / f"TDBK1800_{number}.001").write_bytes(fbody)
        (data / number / "Company.1800").write_bytes(b"c" * 10)
        (data / number / "Manager.1800").write_bytes(b"m" * 20)
        if prechecked:  # as step 2's line: precheck's lines, then `exit <code>`, in local/evidence/r-precheck.txt
            code, text = go("precheck", src, data.as_posix(), local)
            (local / "evidence").mkdir(parents=True, exist_ok=True)
            (local / "evidence" / "r-precheck.txt").write_text(f"{text}exit {code}\n", encoding="utf-8")
        return src, data.as_posix(), local

    def go(command, src, data, local):
        out = scratch / "out.txt"
        saved = sys.stdout
        with open(out, "w", encoding="utf-8") as f:
            sys.stdout = f
            kw = {"fallback": str(src.parent / "fb"), "fallback_sha": fsha, "fallback_bytes": fsize} \
                if command == "precheck" else {}
            try:
                code = run(command, data, local, source=str(src), want_sha=sha, want_bytes=size, **kw)
            except Exception as error:  # an error that escapes run() is a WRONG case, never a pass
                code = f"escaped: {type(error).__name__}"
            finally:
                sys.stdout = saved
        return code, out.read_text(encoding="utf-8")

    def failing(name, path, fn):
        """FN() with os.NAME raising PermissionError for PATH only (portable: no file modes), then restored."""
        real = getattr(os, name)

        def fake(target, *a, **k):
            if os.path.normcase(os.fspath(target)) == os.path.normcase(os.fspath(path)):
                raise PermissionError(13, "synthetic", os.fspath(target))
            return real(target, *a, **k)
        setattr(os, name, fake)
        try:
            return fn()
        finally:
            setattr(os, name, real)

    def case(what, got, want):
        nonlocal right, total
        total += 1
        right += got == want
        print(f"{'RIGHT' if got == want else 'WRONG'} {what}: {got!r}")

    src, data, local = lay(1)
    code, text = go("plan", src, data, local)
    case("plan, all as recorded: exit and first word", (code, text.split(":")[0]), (0, "PLAN WRITTEN"))
    case("plan, all as recorded: the printed text holds neither the number nor DATA",
         number in text or data in text, False)
    case("plan, all as recorded: the plan holds the number", f"number {number}" in (local / "restore-plan.txt").read_text(), True)
    case("plan, again: refused", go("plan", src, data, local)[0], 2)
    code, text = go("after", src, data, local)
    case("after, nothing changed: exit and NO DIFFERENCE", (code, text.splitlines()[-1]), (0, "NO DIFFERENCE"))
    case("after, again: refused", go("after", src, data, local)[0], 2)
    src, data, local = lay(2)
    go("plan", src, data, local)
    pathlib.Path(data, number, "Company.1800").write_bytes(b"c" * 11)
    pathlib.Path(data, number, "Tranmgr.1800").write_bytes(b"t")
    pathlib.Path(data, number, "Manager.1800").unlink()
    code, text = go("after", src, data, local)
    case("after, one changed, one added, one removed: counts",
         (code, "1 added, 1 removed, 1 changed, 0 unchanged" in text, "NO DIFFERENCE" in text), (0, True, False))
    src, data, local = lay(3)
    (src / "second.001").write_bytes(b"x")
    code, text = go("plan", src, data, local)
    case("plan, two files in the source: STOP-RESTORE-SOURCE",
         (code, text.startswith("STOP-RESTORE-SOURCE: the source folder holds 2 entries")), (1, True))
    src, data, local = lay(4)
    (src / f"TDBK1800_{number}.001").write_bytes(body + b"!")
    case("plan, other bytes: STOP-RESTORE-SOURCE", go("plan", src, data, local)[0], 1)
    src, data, local = lay(5)
    (src / f"TDBK1800_{number}.001").rename(src / "backup.001")
    case("plan, a file not named TDBK1800_<n>.001: STOP-RESTORE-SOURCE", go("plan", src, data, local)[0], 1)
    src, data, local = lay(6)
    code, text = go("plan", src / "absent", data, local)
    case("plan, the source folder absent: STOP-RESTORE-SOURCE", (code, text.split(":")[0]), (1, "STOP-RESTORE-SOURCE"))
    code, text = failing("listdir", src, lambda: go("plan", src, data, local))
    case("plan, the source folder unreadable: FAILED READ, not absent", (code, text.split(":")[0]), (2, "FAILED READ"))
    src, data, local = lay(7)
    code, text = go("plan", src, data + "/elsewhere", local)
    case("plan, the data folder absent: STOP-RESTORE-SOURCE", (code, text.split(":")[0]), (1, "STOP-RESTORE-SOURCE"))
    src, data, local = lay(8)
    case("after, no plan: refused", go("after", src, data, local)[0], 2)
    go("plan", src, data, local)
    for entry in pathlib.Path(data, number).iterdir():
        entry.unlink()
    pathlib.Path(data, number).rmdir()
    code, text = go("after", src, data, local)
    case("after, the data folder absent: STOP-RESTORE, the restore did not complete",
         (code, text.startswith("STOP-RESTORE: the restore of BK-S did not complete")), (1, True))
    src, data, local = lay(9)
    go("plan", src, data, local)
    for entry in pathlib.Path(data, number).iterdir():
        entry.unlink()
    code, text = go("after", src, data, local)
    case("after, the data folder present and empty: recorded, distinct from absent",
         (code, "2 entries before, 0 after; 0 added, 2 removed" in text), (0, True))
    src, data, local = lay(10)
    go("plan", src, data, local)
    (src / f"TDBK1800_{number}.001").write_bytes(body + b"!")
    case("after, the source changed: STOP-RESTORE-SOURCE", go("after", src, data, local)[0], 1)
    # The data path that exists but cannot be read, or is not a directory: FAILED READ, never "absent".
    src, data, local = lay(11)
    code, text = failing("lstat", f"{data}/{number}", lambda: go("plan", src, data, local))
    case("plan, the data folder unreadable (lstat refused): FAILED READ, not absent", (code, text.split(":")[0]),
         (2, "FAILED READ"))
    go("plan", src, data, local)
    code, text = failing("lstat", f"{data}/{number}", lambda: go("after", src, data, local))
    case("after, the data folder unreadable: FAILED READ, not STOP-RESTORE", (code, text.split(":")[0]),
         (2, "FAILED READ"))
    src, data, local = lay(12)
    for entry in pathlib.Path(data, number).iterdir():
        entry.unlink()
    pathlib.Path(data, number).rmdir()
    pathlib.Path(data, number).write_bytes(b"not a folder")
    code, text = go("plan", src, data, local)
    case("plan, the data path a file: FAILED READ, not absent", (code, text), (2, "FAILED READ: the company's data "
                                                                                 "folder path is not a directory\n"))
    # A record that cannot be written: FAILED READ, exit 2 (local/ is a file here).
    src, data, local = lay(13)
    real_write = pathlib.Path.write_text

    def refuse_listing(path, *a, **k):  # the plan's first record write refused (portable: no file modes)
        if path.name == "r-before-listing.txt":
            raise PermissionError(13, "synthetic", str(path))
        return real_write(path, *a, **k)
    pathlib.Path.write_text = refuse_listing
    try:
        code, text = go("plan", src, data, local)
    finally:
        pathlib.Path.write_text = real_write
    case("plan, the record cannot be written: FAILED READ, exit 2", (code, text.split(":")[0]), (2, "FAILED READ"))
    src, data, local = lay(19, prechecked=False)
    local.write_bytes(b"in the way")
    code, text = go("precheck", src, data, local)
    case("precheck, the record cannot be written: FAILED READ, exit 2", (code, text.split(":")[0]), (2, "FAILED READ"))
    # precheck: BK-S, the fallback and the data folder, before the shut.
    src, data, local = lay(14, prechecked=False)
    code, text = go("precheck", src, data, local)
    case("precheck, all as recorded: exit and first word", (code, text.split(":")[0]), (0, "PRECHECK MET"))
    case("precheck, all as recorded: the printed text holds neither the number nor DATA",
         number in text or data in text, False)
    case("precheck, again: refused", go("precheck", src, data, local)[0], 2)
    src, data, local = lay(15, prechecked=False)
    (src.parent / "fb" / f"TDBK1800_{number}.001").write_bytes(fbody + b"!")
    code, text = go("precheck", src, data, local)
    case("precheck, the fallback's bytes not recorded: STOP-RESTORE-SOURCE",
         (code, text.startswith("STOP-RESTORE-SOURCE: the fallback backup is")), (1, True))
    src, data, local = lay(16, prechecked=False)
    (src.parent / "fb" / f"TDBK1800_{number}.001").unlink()
    (src.parent / "fb").rmdir()
    code, text = go("precheck", src, data, local)
    case("precheck, the fallback folder missing: STOP-RESTORE-SOURCE",
         (code, text.strip()), (1, "STOP-RESTORE-SOURCE: the fallback folder is absent"))
    src, data, local = lay(17, prechecked=False)
    (src.parent / "fb" / f"TDBK1800_{number}.001").rename(src.parent / "fb" / "TDBK1800_73520.001")
    code, text = go("precheck", src, data, local)
    case("precheck, the two company numbers differ: STOP-RESTORE-SOURCE",
         (code, text.strip()), (1, "STOP-RESTORE-SOURCE: the fallback's company number is not BK-S's"))
    src, data, local = lay(18, prechecked=False)
    code, text = go("precheck", src, data + "/elsewhere", local)
    case("precheck, a DATA typo: STOP-RESTORE-SOURCE, nothing shut",
         (code, text.strip()), (1, "STOP-RESTORE-SOURCE: the company's data folder is absent before the restore"))
    # plan only after a precheck that met, with the same company number.
    src, data, local = lay(20, prechecked=False)
    code, text = go("plan", src, data, local)
    case("plan, no precheck: refused", (code, text.strip()),
         (2, "REFUSED: local/precheck.txt is missing: restore.py precheck first"))
    src, data, local = lay(21)
    (local / "precheck.txt").unlink()
    code, text = go("plan", src, data, local)
    case("plan, the precheck record gone though its evidence says met: refused", (code, text.strip()),
         (2, "REFUSED: local/precheck.txt is missing: restore.py precheck first"))
    src, data, local = lay(22)
    (local / "evidence" / "r-precheck.txt").write_text("STOP-RESTORE-SOURCE: the fallback folder is absent\nexit 1\n",
                                                        encoding="utf-8")
    code, text = go("plan", src, data, local)
    case("plan, a precheck that failed: refused", (code, text.split(":")[0]), (2, "REFUSED"))
    src, data, local = lay(23)
    evidence = local / "evidence" / "r-precheck.txt"
    evidence.write_text(evidence.read_text(encoding="utf-8").replace("exit 0", "exit 2"), encoding="utf-8")
    code, text = go("plan", src, data, local)
    case("plan, a PRECHECK MET line not ending exit 0: refused", (code, text.split(":")[0]), (2, "REFUSED"))
    src, data, local = lay(24)
    (src / f"TDBK1800_{number}.001").rename(src / "TDBK1800_73520.001")
    code, text = go("plan", src, data, local)
    case("plan, the company number not the precheck's: STOP-RESTORE-SOURCE", (code, text.strip()),
         (1, "STOP-RESTORE-SOURCE: the source's company number is not the precheck's"))
    print(f"{total} cases, {right} right")
    return 0 if right == total else 1


def main(argv):
    if len(argv) == 2 and argv[0] == "--selftest":
        return selftest(argv[1])
    if len(argv) == 2 and argv[0] in ("precheck", "plan", "after") and not argv[1].startswith("<"):
        return run(argv[0], argv[1].rstrip("/"), ROOT / "local")
    print("usage: restore.py precheck DATA | plan DATA | after DATA | --selftest SCRATCH (DATA: Tally's data folder, not a placeholder)")
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
