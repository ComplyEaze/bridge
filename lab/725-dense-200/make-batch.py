"""Writes batches/b200.json for row 725-D1. Not for merge.

python -I make-batch.py OUT

200 numberless Journals on 1 Apr 2026: Cash Dr, DN Sales Cr, amounts 1.25 to 200.25
(k + 0.25 for k = 1 to 200), ids r725-001 to r725-200, narration "r725 cap 200 k".
"""
import json
import sys

rows = []
for k in range(1, 201):
    amount = f"{k}.25"
    rows.append({
        "bridge_txn_id": f"r725-{k:03d}",
        "date": "2026-04-01",
        "voucher_type": "Journal",
        "narration": f"r725 cap 200 {k}",
        "entries": [
            {"ledger": "Cash", "amount": amount, "side": "Dr"},
            {"ledger": "DN Sales", "amount": amount, "side": "Cr"},
        ],
    })
with open(sys.argv[1], "w", encoding="utf-8", newline="\n") as out:
    json.dump(rows, out, separators=(",", ":"))
