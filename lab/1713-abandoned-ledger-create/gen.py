"""Writes this sheet's requests, as UTF-16LE with a byte-order mark and no final line break.

`python3 -I gen.py` writes the small requests into requests/ and SHA256.txt. `python3 -I gen.py bulk` writes the 192
bulk Journal imports into bulk/ (not committed: about 56 MB) and checks every one against BULK-SHA256.txt, printing
`bulk ok` only when all 192 match. Every body is copied from the builder in ComplyEaze Bridge's source that sends it,
cited beside it; only the company, the names, the dates, the amounts and the REMOTEIDs are this sheet's own. A builder
cited "on master" is on master at 3d3dfdf2; one cited "at a71f416c" exists only on the head of the open pull request
#1650 (branch masters-ledger-create-series, commit a71f416c9f94254e9786818965c93f37c5fb7863), not on master.
"""
import hashlib
import pathlib
import sys

COMPANY = "BRIDGE ABANDON LAB"
HERE = pathlib.Path(__file__).resolve().parent
BULK_FILES = 192
PER_FILE = 250


def esc(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def utf16(body):
    return b"\xff\xfe" + body.encode("utf-16-le")


# src-tauri/src/agent_read_profiles.rs, render_agent_company_high_water (on master).
def marks():
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES>"
        "<TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Agent Company High Water\" ISMODIFY=\"No\"><TYPE>Company</TYPE>"
        "<FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
    ).format(esc(COMPANY))


# The ledger index read Bridge's own post and reconcile send (`ledger_name_index_read` in
# src-tauri/src/agent_read_profiles.rs, rendering the `StandardLedgerCatalogV3` profile; both at a71f416c), byte for
# byte the file of lab/ml1-live-ledger-create-run-sheet requests/ML1/ledgers.utf16le.xml with only the company changed;
# its line breaks and indentation are kept.
LEDGERS = """<ENVELOPE>
    <HEADER>
        <VERSION>1</VERSION>
        <TALLYREQUEST>EXPORT</TALLYREQUEST>
        <TYPE>COLLECTION</TYPE>
        <ID>List of Ledgers</ID>
    </HEADER>
    <BODY>
        <DESC>
            <STATICVARIABLES>
                <SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
                <SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY>
            </STATICVARIABLES>
            <TDL>
                <TDLMESSAGE>
                    <COLLECTION NAME="List of Ledgers" ISMODIFY="Yes">
                        <NATIVEMETHOD>Name</NATIVEMETHOD>
                        <NATIVEMETHOD>GUID</NATIVEMETHOD>
                        <NATIVEMETHOD>Parent</NATIVEMETHOD>
                        <NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>
                        <NATIVEMETHOD>AlterID</NATIVEMETHOD>
                        <COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>
                        <COMPUTE>BRIDGECOMPANYNAME:##SVCurrentCompany</COMPUTE>
                    </COLLECTION>
                </TDLMESSAGE>
            </TDL>
        </DESC>
    </BODY>
</ENVELOPE>"""


def ledgers():
    return LEDGERS.format(esc(COMPANY))


# src-tauri/src/agent_import.rs, the verification window read (no AlterID span filter; on master), over the whole
# year: the BUSY read. On about 24,000 vouchers it is expected to take tens of seconds (reference: ~710 rows a second).
def busy():
    d1, d2 = "20250401", "20260331"
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>Bridge Agent Import Verification</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{c}</SVCURRENTCOMPANY>"
        "<SVFROMDATE TYPE=\"Date\">{a}</SVFROMDATE><SVTODATE TYPE=\"Date\">{b}</SVTODATE></STATICVARIABLES>"
        "<TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeImportWindow\">$Date &gt;= $$Date:\"{a}\" AND "
        "$Date &lt;= $$Date:\"{b}\"</SYSTEM><COLLECTION NAME=\"Bridge Agent Import Verification\" ISMODIFY=\"No\">"
        "<TYPE>Voucher</TYPE><FETCH>DATE,VOUCHERNUMBER,VOUCHERTYPENAME,REMOTEID,GUID,MASTERID,ALTERID,NARRATION,"
        "ISCANCELLED,ISOPTIONAL,ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,ALLLEDGERENTRIES.ISDEEMEDPOSITIVE,"
        "EFFECTIVEDATE</FETCH><FILTERS>BridgeImportWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY>"
        "</ENVELOPE>"
    ).format(c=esc(COMPANY), a=d1, b=d2)


# src-tauri/crates/bridge-tally-protocol/src/import_envelope.rs, render_import_envelope (at a71f416c), for both
# reports. On master the same bytes for "Vouchers" come from render_import_envelope in src-tauri/src/agent_import.rs.
def envelope(report, messages):
    return (
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>"
        "</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>{r}</REPORTNAME><STATICVARIABLES>"
        "<SVCURRENTCOMPANY>{c}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{m}</REQUESTDATA>"
        "</IMPORTDATA></BODY></ENVELOPE>"
    ).format(r=report, c=esc(COMPANY), m=messages)


