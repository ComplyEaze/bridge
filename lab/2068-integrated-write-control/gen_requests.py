#!/usr/bin/env python3 -I
"""Writes this sheet's six read requests from the six in templates/, which are byte copies of the requests of the #692
NONINT row (lab/692-nonint-lab-requests at f22e68d9: the year's Profit and Loss, Balance Sheet, ledger collection, group
tree and Stock Summary, and the company-marks template). Each output is its template with exactly one thing changed: the
company text `BRIDGE NONINT LAB` becomes the company named here (default `BRIDGE INTEGRATED CTRL LAB`). The marks
template keeps its `@@GUID@@` placeholder, which the lab fills with the company's GUID once it has it (and nothing else).

Run:  python -I gen_requests.py --out requests [--company NAME]
      python -I gen_requests.py --check

--check builds the six files in memory and fails unless each differs from its template only in the company text, still
starts with a UTF-16LE byte-order mark, and names the new company exactly once.
"""
import argparse
import glob
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
OLD = "BRIDGE NONINT LAB"
NEW = "BRIDGE INTEGRATED CTRL LAB"
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")


def templates():
    return sorted(glob.glob(os.path.join(HERE, "templates", "*.utf16le.xml")))


def build(path, company):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    text = data[2:].decode("utf-16-le")
    if text.count(OLD) != 1:
        raise SystemExit("%s does not hold the company text exactly once" % os.path.basename(path))
    return b"\xff\xfe" + text.replace(OLD, company).encode("utf-16-le")


def check():
    files = templates()
    assert len(files) == 6, "expected six templates, found %d" % len(files)
    for path in files:
        out = build(path, NEW)
        old = open(path, "rb").read()
        assert out != old and out[:2] == b"\xff\xfe"
        assert out[2:].decode("utf-16-le").count(NEW) == 1
        assert out[2:].decode("utf-16-le").replace(NEW, OLD) == old[2:].decode("utf-16-le"), "differs elsewhere"
    for bad in ('A"B', "A<B", "A&B", "", " A", "A  B"):
        try:
            build(files[0], bad)
        except SystemExit:
            continue
        raise AssertionError("a company name was accepted: %r" % bad)
    print("gen_requests: ok (6 requests, each its template with the company text changed)")
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--company", default=NEW)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not args.out:
        print("usage: gen_requests.py --out DIR [--company NAME] | --check")
        return 2
    os.makedirs(args.out, exist_ok=True)
    for path in templates():
        data = build(path, args.company)
        name = os.path.basename(path)
        with open(os.path.join(args.out, name), "wb") as out:
            out.write(data)
        print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
