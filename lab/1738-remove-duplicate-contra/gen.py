"""Writes this sheet's requests into requests/, as UTF-16LE with a byte-order mark and no final line break.

Run from this folder with `python3 -I gen.py`; it rewrites every request and SHA256.txt, and the result must equal the
committed files byte for byte. Each body is copied from the builder in ComplyEaze Bridge's source that sends it today,
cited beside it; only the company, the dates, the amounts and the REMOTEIDs are this sheet's own. The cancel and the
delete have no builder yet: each is the candidate shape the builder named beside it is meant to send.
"""
import hashlib
import pathlib

COMPANY = "BRIDGE REMOVE LAB"
HERE = pathlib.Path(__file__).resolve().parent
OUT = HERE / "requests"


def xml_escape(text):
    return (
        text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace('"', "&quot;")
        .replace("'", "&apos;")
    )


# src-tauri/src/agent_read_profiles.rs, render_agent_company_high_water.
def marks():
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES>"
        "<TDL><TDLMESSAGE><COLLECTION NAME=\"Bridge Agent Company High Water\" ISMODIFY=\"No\"><TYPE>Company</TYPE>"
        "<FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
    ).format(xml_escape(COMPANY))


# src-tauri/src/agent_import.rs, the verification window read (no AlterID span filter).
def vouchers(day):
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>Bridge Agent Import Verification</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY>"
        "<SVFROMDATE TYPE=\"Date\">{d}</SVFROMDATE><SVTODATE TYPE=\"Date\">{d}</SVTODATE></STATICVARIABLES>"
        "<TDL><TDLMESSAGE><SYSTEM TYPE=\"Formulae\" NAME=\"BridgeImportWindow\">$Date &gt;= $$Date:\"{d}\" AND "
        "$Date &lt;= $$Date:\"{d}\"</SYSTEM><COLLECTION NAME=\"Bridge Agent Import Verification\" ISMODIFY=\"No\">"
        "<TYPE>Voucher</TYPE><FETCH>DATE,VOUCHERNUMBER,VOUCHERTYPENAME,REMOTEID,GUID,MASTERID,ALTERID,NARRATION,"
        "ISCANCELLED,ISOPTIONAL,ALLLEDGERENTRIES.LEDGERNAME,ALLLEDGERENTRIES.AMOUNT,ALLLEDGERENTRIES.ISDEEMEDPOSITIVE,"
        "EFFECTIVEDATE</FETCH><FILTERS>BridgeImportWindow</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY>"
        "</ENVELOPE>"
    ).format(company=xml_escape(COMPANY), d=day)


# src-tauri/crates/bridge-tally-protocol/src/native_voucher_type_numbering.rs, render_voucher_type_numbering_request
# (render_native_voucher_type_export_request with its FETCH replaced).
def voucher_types():
    return (
        "<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>"
        "<ID>List of VoucherTypes</ID></HEADER><BODY><DESC><STATICVARIABLES>"
        "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES>"
        "<TDL><TDLMESSAGE><COLLECTION NAME=\"List of VoucherTypes\" ISMODIFY=\"Yes\"><TYPE>VoucherType</TYPE>"
        "<FETCH>NAME, PARENT, GUID, MASTERID, ALTERID, NUMBERINGMETHOD, PREVENTDUPLICATES, VOUCHERNUMBERSERIES.*"
        "</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>"
    ).format(xml_escape(COMPANY))


# The ledger create shape of lab/0.7.0-tonight-answers b3/requests/i3-ledgers.xml (CREATED 2 on 25 Oct 2026).
def ledgers():
    rows = "".join(
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><LEDGER NAME=\"{n}\" ACTION=\"Create\"><NAME.LIST><NAME>{n}</NAME>"
        "</NAME.LIST><PARENT>Bank Accounts</PARENT></LEDGER></TALLYMESSAGE>".format(n=name)
        for name in ("Lab Bank A", "Lab Bank B")
    )
    return (
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>"
        "</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES>"
        "<SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{}</REQUESTDATA>"
        "</IMPORTDATA></BODY></ENVELOPE>"
    ).format(xml_escape(COMPANY), rows)


# src-tauri/src/agent_import.rs, render_import_envelope.
def envelope(messages):
    return (
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>"
        "</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES>"
        "<SVCURRENTCOMPANY>{}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{}</REQUESTDATA>"
        "</IMPORTDATA></BODY></ENVELOPE>"
    ).format(xml_escape(COMPANY), messages)


