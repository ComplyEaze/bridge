#!/usr/bin/env python3 -I
"""Writes this sheet's raw reads from one committed request. Nothing else is ever sent raw.

  templates/a07-vouchers-shape-20250424.xml  the shipped `Bridge Agent Vouchers` request: the fixture
                                             voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml, byte for byte

For each day D of the set below it writes requests/r-a07-D.utf16le.xml: the template with only the company text and
the day changed (the day stands in four places: the two date variables and the two bounds of the window formula).

Run:  python -I gen_requests.py --out requests
      python -I gen_requests.py --check     (rebuilds the set and compares it with requests/ byte for byte)
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "templates", "a07-vouchers-shape-20250424.xml")
TEMPLATE_SHA256 = "47cf36c300a5cc31297033dba27c6233729e93d3ad6e17d72022e73d28352d9c"
OLD_COMPANY, OLD_DAY = "BRIDGE SHAPE LAB", "20250424"
COMPANY = "BRIDGE CLASS PURCHASE LAB"
DAYS = ["20250610", "20250611", "20250612"]


def request(day):
    if not re.fullmatch(r"[0-9]{8}", day):
        raise SystemExit("not a day: %r" % day)
    datetime.datetime.strptime(day, "%Y%m%d")
    data = open(TEMPLATE, "rb").read()
    if hashlib.sha256(data).hexdigest() != TEMPLATE_SHA256 or data[:2] != b"\xff\xfe":
        raise SystemExit("the template is not the committed file")
    text = data[2:].decode("utf-16-le")
    if text.count(OLD_DAY) != 4 or text.count(OLD_COMPANY) != 1:
        raise SystemExit("the template is not the shape this script was written for")
    return b"\xff\xfe" + text.replace(OLD_COMPANY, COMPANY).replace(OLD_DAY, day).encode("utf-16-le")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    wanted = {"r-a07-%s.utf16le.xml" % day: request(day) for day in DAYS}
    if args.check:
        directory = os.path.join(HERE, "requests")
        present = sorted(name for name in os.listdir(directory) if name.endswith(".xml"))
        if present != sorted(wanted):
            raise SystemExit("requests/ does not hold exactly the set")
        for name, data in wanted.items():
            if open(os.path.join(directory, name), "rb").read() != data:
                raise SystemExit("%s is not what the template gives" % name)
        print("gen_requests: ok (%d requests; each is the template with only the company and the day changed)" % len(wanted))
        return 0
    if not args.out:
        raise SystemExit(__doc__)
    os.makedirs(args.out, exist_ok=True)
    for name, data in wanted.items():
        with open(os.path.join(args.out, name), "wb") as handle:
            handle.write(data)
        print("%s %d %s" % (name, len(data), hashlib.sha256(data).hexdigest()))
    return 0


if __name__ == "__main__":
    sys.exit(main())
