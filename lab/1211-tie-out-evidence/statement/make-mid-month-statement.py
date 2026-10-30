"""Write the synthetic Union Bank statement for the #1211 mid-month row (R3).

No client data: every name, number and amount is invented. The layout is the
`ubi-synthetic.pdf` one, from the repository's fixture generator as it stood
before its removal: `scripts/generate-bank-statement-fixtures.py` at the parent
of c9e0ba21. To run this script, put that file beside it unchanged
(`git show c9e0ba21^:scripts/generate-bank-statement-fixtures.py`). Standard
library only and deterministic: the same script writes the same bytes.

The statement runs from 3 to 28 September 2026, so the tie-out of the whole
statement reads the bank ledger at 3 September and at 29 September, neither of
them the 1st of a month. It has one row on each edge day. It opens at 12,300.00
Cr: the 11,800.00 Union Bank holds at the end of August in the run sheet's book,
plus the 500.00 Receipt the sheet keys on 2 September, before the statement's
first row, so that a read moved back to the month's start would not tie.

Usage: python3 -I make-mid-month-statement.py OUT.pdf
"""
import hashlib
import importlib.util
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location(
    "generator", HERE / "generate-bank-statement-fixtures.py")
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)

ubi_row, ubi_footer = generator.ubi_row, generator.ubi_footer

ROWS = [
    # date, transaction id, remarks, amount, side
    ("03-09-2026", "S10000003", "NEFT:NORTHWIND TRADERS ZZZZN00000000003", "2500.00", "Cr"),
    ("09-09-2026", "S10000009", "IMPSAB/700000000009/BLUE RIVER CO/9000000009", "1200.00", "Dr"),
    ("17-09-2026", "S10000017", "UPIAB/600000000017/CR/GREEN FIELD LTD/ZZZZ/gf@okzz", "650.00", "Cr"),
    ("28-09-2026", "S10000028", "CLG/SILVER OAK MUTUAL", "300.00", "Dr"),
]
OPENING_PAISE = 1_230_000  # 12,300.00 Cr


def statement_page():
    lines = [
        (40, [(20, "SYNTHETIC STATEMENT - NOT A REAL ACCOUNT")]),
        (60, [(20, "Account Number"), (120, "000000000007788")]),
        (72, [(20, "Savings Account No"), (120, "9**** *1234")]),
        (84, [(20, "CIF ID"), (120, "000001234")]),
        (96, [(20, "Statement Period"), (120, "03-09-2026 to 28-09-2026")]),
        generator.UBI_HEADER_ROW,
    ]
    balance = OPENING_PAISE
    for index, (date, txn, remarks, amount, side) in enumerate(ROWS):
        paise = round(float(amount) * 100)
        balance += paise if side == "Cr" else -paise
        printed = f"{abs(balance) // 100}.{abs(balance) % 100:02d}({'Cr' if balance >= 0 else 'Dr'})"
        lines.append(ubi_row(160 + 12 * index, date, txn, remarks, f"{amount}({side})", printed))
    lines.append(ubi_footer(1, 1))
    return tuple(lines), balance


def main(argv):
    if len(argv) != 2:
        raise SystemExit(__doc__)
    page, closing = statement_page()
    data = generator.write_pdf(
        pages=[page], width=595, height=842,
        user="synthetic-user-7788", owner="synthetic-owner-unused-1211", seed="ubi-1211-r3")
    out = pathlib.Path(argv[1])
    out.write_bytes(data)
    print(f"wrote {out.name} ({len(data)} bytes, sha256 {hashlib.sha256(data).hexdigest()}); "
          f"opening 12300.00 Cr, closing {closing // 100}.{closing % 100:02d} Cr")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
