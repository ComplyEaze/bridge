#!/usr/bin/env python3
# calls.py: reads the answers client.py saves (answers\<name>.json, one call each) for row 2162-SALES-Q, and writes
# the two calls that need a batch id from an earlier answer. It parses JSON; it never searches the text. Read-only on
# answers; it writes only the call file it is told to write.
#
#   calls.py show ANSWER...       one block per answer: the tool, the time sent, isError, and the fields this row
#                                 grades for that tool (below). Exit 2, printing STOP, when an answer is "the request
#                                 failed": no file, not JSON, a JSON-RPC error, or no structuredContent. A refusal
#                                 (isError true, with an error code) is an answer, printed as "refused <code>".
#   calls.py status-diff A B      two gst_status answers: every item by master_id in one only, and every item whose
#                                 fields differ, field by field; then the counts of each. Exit 2 as for show.
#   calls.py post BUILD OUT       writes OUT (whole, then renamed in), a post_import call with the company and batch id of the build answer
#                                 BUILD; refuses (exit 2) unless BUILD built exactly one voucher.
#   calls.py verify BUILD OUT     the same, a verify_import call.
#   calls.py batch BUILD DATADIR  finds <batch_id>.xml under DATADIR (the launch's data folder), and checks its sha256
#                                 against the build answer's sha256. Exit 1, printing FINDING, if it is missing,
#                                 repeated or differs (a result to record, not a failed request); exit 2 as for show.
#   calls.py reqdiff A B          two import files (UTF-16LE with a BOM, or UTF-8): their sizes and sha256, then the
#                                 lines that differ (at most CAP lines printed, with the count of the rest).
#
# Fields shown: ledger_masters: snapshot master_alter_id, voucher_alter_id, reused. list_companies: each company's
# name and guid. gst_status: state, window, total, next_offset, counts, then one line per item (voucher_number,
# date, master_id, alter_id, gst_status, status_flags, accepted_as_it_stands). build_import_xml: batch_id, sha256,
# voucher_count, and the warnings count. post_import: approval state and its seconds, attempt_recorded, dispatch
# state and counters. verify_import: verification_status. tools/list: how many tools and whether this row's five
# are among them.
# Example: python3 -I calls.py show answers\04-status-aug.json
import difflib, hashlib, json, pathlib, sys
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as count19.py holds it
CAP = 200
NEEDED = ["list_companies", "ledger_masters", "gst_status", "build_import_xml", "post_import", "verify_import"]
ITEM = ["voucher_number", "date", "master_id", "alter_id", "gst_status", "status_flags", "accepted_as_it_stands"]

def stop(why):
    print("STOP:", why)
    raise SystemExit(2)

def load(path):
    p = pathlib.Path(path)
    if not p.is_file():
        stop(f"{path}: no answer file (the request failed, not 'nothing found')")
    try:
        body = json.loads(p.read_text(encoding="utf-8"))
    except ValueError as e:
        stop(f"{path}: not JSON ({e})")
    response = body.get("response") or {}
    if "error" in response:
        stop(f"{path}: JSON-RPC error {response['error']!r}")
    result = response.get("result")
    if not isinstance(result, dict):
        stop(f"{path}: no result")
    return body, result

def content(path):
    body, result = load(path)
    if body.get("tool") == "list":
        return body, result, None
    sc = result.get("structuredContent")
    if not isinstance(sc, dict):
        stop(f"{path}: no structuredContent")
    return body, result, sc

def code_of(sc):
    inner = sc.get("result") if isinstance(sc.get("result"), dict) else {}
    for err in (inner.get("error"), sc.get("error")):
        if isinstance(err, dict) and err.get("code"):
            return err["code"]
    return None

def get(d, *path):
    for key in path:
        if not isinstance(d, dict) or key not in d:
            return "<absent>"
        d = d[key]
    return d

def show(path):
    body, result, sc = content(path)
    tool = body.get("tool")
    print(f"== {pathlib.Path(path).name}: tool {tool}, sent {body.get('call_sent_utc')}, "
          f"{body.get('round_trip_seconds')} s, isError {result.get('isError')}")
    if tool == "list":
        names = [t.get("name") for t in result.get("tools", [])]
        print(f"  tools: {len(names)}; missing of this row's: {[n for n in NEEDED if n not in names]}")
        return
    code = code_of(sc)
    if code:
        print(f"  refused {code}")
    r = sc.get("result") if isinstance(sc.get("result"), dict) else {}
    print(f"  evidence state {get(sc, 'evidence', 'state')}, reason {get(sc, 'evidence', 'reason_code')}")
    if tool == "ledger_masters":
        s = r.get("snapshot", sc.get("snapshot"))
        print(f"  M {get(s, 'master_alter_id')} V {get(s, 'voucher_alter_id')} reused {get(s, 'reused')}")
    elif tool == "list_companies":
        rows = r.get("companies", sc.get("companies", []))
        print(f"  companies: {len(rows) if isinstance(rows, list) else rows!r}")
        for c in rows if isinstance(rows, list) else []:
            guid = str(c.get("guid", c.get("company_guid", "")))
            print(f"   {c.get('name')!r} guid {guid} is the book's: {'yes' if guid.lower() == BOOK else 'no'}")
    elif tool == "gst_status":
        print(f"  state {get(r, 'state')} window {get(r, 'window')} type {get(r, 'voucher_type_name')!r} "
              f"total {get(r, 'total')} offset {get(r, 'offset')} next_offset {get(r, 'next_offset')}")
        print(f"  counts {json.dumps(get(r, 'counts'), sort_keys=True)}")
        items = r.get("items", [])
        print(f"  items on this page: {len(items)}")
        for it in sorted(items, key=lambda i: int(i.get("master_id") or 0)):
            print("   ", "; ".join(f"{k} {json.dumps(it.get(k, '<absent>'))}" for k in ITEM))
    elif tool == "build_import_xml":
        print(f"  batch_id {get(r, 'batch_id')} sha256 {get(r, 'sha256')} voucher_count {get(r, 'voucher_count')} "
              f"warnings {len(r.get('warnings', [])) if isinstance(r.get('warnings'), list) else get(r, 'warnings')}")
    elif tool == "post_import":
        print(f"  approval {json.dumps(get(r, 'approval'), sort_keys=True)}")
        print(f"  attempt_recorded {get(r, 'attempt_recorded')} dispatch state {get(r, 'dispatch', 'state')} "
              f"counters {json.dumps(get(r, 'dispatch', 'counters'), sort_keys=True)}")
        print(f"  verification_status {get(r, 'verification_status')}")
    elif tool == "verify_import":
        print(f"  verification_status {get(r, 'verification_status')}")

