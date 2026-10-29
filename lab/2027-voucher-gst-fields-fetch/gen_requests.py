#!/usr/bin/env python3
"""Writes this sheet's voucher reads from one committed request, with only the company text, the day and (for the
candidate) six FETCH words changed.

  templates/a07-vouchers-shape-20250424.xml  the shipped `Bridge Agent Vouchers` request (lab/missing-reads-answers a07),
                                            one day, byte for byte lab/2027-gst-note-fields-capture at 4cde7eb3. Its FETCH
                                            equals master's AGENT_VOUCHER_FETCH (src-tauri/src/agent_read_profiles.rs).

Run:  python -I gen_requests.py --out requests --from 20250603 --to 20250607 [--company "BRIDGE NOTES LAB"]
      python -I gen_requests.py --check

For every day of the window it writes r-control-<day>.utf16le.xml (the template, its 20250424 the day in the four places it
stands) and r-candidate-<day>.utf16le.xml (the same, with the six voucher-level words of CANDIDATE_WORDS appended to its FETCH
after REFERENCEDATE, its last word). The six are a closed set: the elements the 28 Oct capture
(lab/2027-gst-note-fields-capture-answers at 07e3bc34) found at the voucher's top level in the native read and not in the
shipped one. Dates must be real dates with --from not after --to and a window of at most 7 days. --check proves each output
differs from the template only in those places.
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
A07 = os.path.join(HERE, "templates", "a07-vouchers-shape-20250424.xml")
A_COMPANY, A_DAY = "BRIDGE SHAPE LAB", "20250424"
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")
FETCH_TAIL = "PARTYGSTIN,REFERENCEDATE</FETCH>"
CANDIDATE_WORDS = (
    "PLACEOFSUPPLY", "STATENAME", "GSTREGISTRATIONTYPE",
    "GSTNATUREOFRETURN", "VATPARTYTRANSRETURNNUMBER", "VATPARTYTRANSRETURNDATE",
)


def real_date(value):
    if not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("not a date: %r" % value)
    try:
        return datetime.datetime.strptime(value, "%Y%m%d")
    except ValueError:
        raise SystemExit("not a date: %r" % value)


def text_of(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    return data[2:].decode("utf-16-le")


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def control(company, day):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    real_date(day)
    text = text_of(A07)
    if text.count(A_COMPANY) != 1 or text.count(A_DAY) != 4 or text.count(FETCH_TAIL) != 1:
        raise SystemExit("the a07 template is not the shape this script was written for")
    return encode(text.replace(A_COMPANY, company).replace(A_DAY, day))


def candidate(company, day):
    text = control(company, day)[2:].decode("utf-16-le")
    added = "".join("," + word for word in CANDIDATE_WORDS)
    return encode(text.replace(FETCH_TAIL, FETCH_TAIL.replace("</FETCH>", added + "</FETCH>")))


def check():
    tpl = open(A07, "rb").read()
    assert control(A_COMPANY, A_DAY) == tpl, "the control does not reproduce the template"
    out = control("BRIDGE X LAB", "20260516")[2:].decode("utf-16-le")
    assert out.replace("BRIDGE X LAB", A_COMPANY).replace("20260516", A_DAY) == tpl[2:].decode("utf-16-le")
    cand = candidate(A_COMPANY, A_DAY)[2:].decode("utf-16-le")
    added = "".join("," + word for word in CANDIDATE_WORDS)
    assert cand.replace(added, "", 1) == tpl[2:].decode("utf-16-le"), "the candidate differs elsewhere"
    assert cand.count(added) == 1 and all(re.fullmatch(r"[A-Z]+", word) for word in CANDIDATE_WORDS)
    for bad in ("20260230", "2026051", "abcdefgh"):
        try:
            control(A_COMPANY, bad)
        except SystemExit:
            continue
        raise AssertionError("a day was accepted: %r" % bad)
    committed = os.path.join(HERE, "requests")
    names = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    for name in names:
        m = re.fullmatch(r"r-(control|candidate)-([0-9]{8})\.utf16le\.xml", name)
        assert m, "unexpected file in requests: %s" % name
        kind, day = m.groups()
        fresh = (control if kind == "control" else candidate)("BRIDGE NOTES LAB", day)
        assert open(os.path.join(committed, name), "rb").read() == fresh, "committed request differs: %s" % name
    print("gen_requests: ok (the template is reproduced byte for byte; only company, day and the six words change; %d committed)"
          % len(names))
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--from", dest="date_from")
    parser.add_argument("--to", dest="date_to")
    parser.add_argument("--company", default="BRIDGE NOTES LAB")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not (args.out and args.date_from and args.date_to):
        print("usage: gen_requests.py --out DIR --from D --to D [--company N] | --check")
        return 2
    start, end = real_date(args.date_from), real_date(args.date_to)
    if start > end or (end - start).days > 6:
        raise SystemExit("the window must run forward and cover at most 7 days")
    os.makedirs(args.out, exist_ok=True)
    day = start
    while day <= end:
        d = day.strftime("%Y%m%d")
        for name, data in (("r-control-%s.utf16le.xml" % d, control(args.company, d)),
                           ("r-candidate-%s.utf16le.xml" % d, candidate(args.company, d))):
            with open(os.path.join(args.out, name), "wb") as out:
                out.write(data)
            print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
        day += datetime.timedelta(days=1)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
