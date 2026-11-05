#!/usr/bin/env python3
"""Row 1392-E: the statement PDFs, written with the restored fixture generator. Not for merge.

  python -I make_statements.py OUTDIR    OUTDIR new or empty. Same arguments, same bytes.

Seven one-page HDFC-layout statements for BRIDGE TRANSFER LAB, two accounts. Every line is
drawn by `generate-bank-statement-fixtures.py` in this folder (a byte copy, see ROW.md): its
HDFC header and column geometry, the two-line IMPS row of its page 1, the summary block of its
page 2 and its `write_pdf`. Every account digit, reference, name and amount here is invented.
The user password of every file is PASSWORD.

  python -I make_statements.py --calls    prints each statement's parse_bank_statement arguments

Each file's sha256 is in statements.sha256.
"""
import importlib.util
import json
import os
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
_spec = importlib.util.spec_from_file_location(
    "generator", HERE / "generate-bank-statement-fixtures.py")
gen = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(gen)

PASSWORD = "lab-1392e-user"
OWNER = "lab-1392e-owner-unused"
TAIL = {"A": "7701", "B": "7702"}
TRANSFER, FILLER = "TLAB SELF XFER", "TLAB FILLER"
LEDGER = {"A": "TLab Bank A", "B": "TLab Bank B"}
FOLDER = "C:\\bridge-lab\\1392e"
# the transfer line: skipped on A's statements, a Contra on B's; FILLER is left unmapped,
# so its rows go to the suspense ledger
TREATMENT = {"A": ("TLab Bank B", "skip"), "B": ("TLab Bank A", "contra")}
NARRATION_EDGE = 40  # characters from x=72 to the 240 wrap edge


def reference(n):
    """Twelve digits, as an IMPS reference (bank.rs admits twelve)."""
    return "6113920" + "%05d" % n


def imps(party, n, remark):
    line = "IMPS-%s-%s-ZZZZ-" % (reference(n), party)
    # short of the edge, so the wrap is read as a real space, as in the generator's IMPS row
    assert len(line) < NARRATION_EDGE, line
    return [line, "XXXXXXXX0042-" + remark], "0000" + reference(n)


def tpt(party, n):
    """A transfer with no network reference: bank.rs reads its mode as TXN."""
    line = "50013920%02d-TPT-SELF-%s" % (n, party)
    assert len(line) < NARRATION_EDGE, line
    return [line], "0" * 15 + str(n)


# (date, narration lines and reference column, withdrawal paise, deposit paise)
STATEMENTS = {
    "b1": ("B", 5000000, [
        ("20/10/26", imps(FILLER, 11, "FILL"), 0, 120100),
        ("28/10/26", imps(FILLER, 12, "FILL"), 0, 120200),
    ]),
    "a1": ("A", 8000000, [
        ("23/10/26", imps(TRANSFER, 1, "OWN"), 500100, 0),
    ]),
    "a2": ("A", 7499900, [
        ("23/10/26", imps(TRANSFER, 2, "OWN"), 500200, 0),
    ]),
    "b3": ("B", 5240300, [
        ("29/10/26", imps(TRANSFER, 3, "OWN"), 0, 700300),
    ]),
    "a3": ("A", 6999700, [
        ("29/10/26", imps(TRANSFER, 3, "OWN"), 700300, 0),
    ]),
    "b4": ("B", 5940600, [
        ("01/11/26", imps(FILLER, 13, "FILL"), 0, 120400),
        ("02/11/26", imps(TRANSFER, 4, "OWN"), 0, 900400),
        ("03/11/26", imps(FILLER, 14, "FILL"), 0, 120500),
    ]),
    "a4": ("A", 6299400, [
        ("01/11/26", imps(FILLER, 15, "FILL"), 0, 120600),
        ("02/11/26", tpt(TRANSFER, 4), 900400, 0),
        ("03/11/26", imps(FILLER, 16, "FILL"), 0, 120700),
    ]),
}


def header(account):
    tail = TAIL[account]
    replaced = {
        150: [(340, "Account No :%s%s  ZZ01" % ("0" * 10, tail))],
        162: [(340, "Cust ID :%s%s" % ("0" * 10, "1392"))],
        174: [(340, "Phone no. :%s%s" % ("0" * 7, "1392"))],
    }
    return tuple((top, replaced.get(top, cells)) for top, cells in gen.HDFC_HEADER)


def page(name):
    account, opening, rows = STATEMENTS[name]
    lines, balance, top = list(header(account)), opening, 240
    debits = credits = dr_count = cr_count = 0
    for date, (narration, ref_column), withdrawal, deposit in rows:
        assert (withdrawal == 0) != (deposit == 0), (name, date)
        balance += deposit - withdrawal
        debits, credits = debits + withdrawal, credits + deposit
        dr_count, cr_count = dr_count + (withdrawal > 0), cr_count + (deposit > 0)
        amount = (402, gen.indian(withdrawal)) if withdrawal else (482, gen.indian(deposit))
        lines.append((top, [(2, date), (72, narration[0]), (282, ref_column), (360, date),
                            amount, (562, gen.indian(balance))]))
        for extra in narration[1:]:
            top += 12
            lines.append((top, [(72, extra)]))
        top += 20
    assert top <= 340, name
    lines += [
        (340, [(200, "STATEMENT SUMMARY")]),
        (352, [(70, "Opening Balance"), (200, "Dr Count"), (300, "Cr Count"),
               (400, "Debits"), (480, "Credits"), (560, "Closing Bal")]),
        (364, [(70, gen.indian(opening)), (200, str(dr_count)), (300, str(cr_count)),
               (400, gen.indian(debits)), (480, gen.indian(credits)), (560, gen.indian(balance))]),
        (400, [(28, "HDFC BANK LIMITED")]),
    ]
    other, treatment = TREATMENT[account]
    call = {
        "statement_path": "%s\\pdf\\e1392-%s.pdf" % (FOLDER, name),
        "password_file": FOLDER + "\\pw.txt",
        "bank": "hdfc",
        "account_label": "TLab %s xx%s" % (account, TAIL[account]),
        "opening_balance": gen.indian(opening),
        "closing_balance": gen.indian(balance),
        "total_debits": gen.indian(debits),
        "total_credits": gen.indian(credits),
        "bank_ledger": LEDGER[account],
        "suspense_ledger": "TLab Clearing",
        "mapping": [{"party": TRANSFER, "ledger": other, "treatment": treatment}]
        if any(n[0][0].find(TRANSFER) >= 0 for _, n, _, _ in rows) else [],
    }
    return tuple(lines), call


def main(argv):
    if len(argv) == 2 and argv[1] == "--calls":
        print(json.dumps({name: page(name)[1] for name in STATEMENTS}, indent=1))
        return 0
    if len(argv) != 2:
        raise SystemExit(__doc__)
    directory = pathlib.Path(argv[1])
    if directory.exists() and os.listdir(directory):
        raise SystemExit("OUTDIR is not empty")
    directory.mkdir(parents=True, exist_ok=True)
    for name in STATEMENTS:
        lines, _ = page(name)
        data = gen.write_pdf(pages=[lines], width=638, height=842, user=PASSWORD,
                             owner=OWNER, seed="lab-1392e-" + name)
        (directory / ("e1392-%s.pdf" % name)).write_bytes(data)
        print("wrote e1392-%s.pdf (%d bytes)" % (name, len(data)))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
