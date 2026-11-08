#!/usr/bin/env python3
"""Row 2377-L, Part B: the lab-made parties' writes and reads (L5, L7) on BRIDGE BILLTRAIL LAB, and the dense book's
largest parties (L8) on BRIDGE CORPUS DENSE.

  python -I l2377b.py writes --out <dir>
      Writes every Part B request for BRIDGE BILLTRAIL LAB, UTF-16LE with a byte-order mark, and prints one line
      each (name, bytes, sha256). One request holds one change:
        w0-income             create the ledger "L5 Income" under Indirect Incomes, bill-wise off
        w1-party-<class>      create one party ledger under Sundry Debtors, bill-wise on, named by CLASSES below
        w2-bill-<class>       one Journal on 20251001: the party Dr 100.00 with one New Ref bill "L5-<class>", and
                              L5 Income Cr 100.00
        r-lo-<class>          that party's Ledger Outstandings to 20260331 (the shape of l2377.py, r04's text)
        w1-party-l7, w2-bill-l7  "L7 Party" and a Journal with one New Ref bill named "On Account", 100.00
        w3-onacct-l7          a second Journal for L7 Party: 50.00 Dr allocated as On Account (BILLTYPE On Account)
        r-lo-l7               L7 Party's Ledger Outstandings to 20260331
      Each value is escaped once by master's escape_text rule, in text and in NAME="..." alike, and never put in a
      formula. The ledger create is master's LedgerCreate message (ledger_create.rs:43-46) in master's import envelope
      (import_envelope.rs:30); the Journal is lab/1108-a2's cash Journal template with one bill allocation added.
  python -I l2377b.py present <snapshot answer>
      For each class, and for L7 Party and L5 Income: "exact" when a LEDGER of exactly that NAME is in the answer,
      "trimmed" when only a name equal to it after trimming spaces is, else "absent". Names come from this script,
      so none is typed in a shell.
  python -I l2377b.py tie <class or l7> <lo answer> <snapshot answer>
      Runs l2377.py's tie (from this folder, python -I, no shell) with that party's exact name.
  python -I l2377b.py counters <import answer>
      Prints CREATED, ALTERED, DELETED, LASTVCHID, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS and any
      LINEERROR text. "one created" only when CREATED is 1 and every other counter present is 0.
  python -I l2377b.py top3 <Bills Receivable answer of bills.py's ref-receivable> --out <dir>
      The three parties with the most bills in that answer whose names pass bills.py's admitted alphabet, and for
      each lo-top<k> to 20260331 for BRIDGE CORPUS DENSE. Writes top3.txt (name, bills).

Exit 0 = done, 1 = failed (the line says why; the sheet treats it as STOP), 2 = the command line is wrong.
"""
import hashlib
import os
import re
import subprocess
import sys
import uuid
import xml.etree.ElementTree as ET
from collections import Counter

LAB = "BRIDGE BILLTRAIL LAB"
DENSE = "BRIDGE CORPUS DENSE"
INCOME = "L5 Income"
DAY = "20251001"
TO = "20260331"
# One class of mark a party, each the only mark in its name (P6).
CLASSES = (
    ("amp", "L5 Amp & Co"),
    ("angle", "L5 Angle <A>"),
    ("dquote", 'L5 Quote "Q"'),
    ("squote", "L5 O'Neil"),
    ("dollar", "L5 Dollar $1"),
    ("dollar2", "L5 $$Name"),
    ("hash", "L5 Hash #1"),
    ("colon", "L5 Colon: A"),
    ("dot", "L5 Dot Co."),
    ("dash", "L5 Dash-Co"),
    ("comma", "L5 Comma, A"),
    ("paren", "L5 Paren (A)"),
    ("slash", "L5 Slash A/B"),
    ("nonascii", "L5 Café"),
    ("trail", "L5 Trail "),
)
MASTERS = (
    '<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY>'
    "<IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{company}"
    "</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE>"
    '<LEDGER NAME="{attr}" ACTION="Create"><NAME>{name}</NAME><PARENT>{parent}</PARENT>'
    "<ISBILLWISEON>{billwise}</ISBILLWISEON></LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>"
)
JOURNAL = (
    '<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY>'
    "<IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{company}"
    "</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE xmlns:UDF=\"TallyUDF\">"
    '<VOUCHER REMOTEID="{remote}" VCHTYPE="Journal" ACTION="Create" OBJVIEW="Accounting Voucher View"><DATE>{day}</DATE>'
    "<VOUCHERTYPENAME>Journal</VOUCHERTYPENAME><NARRATION>Row 2377-L Part B</NARRATION>"
    "<ALLLEDGERENTRIES.LIST><LEDGERNAME>{party}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>"
    "<AMOUNT>-{amount}</AMOUNT><BILLALLOCATIONS.LIST><NAME>{bill}</NAME><BILLTYPE>{billtype}</BILLTYPE>"
    "<AMOUNT>-{amount}</AMOUNT></BILLALLOCATIONS.LIST></ALLLEDGERENTRIES.LIST>"
    "<ALLLEDGERENTRIES.LIST><LEDGERNAME>{income}</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE>"
    "<AMOUNT>{amount}</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY>"
    "</ENVELOPE>"
)
# r04's text, as in l2377.py and bills.py.
LEDGER_OUTSTANDINGS = (
    "\n<ENVELOPE>\n    <HEADER>\n        <VERSION>1</VERSION>\n        <TALLYREQUEST>Export</TALLYREQUEST>\n"
    "        <TYPE>Data</TYPE>\n        <ID>Ledger Outstandings</ID>\n    </HEADER>\n    <BODY>\n        <DESC>\n"
    "            <STATICVARIABLES>\n                <SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>\n"
    "                <LedgerName>{ledger}</LedgerName>\n"
    '                <SVTODATE TYPE="Date">{day}</SVTODATE>\n'
    "                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>\n                <EXPLODEFLAG>Yes</EXPLODEFLAG>\n"
    "            </STATICVARIABLES>\n        </DESC>\n    </BODY>\n</ENVELOPE>\n"
)
R04 = ("BRIDGE BILLTRAIL LAB", "Lab Party P1", "20260331", 1198,
       "f560dba1651ca4b4d89359feffd8e4f6058ee15ae380c05929cca072313d95dc")
