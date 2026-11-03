"""#2210 hand-import row: writes the voucher list and the hand-entered copy. Not for merge.

python -I make_files.py OUTDIR

OUTDIR must exist and be empty. Writes:
- batch-vouchers.json: 50 Journals dated 6 April 2026, ids H-001 to H-050, narrations
  `Hand row N`, amounts N.00, Test Expense B Dr / Cash Cr;
- copy-voucher.utf16le.xml: one Journal dated 6 April 2026, Test Expense B Dr / Cash Cr
  5.00, narration `Hand row 5` with no marker and no REMOTEID, in the census row's set-up
  request shape with one voucher (UTF-16LE with its byte-order mark).
"""
import json
import os
import sys

COMPANY = "BRIDGE REVIEW LAB"
DATE = "20260406"


def copy_request():
    voucher = (
        '<VOUCHER VCHTYPE="Journal" ACTION="Create" OBJVIEW="Accounting Voucher View">'
        f"<DATE>{DATE}</DATE><EFFECTIVEDATE>{DATE}</EFFECTIVEDATE>"
        "<VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
        "<NARRATION>Hand row 5</NARRATION>"
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Test Expense B</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-5.00</AMOUNT></ALLLEDGERENTRIES.LIST>"
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Cash</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>5.00</AMOUNT></ALLLEDGERENTRIES.LIST>"
        "</VOUCHER>"
    )
    return (
        '<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER>'
        "<TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC>"
        "<REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES>"
        f"<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC>"
        f'<REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF">{voucher}</TALLYMESSAGE>'
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
            "bridge_txn_id": f"H-{n:03d}",
            "date": DATE,
            "voucher_type": "Journal",
            "narration": f"Hand row {n}",
            "entries": entries(f"{n}.00"),
        }
        for n in range(1, 51)
    ]


def main():
    outdir = sys.argv[1]
    if os.listdir(outdir):
        sys.exit("OUTDIR is not empty")
    with open(os.path.join(outdir, "copy-voucher.utf16le.xml"), "wb") as out:
        out.write(b"\xff\xfe" + copy_request().encode("utf-16-le"))
    with open(os.path.join(outdir, "batch-vouchers.json"), "w", encoding="utf-8", newline="\n") as out:
        out.write(json.dumps(batch(), indent=1) + "\n")


if __name__ == "__main__":
    main()
