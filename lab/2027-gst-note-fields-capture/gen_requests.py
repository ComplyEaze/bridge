#!/usr/bin/env python3 -I
"""Writes this sheet's voucher reads from two committed requests, each with only the company text and the dates changed.

  templates/r3-native-window.utf16le.xml   the native voucher read of the 628-C0b row (`PLPVA`, a Voucher collection whose
                                           `NATIVEMETHOD`s are `*`, `ALLLEDGERENTRIES.LIST` and `LEDGERENTRIES.LIST`,
                                           filtered to a date window): every native field of each voucher in the window,
                                           byte for byte lab/1904-purchase-identity-capture at e1e579c0,
                                           lab/628-c0b/requests/r3-native-dec.utf16le.xml
  templates/a07-vouchers-shape-20250424.xml the shipped `Bridge Agent Vouchers` request (lab/missing-reads-answers a07),
                                           one day

Run:  python -I gen_requests.py --out requests --from 20250603 --to 20250606 [--company "BRIDGE NOTES LAB"]
      python -I gen_requests.py --check

It writes r-native-<from>-<to>.utf16le.xml (the native read for the window: the template's 20251201 and 20251231
become --from and --to, in the two places each stands), r-native-lists-<from>-<to>.utf16le.xml (the same request with one
`<NATIVEMETHOD>` line added for each voucher-level list named in EXTRA_LISTS, a closed set taken from the empty
placeholders Tally itself returned in the 628-C0b native answer: whether the native read's `*` expands those lists is what the
second read tests, and a list left empty proves nothing) and, for every day of the window, r-a07-<day>.utf16le.xml (the
template's 20250424 becomes the day, in the four places it stands). Dates must be real dates with --from not after
--to and a window of at most 31 days. --check proves each output differs from its template only in those places.
"""
import argparse
import datetime
import hashlib
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
NATIVE = os.path.join(HERE, "templates", "r3-native-window.utf16le.xml")
A07 = os.path.join(HERE, "templates", "a07-vouchers-shape-20250424.xml")
N_COMPANY, N_FROM, N_TO = "BRIDGE GST LEDGER LAB", "20251201", "20251231"
A_COMPANY, A_DAY = "BRIDGE SHAPE LAB", "20250424"
PLAIN = re.compile(r"[A-Za-z0-9._()-]+(?: [A-Za-z0-9._()-]+)*")
# Voucher-level lists that Tally returned as empty placeholders in the 628-C0b native answer
# (lab/628-c0-c0b-answers, c0b/answers/ans-r3-native-dec.xml). Names only from that answer; none is invented here.
# The audit-trail, attendance and Haryana VAT lists are left out as irrelevant to a GST note.
EXTRA_LISTS = (
    "ORIGINVOICEDETAILS.LIST", "INVOICEDELNOTES.LIST", "INVOICEORDERLIST.LIST", "INVOICEINDENTLIST.LIST",
    "INVOICEEXPORTLIST.LIST", "EWAYBILLDETAILS.LIST", "EWAYBILLERRORLIST.LIST", "IRNERRORLIST.LIST",
    "GSTADVADJDETAILS.LIST", "ADVADJUSTMENTREFERENCEDETAILS.LIST", "DUTYHEADDETAILS.LIST",
    "SUPPLEMENTARYDUTYHEADDETAILS.LIST", "EXCLUDEDTAXATIONS.LIST", "CONTRITRANS.LIST", "ALLINVENTORYENTRIES.LIST",
)
ANCHOR = "<NATIVEMETHOD>LEDGERENTRIES.LIST</NATIVEMETHOD>"


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


def native(company, date_from, date_to, extra=False):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    start, end = real_date(date_from), real_date(date_to)
    if start > end or (end - start).days > 30:
        raise SystemExit("the window must run forward and cover at most 31 days")
    text = text_of(NATIVE)
    if text.count(N_COMPANY) != 1 or text.count(N_FROM) != 2 or text.count(N_TO) != 2:
        raise SystemExit("the native template is not the shape this script was written for")
    text = text.replace(N_COMPANY, company).replace(N_FROM, date_from).replace(N_TO, date_to)
    if extra:
        if text.count(ANCHOR) != 1:
            raise SystemExit("the native template has no single anchor line")
        text = text.replace(ANCHOR, ANCHOR + "".join("<NATIVEMETHOD>%s</NATIVEMETHOD>" % name for name in EXTRA_LISTS))
    return encode(text)