# src-tauri/crates/bridge-tally-protocol/src/ledger_create.rs, LedgerCreate::render_message (on master), as
# LedgerCreateItem::render_import (src-tauri/src/tally/ledger_create_item.rs, at a71f416c) calls it for a Suspense
# ledger (parent Suspense A/c, bill-wise No, nothing else).
def ledger_create(name, parent="Suspense A/c"):
    return (
        "<TALLYMESSAGE><LEDGER NAME=\"{n}\" ACTION=\"Create\"><NAME>{n}</NAME><PARENT>{p}</PARENT>"
        "<ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE>"
    ).format(n=esc(name), p=esc(parent))


# src-tauri/src/agent_import.rs (on master), render_voucher_xml for a Journal (no EFFECTIVEDATE, the caller's order, here the
# debit first), as render_native_vouchers_xml sends it (narration untagged).
def journal(index):
    day = index % 365
    # 1 April 2025 plus `day` days, computed without a date library.
    months = [("2025", "04", 30), ("2025", "05", 31), ("2025", "06", 30), ("2025", "07", 31), ("2025", "08", 31),
              ("2025", "09", 30), ("2025", "10", 31), ("2025", "11", 30), ("2025", "12", 31), ("2026", "01", 31),
              ("2026", "02", 28), ("2026", "03", 31)]
    for year, month, length in months:
        if day < length:
            date = "{}{}{:02d}".format(year, month, day + 1)
            break
        day -= length
    amount = "{}.00".format(1 + index % 100)
    remote_id = "1713b000-0000-4000-8000-{:012x}".format(index)
    entries = (
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bulk Expense</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>"
        "<AMOUNT>-{a}</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bulk Income</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{a}</AMOUNT></ALLLEDGERENTRIES.LIST>"
    ).format(a=amount)
    return (
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{r}\" VCHTYPE=\"Journal\" ACTION=\"Create\" "
        "OBJVIEW=\"Accounting Voucher View\"><DATE>{d}</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>"
        "<NARRATION>Lab bulk {i}</NARRATION>{e}</VOUCHER></TALLYMESSAGE>"
    ).format(r=remote_id, d=date, i=index, e=entries)


def bulk_file(number):
    start = number * PER_FILE
    return envelope("Vouchers", "".join(journal(index) for index in range(start, start + PER_FILE)))


def bulk_name(number):
    return "bulk-{:03d}.xml".format(number + 1)


SMALL = {
    "marks.xml": marks(),
    "ledgers.xml": ledgers(),
    "busy-year.xml": busy(),
    "s3-bulk-ledgers.xml": envelope(
        "All Masters",
        ledger_create("Lab Bulk Expense", "Indirect Expenses") + ledger_create("Lab Bulk Income", "Indirect Incomes"),
    ),
    "l0-create.xml": envelope("All Masters", ledger_create("Lab Probe L0")),
    "l1-create.xml": envelope("All Masters", ledger_create("Lab Probe L1")),
    "l1b-create.xml": envelope("All Masters", ledger_create("Lab Probe L1B")),
    "l2-create.xml": envelope("All Masters", ledger_create("Lab Probe L2")),
    "l3-create.xml": envelope("All Masters", ledger_create("Lab Probe L3")),
    "l4-create.xml": envelope("All Masters", ledger_create("Lab Probe L4")),
    "l5-create.xml": envelope("All Masters", ledger_create("Lab Probe L5")),
}


def write_small():
    out = HERE / "requests"
    out.mkdir(exist_ok=True)
    for name, body in SMALL.items():
        (out / name).write_bytes(utf16(body))
    manifest = "\n".join(
        "{}  {}".format(hashlib.sha256(utf16(bulk_file(n))).hexdigest(), "bulk/" + bulk_name(n))
        for n in range(BULK_FILES)
    )
    (HERE / "BULK-SHA256.txt").write_text(manifest + "\n")
    lines = []
    for path in sorted(HERE.rglob("*")):
        if path.is_file() and path.name != "SHA256.txt" and not {"__pycache__", "bulk"} & set(path.parts):
            lines.append("{}  {}".format(hashlib.sha256(path.read_bytes()).hexdigest(), path.relative_to(HERE).as_posix()))
    (HERE / "SHA256.txt").write_text("\n".join(lines) + "\n")
    print("{} requests, {} files hashed, {} bulk hashes".format(len(SMALL), len(lines), BULK_FILES))


def write_bulk():
    out = HERE / "bulk"
    out.mkdir(exist_ok=True)
    expected = dict(
        (line.split("  ")[1], line.split("  ")[0]) for line in (HERE / "BULK-SHA256.txt").read_text().splitlines()
    )
    bad = 0
    for n in range(BULK_FILES):
        data = utf16(bulk_file(n))
        (out / bulk_name(n)).write_bytes(data)
        if hashlib.sha256(data).hexdigest() != expected.get("bulk/" + bulk_name(n)):
            bad += 1
    print("bulk ok" if bad == 0 else "bulk MISMATCH {}".format(bad))
    return bad


if __name__ == "__main__":
    if sys.argv[1:] == ["bulk"]:
        sys.exit(1 if write_bulk() else 0)
    write_small()
