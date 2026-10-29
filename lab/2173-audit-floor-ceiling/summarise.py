#!/usr/bin/env python3 -I
"""Summarise one recording folder of read_relay.py: what ComplyEaze Bridge sent and what Tally answered.

Run:  python -I summarise.py DIR

Reads DIR/manifest.tsv only (the relay's own record; nothing is sent anywhere) and prints, one line each:
  requests: the count of each verdict;
  census: one line per census request (`Bridge Agent Voucher Census`): its dates, AlterID span, rows, bytes and seconds;
  census total: the census requests and their rows added up;
  data parts: their count; the first part's rows; the most rows and the most bytes in one part; the seconds of the slowest;
    the total rows; the mean bytes per row (the measured wire weight of a voucher on this book, the envelope included);
  marks: the count of `Bridge Agent Company High Water` reads;
  over: every request whose answer took more than 20 s to its last byte, or was larger than 16,777,216 bytes (a data part)
    or 33,554,432 bytes (any answer), or did not parse.
Exit 0 when the folder holds a readable manifest; 2 when it does not. The figures are recorded, not judged: the run sheet's
EXPECT lines judge them.
"""
import os
import sys

CENSUS = "Bridge Agent Voucher Census"
DATA = "Bridge Agent Vouchers"
MARKS = "Bridge Agent Company High Water"
BUDGET = 16_777_216
CAP = 33_554_432


def main(argv):
    if len(argv) != 2:
        print("usage: summarise.py DIR")
        return 2
    path = os.path.join(argv[1], "manifest.tsv")
    try:
        lines = [line.split("\t") for line in open(path, encoding="utf-8").read().splitlines()]
    except OSError as error:
        print("no manifest: %s" % error)
        return 2
    if not lines or any(len(line) != 19 for line in lines):
        print("manifest unreadable: %d lines, columns %s" % (len(lines), sorted({len(line) for line in lines})))
        return 2
    verdicts = {}
    for line in lines:
        verdicts[line[4]] = verdicts.get(line[4], 0) + 1
    print("requests: %d (%s)" % (len(lines), ", ".join("%s %d" % item for item in sorted(verdicts.items()))))
    forwarded = [line for line in lines if line[4] == "forwarded"]
    census = [line for line in forwarded if line[5] == CENSUS]
    for line in census:
        print("census: #%s %s-%s span (%s, %s] rows %s bytes %s seconds %s" % (
            line[0], line[6], line[7], line[8], line[9], line[15], line[13], line[17]))
    print("census total: %d requests, rows %d" % (
        len(census), sum(int(line[15]) for line in census if line[15].isdigit())))
    parts = [line for line in forwarded if line[5] == DATA]
    counted = [line for line in parts if line[15].isdigit()]
    if parts:
        total_rows = sum(int(line[15]) for line in counted)
        total_bytes = sum(int(line[13]) for line in counted)
        print("data parts: %d; first part rows %s; most rows %d; most bytes %d; slowest %.3f s; total rows %d; "
              "mean bytes per row %s" % (
                  len(parts), parts[0][15], max((int(line[15]) for line in counted), default=0),
                  max(int(line[13]) for line in parts), max(float(line[17] or 0) for line in parts), total_rows,
                  total_bytes // total_rows if total_rows else "none"))
    else:
        print("data parts: 0")
    print("marks: %d" % sum(1 for line in forwarded if line[5] == MARKS))
    over = [line for line in forwarded
            if float(line[17] or 0) > 20 or int(line[13]) > CAP or (line[5] == DATA and int(line[13]) > BUDGET)
            or line[15] == "unparsed"]
    for line in over:
        print("over: #%s %s bytes %s rows %s seconds %s" % (line[0], line[5], line[13], line[15], line[17]))
    if not over:
        print("over: none")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