NAME = re.compile(r"\A[A-Za-z0-9]+( [A-Za-z0-9]+)*\Z")
FORMULA_WORDS = ("and", "or", "not", "is", "in", "contains", "like", "between", "starting", "ending", "with",
                 "equal", "greater", "less", "than")
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED", "EXCEPTIONS")
FORBIDDEN_RAW = re.compile("[\x00-\x08\x0b\x0c\x0e-\x1f]")
REFERENCE = re.compile(r"&#(x[0-9A-Fa-f]+|[0-9]+);")


class Failed(Exception):
    pass


def encode(text):
    return b"\xff\xfe" + text.encode("utf-16-le")


def text(value):
    """Master's escape_text (bridge-tally-protocol/src/xml_text.rs:41-50), for text and attribute alike."""
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))


def write(folder, name, body):
    data = encode(body)
    os.makedirs(folder, exist_ok=True)
    with open(os.path.join(folder, name + ".utf16le.xml"), "xb") as handle:
        handle.write(data)
    return "{}.utf16le.xml {} {}".format(name, len(data), hashlib.sha256(data).hexdigest())


def ledger(name, parent, billwise):
    return MASTERS.format(company=text(LAB), attr=text(name), name=text(name), parent=text(parent), billwise=billwise)


def journal(tag, party, bill, billtype, amount):
    remote = uuid.uuid5(uuid.NAMESPACE_URL, "row-2377-b/" + tag)
    return JOURNAL.format(company=text(LAB), remote=remote, day=DAY, party=text(party), bill=text(bill), billtype=billtype,
                          amount=amount, income=text(INCOME))


def lo(company, name):
    return LEDGER_OUTSTANDINGS.format(company=text(company), ledger=text(name), day=TO)


def check_template():
    company, name, day, size, digest = R04
    data = encode(LEDGER_OUTSTANDINGS.format(company=text(company), ledger=text(name), day=day))
    if len(data) != size or hashlib.sha256(data).hexdigest() != digest:
        raise Failed("the Ledger Outstandings template does not render r04's bytes")


def writes(folder):
    check_template()
    out = [write(folder, "w0-income", ledger(INCOME, "Indirect Incomes", "No"))]
    for tag, name in CLASSES:
        out.append(write(folder, "w1-party-" + tag, ledger(name, "Sundry Debtors", "Yes")))
        out.append(write(folder, "w2-bill-" + tag, journal(tag, name, "L5-" + tag, "New Ref", "100.00")))
        out.append(write(folder, "r-lo-" + tag, lo(LAB, name)))
    out.append(write(folder, "w1-party-l7", ledger("L7 Party", "Sundry Debtors", "Yes")))
    out.append(write(folder, "w2-bill-l7", journal("bill-l7", "L7 Party", "On Account", "New Ref", "100.00")))
    out.append(write(folder, "w3-onacct-l7", journal("onacct-l7", "L7 Party", "On Account", "On Account", "50.00")))
    out.append(write(folder, "r-lo-l7", lo(LAB, "L7 Party")))
    return "\n".join(out)


def decoded(path):
    with open(path, "rb") as handle:
        data = handle.read()
    if data.startswith(b"\xff\xfe"):
        body = data[2:].decode("utf-16-le")
    elif len(data) >= 2 and len(data) % 2 == 0 and data[1] == 0:
        body = data.decode("utf-16-le")
    else:
        raise Failed("{}: not UTF-16LE".format(path))
    return REFERENCE.sub(keep_reference, FORBIDDEN_RAW.sub("", body))


