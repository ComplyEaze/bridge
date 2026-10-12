"""Two synthetic Union Bank-shaped statements for the transfer-pairing lab item.
Reuses the repository generator's layout and PDF writer; every value is invented."""
import importlib.util, sys, hashlib, json, os
REPO, OUT = sys.argv[1], sys.argv[2]
spec = importlib.util.spec_from_file_location("gen", os.path.join(REPO, "scripts/generate-bank-statement-fixtures.py"))
g = importlib.util.module_from_spec(spec); spec.loader.exec_module(g)

# oldest first: (date, transaction id, remarks, side, paise, evidence tag)
A_ROWS = [
    ("03-08-2026", "T0000101", "IMPSAB/600000000101/TLAB BANK B/9000000101", "dr", 500000, "E1"),
    ("04-08-2026", "T0000102", "CLG/TLAB BANK B", "dr", 200000, "E2"),
    ("05-08-2026", "T0000103", "IMPSAB/600000000103/TLAB BANK B/9000000103", "dr", 300000, "E3a"),
    ("07-08-2026", "T0000104", "IMPSAB/600000000104/TLAB BANK B/9000000104", "dr", 300000, "E3b"),
    ("08-08-2026", "T0000106", "CLG/TLAB BANK B", "dr", 100000, "E7"),
]
B_ROWS = [
    ("03-08-2026", "T0000201", "IMPSAB/600000000101/TLAB BANK A/9000000101", "cr", 500000, "E1"),
    ("04-08-2026", "T0000202", "CLG/TLAB BANK A", "cr", 200000, "E2"),
    ("05-08-2026", "T0000203", "IMPSAB/600000000103/TLAB BANK A/9000000103", "cr", 300000, "E3a"),
    ("07-08-2026", "T0000204", "IMPSAB/600000000104/TLAB BANK A/9000000104", "cr", 300000, "E3b"),
    ("08-08-2026", "T0000207", "CLG/TLAB BANK A", "cr", 100000, "E7a"),
    ("08-08-2026", "T0000208", "CLG/TLAB BANK A", "cr", 100000, "E7b"),
    ("09-08-2026", "T0000205", "IMPSAB/600000000105/TLAB DIRECT/9000000105", "cr", 70000, "E4"),
]

def amount(paise, side):
    return f"{paise // 100}.{paise % 100:02d}({'Dr' if side == 'dr' else 'Cr'})"

def balance_text(paise):
    return f"{abs(paise) // 100}.{abs(paise) % 100:02d}({'Cr' if paise >= 0 else 'Dr'})"

def statement(rows, opening, account):
    lines = [
        (40, [(20, "SYNTHETIC STATEMENT - NOT A REAL ACCOUNT")]),
        (60, [(20, "Account Number"), (120, account)]),
        (72, [(20, "Savings Account No"), (120, "9**** *0000")]),
        (84, [(20, "CIF ID"), (120, "000009999")]),
        (96, [(20, "Statement Period"), (120, "01-08-2026 to 31-08-2026")]),
        g.UBI_HEADER_ROW,
    ]
    balance, top = opening, 160
    for date, txn, remarks, side, paise, _ in rows:
        balance += paise if side == "cr" else -paise
        lines.append(g.ubi_row(top, date, txn, remarks, amount(paise, side), balance_text(balance)))
        top += 12
    lines.append(g.ubi_footer(1, 1))
    return [tuple(lines)], balance

manifest = {}
for name, rows, opening, account in [
    ("tlab-bank-a.pdf", A_ROWS, 10000000, "000000000000101"),
    ("tlab-bank-b.pdf", B_ROWS, 5000000, "000000000000202"),
]:
    pages, closing = statement(rows, opening, account)
    data = g.write_pdf(pages, 595, 842, "", "synthetic-owner-unused-tlab", name)
    open(os.path.join(OUT, name), "wb").write(data)
    manifest[name] = dict(sha256=hashlib.sha256(data).hexdigest(), account=account,
                          opening=f"{opening/100:.2f}", closing=f"{closing/100:.2f}",
                          rows=[(d, t, r, s, f"{p/100:.2f}", e) for d, t, r, s, p, e in rows])
print(json.dumps(manifest, indent=1))