# Each voucher: (key, date, amount, REMOTEID, origin). "native" is the body render_native_vouchers_xml sends
# (narration untagged); "file" is the body of the hand-import file render_import_xml writes, whose one import
# identity is both the REMOTEID and the [BRIDGE:...] tag after the narration. Both are sent by curl here.
VOUCHERS = {
    "V1": ("20250505", "101.00", "1738a001-0000-4000-8000-000000000001", "native"),
    "V2": ("20250505", "102.00", "1738a001-0000-4000-8000-000000000002", "native"),
    "V3": ("20250505", "103.00", "1738a001-0000-4000-8000-000000000003", "file"),
    "V4": ("20250505", "104.00", "1738a001-0000-4000-8000-000000000004", "file"),
    "V5": ("20250505", "105.00", "1738a001-0000-4000-8000-000000000005", "native"),
    "V6": ("20250505", "106.00", "1738a001-0000-4000-8000-000000000006", "native"),
    "W1": ("20250506", "201.00", "1738a001-0000-4000-8000-000000000011", "native"),
    "W2": ("20250506", "202.00", "1738a001-0000-4000-8000-000000000012", "native"),
    "W3": ("20250506", "203.00", "1738a001-0000-4000-8000-000000000013", "native"),
    "W4": ("20250506", "204.00", "1738a001-0000-4000-8000-000000000014", "native"),
    # Never created: the targets of the absent-ID rows, one each, so neither row can meet the other's write.
    "X1": ("20250505", "107.00", "1738a001-0000-4000-8000-000000000021", "native"),
    "X2": ("20250505", "108.00", "1738a001-0000-4000-8000-000000000022", "native"),
}


# src-tauri/src/agent_import.rs, render_voucher_xml for a Contra (bank shape: debit first, EFFECTIVEDATE beside
# DATE, no PARTYLEDGERNAME for a Contra, no VOUCHERNUMBER, no REFERENCE). `action` is "Create" as Bridge sends it,
# or "Cancel" for the candidate cancel.
def voucher(key, action="Create"):
    date, amount, remote_id, origin = VOUCHERS[key]
    text = "Lab transfer {}".format(key)
    if origin == "file":
        text = "{} [BRIDGE:{}]".format(text, remote_id)
    entries = (
        "<ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bank B</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>"
        "<AMOUNT>-{a}</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Lab Bank A</LEDGERNAME>"
        "<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{a}</AMOUNT></ALLLEDGERENTRIES.LIST>"
    ).format(a=amount)
    return (
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{r}\" VCHTYPE=\"Contra\" ACTION=\"{action}\" "
        "OBJVIEW=\"Accounting Voucher View\"><DATE>{d}</DATE><EFFECTIVEDATE>{d}</EFFECTIVEDATE>"
        "<VOUCHERTYPENAME>Contra</VOUCHERTYPENAME><NARRATION>{n}</NARRATION>{e}</VOUCHER></TALLYMESSAGE>"
    ).format(r=remote_id, action=action, d=date, n=xml_escape(text), e=entries)


# Candidate shape of the delete builder: the REMOTEID the voucher was created with, its type and date, nothing else.
def delete(key):
    date, _, remote_id, _ = VOUCHERS[key]
    return (
        "<TALLYMESSAGE xmlns:UDF=\"TallyUDF\"><VOUCHER REMOTEID=\"{r}\" VCHTYPE=\"Contra\" ACTION=\"Delete\">"
        "<DATE>{d}</DATE><VOUCHERTYPENAME>Contra</VOUCHERTYPENAME></VOUCHER></TALLYMESSAGE>"
    ).format(r=remote_id, d=date)


REQUESTS = {
    "marks.xml": marks(),
    "vch-20250505.xml": vouchers("20250505"),
    "vch-20250506.xml": vouchers("20250506"),
    "vtypes.xml": voucher_types(),
    "s3-ledgers.xml": ledgers(),
    "s4a-create-v1-v2.xml": envelope(voucher("V1") + voucher("V2")),
    "s4b-create-v3-v4.xml": envelope(voucher("V3") + voucher("V4")),
    "s4c-create-v5-v6.xml": envelope(voucher("V5") + voucher("V6")),
    "l1-delete-v6.xml": envelope(delete("V6")),
    "l2-delete-v2.xml": envelope(delete("V2")),
    "l3-delete-v4.xml": envelope(delete("V4")),
    "l4-cancel-v1.xml": envelope(voucher("V1", "Cancel")),
    "l5-cancel-v5.xml": envelope(voucher("V5", "Cancel")),
    "l6-cancel-v3.xml": envelope(voucher("V3", "Cancel")),
    "l7-cancel-absent-x1.xml": envelope(voucher("X1", "Cancel")),
    "l8-delete-absent-x2.xml": envelope(delete("X2")),
    "b1-create-w1-w4.xml": envelope("".join(voucher(key) for key in ("W1", "W2", "W3", "W4"))),
    "l9-delete-w1.xml": envelope(delete("W1")),
    "l10-cancel-w3.xml": envelope(voucher("W3", "Cancel")),
}


def main():
    OUT.mkdir(exist_ok=True)
    for name, body in REQUESTS.items():
        (OUT / name).write_bytes(b"\xff\xfe" + body.encode("utf-16-le"))
    lines = []
    for path in sorted(HERE.rglob("*")):
        if path.is_file() and path.name != "SHA256.txt" and "__pycache__" not in path.parts:
            rel = path.relative_to(HERE).as_posix()
            lines.append("{}  {}".format(hashlib.sha256(path.read_bytes()).hexdigest(), rel))
    (HERE / "SHA256.txt").write_text("\n".join(lines) + "\n")
    print("{} requests, {} files hashed".format(len(REQUESTS), len(lines)))


if __name__ == "__main__":
    main()
