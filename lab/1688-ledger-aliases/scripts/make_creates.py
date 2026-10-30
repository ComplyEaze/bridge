"""Writes the 1688 ledger creates, each one ledger under Sundry Debtors whose
first language list carries its own name and then one alias, as UTF-16LE files
with a byte-order mark.

Run: python -I make_creates.py <out folder, absent or empty>. Writes create-i0
to create-i4 and prints one line per file in SHA256.txt's form: sha256, bytes,
requests/creates/<file>. The output must equal the create lines of SHA256.txt,
else STOP: nothing is sent.

The language list is the shape a ledger catalogue answer returns for a ledger
with an alias (lab/masters-m1-answers, lab/1688-b1-alias): NAME.LIST with the
own name first and the alias after it, then LANGUAGEID " 1033".
"""

import hashlib
import os
import sys

COMPANY = "BRIDGE ALIAS LAB"

# (file, the ledger's own name, its one alias)
CREATES = [
    ("create-i0", "Lab Alias Shape", "Lab Shape Alias"),
    ("create-i1", "Lab Alias Script", "लैब खाता"),
    ("create-i2", "Lab Alias Clash", "Lab Other Ledger"),
    ("create-i3", "Lab Alias Fold", "LAB SALES"),
    ("create-i4", "Lab Alias Shared", "Lab Plain Alias"),
]


def create(name, alias):
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Import</TALLYREQUEST>"
        "<TYPE>Data</TYPE><ID>All Masters</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        f"<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY></STATICVARIABLES></DESC><DATA>"
        f'<TALLYMESSAGE><LEDGER NAME="{name}" ACTION="Create">'
        '<LANGUAGENAME.LIST><NAME.LIST TYPE="String">'
        f"<NAME>{name}</NAME><NAME>{alias}</NAME>"
        "</NAME.LIST><LANGUAGEID> 1033</LANGUAGEID></LANGUAGENAME.LIST>"
        "<PARENT>Sundry Debtors</PARENT></LEDGER></TALLYMESSAGE></DATA></BODY></ENVELOPE>"
    )


def main():
    out = sys.argv[1]
    os.makedirs(out, exist_ok=True)
    if os.listdir(out):
        sys.exit(f"{out} is not empty")
    for stem, name, alias in CREATES:
        data = b"\xff\xfe" + create(name, alias).encode("utf-16-le")
        file = f"{stem}.utf16le.xml"
        with open(os.path.join(out, file), "xb") as handle:
            handle.write(data)
        print(f"{hashlib.sha256(data).hexdigest()}  {len(data)}  requests/creates/{file}")


if __name__ == "__main__":
    main()
