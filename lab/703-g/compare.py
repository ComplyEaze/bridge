"""Row 703-G: compare every answered request of two relay folders by position. Not for merge.

python -I compare.py BEFORE_OUTDIR AFTER_OUTDIR

Each folder is as relay.py (pass mode) leaves it: relay.jsonl and bodies/. The answered
requests of each folder are taken in the order they were answered and compared by position:
the request bytes, then the answer bytes, by sha256. Each is labelled by the collection its
request names: census ("Bridge Agent Voucher Census"), vouchers ("Bridge Agent Vouchers"),
status ("Bridge Agent Import Verification", the gst_status read), marks ("Bridge Agent Company
High Water"), get (a request with no body), other (anything else).

Tally puts a CMPINFO block of counters in its answers, and those counters move without the
book changing (protocol reference, the open question "What do the CMPINFO counters report"). So
each answer is compared twice: whole, and with every CMPINFO element removed ("outside
CMPINFO"). Only a difference outside CMPINFO counts; a CMPINFO-only difference is printed with
the counters that moved. An answer that does not decode, or whose CMPINFO tags do not pair, is
compared whole only, and says so.

Each folder must hold at least one vouchers answer and at least 8 status answers (the sheet's
one vouchers call and eight gst_status calls), or the phase did not reach Tally as the sheet
expects and nothing is compared.

A relay_error logged after a folder's last answer (A's process ending) is printed and not counted.
Exit codes: 0 every answer equal outside CMPINFO; 4 a pair of equal requests got answers that
differ outside CMPINFO; 1 the folders cannot be compared (a failed or unanswered request, a body
that does not match its record, too few vouchers or status answers, unequal counts, or requests
that differ at a position: their answers prove nothing).
"""
import hashlib
import json
import os
import re
import sys

LABELS = (
    ("census", "Bridge Agent Voucher Census"),
    ("vouchers", "Bridge Agent Vouchers"),
    ("status", "Bridge Agent Import Verification"),
    ("marks", "Bridge Agent Company High Water"),
)
EXPECT_AT_LEAST = (("vouchers", 1), ("status", 8))
CMPINFO = re.compile(r"<CMPINFO>.*?</CMPINFO>", re.S)
COUNTER = re.compile(r"<([A-Z][A-Z0-9.]*)>([^<]*)</\1>")


class Failed(Exception):
    pass


def body(outdir, digest, size):
    path = os.path.join(outdir, "bodies", digest)
    with open(path, "rb") as handle:
        data = handle.read()
    if len(data) != size or hashlib.sha256(data).hexdigest() != digest:
        raise Failed("{}: bodies/{} does not match its record ({} B recorded, {} B kept)".format(
            outdir, digest, size, len(data)))
    return data


def text(data):
    """The answer as text, or None when it does not decode."""
    try:
        if data.startswith(b"\xff\xfe"):
            return data[2:].decode("utf-16-le")
        if len(data) >= 2 and len(data) % 2 == 0 and data[1] == 0:
            return data.decode("utf-16-le")
        return data.decode("utf-8")
    except UnicodeDecodeError:
        return None


def outside(data):
    """(sha256 of the answer with every CMPINFO element removed, the CMPINFO counters), or None."""
    body = text(data)
    if body is None or body.count("<CMPINFO>") != body.count("</CMPINFO>"):
        return None
    counters = {}
    for block in CMPINFO.findall(body):
        for name, value in COUNTER.findall(block):
            counters.setdefault(name, []).append(value.strip())
    rest = CMPINFO.sub("", body)
    if "<CMPINFO>" in rest or "</CMPINFO>" in rest:
        return None
    return hashlib.sha256(rest.encode("utf-8")).hexdigest(), counters


def label(sent):
    if not sent:
        return "get"
    for name, marker in LABELS:
        if marker.encode("utf-16-le") in sent or marker.encode("utf-8") in sent:
            return name
    return "other"


