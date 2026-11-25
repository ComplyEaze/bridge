#!/usr/bin/env python3
"""Read raw invoice read-back answers as the build reads them, and compare them with the build's own post answer.

Row 2162-PA, new, in the shape of row 2162-RC's ratecmp.py; not product code.

  rbcmp.py show ANSWER
      one answer: its size and sha256, the rows the build would take (each VOUCHER that is a direct child of the
      COLLECTION: agent_import_invoice_wire.rs:126-131), and for one row the status fields and what the build's own
      rules make of them. ROWS 0 is printed as such: a read that found nothing, never a failed read.
  rbcmp.py cmp POST A B
      POST is the saved post_import answer whose read-back the raw reads repeat; A and B the raw answers at about
      +5 s and +60 s. Grades the post answer's own count fields, ties each raw row to the voucher the post reports
      (its GUID, ALTERID and voucher number), compares A with B by bytes and by structure, and compares the status each raw read
      states with the status the build's own read must have read (below). Prints ITEM 2 EQUAL or ITEM 2 NOT EQUAL.
  rbcmp.py --selftest
      cases on the captured answer selftest/rb-0046.xml, changed in memory, and a synthetic post answer.

What the build's own read stated is INFERRED, not read: the build keeps none of its read-back answer (its evidence
holds hashes only, agent.rs:4136-4154). For posted_verified the code leaves one possibility: ISINVOICE Yes,
ISCANCELLED No, ISOPTIONAL No (agent_import_invoice.rs:1555-1557), ISPOSTDATED No, VCHGSTSTATUSISINCLUDED Yes,
VCHGSTSTATUSISUNCERTAIN No, VCHGSTSTATUSISEXCLUDED No and ISGSTOVERRIDDEN No (gst_status_differences,
agent_import_invoice.rs:1885-1918; status_of_voucher and overridden_of, agent_gst_status.rs:115-159). For
posted_divergent only the codes are known, so the codes are compared.

Values printed: the status fields, DATE, VOUCHERNUMBER, VOUCHERTYPENAME and ALTERID. No other field's value is ever
printed (the answer holds a GSTIN: agent_import_invoice_wire.rs:745-749); a GUID is printed only as equal or not.

Exit 0: read and, for cmp, ITEM 2 EQUAL with every tie holding. Exit 1: a finding (rows other than one, a tie that
fails, ITEM 2 NOT EQUAL, a post answer whose counts are not one voucher). Exit 2: a failed read (a file missing,
empty, not XML, STATUS absent or not 1, no BODY/DATA/COLLECTION; a post answer that is not one).
"""

import hashlib
import json
import pathlib
import re
import sys
import xml.etree.ElementTree as ET

STATUS_FIELDS = ("ISINVOICE", "ISCANCELLED", "ISOPTIONAL", "ISPOSTDATED", "VCHGSTSTATUSISINCLUDED",
                 "VCHGSTSTATUSISUNCERTAIN", "VCHGSTSTATUSISEXCLUDED", "ISGSTOVERRIDDEN")