def keep_reference(match):
    body = match.group(1)
    code = int(body[1:], 16) if body[0] == "x" else int(body)
    return "" if code <= 8 or code in (11, 12) or 14 <= code <= 31 else match.group(0)


def counters(path):
    try:
        root = ET.fromstring(decoded(path))
    except ET.ParseError as error:
        raise Failed("{}: does not parse ({})".format(path, error))
    found = {}
    for tag in COUNTERS:
        values = [e.text.strip() for e in root.iter(tag) if e.text is not None]
        if len(values) > 1:
            raise Failed("{} appears {} times".format(tag, len(values)))
        if values:
            found[tag] = values[0]
    errors = [e.text.strip() for e in root.iter("LINEERROR") if e.text]
    if not found:
        raise Failed("{}: no import counter".format(path))
    others_zero = all(v == "0" for k, v in found.items() if k not in ("CREATED", "LASTVCHID"))
    verdict = "one created" if found.get("CREATED") == "1" and others_zero else "not one created"
    return "{} {} lineerrors {}".format(" ".join("{}={}".format(k, v) for k, v in found.items()), verdict,
                                         " | ".join(errors) or "-")


def parties():
    return dict(CLASSES + (("l7", "L7 Party"), ("income", INCOME)))


def present(path):
    try:
        root = ET.fromstring(decoded(path))
    except ET.ParseError as error:
        raise Failed("{}: does not parse ({})".format(path, error))
    if root.tag != "ENVELOPE":
        raise Failed("{}: the root is {!r}, not ENVELOPE".format(path, root.tag))
    # Rows sit only under BODY/DATA/COLLECTION; BODY/DESC/CMPINFO carries a bare LEDGER count line, which is not a row.
    names = [e.get("NAME") or (e.findtext("NAME") or "") for e in root.findall("./BODY/DATA/COLLECTION/LEDGER")]
    if not names:
        raise Failed("no LEDGER in the snapshot")
    out = []
    for tag, name in parties().items():
        word = ("exact" if name in names else
                "trimmed" if any(n.strip() == name.strip() for n in names) else "absent")
        out.append("{} {} {!r} len {}".format(tag, word, name, len(name)))
    return "\n".join(out)


def tie(tag, lo_path, snap_path):
    name = parties().get(tag)
    if name is None or tag == "income":
        raise Failed("no party {!r}".format(tag))
    script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "l2377.py")
    done = subprocess.run([sys.executable, "-I", script, "tie", lo_path, snap_path, name], capture_output=True,
                          text=True)
    if done.returncode != 0:
        raise Failed("l2377.py tie: {}".format(done.stdout.strip() or done.stderr.strip()))
    return "{} {}".format(tag, done.stdout.strip())


def admitted(name):
    return NAME.match(name) is not None and not any(w.lower() in FORMULA_WORDS for w in name.split(" "))


def top3(path, folder):
    check_template()
    try:
        root = ET.fromstring(decoded(path))
    except ET.ParseError as error:
        raise Failed("{}: does not parse ({})".format(path, error))
    names = Counter((e.text or "").strip() for e in root.iter("BILLPARTY"))
    names.pop("", None)
    if not names:
        raise Failed("no BILLPARTY in the answer")
    ranked = sorted((n for n in names if admitted(n)), key=lambda n: (-names[n], n.encode("utf-8")))[:3]
    if len(ranked) < 3:
        raise Failed("fewer than three admitted parties ({})".format(len(ranked)))
    os.makedirs(folder, exist_ok=True)
    out = ["parties {}; admitted {}".format(len(names), sum(1 for n in names if admitted(n)))]
    with open(os.path.join(folder, "top3.txt"), "x", encoding="utf-8") as handle:
        for k, name in enumerate(ranked, 1):
            handle.write("{}\t{}\n".format(name, names[name]))
            out.append("top{} bills {} {}".format(k, names[name], write(folder, "lo-top{}".format(k), lo(DENSE, name))))
    return "\n".join(out)


def main(argv):
    if len(argv) == 3 and argv[0] == "writes" and argv[1] == "--out":
        return writes(argv[2])
    if len(argv) == 2 and argv[0] == "present":
        return present(argv[1])
    if len(argv) == 4 and argv[0] == "tie":
        return tie(argv[1], argv[2], argv[3])
    if len(argv) == 2 and argv[0] == "counters":
        return counters(argv[1])
    if len(argv) == 4 and argv[0] == "top3" and argv[2] == "--out":
        return top3(argv[1], argv[3])
    return None


if __name__ == "__main__":
    try:
        result = main(sys.argv[1:])
    except (Failed, OSError, ValueError) as error:
        print("FAIL: {}".format(error))
        sys.exit(1)
    if result is None:
        print(__doc__)
        sys.exit(2)
    print(result)
    sys.exit(0)
