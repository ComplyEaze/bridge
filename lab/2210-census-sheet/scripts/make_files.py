"""#2210 census row: writes the set-up import and the two voucher lists. Not for merge.

python -I make_files.py OUTDIR

OUTDIR must exist and be empty. Writes:
- setup-vouchers.utf16le.xml: 148 Journals dated 1 April 2026, each Test Expense B Dr /
  Cash Cr 0.01, narrations `set-up 2` to `set-up 149`, in the first row's set-up request
  shape (UTF-16LE with its byte-order mark, as Tally reads Bridge's own requests);
- batch-vouchers.json: 50 Journals dated 2 April 2026, ids C-001 to C-050, narrations
  `Census row N`, amounts N.00, Test Expense B Dr / Cash Cr;
- payment-vouchers.json: one Payment dated 4 April 2026, id C-PAY, Test Expense B Dr /
  Cash Cr 1.00, narration `Census row payment`.
"""
import json
import os
import sys

COMPANY = "BRIDGE REVIEW LAB"
SETUP_FIRST, SETUP_LAST = 2, 149


def setup_voucher(index):
    return (
        '<VOUCHER VCHTYPE="Journal" ACTION="Create" OBJVIEW="Accounting Voucher View">'
        "<DATE>20260401</DATE><EFFECTIVEDATE>20260401</EFFECTIVEDATE>"
        "<VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
        f"<NARRATION>set-up {index}</NARRATION>"
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Test Expense B</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-0.01</AMOUNT></ALLLEDGERENTRIES.LIST>"
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Cash</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>0.01</AMOUNT></ALLLEDGERENTRIES.LIST>"
        "</VOUCHER>"
    )


def setup_request():
    vouchers = "".join(setup_voucher(i) for i in range(SETUP_FIRST, SETUP_LAST + 1))
    return (
        '<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER>'
        "<TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC>"
        "<REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES>"
        f"<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC>"
        f'<REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF">{vouchers}</TALLYMESSAGE>'
        "</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>"
    )


def entries(amount):
    return [
        {"ledger": "Test Expense B", "amount": amount, "side": "Dr"},
        {"ledger": "Cash", "amount": amount, "side": "Cr"},
    ]


def batch():
    return [
        {
            "bridge_txn_id": f"C-{n:03d}",
            "date": "20260402",
            "voucher_type": "Journal",
            "narration": f"Census row {n}",
            "entries": entries(f"{n}.00"),
        }
        for n in range(1, 51)
    ]


def payment():
    return [
        {
            "bridge_txn_id": "C-PAY",
            "date": "20260404",
            "voucher_type": "Payment",
            "narration": "Census row payment",
            "entries": entries("1.00"),
        }
    ]


def main():
    outdir = sys.argv[1]
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    with open(os.path.join(outdir, "setup-vouchers.utf16le.xml"), "wb") as out:
        out.write(b"\xff\xfe" + setup_request().encode("utf-16-le"))
    for name, rows in (("batch-vouchers.json", batch()), ("payment-vouchers.json", payment())):
        with open(os.path.join(outdir, name), "w", encoding="utf-8", newline="\n") as out:
            out.write(json.dumps(rows, indent=1) + "\n")


if __name__ == "__main__":
    main()