SHOWN = ("DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "ALTERID") + STATUS_FIELDS
# readback_fields, agent_import_invoice_wire.rs:1011-1041: each must stand at most once below the row.
READ_FIELDS = ("DATE", "VOUCHERNUMBER", "VOUCHERTYPENAME", "REFERENCE", "REFERENCEDATE", "PARTYLEDGERNAME",
               "PARTYGSTIN", "STATENAME", "PLACEOFSUPPLY", "GSTREGISTRATIONTYPE", "ISINVOICE", "ISCANCELLED",
               "ISOPTIONAL", "GUID", "ALTERID", "VCHGSTSTATUSISINCLUDED", "VCHGSTSTATUSISUNCERTAIN",
               "VCHGSTSTATUSISEXCLUDED", "ISGSTOVERRIDDEN", "ISPOSTDATED", "GSTREGISTRATION", "CMPGSTIN")
# The codes of the status part of the build's comparison: the field names for the three wanted flags
# (agent_import_invoice.rs:1526-1557) and the GST status codes (:1859-1865).
STATUS_CODES = {"ISINVOICE", "ISCANCELLED", "ISOPTIONAL", "gst_status_uncertain", "gst_status_excluded",
                "gst_status_not_in_return", "gst_status_post_dated", "gst_status_overridden",
                "gst_status_unreadable", "gst_status_not_reported"}
VERIFIED_FLAGS = {"ISINVOICE": "Yes", "ISCANCELLED": "No", "ISOPTIONAL": "No", "ISPOSTDATED": "No",
                  "VCHGSTSTATUSISINCLUDED": "Yes", "VCHGSTSTATUSISUNCERTAIN": "No", "VCHGSTSTATUSISEXCLUDED": "No",
                  "ISGSTOVERRIDDEN": "No"}
# As ratecmp.py: Tally's references to control characters are not legal XML (the build marks them first,
# agent_import_invoice_wire.rs:105-108); here they are removed before parsing.
CONTROL_REFERENCE = re.compile(r"&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.IGNORECASE)
RAW_CONTROL = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]")
XML_DECLARATION = re.compile(r"^\s*<\?xml[^>]*\?>")
DIFF_CAP = 60


class FailedRead(Exception):
    pass


def decode(raw):
    # ratecmp.py's decode: Tally answers UTF-16LE without a BOM (row 2162-S2).
    if raw.startswith(b"\xff\xfe"):
        return raw[2:].decode("utf-16-le")
    if raw.startswith(b"\xef\xbb\xbf"):
        return raw[3:].decode("utf-8")
    if raw.startswith(b"\xfe\xff"):
        raise FailedRead("UTF-16BE is outside this row's inputs")
    if len(raw) >= 2 and raw[1] == 0 and raw[0] in (9, 10, 13, 32, 60):
        return raw.decode("utf-16-le")
    return raw.decode("utf-8")


def parse(raw, name):
    if not raw:
        raise FailedRead(f"{name}: empty answer: the request failed")
    try:
        text = decode(raw)
    except UnicodeError as error:
        raise FailedRead(f"{name}: does not decode as UTF-16LE or UTF-8 ({error.__class__.__name__})")
    text = RAW_CONTROL.sub("", CONTROL_REFERENCE.sub("", text))
    text = XML_DECLARATION.sub("", text, count=1)
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        raise FailedRead(f"{name}: not XML after control removal ({error})")
    if root.tag != "ENVELOPE":
        raise FailedRead(f"{name}: root is {root.tag}, not ENVELOPE")
    header = root.find("HEADER")
    statuses = [] if header is None else [c for c in header if c.tag == "STATUS"]
    if len(statuses) != 1 or (statuses[0].text or "").strip() != "1":
        raise FailedRead(f"{name}: HEADER/STATUS absent, repeated or not 1: the request failed")
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        raise FailedRead(f"{name}: no BODY/DATA/COLLECTION: the request failed")
    return root, [c for c in collection if c.tag == "VOUCHER"]


def own_text(node):
    return (node.text or "").strip() if len(node) == 0 else None


def fields_of(row):
    """readback_fields: a field absent or empty is absent; one that stands twice (or empty and with text) fails
    the read, as Row::one does (agent_import_invoice_wire.rs:47-57)."""
    fields, repeated = {}, []
    for key in READ_FIELDS:
        found = [c for c in row if c.tag == key]
        if len(found) > 1:
            repeated.append(f"{key}x{len(found)}")
            continue
        if found:
            value = own_text(found[0])
            if value:
                fields[key] = value
    return fields, repeated


def status_of_flags(i, u, e):
    combos = {("Yes", "No", "No"): "included", ("No", "Yes", "No"): "uncertain",
              ("No", "No", "Yes"): "excluded", ("No", "No", "No"): "not_in_return"}
    if (i, u, e) in combos:
        return combos[(i, u, e)], None
    return None, "gst_status_not_reported" if (i, u, e) == (None, None, None) else "gst_status_unreadable"


def status_codes(fields):
    """The status part of the build's comparison for one row: invoice_readback_differences' three wanted flags
    (agent_import_invoice.rs:1555-1557) and gst_status_differences (:1885-1918)."""
    codes = [k for k, want in (("ISINVOICE", "Yes"), ("ISCANCELLED", "No"), ("ISOPTIONAL", "No"))
             if fields.get(k) != want]
    f = fields.get
    status = None
    if f("ISPOSTDATED") is None:
        status = "gst_status_not_reported"
    else:
        state, code = status_of_flags(f("VCHGSTSTATUSISINCLUDED"), f("VCHGSTSTATUSISUNCERTAIN"),
                                      f("VCHGSTSTATUSISEXCLUDED"))
        head = (f("ISCANCELLED"), f("ISOPTIONAL"), f("ISPOSTDATED"))
        if head in (("Yes", "No", "No"), ("No", "Yes", "No")):
            status = code  # cancelled or optional: no finding when the flags read
        elif head == ("No", "No", "Yes"):
            status = code or "gst_status_post_dated"
        elif head == ("No", "No", "No"):
            status = code or {"included": None, "uncertain": "gst_status_uncertain",
                              "excluded": "gst_status_excluded", "not_in_return": "gst_status_not_in_return"}[state]
        else:
            status = "gst_status_not_reported" if head == (None, None, None) else "gst_status_unreadable"
    over = {"Yes": "gst_status_overridden", "No": None}.get(f("ISGSTOVERRIDDEN"),
                                                             "gst_status_not_reported" if f("ISGSTOVERRIDDEN") is None
                                                             else "gst_status_unreadable")
    found = [c for c in (status, over) if c]
    if len(found) == 2 and found[0] == found[1]:
        found = found[:1]
    return codes + found


def read_answer(raw, name):
    root, rows = parse(raw, name)
    out = {"name": name, "size": len(raw), "sha": hashlib.sha256(raw).hexdigest(), "root": root, "rows": len(rows)}
    if len(rows) == 1:
        fields, repeated = fields_of(rows[0])
        out.update(fields=fields, repeated=repeated,
                   codes=["invoice_readback_unreadable"] if repeated else status_codes(fields))
    else:
        out["codes"] = ["invoice_not_found"] if not rows else ["invoice_number_not_unique"]
    return out


def show_lines(a):
    lines = [f"{a['name']}: {a['size']} bytes, sha256 {a['sha']}", f"  ROWS {a['rows']}"]
    if a["rows"] == 1:
        if a["repeated"]:
            lines.append(f"  a field repeated (the build's read fails: invoice_readback_unreadable): {a['repeated']}")
        lines.append("  " + "; ".join(f"{k} {a['fields'].get(k, '<absent>')!r}" for k in SHOWN))
        lines.append(f"  GUID present: {'yes' if 'GUID' in a['fields'] else 'no'}")
    lines.append(f"  the build's rules on this answer, status part: {a['codes'] or 'no difference'}")
    return lines


def flatten(root):
    values = {}

    def visit(node, path):
        text = (node.text or "") + "".join(c.tail or "" for c in node)
        values[path] = "" if not text.strip() else text
        for key, value in node.attrib.items():
            values[f"{path}/@{key}"] = value
        counts = {}
        for child in node:
            counts[child.tag] = counts.get(child.tag, 0) + 1
            visit(child, f"{path}/{child.tag}[{counts[child.tag]}]")

    visit(root, f"/{root.tag}")
    return values


def shown(path):
    # A value is printed only for a field named in SHOWN, as the leaf of its path.
    leaf = path.rsplit("/", 1)[-1].split("[", 1)[0]
    return leaf in SHOWN and "/@" not in path.rsplit("/", 1)[-1]


def diff_lines(a, b):
    x, y = flatten(a["root"]), flatten(b["root"])
    added, removed = sorted(set(y) - set(x)), sorted(set(x) - set(y))
    changed = sorted(p for p in set(x) & set(y) if x[p] != y[p])
    lines = [f"STRUCTURAL DIFF added {len(added)}, removed {len(removed)}, text changed {len(changed)}"]
    body = [f"  added {p}" for p in added] + [f"  removed {p}" for p in removed]
    body += [f"  text changed {p}: {x[p].strip()!r} -> {y[p].strip()!r}" if shown(p)
             else f"  text changed {p} (value not printed)" for p in changed]
    lines += body[:DIFF_CAP]
    if len(body) > DIFF_CAP:
        lines.append(f"  ... {len(body) - DIFF_CAP} lines not printed")
    if not body:
        lines.append("  no structural difference")
    return lines


def post_of(raw, name):
    try:
        body = json.loads(raw.decode("utf-8"))
        response = body["response"]
        if "error" in response:
            raise FailedRead(f"{name}: a JSON-RPC error: the request failed")
        r = response["result"]["structuredContent"]["result"]
    except FailedRead:
        raise
    except (ValueError, KeyError, TypeError, AttributeError, UnicodeDecodeError) as error:
        raise FailedRead(f"{name}: not a saved answer with a result ({type(error).__name__})")
    if body.get("tool") != "post_import" or not isinstance(r, dict):
        raise FailedRead(f"{name}: not a post_import answer")
    return r


def get(d, *path):
    for key in path:
        if not isinstance(d, dict) or key not in d:
            return None
        d = d[key]
    return d


def cmp(post_raw, post_name, raws):
    lines, findings = [], []
    r = post_of(post_raw, post_name)
    answers = [read_answer(raw, name) for raw, name in raws]
    vouchers = r.get("vouchers") if isinstance(r.get("vouchers"), list) else []
    v = vouchers[0] if len(vouchers) == 1 and isinstance(vouchers[0], dict) else {}
    status = v.get("status")
    checks = [
        ("dispatch.state posted_verified", get(r, "dispatch", "state") == "posted_verified"),
        ("dispatch.counters.created 1", get(r, "dispatch", "counters", "created") == 1),
        ("alter_id_delta.delta 1", get(r, "alter_id_delta", "delta") == 1),
        ("post_location.target_voucher_step.step 1", get(r, "post_location", "target_voucher_step", "step") == 1),
        ("post_span_binding.state bound", get(r, "post_span_binding", "state") == "bound"),
        ("one voucher in the answer", len(vouchers) == 1),
    ]
    lines.append(f"POST {post_name}: verification_status {r.get('verification_status')!r}; voucher status {status!r}")
    for name, ok in checks:
        lines.append(f"  {'PASS' if ok else 'FAIL'} {name}")
    if not all(ok for name, ok in checks[1:4] + checks[5:]):
        findings.append("the post answer does not report exactly one voucher created")
    build_codes = None
    if status == "posted_verified":
        build_codes = []
        lines.append("  the build's own read (inferred from code, partial): " +
                     "; ".join(f"{k} {w!r}" for k, w in VERIFIED_FLAGS.items()))
    elif status == "posted_divergent":
        diffs = v.get("diffs") if isinstance(v.get("diffs"), list) else []
        codes = [c for d in diffs if isinstance(d, dict) for c in (d.get("invoice_fields") or [])]
        build_codes = sorted(c for c in codes if c in STATUS_CODES)
        lines.append(f"  the build's own read: codes only; its status codes {build_codes or 'none'}; "
                     f"other codes {sorted(set(codes) - STATUS_CODES) or 'none'}")
    else:
        findings.append(f"the voucher status is {status!r}: the build did not read the invoice back")
    want_alter = v.get("alter_id")
    want_guid = v.get("guid")
    want_number = v.get("voucher_number")
    for a in answers:
        lines += show_lines(a)
        if a["rows"] != 1:
            findings.append(f"{a['name']}: {a['rows']} rows, not one")
            continue
        alter = a["fields"].get("ALTERID")
        guid_ok = isinstance(want_guid, str) and a["fields"].get("GUID") == want_guid
        alter_ok = alter is not None and str(want_alter) == alter.strip()
        number_ok = isinstance(want_number, str) and a["fields"].get("VOUCHERNUMBER") == want_number
        lines.append(f"  tie: GUID equal to the post's voucher: {'yes' if guid_ok else 'NO'}; "
                     f"ALTERID equal to the post's {want_alter!r}: {'yes' if alter_ok else 'NO'}; "
                     f"VOUCHERNUMBER equal to the post's {want_number!r}: {'yes' if number_ok else 'NO'}")
        if not (guid_ok and alter_ok and number_ok):
            findings.append(f"{a['name']}: not the voucher the post reports")
        if build_codes is not None:
            if status == "posted_verified":
                equal = all(a["fields"].get(k) == w for k, w in VERIFIED_FLAGS.items()) and not a["repeated"]
            else:
                equal = sorted(set(a["codes"])) == sorted(set(build_codes))
            lines.append(f"  status equal to the build's own read: {'yes' if equal else 'NO'}")
            if not equal:
                findings.append(f"{a['name']}: the status differs from the build's own read")
    if len(answers) == 2:
        first, second = answers
        lines.append(f"BYTES {first['name']} and {second['name']}: {'equal' if first['sha'] == second['sha'] else 'differ'}")
        lines += diff_lines(first, second)
        if first["rows"] == second["rows"] == 1:
            same = all(first["fields"].get(k) == second["fields"].get(k) for k in STATUS_FIELDS + ("ALTERID", "GUID"))
            lines.append(f"STATUS FIELDS, ALTERID and GUID of the two reads: {'equal' if same else 'DIFFER'}")
            if not same:
                findings.append("the two raw reads differ in status, ALTERID or GUID")
    status_ok = build_codes is not None and not any("status differs" in x or "rows, not one" in x for x in findings)
    lines.append("ITEM 2 EQUAL" if status_ok and not any("two raw reads differ" in x for x in findings)
                 else "ITEM 2 NOT EQUAL")
    for finding in findings:
        lines.append("FINDING: " + finding)
    return lines, (1 if findings else 0)


def selftest():
    here = pathlib.Path(__file__).resolve().parent
    base = (here / "selftest" / "rb-0046.xml").read_bytes()
    text = base.decode("utf-16-le")
    a = read_answer(base, "rb-0046.xml")
    if a["rows"] != 1 or a["codes"]:
        print("HALT: the captured answer does not read as one clean row")
        return 1
    guid, alter, number = a["fields"]["GUID"], int(a["fields"]["ALTERID"].strip()), a["fields"]["VOUCHERNUMBER"]

    def enc(t):
        return t.encode("utf-16-le")

    def swap(old, new, t=text, count=1):
        if t.count(old) < 1:
            raise SystemExit(f"HALT: the fixture lacks {old!r}")
        return t.replace(old, new, count)

    def flag(name, value, t=text):
        m = re.search(rf"<{name}( [^>]*)?>[^<]*</{name}>", t)
        if m is None:
            raise SystemExit(f"HALT: the fixture lacks {name}")
        return t[:m.start()] + f"<{name}{m.group(1) or ''}>{value}</{name}>" + t[m.end():]

    def post(status="posted_verified", codes=(), alter_id=alter, g=guid, created=1, delta=1, n=number):
        v = {"status": status, "guid": g, "alter_id": alter_id, "voucher_number": n, "bridge_txn_id": "t"}
        if codes:
            v["diffs"] = [{"invoice_fields": list(codes)}]
        r = {"verification_status": status, "vouchers": [v], "dispatch": {"state": "posted_verified",
             "counters": {"created": created}}, "alter_id_delta": {"delta": delta},
             "post_location": {"target_voucher_step": {"step": 1}}, "post_span_binding": {"state": "bound"}}
        return json.dumps({"tool": "post_import", "response": {"result": {"structuredContent": {"result": r}}}}).encode()

    m = re.search(r"<VOUCHER [^>]*>.*?</VOUCHER>", text, re.S)
    two = text[:m.end()] + m.group(0) + text[m.end():]
    none = text[:m.start()] + text[m.end():]
    end = m.end() - len("</VOUCHER>")  # inside the row: CMPINFO holds a counter of the same name
    twice = text[:end] + "<ISPOSTDATED>No</ISPOSTDATED>" + text[end:]
    gstin = re.search(r"<PARTYGSTIN[^>]*>([^<]*)</PARTYGSTIN>", text)
    secret = "29ZZZZZ0000Z9Z9"  # the synthetic GSTIN family, not the fixture's
    other_gstin = (text[:gstin.start(1)] + secret + text[gstin.end(1):]) if gstin else None
    uncertain = flag("VCHGSTSTATUSISUNCERTAIN", "Yes", flag("VCHGSTSTATUSISINCLUDED", "No"))
    cases = [
        ("c01 the same answer twice, the post verified", post(), base, base, 0, "ITEM 2 EQUAL"),
        ("c02 +60 reads uncertain", post(), base, enc(uncertain), 1, "ITEM 2 NOT EQUAL"),
        ("c03 a flag stands twice at +5", post(), enc(twice), base, 1, "ITEM 2 NOT EQUAL"),
        ("c04 two rows at +60", post(), base, enc(two), 1, "ITEM 2 NOT EQUAL"),
        ("c05 no row at +5", post(), enc(none), base, 1, "ITEM 2 NOT EQUAL"),
        ("c06 the post reports another ALTERID", post(alter_id=alter + 1), base, base, 1, None),
        ("c07 the post reports another GUID", post(g=guid[:-1] + ("0" if guid[-1] != "0" else "1")), base, base, 1,
         None),
        ("c08 the post created 2", post(created=2), base, base, 1, None),
        ("c15 the post reports another voucher number", post(n=number[:-1] + ("0" if number[-1] != "0" else "1")),
         base, base, 1, None),
        ("c09 ISPOSTDATED absent at +60", post(), base, enc(re.sub(r"<ISPOSTDATED[^>]*>[^<]*</ISPOSTDATED>", "", text)),
         1, "ITEM 2 NOT EQUAL"),
        ("c10 overridden at +5", post(), enc(flag("ISGSTOVERRIDDEN", "Yes")), base, 1, "ITEM 2 NOT EQUAL"),
        ("c11 the post divergent uncertain, both raw reads uncertain", post("posted_divergent", ["gst_status_uncertain"]),
         enc(uncertain), enc(uncertain), 0, "ITEM 2 EQUAL"),
        ("c12 the post divergent uncertain, the raw reads included", post("posted_divergent", ["gst_status_uncertain"]),
         base, base, 1, "ITEM 2 NOT EQUAL"),
        ("c13 the post did not read back (posted_unverified)", post("posted_unverified"), base, base, 1,
         "ITEM 2 NOT EQUAL"),
    ]
    if other_gstin is not None:
        cases.append(("c14 a GSTIN differs at +60 (value never printed)", post(), base, enc(other_gstin), 0,
                      "ITEM 2 EQUAL"))
    failed = [
        ("f01 an empty +5 answer", b""),
        ("f02 +5 not XML", enc("<ENVELOPE><HEADER>")),
        ("f03 +5 with STATUS 0", enc(swap("<STATUS>1</STATUS>", "<STATUS>0</STATUS>"))),
        ("f04 +5 with no COLLECTION", enc(re.sub(r"<COLLECTION.*</COLLECTION>", "", text, flags=re.S))),
    ]
    right = total = 0
    for name, p, x, y, want_rc, want_line in cases:
        total += 1
        lines, rc = cmp(p, "post.json", [(x, "a5.xml"), (y, "a60.xml")])
        out = "\n".join(lines)
        ok = rc == want_rc and (want_line is None or want_line in lines) and secret not in out
        right += ok
        print(f"{name}: exit {rc} (want {want_rc}) {'OK' if ok else 'WRONG'}")
    for name, bad in failed:
        total += 1
        try:
            cmp(post(), "post.json", [(bad, "a5.xml"), (base, "a60.xml")])
            rc = 0
        except FailedRead:
            rc = 2
        right += rc == 2
        print(f"{name}: exit {rc} (want 2) {'OK' if rc == 2 else 'WRONG'}")
    total += 1
    try:
        cmp(b'{"tool": "post_import", "response": {"error": {"code": -1}}}', "post.json", [(base, "a5.xml")])
        rc = 0
    except FailedRead:
        rc = 2
    right += rc == 2
    print(f"f05 a post answer that is a JSON-RPC error: exit {rc} (want 2) {'OK' if rc == 2 else 'WRONG'}")
    print(f"{total} cases, {right} right")
    return 0 if right == total == 20 else 1


def main(argv=None):
    args = sys.argv[1:] if argv is None else list(argv)
    try:
        if args == ["--selftest"]:
            return selftest()
        if len(args) == 2 and args[0] == "show":
            path = pathlib.Path(args[1])
            raw = path.read_bytes() if path.is_file() else None
            if raw is None:
                raise FailedRead(f"{args[1]}: no file: the request failed, not 'nothing found'")
            print("\n".join(show_lines(read_answer(raw, path.name))))
            return 0
        if len(args) in (3, 4) and args[0] == "cmp":
            files = [pathlib.Path(x) for x in args[1:]]
            for f in files:
                if not f.is_file():
                    raise FailedRead(f"{f}: no file: the request failed, not 'nothing found'")
            lines, rc = cmp(files[0].read_bytes(), files[0].name, [(f.read_bytes(), f.name) for f in files[1:]])
            print("\n".join(lines))
            return rc
    except FailedRead as error:
        print(f"FAILED READ: {error}")
        return 2
    print("usage: rbcmp.py show ANSWER | cmp POST A [B] | --selftest", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