def a07(company, day):
    if len(company) > 100 or not PLAIN.fullmatch(company):
        raise SystemExit("company name not plain: %r" % company)
    real_date(day)
    text = text_of(A07)
    if text.count(A_COMPANY) != 1 or text.count(A_DAY) != 4:
        raise SystemExit("the a07 template is not the shape this script was written for")
    return encode(text.replace(A_COMPANY, company).replace(A_DAY, day))


def check():
    tpl = open(NATIVE, "rb").read()
    assert native(N_COMPANY, N_FROM, N_TO) == tpl, "native not reproduced"
    out = native("BRIDGE X LAB", "20260515", "20260518")
    back = out[2:].decode("utf-16-le").replace("BRIDGE X LAB", N_COMPANY).replace("20260515", N_FROM).replace("20260518", N_TO)
    assert back == tpl[2:].decode("utf-16-le"), "native differs elsewhere"
    ext = native(N_COMPANY, N_FROM, N_TO, extra=True)[2:].decode("utf-16-le")
    added = "".join("<NATIVEMETHOD>%s</NATIVEMETHOD>" % name for name in EXTRA_LISTS)
    assert ext.replace(added, "") == tpl[2:].decode("utf-16-le") and ext.count("NATIVEMETHOD>") == tpl[2:].decode("utf-16-le").count("NATIVEMETHOD>") + 2 * len(EXTRA_LISTS)
    assert all(re.fullmatch(r"[A-Z0-9]+(?:\.[A-Z0-9]+)*\.LIST", name) for name in EXTRA_LISTS)
    tpl = open(A07, "rb").read()
    assert a07(A_COMPANY, A_DAY) == tpl, "a07 not reproduced"
    out = a07("BRIDGE X LAB", "20260516")
    assert out[2:].decode("utf-16-le").replace("BRIDGE X LAB", A_COMPANY).replace("20260516", A_DAY) == tpl[2:].decode("utf-16-le")
    for bad in (("20260230", "20260301"), ("20260518", "20260515"), ("20260501", "20260602")):
        try:
            native(N_COMPANY, *bad)
        except SystemExit:
            continue
        raise AssertionError("a window was accepted: %r" % (bad,))
    committed = os.path.join(HERE, "requests")
    for name in sorted(os.listdir(committed)) if os.path.isdir(committed) else ():
        m = re.fullmatch(r"r-(native-lists|native|a07)-([0-9]{8})(?:-([0-9]{8}))?\.utf16le\.xml", name)
        assert m, "unexpected file in requests: %s" % name
        kind, first, last = m.groups()
        company = "BRIDGE NOTES LAB"
        fresh = a07(company, first) if kind == "a07" else native(company, first, last, extra=(kind == "native-lists"))
        assert open(os.path.join(committed, name), "rb").read() == fresh, "committed request differs from a fresh one: %s" % name
    print("gen_requests: ok (the two templates are reproduced byte for byte; only company and dates change)")
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
    os.makedirs(args.out, exist_ok=True)
    jobs = [("r-native-%s-%s.utf16le.xml" % (args.date_from, args.date_to), native(args.company, args.date_from, args.date_to)),
            ("r-native-lists-%s-%s.utf16le.xml" % (args.date_from, args.date_to), native(args.company, args.date_from, args.date_to, extra=True))]
    day = real_date(args.date_from)
    while day <= real_date(args.date_to):
        d = day.strftime("%Y%m%d")
        jobs.append(("r-a07-%s.utf16le.xml" % d, a07(args.company, d)))
        day += datetime.timedelta(days=1)
    for name, data in jobs:
        with open(os.path.join(args.out, name), "wb") as out:
            out.write(data)
        print("%s  %d  %s" % (hashlib.sha256(data).hexdigest(), len(data), name))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
