"""Row 703-C: compare the census posts of two `vouchers` calls in one relay log. Not for merge.

python -I census.py RELAY_OUTDIR

Reads RELAY_OUTDIR/relay.jsonl and RELAY_OUTDIR/bodies/ as relay.py (pass mode) writes them.
A census post is an answered request whose kept body names the collection
"Bridge Agent Voucher Census" (UTF-16LE or UTF-8). The census posts are split into the two
calls at the one gap of 45 s or more between them (the row pauses 60 s between the calls).
Each pair is compared by position: the request bytes, then the answer bytes, by sha256.

A relay_error logged after the last answer (A's process ending) is printed and not counted.
Exit codes: 0 every pair equal; 4 a pair differs; 3 no census post in the log;
1 the log cannot be read as two whole calls (a failed or unanswered request, a body that
does not match its record, no single gap, unequal counts). A 3 is not a 1: "no census was
sent" and "the request failed" are different results.
"""
import hashlib
import json
import os
import sys

MARKER = "Bridge Agent Voucher Census"
MARKS = (MARKER.encode("utf-16-le"), MARKER.encode("utf-8"))
GAP_S = 45.0


class Failed(Exception):
    pass


def body(outdir, digest, size):
    path = os.path.join(outdir, "bodies", digest)
    with open(path, "rb") as handle:
        data = handle.read()
    if len(data) != size or hashlib.sha256(data).hexdigest() != digest:
        raise Failed("bodies/{} does not match its record ({} B recorded, {} B kept)".format(digest, size, len(data)))
    return data


def records(outdir):
    out = []
    with open(os.path.join(outdir, "relay.jsonl"), encoding="utf-8") as handle:
        for number, line in enumerate(handle, 1):
            try:
                out.append(json.loads(line))
            except ValueError:
                raise Failed("relay.jsonl line {} is not JSON".format(number))
    return out


def main(outdir):
    log = records(outdir)
    last_answer = max((e["at"] for e in log if "answer_sha256" in e), default=None)
    for event in log:
        kind = event.get("event")
        # A socket reset when A's process ends logs relay_error after every answer: reported, not a failure.
        if kind == "relay_error" and last_answer is not None and event["at"] > last_answer:
            print("after the last answer (teardown): {}".format(json.dumps(event)))
            continue
        if kind in ("bridge_cut", "tally_closed", "relay_error", "front_send"):
            raise Failed("the log holds a {} record: {}".format(kind, json.dumps(event)))
    forwarded = {(e["conn"], e["request"]) for e in log if e.get("event") == "forwarded"}
    answered = [e for e in log if "answer_sha256" in e]
    missing = forwarded - {(e["conn"], e["request"]) for e in answered}
    if missing:
        raise Failed("{} forwarded request(s) have no answer record".format(len(missing)))
    census = []
    for e in answered:
        sent = body(outdir, e["sent_sha256"], e["sent"])
        body(outdir, e["answer_sha256"], e["answered"])
        if any(mark in sent for mark in MARKS):
            census.append(e)
    print("answered requests {}; census posts {}".format(len(answered), len(census)))
    if not census:
        print("NO CENSUS: no answered request names {}".format(MARKER))
        return 3
    census.sort(key=lambda e: e["at"])
    gaps = [i for i in range(1, len(census)) if census[i]["at"] - census[i - 1]["at"] >= GAP_S]
    if len(gaps) != 1:
        raise Failed("{} gaps of {} s or more between census posts, not 1".format(len(gaps), GAP_S))
    first, second = census[:gaps[0]], census[gaps[0]:]
    print("call 1 census posts {}; call 2 census posts {}; gap {:.3f} s".format(
        len(first), len(second), second[0]["at"] - first[-1]["at"]))
    if len(first) != len(second):
        raise Failed("the calls sent {} and {} census posts".format(len(first), len(second)))
    split_at = second[0]["at"]
    for label, group in (("call 1", [e for e in answered if e["at"] < split_at]),
                         ("call 2", [e for e in answered if e["at"] >= split_at])):
        print("{} answered requests {} (all kinds)".format(label, len(group)))
    differ = 0
    print("k sent_bytes request_equal answered_1 answered_2 answer_equal")
    for k, (a, b) in enumerate(zip(first, second), 1):
        request_equal = a["sent_sha256"] == b["sent_sha256"]
        answer_equal = a["answer_sha256"] == b["answer_sha256"]
        print("{} {} {} {} {} {}".format(k, a["sent"], "yes" if request_equal else "NO", a["answered"],
                                         b["answered"], "yes" if answer_equal else "NO"))
        if not request_equal:
            raise Failed("census post {}: the two requests differ, so the answers cannot be compared".format(k))
        if not answer_equal:
            differ += 1
            print("  answer {} call 1 sha256 {}".format(k, a["answer_sha256"]))
            print("  answer {} call 2 sha256 {}".format(k, b["answer_sha256"]))
    if differ:
        print("CENSUS DIFFERS: {} of {} answers".format(differ, len(first)))
        return 4
    print("CENSUS EQUAL: {} of {} answers".format(len(first), len(first)))
    return 0


if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(__doc__)
        sys.exit(2)
    try:
        sys.exit(main(sys.argv[1]))
    except (Failed, OSError, KeyError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