def answered(outdir):
    log = []
    with open(os.path.join(outdir, "relay.jsonl"), encoding="utf-8") as handle:
        for number, line in enumerate(handle, 1):
            try:
                log.append(json.loads(line))
            except ValueError:
                raise Failed("{}: relay.jsonl line {} is not JSON".format(outdir, number))
    last_answer = max((e["at"] for e in log if "answer_sha256" in e), default=None)
    for event in log:
        kind = event.get("event")
        # A socket reset when A's process ends logs relay_error after every answer: reported, not a failure.
        if kind == "relay_error" and last_answer is not None and event["at"] > last_answer:
            print("{}: after the last answer (teardown): {}".format(outdir, json.dumps(event)))
            continue
        if kind in ("bridge_cut", "tally_closed", "relay_error", "front_send"):
            raise Failed("{}: the log holds a {} record: {}".format(outdir, kind, json.dumps(event)))
    forwarded = {(e["conn"], e["request"]) for e in log if e.get("event") == "forwarded"}
    done = sorted((e for e in log if "answer_sha256" in e), key=lambda e: e["at"])
    missing = forwarded - {(e["conn"], e["request"]) for e in done}
    if missing:
        raise Failed("{}: {} forwarded request(s) have no answer record".format(outdir, len(missing)))
    for e in done:
        e["label"] = label(body(outdir, e["sent_sha256"], e["sent"]))
        e["outside"] = outside(body(outdir, e["answer_sha256"], e["answered"]))
    for name, least in EXPECT_AT_LEAST:
        found = sum(1 for e in done if e["label"] == name)
        if found < least:
            raise Failed("{}: {} {} answers, fewer than {}".format(outdir, found, name, least))
    return done


def main(before_dir, after_dir):
    before, after = answered(before_dir), answered(after_dir)
    for name, group in (("before", before), ("after", after)):
        kinds = {}
        for e in group:
            kinds[e["label"]] = kinds.get(e["label"], 0) + 1
        print("{} answered requests {}: {}".format(
            name, len(group), ", ".join("{} {}".format(k, kinds[k]) for k in sorted(kinds))))
    if len(before) != len(after):
        raise Failed("before answered {} requests and after {}".format(len(before), len(after)))
    print("k label sent_bytes request_equal answered_before answered_after whole_equal outside_cmpinfo_equal")
    request_differs, answer_differs, cmpinfo_only = [], [], []
    for k, (a, b) in enumerate(zip(before, after), 1):
        request_equal = a["sent_sha256"] == b["sent_sha256"] and a["label"] == b["label"]
        whole_equal = a["answer_sha256"] == b["answer_sha256"]
        if whole_equal:
            outside_equal = "yes"
        elif a["outside"] is None or b["outside"] is None:
            outside_equal = "not split (compared whole)"
        else:
            outside_equal = "yes" if a["outside"][0] == b["outside"][0] else "NO"
        print("{} {} {} {} {} {} {} {}".format(k, a["label"], a["sent"], "yes" if request_equal else "NO",
                                               a["answered"], b["answered"], "yes" if whole_equal else "NO",
                                               outside_equal))
        if not request_equal:
            request_differs.append(k)
        elif outside_equal != "yes":
            answer_differs.append((k, a["label"]))
            print("  answer {} before sha256 {}".format(k, a["answer_sha256"]))
            print("  answer {} after sha256 {}".format(k, b["answer_sha256"]))
        elif not whole_equal:
            cmpinfo_only.append((k, a["label"]))
            before_counters, after_counters = a["outside"][1], b["outside"][1]
            moved = sorted(n for n in set(before_counters) | set(after_counters)
                           if before_counters.get(n) != after_counters.get(n))
            print("  CMPINFO only: {}".format(", ".join("{} {} -> {}".format(
                n, ",".join(before_counters.get(n, ["absent"])), ",".join(after_counters.get(n, ["absent"])))
                for n in moved) or "no counter value differs (layout only)"))
    if request_differs:
        raise Failed("the requests differ at position(s) {}, so their answers cannot be compared".format(
            ", ".join(str(k) for k in request_differs)))
    if cmpinfo_only:
        print("CMPINFO ONLY: {} answers differ only inside CMPINFO ({})".format(len(cmpinfo_only), ", ".join(
            sorted({name for _, name in cmpinfo_only}))))
    if answer_differs:
        by_label = {}
        for _, name in answer_differs:
            by_label[name] = by_label.get(name, 0) + 1
        print("ANSWERS DIFFER OUTSIDE CMPINFO: {} of {} ({})".format(len(answer_differs), len(before), ", ".join(
            "{} {}".format(name, by_label[name]) for name in sorted(by_label))))
        return 4
    print("ALL EQUAL OUTSIDE CMPINFO: {} of {} answers".format(len(before), len(before)))
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(__doc__)
        sys.exit(2)
    try:
        sys.exit(main(sys.argv[1], sys.argv[2]))
    except (Failed, OSError, KeyError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
