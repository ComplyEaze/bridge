#!/usr/bin/env python3 -I
"""Writes this sheet's two raw reads of the ledger collection from the one request in templates/, which is a byte copy of
the #692 NONINT row's `List of Ledgers` read (lab/692-nonint-lab-requests at f22e68d9, as lab/2068-integrated-write-control
carried it). Each output is the template with exactly one thing changed: the company text.

  requests/r-ledgers-nonint.utf16le.xml       the template itself (company BRIDGE NONINT LAB)
  requests/r-ledgers-integrated.utf16le.xml   the template with BRIDGE INTEGRATED CTRL LAB

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check

--check proves the first output equals the template byte for byte, the second differs only in the company text, both begin
with a UTF-16LE byte-order mark, a company name with a character outside the plain alphabet is refused, and every file
already in requests/ equals a fresh generation.
"""
import argparse
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "templates", "n-r3-ledgers-fy.utf16le.xml")
OLD = "BRIDGE NONINT LAB"
JOBS = (("r-ledgers-nonint.utf16le.xml", "BRIDGE NONINT LAB"), ("r-ledgers-integrated.utf16le.xml", "BRIDGE INTEGRATED CTRL LAB"))
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")


def build(company):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    data = open(TEMPLATE, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("the template has no UTF-16LE byte-order mark")
    text = data[2:].decode("utf-16-le")
    if text.count(OLD) != 1:
        raise SystemExit("the template does not hold the company text exactly once")
    return b"\xff\xfe" + text.replace(OLD, company).encode("utf-16-le")


def check():
    tpl = open(TEMPLATE, "rb").read()
    assert build(OLD) == tpl, "the first output is not the template"
    other = build(JOBS[1][1])
    assert other != tpl and other[:2] == b"\xff\xfe"
    assert other[2:].decode("utf-16-le").replace(JOBS[1][1], OLD) == tpl[2:].decode("utf-16-le"), "differs elsewhere"
    for bad in ('A"B', "A<B", "A&B", "", " A", "A  B"):
        try:
            build(bad)
        except SystemExit:
            continue
        raise AssertionError("a company name was accepted: %r" % bad)
    committed = os.path.join(HERE, "requests")
    names = sorted(os.listdir(committed)) if os.path.isdir(committed) else []
    assert sorted(names) == sorted(name for name, _ in JOBS), "requests/ holds %r" % names
    for name, company in JOBS:
        assert open(os.path.join(committed, name), "rb").read() == build(company), "committed request differs from a fresh one: %s" % name
    print("gen_requests: ok (2 requests, each the template with the company text changed)")
    return 0


def main(argv):
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv[1:])
    if args.check:
        try:
            return check()
        except AssertionError as error:
            print("gen_requests FAILED: %s" % error)
            return 1
    if not args.out:
        print("usage: gen_requests.py --out DIR | --check")
        return 2
    os.makedirs(args.out, exist_ok=True)
    for name, company in JOBS:
        with open(os.path.join(args.out, name), "wb") as out:
            out.write(build(company))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
