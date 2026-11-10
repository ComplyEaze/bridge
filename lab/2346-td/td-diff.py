# td-diff.py: compares one of this row's answers with another answer, and prints what differs. It records; it never
# decides, and it exits 0 whether the two are equal or not. It exits 1 (STOP) only when it cannot compare.
#
#   python3 -I td-diff.py --map  TD_ANSWER PB_ANSWER   row 2346-PB's answer, with PB's customer name turned into
#                                                      this row's in PB's text first (the one change gen-td.py made)
#   python3 -I td-diff.py --same TD_ANSWER OTHER       two answers of this row, compared as they are
#
# It prints:
#   - each file's size and sha256, and whether the bytes are equal as they are;
#   - with --map, how many times PB's name was turned into this row's;
#   - EQUAL, or DIFFER with the number of differing lines and the first 40 of them (a bounded output, AGENTS.md P5).
# Answers are decoded as Tally sends them: UTF-16LE (with or without a BOM) when the second byte is zero, else UTF-8.
import difflib
import hashlib
import sys

OLD = "BRIDGE Walk-in BC"
NEW = "BRIDGE Walkin BC"
SHOWN = 40


def stop(message):
    print(f"STOP: {message}")
    sys.exit(1)


def read(path):
    try:
        with open(path, "rb") as handle:
            data = handle.read()
    except OSError as error:
        stop(f"{path}: {error.strerror}")
    if not data:
        stop(f"{path}: empty, nothing to compare")
    if data.startswith(b"\xff\xfe"):
        return data, data[2:].decode("utf-16-le")
    if len(data) > 1 and data[1] == 0:
        return data, data.decode("utf-16-le")
    return data, data.decode("utf-8")


args = sys.argv[1:]
if len(args) != 3 or args[0] not in ("--map", "--same"):
    stop("usage: td-diff.py --map|--same TD_ANSWER OTHER_ANSWER")
mode, td_path, other_path = args
td_bytes, td_text = read(td_path)
other_bytes, other_text = read(other_path)
for path, data in ((td_path, td_bytes), (other_path, other_bytes)):
    print(f"{path}: {len(data)} B sha256 {hashlib.sha256(data).hexdigest()}")
print(f"bytes as they are: {'equal' if td_bytes == other_bytes else 'not equal'}")
if mode == "--map":
    if NEW in other_text:
        stop(f"{other_path} already holds this row's name")
    print(f"name mapped: {other_text.count(OLD)} place(s) in {other_path}")
    other_text = other_text.replace(OLD, NEW)
if td_text == other_text:
    print("EQUAL")
    sys.exit(0)
lines = list(
    difflib.unified_diff(
        other_text.splitlines(), td_text.splitlines(), other_path, td_path, n=0, lineterm=""
    )
)
changed = [line for line in lines if line[:1] in "+-" and not line.startswith(("+++", "---"))]
print(f"DIFFER: {len(changed)} line(s) differ")
for line in lines[:SHOWN]:
    print(f"  {line}")
if len(lines) > SHOWN:
    print(f"  ... {len(lines) - SHOWN} more diff line(s) not shown")
