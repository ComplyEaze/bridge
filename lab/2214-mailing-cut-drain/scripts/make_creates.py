"""Writes the 2214 party creates, each one party with its dated mailing entries in
its one creating save, as UTF-16LE files with a byte-order mark.

Run: python -I make_creates.py <empty out folder>. Writes create-000 (5 entries,
the shape check) and create-001 to create-200 (200 entries each), and prints one
line per file: name, bytes, sha256. The output must equal requests-manifest.txt
line for line, else STOP: nothing is sent.

Entry j of a party is dated 1 April 2026 plus (j - 1) days, with address lines
"<j> Lab Street" and "Lab Area", PIN 560001, state Karnataka, country India and
the party's name as its mailing name: the entries of 2074-L's A3, one per day.
"""

import datetime
import hashlib
import os
import sys

COMPANY = "BRIDGE BATCH LAB"
FIRST_DATE = datetime.date(2026, 4, 1)
PARTIES = 200
ENTRIES_PER_PARTY = 200
SHAPE_CHECK_ENTRIES = 5


def entry(name, j):
    day = (FIRST_DATE + datetime.timedelta(days=j - 1)).strftime("%Y%m%d")
    return (
        "<LEDMAILINGDETAILS.LIST>"
        f'<ADDRESS.LIST TYPE="String"><ADDRESS>{j} Lab Street</ADDRESS><ADDRESS>Lab Area</ADDRESS></ADDRESS.LIST>'
        f'<APPLICABLEFROM TYPE="Date">{day}</APPLICABLEFROM>'
        '<PINCODE TYPE="String">560001</PINCODE>'
        f'<MAILINGNAME TYPE="String">{name}</MAILINGNAME>'
        '<STATE TYPE="String">Karnataka</STATE>'
        '<COUNTRY TYPE="String">India</COUNTRY>'
        "</LEDMAILINGDETAILS.LIST>"
    )


def create(index, entries):
    name = f"Lab Mail Load {index:03d}"
    body = "".join(entry(name, j) for j in range(1, entries + 1))
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Import</TALLYREQUEST>"
        "<TYPE>Data</TYPE><ID>All Masters</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        f"<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY></STATICVARIABLES></DESC><DATA>"
        f'<TALLYMESSAGE><LEDGER NAME="{name}" ACTION="Create"><NAME>{name}</NAME>'
        f"<PARENT>Sundry Debtors</PARENT>{body}</LEDGER></TALLYMESSAGE></DATA></BODY></ENVELOPE>"
    )


def main():
    out = sys.argv[1]
    if os.listdir(out):
        sys.exit(f"{out} is not empty")
    files = [(0, SHAPE_CHECK_ENTRIES)] + [(i, ENTRIES_PER_PARTY) for i in range(1, PARTIES + 1)]
    for index, entries in files:
        data = b"\xff\xfe" + create(index, entries).encode("utf-16-le")
        name = f"create-{index:03d}.utf16le.xml"
        with open(os.path.join(out, name), "xb") as handle:
            handle.write(data)
        print(name, len(data), hashlib.sha256(data).hexdigest())


if __name__ == "__main__":
    main()
