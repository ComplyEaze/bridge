"""Writes this row's front imports and the two ledgers they post to. Not for merge.

python -I make_front.py <folder> ledgers
python -I make_front.py <folder> <file name> <first index> <journals>

UTF-16LE with a byte-order mark. The bodies are the builders of gen.py in
lab/1713-abandoned-ledger-create at 811aed8b: its envelope, its two bulk ledgers and
its Journal, with this row's company, one date (2 April 2026) and REMOTEIDs drawn from
the index, so ranges must not overlap. Prints the file's name, bytes and sha256.
"""
import hashlib
import os
import sys

COMPANY = "BRIDGE BATCH LAB"


def utf16(body):
    return b"\xff\xfe" + body.encode("utf-16-le")


def envelope(report, messages):
    return (
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>"
        "</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>{r}</REPORTNAME><STATICVARIABLES>"
        "<SVCURRENTCOMPANY>{c}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{m}</REQUESTDATA>"
        "</IMPORTDATA></BODY></ENVELOPE>"
    ).format(r=report, c=COMPANY, m=messages)


def ledger_create(name, parent):
    return (
        "<TALLYMESSAGE><LEDGER NAME=\"{n}\" ACTION=\"Create\"><NAME>{n}</NAME><PARENT>{p}</PARENT>"
        "<ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE>"
    ).format(n=name, p=parent)


def journal(index):
    amount = "{}.00".format(1 + index % 100)
    remote_id = "2214c000-0000-4000-8000-{:012x}".format(index)
    entries = (
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bulk Expense</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>"
        "<AMOUNT>-{a}</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bulk Income</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{a}</AMOUNT></ALLLEDGERENTRIES.LIST>"
    ).format(a=amount)
    return (
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{r}\" VCHTYPE=\"Journal\" ACTION=\"Create\" "
        "OBJVIEW=\"Accounting Voucher View\"><DATE>20260402</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
        "<NARRATION>Lab front {i}</NARRATION>{e}</VOUCHER></TALLYMESSAGE>"
    ).format(r=remote_id, i=index, e=entries)


def write(path, body):
    if os.path.exists(path):
        sys.exit("the file is there already")
    raw = utf16(body)
    with open(path, "wb") as out:
        out.write(raw)
    print(os.path.basename(path), len(raw), hashlib.sha256(raw).hexdigest())


def main():
    folder = sys.argv[1]
    if sys.argv[2] == "ledgers":
        body = ledger_create("Lab Bulk Expense", "Indirect Expenses") + ledger_create("Lab Bulk Income", "Indirect Incomes")
        write(os.path.join(folder, "front-ledgers.xml"), envelope("All Masters", body))
        return
    name, first, journals = sys.argv[2], int(sys.argv[3]), int(sys.argv[4])
    body = "".join(journal(index) for index in range(first, first + journals))
    write(os.path.join(folder, name), envelope("Vouchers", body))


if __name__ == "__main__":
    main()
