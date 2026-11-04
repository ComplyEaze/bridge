#!/usr/bin/env python3 -I
"""Writes this sheet's two raw reads from two committed requests. Nothing else is ever sent raw.

  templates/a07-vouchers-shape-20250424.xml  the shipped `Bridge Agent Vouchers` request with ALLLEDGERENTRIES.*:
                                             the fixture voucher_goods_lines_two_godowns_shape_lab_request.utf16le.xml,
                                             byte for byte (the 2057-B sheet's template)
  templates/sl-stock-lines-20260520.xml      the shipped `vouchers` stock_lines request with ALLINVENTORYENTRIES.*:
                                             the fixture stock_movement_mixed_window_stock_lines_shape_lab_request.utf16le.xml,
                                             byte for byte

For the day 20250615 it writes requests/r-a07-20250615.utf16le.xml and requests/r-sl-20250615.utf16le.xml: each template
with only the company text and the day changed (the day stands in four places: the two date variables and the two
bounds of the window formula).

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
COMPANY = "BRIDGE CLASS PURCHASE LAB"
DAY = "20250615"
TEMPLATES = {
    "a07": ("a07-vouchers-shape-20250424.xml",
            "47cf36c300a5cc31297033dba27c6233729e93d3ad6e17d72022e73d28352d9c",
            "BRIDGE SHAPE LAB", "20250424"),
    "sl": ("sl-stock-lines-20260520.xml",
           "df30b272695f5ce6170d41f05730b8dac0ecdd7c514c8c598c7d42e8e5835cd1",
           "BRIDGE SHAPE LAB", "20260520"),
}


def request(kind, day):
    if not re.fullmatch(r"[0-9]{8}", day):
        raise SystemExit("not a day: %r" % day)
    datetime.datetime.strptime(day, "%Y%m%d")
    name, sha, old_company, old_day = TEMPLATES[kind]
    data = open(os.path.join(HERE, "templates", name), "rb").read()
    if hashlib.sha256(data).hexdigest() != sha or data[:2] != b"\xff\xfe":
        raise SystemExit("%s is not the committed file" % name)
    text = data[2:].decode("utf-16-le")
    if text.count(old_day) != 4 or text.count(old_company) != 1:
        raise SystemExit("%s is not the shape this script was written for" % name)
    return b"\xff\xfe" + text.replace(old_company, COMPANY).replace(old_day, day).encode("utf-16-le")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    wanted = {"r-%s-%s.utf16le.xml" % (kind, DAY): request(kind, DAY) for kind in TEMPLATES}
    if args.check:
        directory = os.path.join(HERE, "requests")
        present = sorted(name for name in os.listdir(directory) if name.endswith(".xml"))
        if present != sorted(wanted):
            raise SystemExit("requests/ does not hold exactly the set")
        for name, data in wanted.items():
            if open(os.path.join(directory, name), "rb").read() != data:
                raise SystemExit("%s is not what the template gives" % name)
        print("gen_requests: ok (%d requests; each is its template with only the company and the day changed)" % len(wanted))
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
