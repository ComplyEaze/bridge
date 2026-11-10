#!/usr/bin/env python3
# gen.py I-0105.xml N-0105.xml PB-L-ledger.xml DAY-REQUEST.xml: writes this row's four request files from earlier
# rows' files, by replacing exact strings and nothing else. Every replacement is counted and must occur exactly as
# many times as stated, else it stops; every input's sha256 must be the one named here, else it stops.
#   I-0105.xml   row 2464-RS's (lab/2464-rs-sheet at 29b06f41), sha256 b333331a34b4834a...  -> I-0399.xml
#   N-0105.xml   row 2464-RS's, sha256 588f998d6a820735...                                   -> N-0399.xml
#   PB-L-ledger  row 2346-PB's L-ledger.xml (lab/2346-pb-sheet at 53540665; row 2346-TD sent it again with another
#                name), sha256 00ae11c328069b7d...                                           -> I-L2399.xml
#   DAY-REQUEST  src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/vouchers-forex-composite-20260915.request.xml
#                (the build's day-window `vouchers` request as sent live on 25 Sep 2026), sha256 dd2352aff9aa5b0b... -> D-empty.xml
# Output is UTF-16LE with a BOM, as the lab sends. Run: python3 -I gen.py <four inputs in that order>
import hashlib, pathlib, sys
here = pathlib.Path(__file__).resolve().parent
INPUTS = [("b333331a34b4834a", "utf-16"), ("588f998d6a820735", "utf-16"), ("00ae11c328069b7d", "utf-16"), ("dd2352aff9aa5b0b", "ascii")]
def load(path, prefix, kind):
    raw = pathlib.Path(path).read_bytes()
    if hashlib.sha256(raw).hexdigest()[:16] != prefix:
        raise SystemExit(f"{path}: sha256 does not start {prefix}: stop")
    if kind == "utf-16":
        if raw[:2] != b"\xff\xfe":
            raise SystemExit(f"{path}: no UTF-16LE byte-order mark")
        return raw[2:].decode("utf-16-le")
    return raw.decode("ascii")
def swap(text, old, new, times):
    if text.count(old) != times:
        raise SystemExit(f"{old!r} occurs {text.count(old)} time(s), expected {times}: stop")
    return text.replace(old, new)
def save(name, text):
    (here / name).write_bytes(b"\xff\xfe" + text.encode("utf-16-le"))
i, n, ledger, day = (load(p, h, k) for p, (h, k) in zip(sys.argv[1:5], INPUTS))
NUMBER_OLD, NUMBER_NEW = "BP/26-27/0105", "BP/26-27/0399"
REMOTE_OLD, REMOTE_NEW = "00000000-0000-4000-8000-000000001105", "00000000-0000-4000-8000-000000002399"
save("I-0399.xml", swap(swap(i, NUMBER_OLD, NUMBER_NEW, 2), REMOTE_OLD, REMOTE_NEW, 1))  # REFERENCE and VOUCHERNUMBER
save("N-0399.xml", swap(n, NUMBER_OLD, NUMBER_NEW, 1))
# PB's ledger create with the name changed in its two places (the attribute and the NAME element).
save("I-L2399.xml", swap(ledger, "BRIDGE Walk-in BC", "BRIDGE Census Probe 2399", 2))
# The day-window request with the company and the day changed: SVFROMDATE, SVTODATE and the formula's two dates.
d = swap(day, "BRIDGE CORPUS FOREX", "BRIDGE PILOT LAB", 1)
save("D-empty.xml", swap(d, "20260915", "20270214", 4))
print("wrote I-0399.xml N-0399.xml I-L2399.xml D-empty.xml")
