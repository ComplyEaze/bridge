"""#1029 row B: the five bank batches on BRIDGE LARGE LAB. Not for merge.

python -I make-batches.py OUTDIR    OUTDIR new or empty. Same arguments, same bytes.

Each file is the `vouchers` array of build_import_xml: 3, 10, 25, 50 and 50 numberless
Payments, Receipts and Contras, the shape of lab/852-capture's batches, dated 1 to 7 Oct
2026, each batch spread over all seven days. Amounts run 500.00, 507.00, ... across all
138 vouchers, so no two share a date, a type and an amount.
"""
import datetime
import json
import os
import sys

EXPENSE, INCOME, BANK, CASH = "Lab Vol Expense", "Lab Vol Income", "Lab Vol Bank", "Lab Vol Cash"
BATCHES = (("r03", 3), ("r10", 10), ("r25", 25), ("r50a", 50), ("r50b", 50))


def entries(debit, credit, amount):
    return [
        {"ledger": debit, "amount": amount, "side": "Dr"},
        {"ledger": credit, "amount": amount, "side": "Cr"},
    ]


def batch(tag, size, first):
    vouchers = []
    for i in range(size):
        amount = "%d.00" % (500 + (first + i) * 7)
        if i % 10 == 9:
            kind, parts = "Contra", entries(CASH, BANK, amount) if i % 20 == 9 else entries(BANK, CASH, amount)
        elif i % 2 == 0:
            kind, parts = "Receipt", entries(BANK, INCOME, amount)
        else:
            kind, parts = "Payment", entries(EXPENSE, BANK, amount)
        day = datetime.date(2026, 10, 1) + datetime.timedelta(days=i % 7)
        vouchers.append(
            {
                "bridge_txn_id": "r1029-%s-%03d" % (tag, i + 1),
                "date": day.isoformat(),
                "voucher_type": kind,
                "narration": "r1029 %s %03d" % (tag, i + 1),
                "entries": parts,
            }
        )
    return vouchers


def main():
    if len(sys.argv) != 2:
        sys.exit("usage: make-batches.py OUTDIR")
    outdir = sys.argv[1]
    if os.path.exists(outdir) and os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    os.makedirs(outdir, exist_ok=True)
    first = 0
    for tag, size in BATCHES:
        with open(os.path.join(outdir, "%s.json" % tag), "w", newline="\n") as out:
            json.dump(batch(tag, size, first), out, separators=(",", ":"))
        first += size


if __name__ == "__main__":
    main()
