#!/usr/bin/env python3
# grade05.py: row 2162-RC's one new helper (not product code). Read-only: it reads saved answers and batch files and
# writes nothing. It parses JSON; it never searches the text of an answer.
#
#   grade05.py grade ANSWER        one build_import_xml answer that client.py saved: prints result.state,
#                                  result.reason, every entry of result.refusals (code and detail, at most CAP),
#                                  voucher_count, batch_id, sha256 and the evidence bytes and digests, then ONE verdict
#                                  line: "VERDICT admitted voucher_count 1, no refusal", "VERDICT refused <reason>: <codes>"
#                                  or "VERDICT other". calls.py show does not print this refusal: it reads only
#                                  result.error, and a refused invoice answers result.reason and result.refusals
#                                  (row 2162-RX's run-1c answer). Exit 2, printing STOP, when the answer is "the request
#                                  failed": no file, not JSON, a JSON-RPC error, no structuredContent, or another tool.
#                                  Exit 0 for every verdict: a refusal is an answer.
#   grade05.py importeq BUILD_A DATA_A BUILD_B DATA_B
#                                  two admitted builds and their data folders: finds the one <batch_id>.xml of each
#                                  under its data folder, checks its sha256 against its answer's, and compares the two
#                                  files with ONE value set aside in each: the voucher identity the build derives from
#                                  that build's batch id and the voucher's bridge_txn_id (agent_import_identity.rs,
#                                  import_identity; the hand derivation of agent_import_identity_tests.rs,
#                                  identity_by_hand). The batch id itself is not written in the file; this identity
#                                  is, as the voucher's REMOTEID attribute and inside the narration's [BRIDGE:...] tag
#                                  (agent_import.rs, render_import_xml and render_voucher_xml). Each file must hold it
#                                  exactly twice, once at each of those two places; each occurrence is replaced by the
#                                  same text "<identity>", and the two results are
#                                  compared byte for byte. Nothing else is ignored: no whitespace, no encoding, no
#                                  other field. Prints "EQUAL once the identity is set aside" (exit 0) or "DIFFER" with
#                                  at most CAP differing lines (exit 1). Exit 2, printing STOP, for a failed read: an
#                                  answer as above, not exactly one voucher built, no batch file or more than one, a
#                                  sha256 that differs from the answer's, or the identity not exactly at its two places.
#   grade05.py --selftest          grades the two answers under selftest/ (row 2162-RX's run-1c and run-2 answers,
#                                  byte-equal copies) and checks the identity derivation on one fixed input; prints one
#                                  line per case and exits 0 only if every case is right.
# Example: python -I grade05.py grade run-g1/answers/05-build-0074-0801.json
import difflib, hashlib, json, pathlib, sys

CAP = 40
MASK = "<identity>"


def stop(why):
    print("STOP:", why)
    raise SystemExit(2)


def answer(path):
    p = pathlib.Path(path)
    if not p.is_file():
        stop(f"{path}: no answer file (the request failed, not 'nothing found')")
    try:
        body = json.loads(p.read_text(encoding="utf-8"))
    except ValueError as e:
        stop(f"{path}: not JSON ({e})")
    if body.get("tool") != "build_import_xml":
        stop(f"{path}: tool {body.get('tool')!r}, not build_import_xml")
    response = body.get("response") or {}
    if "error" in response:
        stop(f"{path}: JSON-RPC error {response['error']!r}")
    result = response.get("result")
    if not isinstance(result, dict) or not isinstance(result.get("structuredContent"), dict):
        stop(f"{path}: no structuredContent")
    sc = result["structuredContent"]
    inner = sc.get("result") if isinstance(sc.get("result"), dict) else {}
    return body, result, sc, inner


def verdict(inner, sc):
    refusals = inner.get("refusals")
    error = inner.get("error") if isinstance(inner.get("error"), dict) else sc.get("error")
    if refusals is None and not error and inner.get("voucher_count") == 1 and inner.get("batch_id"):
        return "admitted voucher_count 1, no refusal"
    if inner.get("state") == "refused" and isinstance(refusals, list) and refusals:
        codes = ", ".join(f"{r.get('code')} ({r.get('detail')})" for r in refusals if isinstance(r, dict))
        return f"refused {inner.get('reason')}: {codes}"
    return "other"


def grade(path):
    body, result, sc, inner = answer(path)
    ev = sc.get("evidence") if isinstance(sc.get("evidence"), dict) else {}
    print(f"== {pathlib.Path(path).name}: sent {body.get('call_sent_utc')}, isError {result.get('isError')}")
    print(f"  state {inner.get('state', '<absent>')} reason {inner.get('reason', '<absent>')}")
    refusals = inner.get("refusals")
    if isinstance(refusals, list):
        print(f"  refusals: {len(refusals)}")
        for r in refusals[:CAP]:
            print(f"   code {r.get('code')!r} detail {r.get('detail')!r}" if isinstance(r, dict) else f"   {r!r}")
        if len(refusals) > CAP:
            print(f"   ... {len(refusals) - CAP} not printed")
    else:
        print(f"  refusals: {'<absent>' if refusals is None else repr(refusals)}")
    error = inner.get("error") if isinstance(inner.get("error"), dict) else sc.get("error")
    if error:
        print(f"  error code {error.get('code') if isinstance(error, dict) else error!r}")
    print(f"  voucher_count {inner.get('voucher_count', '<absent>')} batch_id {inner.get('batch_id', '<absent>')} "
          f"sha256 {inner.get('sha256', '<absent>')}")
    print(f"  evidence state {ev.get('state', '<absent>')} bytes {ev.get('bytes', '<absent>')} "
          f"request {ev.get('request_sha256', '<absent>')} response {ev.get('response_sha256', '<absent>')}")
    v = verdict(inner, sc)
    print(f"VERDICT {v}")
    return v


