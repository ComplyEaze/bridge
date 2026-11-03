"""#852: the two 50-voucher bank batches on BRIDGE LARGE LAB. Not for merge.

python -I make-batches.py OUTDIR    OUTDIR new or empty. Same arguments, same bytes.

Each file is the `vouchers` array of build_import_xml. Numberless, 10 days each, bank
Payments, Receipts and Contras, the kind #852 failed on. The first voucher of each is a
Contra that moves cash into the bank, dated first, so the bank never goes below zero.
"""
import datetime
import json
import os
import sys

EXPENSE, INCOME, BANK, CASH = "Lab Vol Expense", "Lab Vol Income", "Lab Vol Bank", "Lab Vol Cash"


def entries(debit, credit, amount):
    return [
        {"ledger": debit, "amount": amount, "side": "Dr"},
        {"ledger": credit, "amount": amount, "side": "Cr"},
    ]


def batch(tag, first_day):
    vouchers = []
    for i in range(50):
        amount = "%d.00" % (500 + (first_day * 37 + i * 13) % 400)
        if i == 0:
            kind, parts = "Contra", entries(BANK, CASH, "300000.00")
        elif i % 10 == 9:
            kind, parts = "Contra", entries(CASH, BANK, amount) if i % 20 == 9 else entries(BANK, CASH, amount)
        elif i % 2 == 0:
            kind, parts = "Receipt", entries(BANK, INCOME, amount)
        else:
            kind, parts = "Payment", entries(EXPENSE, BANK, amount)
        day = datetime.date(2026, 10, first_day) + datetime.timedelta(days=i // 5)
        vouchers.append(
            {
                "bridge_txn_id": "c852-%s-%03d" % (tag, i + 1),
                "date": day.isoformat(),
                "voucher_type": kind,
                "narration": "c852 %s %03d" % (tag, i + 1),
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
    for tag, first_day in (("a", 11), ("b", 21)):
        with open(os.path.join(outdir, "c50%s.json" % tag), "w", newline="\n") as out:
            json.dump(batch(tag, first_day), out, separators=(",", ":"))


if __name__ == "__main__":
    main()
