#!/usr/bin/env python3 -I
"""Writes this sheet's raw reads from three committed requests. Nothing else is ever sent raw.

  templates/a07-vouchers-shape-20250424.xml   the shipped `Bridge Agent Vouchers` request (the fixture
                                              voucher_goods_lines_two_godowns_shape_lab_request, byte for byte)
  templates/a08-inventory-shape-20250424.xml  the same request whose FETCH asks `ALLINVENTORYENTRIES.*` in place of
                                              `ALLLEDGERENTRIES.*` (the 28 Oct 2026 inventory request, byte for byte)
  templates/class-read-request-20250421.xml   a committed class-resolving voucher read (the fixture
                                              stock-lab-day/stock_lab_taxed_day_voucher_window_request): only its ten
                                              COMPUTE elements are taken, and only to prove the block below is theirs

Three kinds of read, for a day D:
  r-a07-D   the a07 template with only the company and the day changed (as the 28 Oct sheets sent it);
  r-m-D     the CANDIDATE STOCK MOVEMENT REQUEST: the a08 template with only the company and the day changed, and the
            class COMPUTE block inserted right after </FETCH> (where the class read puts it);
  r-span-D-A-B  r-m-D with the AlterID span filter ` AND $AlterID &gt; A AND $AlterID &lt;= B` appended inside the
            window formula, as the window machinery's AlterIdSpan::filter writes it.

Run:  python -I gen_requests.py --out requests        (writes the set this sheet names, below)
      python -I gen_requests.py --check               (rebuilds the set and compares it with requests/ byte for byte,
                                                       and proves each file differs from its template only as above)
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
T = {
    "a07": os.path.join(HERE, "templates", "a07-vouchers-shape-20250424.xml"),
    "a08": os.path.join(HERE, "templates", "a08-inventory-shape-20250424.xml"),
    "cls": os.path.join(HERE, "templates", "class-read-request-20250421.xml"),
}
TEMPLATE_SHA256 = {
    "a07": "47cf36c300a5cc31297033dba27c6233729e93d3ad6e17d72022e73d28352d9c",
    "a08": "8dfda99becc0dafc9a6f3862a22d1d0c5bda02395cf02f08ff6212bcd312278f",
    "cls": "32bc080b2715f9f570e65189b66e225889877fb4f056f387e9fc0979ac2a2632",
}
COMPANY, OLD_COMPANY, OLD_DAY = "BRIDGE SHAPE LAB", "BRIDGE SHAPE LAB", "20250424"
COMPUTES = (
    "<COMPUTE>BRIDGEVCHTYPEGUID:$GUID:VoucherType:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHRESERVEDNAME:$ReservedName:VoucherType:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISSALES:$$IsSales:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISPURCHASE:$$IsPurchase:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISPAYMENT:$$IsPayment:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISRECEIPT:$$IsReceipt:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISCONTRA:$$IsContra:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISJOURNAL:$$IsJournal:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISDEBITNOTE:$$IsDebitNote:$VoucherTypeName</COMPUTE>"
    "<COMPUTE>BRIDGEVCHISCREDITNOTE:$$IsCreditNote:$VoucherTypeName</COMPUTE>"
)
# The set this sheet sends (RUN-SHEET.md names each by file).
A07_DAYS = ["20260706", "20260707", "20260708"]
M_DAYS = ["20260603", "20260608", "20260612", "20260616", "20260619", "20260623", "20260626",
          "20260706", "20260707", "20260708", "20260804", "20260805", "20260806", "20260807"]
SPANS = [("20260603", 112, 114)]


def real_day(value):
    if not re.fullmatch(r"[0-9]{8}", value):
        raise SystemExit("not a day: %r" % value)
    datetime.datetime.strptime(value, "%Y%m%d")


def text_of(path):
    data = open(path, "rb").read()
    if data[:2] != b"\xff\xfe":
        raise SystemExit("%s has no UTF-16LE byte-order mark" % os.path.basename(path))
    return data[2:].decode("utf-16-le")


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def templates_checked():
    for kind, want in TEMPLATE_SHA256.items():
        got = hashlib.sha256(open(T[kind], "rb").read()).hexdigest()
        if got != want:
            raise SystemExit("template %s is not the committed file" % kind)
    cls = text_of(T["cls"])
    start, end = cls.find("</FETCH>") + len("</FETCH>"), cls.find("<FILTERS>")
    if cls[start:end] != COMPUTES:
        raise SystemExit("the COMPUTE block is not the class read's own")


def dated(kind, day):
    real_day(day)
    text = text_of(T[kind])
    if text.count(OLD_DAY) != 4 or text.count(OLD_COMPANY) != 1 or text.count("</FETCH>") != 1:
        raise SystemExit("the %s template is not the shape this script was written for" % kind)
    return text.replace(OLD_COMPANY, COMPANY).replace(OLD_DAY, day)


def a07(day):
    return encode(dated("a07", day))


def m(day, span=None):
    text = dated("a08", day).replace("</FETCH>", "</FETCH>" + COMPUTES)
    if span is not None:
        after, through = span
        if not (isinstance(after, int) and isinstance(through, int) and 0 <= after < through):
            raise SystemExit("bad span")
        close = '"%s"</SYSTEM>' % day
        if text.count(close) != 1:
            raise SystemExit("the window formula is not where it was")
        text = text.replace(close, '"%s" AND $AlterID &gt; %d AND $AlterID &lt;= %d</SYSTEM>' % (day, after, through))
    return encode(text)


def the_set():
    out = {}
    for day in A07_DAYS:
        out["r-a07-%s.utf16le.xml" % day] = a07(day)
    for day in M_DAYS:
        out["r-m-%s.utf16le.xml" % day] = m(day)
    for day, after, through in SPANS:
        out["r-span-%s-%d-%d.utf16le.xml" % (day, after, through)] = m(day, (after, through))
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out")
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    templates_checked()
    wanted = the_set()
    if args.check:
        directory = os.path.join(HERE, "requests")
        present = sorted(name for name in os.listdir(directory) if name.endswith(".xml"))
        if present != sorted(wanted):
            raise SystemExit("requests/ does not hold exactly the set: %s" % sorted(set(present) ^ set(wanted)))
        for name, data in wanted.items():
            if open(os.path.join(directory, name), "rb").read() != data:
                raise SystemExit("%s is not what the templates give" % name)
        print("gen_requests: ok (%d requests; each is its template with only the company, the day, the class "
              "COMPUTE block and, for the span read, the AlterID filter changed)" % len(wanted))
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
