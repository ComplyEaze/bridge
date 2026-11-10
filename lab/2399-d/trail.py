#!/usr/bin/env python3
# trail.py EGRESS_FILE [FIRST_LINE] [--expect TOOL SENT]: reads Bridge's egress log (agent-egress.jsonl, one JSON object a line) and
# prints, for each `response_prepared` record from FIRST_LINE on (default 1), its line number, time, tool,
# receipt id and request trail (bridge#918): sends in all (`sent`), how many the trail does not list (`omitted`),
# `failed`, and for the listed sends a count of each kind (`post`, `status`), the sum of request bytes and the sum
# of response bytes. It counts parsed fields, never text. A call that sent nothing has no trail and prints
# `trail none`. It records and never decides, except under --expect (below).
# --expect TOOL SENT is the calibration: the LAST `response_prepared` record of TOOL must carry a trail whose `sent`
# is SENT, else it exits 3 and prints CALIBRATION FAILED (the counter is not trusted until it matches a count that
# was fixed before the row). The listed sends must also number `sent` minus `omitted` and each carry an integer
# `seq`, else STOP.
# STOPs (exit 1): the file is missing or empty, a line does not parse as one JSON object, a record has a
# `request_trail` that is not an object with an integer `sent`, or more than LIMIT records would print.
# A file that parsed whole and holds no `response_prepared` record prints `no records` and exits 0: that is a result
# (the log holds none), and the line counts say how many lines it did read, so "none found" is never "failed to read".
import json, pathlib, sys
LIMIT = 200
expect = None
if "--expect" in sys.argv:
    at = sys.argv.index("--expect")
    expect = (sys.argv[at + 1], int(sys.argv[at + 2]))
    del sys.argv[at:at + 3]
path = pathlib.Path(sys.argv[1])
first = int(sys.argv[2]) if len(sys.argv) > 2 else 1
if not path.is_file() or path.stat().st_size == 0:
    raise SystemExit(f"STOP: {path} is missing or empty (the request failed; this is not 'no record')")
kinds, shown, lines, last_of = {}, 0, 0, {}
with open(path, "rb") as handle:
    for number, raw in enumerate(handle, 1):
        lines = number
        try:
            record = json.loads(raw.decode("utf-8"))
        except ValueError as error:
            raise SystemExit(f"STOP: line {number} does not parse as JSON ({error})")
        if not isinstance(record, dict):
            raise SystemExit(f"STOP: line {number} is not a JSON object")
        kind = record.get("record_type")
        kinds[kind] = kinds.get(kind, 0) + 1
        if kind != "response_prepared" or number < first:
            continue
        shown += 1
        if shown > LIMIT:
            raise SystemExit(f"STOP: more than {LIMIT} response_prepared records from line {first}; pass a later FIRST_LINE")
        trail = record.get("request_trail")
        head = f"line {number}\t{record.get('ts')}\t{record.get('tool')}\t{record.get('receipt_id')}"
        if trail is None:
            print(f"{head}\ttrail none")
            continue
        if not isinstance(trail, dict) or not isinstance(trail.get("sent"), int):
            raise SystemExit(f"STOP: line {number}: request_trail without an integer `sent`")
        last = trail.get("last") or []
        if len(last) != trail["sent"] - (trail.get("omitted") or 0) or not all(isinstance(s.get("seq"), int) for s in last):
            raise SystemExit(f"STOP: line {number}: the listed sends do not number sent minus omitted, or lack a seq")
        last_of[record.get("tool")] = trail["sent"]
        by_kind = {}
        for send in last:
            by_kind[send.get("kind")] = by_kind.get(send.get("kind"), 0) + 1
        req = sum(send.get("request_bytes", 0) for send in last)
        resp = sum(send.get("response_bytes", 0) for send in last)
        print(f"{head}\tsent {trail['sent']}\tomitted {trail.get('omitted')}\tfailed {trail.get('failed')}\t"
              f"listed {len(last)} ({', '.join(f'{k} {v}' for k, v in sorted(by_kind.items(), key=str))})\t"
              f"listed request bytes {req}\tlisted response bytes {resp}")
print("no records" if kinds.get("response_prepared", 0) == 0 else f"records printed {shown}")
print(f"lines read {lines}; records by type {dict(sorted(kinds.items(), key=str))}")
if expect is not None:
    got = last_of.get(expect[0])
    if got != expect[1]:
        print(f"CALIBRATION FAILED: the last {expect[0]} record has sent {got}, expected {expect[1]}")
        raise SystemExit(3)
    print(f"calibration ok: the last {expect[0]} record has sent {got}")