def identity(batch_id, txn_id):
    # agent_import_identity_tests.rs, identity_by_hand: SHA-256 of the JSON array text, its first sixteen bytes,
    # the version nibble set to 8 and the variant bits to 10, written as 8-4-4-4-12 hex.
    text = json.dumps(["bridge.mcp.import.v1", batch_id, txn_id], separators=(",", ":"), ensure_ascii=False)
    b = bytearray(hashlib.sha256(text.encode("utf-8")).digest()[:16])
    b[6] = (b[6] & 0x0F) | 0x80
    b[8] = (b[8] & 0x3F) | 0x80
    h = b.hex()
    return f"{h[:8]}-{h[8:12]}-{h[12:16]}-{h[16:20]}-{h[20:]}"


def masked(build, datadir):
    body, _, sc, inner = answer(build)
    if verdict(inner, sc) != "admitted voucher_count 1, no refusal":
        stop(f"{build}: not an admitted build of exactly one voucher")
    vouchers = (body.get("arguments") or {}).get("vouchers") or []
    if len(vouchers) != 1 or not vouchers[0].get("bridge_txn_id"):
        stop(f"{build}: the call did not name exactly one voucher with a bridge_txn_id")
    batch_id = inner["batch_id"]
    d = pathlib.Path(datadir)
    if not d.is_dir():
        stop(f"{datadir}: no data folder")
    found = [p for p in d.rglob(f"{batch_id}.xml") if p.is_file()]
    if len(found) != 1:
        stop(f"{len(found)} files named <batch_id>.xml under {datadir}; exactly one expected")
    raw = found[0].read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    print(f"  {found[0].relative_to(d)}: {len(raw)} bytes, sha256 {digest}")
    if digest != inner.get("sha256"):
        stop(f"the file's sha256 differs from the answer's {inner.get('sha256')}")
    codec = "utf-16-le" if raw[:2] == b"\xff\xfe" else "utf-8"
    ident = identity(batch_id, vouchers[0]["bridge_txn_id"])
    places = {
        "REMOTEID": f'REMOTEID="{ident}"'.encode(codec),
        "narration tag": f"[BRIDGE:{ident}]".encode(codec),
        "anywhere": ident.encode(codec),
    }
    counts = {k: raw.count(v) for k, v in places.items()}
    print(f"  encoding {codec}; identity derived from the answer's batch id and bridge_txn_id: "
          f"REMOTEID {counts['REMOTEID']}, narration tag {counts['narration tag']}, anywhere {counts['anywhere']}")
    if counts != {"REMOTEID": 1, "narration tag": 1, "anywhere": 2}:
        stop("the identity is not exactly once at REMOTEID and once in the narration tag, and nowhere else")
    return raw.replace(places["anywhere"], MASK.encode(codec)), codec


def importeq(build_a, data_a, build_b, data_b):
    print(f"== A: {build_a}")
    a, codec_a = masked(build_a, data_a)
    print(f"== B: {build_b}")
    b, codec_b = masked(build_b, data_b)
    print(f"  after the identity is set aside: A sha256 {hashlib.sha256(a).hexdigest()}, "
          f"B sha256 {hashlib.sha256(b).hexdigest()}")
    if a == b:
        print("EQUAL once the identity is set aside")
        return 0
    ta = a.decode(codec_a).replace("><", ">\n<").splitlines()
    tb = b.decode(codec_b).replace("><", ">\n<").splitlines()
    lines = [l for l in difflib.unified_diff(ta, tb, "A", "B", n=0, lineterm="")
             if l[:1] in "+-" and l[:3] not in ("---", "+++")]
    for l in lines[:CAP]:
        print("  " + l[:300])
    print(f"  differing lines: {len(lines)}" + (f" ({len(lines) - CAP} not printed)" if len(lines) > CAP else "")
          + ("" if codec_a == codec_b else f"; encodings differ: {codec_a}, {codec_b}"))
    print("DIFFER")
    return 1


def selftest():
    here = pathlib.Path(__file__).resolve().parent / "selftest"
    cases = [
        ("05-refused.json", "refused invoice_not_admitted: invoice_sales_ledger_rate_shape_unmeasured "
         "(BRIDGE Svc 998313 5%), invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998314 5%)"),
        ("05-admitted.json", "admitted voucher_count 1, no refusal"),
    ]
    right = 0
    for name, want in cases:
        got = grade(here / name)
        ok = got == want
        right += ok
        print(f"case {name}: {'right' if ok else 'WRONG, expected ' + want}")
    # The derivation, checked against its own definition written out step by step.
    text = b'["bridge.mcp.import.v1","batch-render","txn-001"]'
    d = bytearray(hashlib.sha256(text).digest()[:16])
    d[6], d[8] = (d[6] & 0x0F) | 0x80, (d[8] & 0x3F) | 0x80
    want = "-".join((d[:4].hex(), d[4:6].hex(), d[6:8].hex(), d[8:10].hex(), d[10:].hex()))
    ok = identity("batch-render", "txn-001") == want and want[14] == "8" and want[19] in "89ab"
    right += ok
    print(f"case identity derivation: {'right' if ok else 'WRONG'}")
    print(f"{len(cases) + 1} cases, {right} right")
    return 0 if right == len(cases) + 1 else 1


args = sys.argv[1:]
if args == ["--selftest"]:
    sys.exit(selftest())
if len(args) == 2 and args[0] == "grade":
    grade(args[1])
    sys.exit(0)
if len(args) == 5 and args[0] == "importeq":
    sys.exit(importeq(*args[1:]))
stop(f"unknown command {args!r}")