def status_items(path):
    body, result, sc = content(path)
    if body.get("tool") != "gst_status" or code_of(sc):
        stop(f"{path}: not an answered gst_status call (refused {code_of(sc)})")
    r = sc["result"]
    if "next_offset" in r:
        stop(f"{path}: more than one page (next_offset {r['next_offset']}); not compared")
    return r, {str(i.get("master_id")): i for i in r.get("items", [])}

def status_diff(a, b):
    ra, ia = status_items(a)
    rb, ib = status_items(b)
    print(f"== status-diff {pathlib.Path(a).name} -> {pathlib.Path(b).name}: total {ra.get('total')} -> {rb.get('total')}")
    for key in sorted(set(ia) | set(ib), key=lambda k: int(k) if k.isdigit() else 0):
        if key not in ib:
            print(f"  in A only: master_id {key}: {json.dumps(ia[key], sort_keys=True)}")
        elif key not in ia:
            print(f"  in B only: master_id {key}: {json.dumps(ib[key], sort_keys=True)}")
        else:
            moved = [k for k in sorted(set(ia[key]) | set(ib[key])) if ia[key].get(k) != ib[key].get(k)]
            for k in moved:
                print(f"  master_id {key} {k}: {json.dumps(ia[key].get(k))} -> {json.dumps(ib[key].get(k))}")
    print(f"  counts A {json.dumps(ra.get('counts'), sort_keys=True)}")
    print(f"  counts B {json.dumps(rb.get('counts'), sort_keys=True)}")
    print(f"  items: A {len(ia)}, B {len(ib)}, in both {len(set(ia) & set(ib))}")

def built(path):
    body, result, sc = content(path)
    r = sc.get("result") if isinstance(sc.get("result"), dict) else {}
    if body.get("tool") != "build_import_xml" or code_of(sc) or r.get("voucher_count") != 1 or not r.get("batch_id"):
        stop(f"{path}: not a build of exactly one voucher (refused {code_of(sc)}, voucher_count {r.get('voucher_count')})")
    return body["arguments"]["company_guid"], r

def write_call(tool, build, out):
    guid, r = built(build)
    out = pathlib.Path(out)
    if out.exists():
        stop(f"{out} exists: a call file is never rewritten")
    tmp = out.parent.parent / (out.name + ".tmp")  # written whole beside the inbox, then renamed in: client.py never reads half a file
    tmp.write_text(json.dumps({"tool": tool, "arguments": {"company_guid": guid, "batch_id": r["batch_id"]}}) + "\n",
                   encoding="utf-8", newline="\n")
    tmp.replace(out)
    print(f"{out.name}: {tool} batch_id {r['batch_id']}, sha256 {hashlib.sha256(out.read_bytes()).hexdigest()}")

def batch(build, datadir):
    _, r = built(build)
    found = [p for p in pathlib.Path(datadir).rglob(f"{r['batch_id']}.xml") if p.is_file()]
    if len(found) != 1:
        print(f"FINDING: {len(found)} files named {r['batch_id']}.xml under the data folder")
        raise SystemExit(1)
    digest = hashlib.sha256(found[0].read_bytes()).hexdigest()
    print(f"  {found[0].relative_to(datadir)}: {found[0].stat().st_size} bytes, sha256 {digest}")
    if digest != r.get("sha256"):
        print(f"FINDING: the file's sha256 differs from the answer's {r.get('sha256')}")
        raise SystemExit(1)
    print("  equal to the answer's sha256")

def text_of(path):
    raw = pathlib.Path(path).read_bytes()
    if raw[:2] == b"\xff\xfe":
        return raw, raw[2:].decode("utf-16-le")
    return raw, raw.decode("utf-8")

def reqdiff(a, b):
    out = []
    for p in (a, b):
        raw, text = text_of(p)
        print(f"  {pathlib.Path(p).name}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
        out.append(text.replace("><", ">\n<").splitlines())
    lines = [l for l in difflib.unified_diff(out[0], out[1], "A", "B", n=0, lineterm="") if l[:1] in "+-"]
    for l in lines[:CAP]:
        print("  " + l)
    print(f"  differing lines: {len(lines)}" + (f" ({len(lines) - CAP} not printed)" if len(lines) > CAP else ""))

args = sys.argv[1:]
if not args:
    stop("no command")
cmd, rest = args[0], args[1:]
if cmd == "show":
    for name in rest:
        show(name)
elif cmd == "status-diff" and len(rest) == 2:
    status_diff(*rest)
elif cmd in ("post", "verify") and len(rest) == 2:
    write_call("post_import" if cmd == "post" else "verify_import", *rest)
elif cmd == "batch" and len(rest) == 2:
    batch(*rest)
elif cmd == "reqdiff" and len(rest) == 2:
    reqdiff(*rest)
else:
    stop(f"unknown command {args!r}")
