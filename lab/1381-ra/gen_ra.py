#!/usr/bin/env python3
"""Render row 1381-RA's five read requests from the code at 6c9e6b33, and prove the port against the code's own tests.

Row 1381-RA, in the shape of row 2162-PA's rbreq.py (lab/2162-pa at f682a6aa) and row 1381-R1's gen_r1.py
(lab/1381-r1 at c76e23d6); not product code. Python 3.10, standard library only. Each function below is a line-for-line
port of the Rust builder it names (paths are at 6c9e6b33):

  marks   render_agent_company_high_water          src-tauri/src/agent_read_profiles.rs:590-600
  count   render_company_inventory_flags_request   src-tauri/crates/bridge-tally-protocol/src/native_stock_summary.rs:79-89
  items   render_candidate_stock_items_request     src-tauri/crates/bridge-tally-protocol/src/native_masters.rs:272-281
  groups  render_native_masters_request(StockGroups) native_masters.rs:257-266, shape :116-121
  units   render_native_masters_request(Units)     native_masters.rs:257-266, shape :110-115

with the one company-variable writer (xml_read_profiles.rs:130-135, reached by :124-126 and :513-519), its escape
(xml_text.rs:41-49) and the wire encoding, a byte-order mark then UTF-16LE with nothing added (text_encoding.rs:42-49).

  gen_ra.py --selftest
      renders each builder for the code's own synthetic inputs and compares the sha256 of its UTF-8 bytes with the
      code's pinned table (request_bytes_tests.rs:114-120, :134-135, :198-199 in the protocol crate; inputs :94, :453;
      src-tauri/src/agent_import_request_bytes_tests.rs:170-171, input :74). Needs no file.
  gen_ra.py fixtures DIR
      DIR is src-tauri/crates/bridge-tally-protocol/tests/fixtures at 6c9e6b33. Renders the five committed request
      captures those tests compare with, and compares byte for byte.
  gen_ra.py write OUTDIR
      writes the five requests for the book into OUTDIR/requests/, never over a file that exists, and prints each
      file's sha256, bytes and path in SHA256.txt's three-column form.
  gen_ra.py check KITDIR
      renders the five again and compares them byte for byte with KITDIR/requests/; then checks the structure of each
      file there: one Export of TYPE Collection, no SVFROMDATE or SVTODATE, no import. Prints the same lines as write.

Exit 0: every comparison equal. Exit 1: a finding (a hash, a fixture or a file differs, or a request's structure is
not a read without dates). Exit 2: a failed read (a file missing or unreadable, an argument of the wrong shape).
Prints no GUID.
"""

import hashlib
import pathlib
import re
import sys

BOOK = "BRIDGE STOCK CREATE LAB"
# The book's company identifier, listed as synthetic in scripts/merge-gate-synthetic-uuids.json on master.
BOOK_GUID = "bac2b5e9-ada9-43c0-b8f8-a2812a813fa1"
GUID_SHAPE = re.compile(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")

FORMAT = "<SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>"  # native_masters.rs:240; the same text at :79-89 and :590-600
SHAPES = {  # native_masters.rs:110-127: collection, ISMODIFY, TYPE, FETCH
    "Units": ("Bridge Master Units", "No", "Unit",
              "NAME, GUID, MASTERID, ALTERID, ORIGINALNAME, DECIMALPLACES, ISSIMPLEUNIT"),
    "StockGroups": ("Bridge Master Stock Groups", "No", "StockGroup", "NAME, PARENT, GUID, MASTERID, ALTERID"),
    "StockItems": ("Bridge Master Stock Items", "No", "StockItem",
                   "NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST"),
}
CANDIDATE_FETCH = ("NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, HSNDETAILS.LIST, "
                   "GSTDETAILS.LIST, LANGUAGENAME.LIST")  # native_masters.rs:241
EXTENT_FETCH = "Name, GUID, CompanyNumber, BooksFrom, LastVoucherDate, ALTVCHID, ALTMSTID"  # outstandings_shared.rs:750-751

# The code's pinned sha256 of each builder's UTF-8 output for its synthetic inputs.
SYNTHETIC = "Synthetic & Co <Lab> \"Q\" Ünïcode"  # protocol request_bytes_tests.rs:94; app tests :74
SYNTHETIC_GUID = "3a6bd6e1-b835-4bff-89dd-8a6af138c346"  # protocol request_bytes_tests.rs:453
PINS = {
    "render_native_masters_request/Units": "f770f30291db013086e095699575da543aaa67ad11430cf8342eb92a0deedea0",
    "render_native_masters_request/StockGroups": "28870e661f3033d826cba42bb0ec5913a577ff414cf0b236f5214571150b3c32",
    "render_candidate_stock_items_request": "fcdd6a527ba17f195f52059534e80ab2912b22daf2553c655cbc35406c88883c",
    "render_company_inventory_flags_request": "17117b83acbbb905dd3436fd891f23aa2e14b168415d8e581059f6dc4b8ee924",
    "render_agent_company_high_water": "3ebd4c55a69823a2980947db575bdb50dbbfb542b4ddc7507f4ecf2984be8264",
}
SHAPE_LAB = "BRIDGE SHAPE LAB"  # native_masters_tests.rs:11, native_stock_summary_tests.rs:13
# The committed request captures, relative to the fixtures folder, and what renders each.
FIXTURES = (
    ("masters_stock_groups_request.utf16le.xml", "groups for BRIDGE SHAPE LAB (native_masters_tests.rs:152-160)"),
    ("masters_units_request.utf16le.xml", "units for BRIDGE SHAPE LAB (native_masters_tests.rs:152-160)"),
    ("company_inventory_flags_request.utf16le.xml",
     "count for BRIDGE SHAPE LAB and its GUID (native_stock_summary_tests.rs:278-283)"),
    ("agent/stock-create-lab/stock-create-lab-items.request.utf16le.xml",
     "items for this book (tests/stock_item_candidate_request.rs:17-28)"),
    ("note-days/shape_lab_company_marks_request.utf16le.xml",
     "marks for BRIDGE SHAPE LAB (a committed request; no Rust test reads it, the pin above is the code's check)"),
)
class FailedRead(Exception):
    pass


def escape_text(value):
    # xml_text.rs:41-49, in its order.
    return (value.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")
            .replace("'", "&apos;").replace("\r", "&#13;").replace("\n", "&#10;"))


def static_variables(company, before, after=""):
    # xml_read_profiles.rs:130-135, reached by xml_static_variables (:124-126) and
    # static_variables_for_unvalidated_company (:513-519) alike.
    return f"<STATICVARIABLES>{before}<SVCURRENTCOMPANY>{escape_text(company)}</SVCURRENTCOMPANY>{after}</STATICVARIABLES>"


def masters_collection(collection, is_modify, master_type, fetch, variables):
    # native_masters.rs:243-253.
    return (f'<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            f'<ID>{collection}</ID></HEADER><BODY><DESC>{variables}<TDL><TDLMESSAGE><COLLECTION NAME="{collection}" '
            f'ISMODIFY="{is_modify}"><TYPE>{master_type}</TYPE><FETCH>{fetch}</FETCH>'
            f'<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION></TDLMESSAGE></TDL>'
            f'</DESC></BODY></ENVELOPE>')


def native_masters_request(kind, company):
    # native_masters.rs:257-266.
    collection, is_modify, master_type, fetch = SHAPES[kind]
    return masters_collection(collection, is_modify, master_type, fetch, static_variables(company, FORMAT))


def candidate_stock_items_request(company):
    # native_masters.rs:272-281: the stock-items shape with only the fetch list changed. The Rust takes a
    # ValidatedCompanyName (xml_read_profiles.rs:90-112); the book's name is one it admits.
    collection, is_modify, master_type, _ = SHAPES["StockItems"]
    return masters_collection(collection, is_modify, master_type, CANDIDATE_FETCH, static_variables(company, FORMAT))


def company_inventory_flags_request(company, guid):
    # native_stock_summary.rs:79-89. The Rust takes a CompanyGuid, stored lower case in the 8-4-4-4-12 spelling
    # (src-tauri/crates/bridge-tally-primitives/src/company_guid.rs:34-47).
    if not GUID_SHAPE.fullmatch(guid):
        raise FailedRead("the GUID is not lower-case hex and hyphens")
    return (f'<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            f'<ID>BridgeCompanyBookExtentV2</ID></HEADER><BODY><DESC>{static_variables(company, FORMAT)}<TDL><TDLMESSAGE>'
            f'<SYSTEM TYPE="Formulae" NAME="BridgeCompanyGuidFilter">$GUID = "{guid}"</SYSTEM>'
            f'<COLLECTION NAME="BridgeCompanyBookExtentV2" ISMODIFY="No"><TYPE>Company</TYPE><FETCH>{EXTENT_FETCH}, '
            f'ISINTEGRATED, ISINVENTORYON, ISBATCHWISEON, NUMLEDGERS, NUMGROUPS, NUMSTOCKITEMS, NUMSTOCKCATEGORIES, '
            f'NUMGODOWNS, NUMUNITS, NUMVOUCHERTYPES</FETCH><FILTERS>BridgeCompanyGuidFilter</FILTERS></COLLECTION>'
            f'</TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')


def company_high_water(company):
    # agent_read_profiles.rs:590-600: every loaded company's GUID and two marks (no filter).
    return (f'<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            f'<ID>Bridge Agent Company High Water</ID></HEADER><BODY><DESC>{static_variables(company, FORMAT)}<TDL>'
            f'<TDLMESSAGE><COLLECTION NAME="Bridge Agent Company High Water" ISMODIFY="No"><TYPE>Company</TYPE>'
            f'<FETCH>GUID,ALTVCHID,ALTMSTID</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')


def wire(text):
    # text_encoding.rs:42-49: the byte-order mark, then UTF-16LE, nothing after.
    return b"\xff\xfe" + text.encode("utf-16-le")


def book_requests():
    return {
        "requests/marks.utf16le.xml": wire(company_high_water(BOOK)),
        "requests/count.utf16le.xml": wire(company_inventory_flags_request(BOOK, BOOK_GUID)),
        "requests/items.utf16le.xml": wire(candidate_stock_items_request(BOOK)),
        "requests/groups.utf16le.xml": wire(native_masters_request("StockGroups", BOOK)),
        "requests/units.utf16le.xml": wire(native_masters_request("Units", BOOK)),
    }


def fixture_renders():
    return {
        FIXTURES[0][0]: wire(native_masters_request("StockGroups", SHAPE_LAB)),
        FIXTURES[1][0]: wire(native_masters_request("Units", SHAPE_LAB)),
        FIXTURES[2][0]: wire(company_inventory_flags_request(SHAPE_LAB, SYNTHETIC_GUID)),
        FIXTURES[3][0]: wire(candidate_stock_items_request(BOOK)),
        FIXTURES[4][0]: wire(company_high_water(SHAPE_LAB)),
    }


def structure(body):
    """What is wrong with one request file as a dated or writing request; [] when it is a plain read with no date."""
    if body[:2] != b"\xff\xfe":
        return ["no byte-order mark"]
    try:
        text = body[2:].decode("utf-16-le")
    except UnicodeDecodeError:
        return ["not UTF-16LE"]
    problems = []
    header = re.fullmatch(r"<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>([A-Za-z]+)</TALLYREQUEST>"
                          r"<TYPE>([A-Za-z]+)</TYPE><ID>[^<]+</ID></HEADER><BODY><DESC>.*</DESC></BODY></ENVELOPE>",
                          text, re.S)
    if not header:
        problems.append("not one ENVELOPE with a HEADER and a BODY/DESC")
    elif header.groups() != ("Export", "Collection"):
        problems.append(f"TALLYREQUEST {header.group(1)!r}, TYPE {header.group(2)!r}: not an Export of a Collection")
    for word in ("SVFROMDATE", "SVTODATE"):
        if word in text.upper():
            problems.append(f"carries {word}: a date window")
    for word in ("IMPORT", "TALLYMESSAGE", "ACTION="):
        if word in text.upper():
            problems.append(f"carries {word}: not a read")
    return problems


def line(name, body):
    return f"{hashlib.sha256(body).hexdigest()}  {len(body)}  {name}"


def selftest():
    rendered = {
        "render_native_masters_request/Units": native_masters_request("Units", SYNTHETIC),
        "render_native_masters_request/StockGroups": native_masters_request("StockGroups", SYNTHETIC),
        "render_candidate_stock_items_request": candidate_stock_items_request(SYNTHETIC),
        "render_company_inventory_flags_request": company_inventory_flags_request(SYNTHETIC, SYNTHETIC_GUID),
        "render_agent_company_high_water": company_high_water(SYNTHETIC),
    }
    wrong = 0
    for label, text in rendered.items():
        digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
        same = digest == PINS[label]
        wrong += not same
        print(f"{'PIN EQUAL' if same else 'PIN DIFFERS'} {label} {digest}")
    for name, body in book_requests().items():
        problems = structure(body)
        wrong += bool(problems)
        print(f"{'READ, NO DATES' if not problems else 'NOT A PLAIN READ: ' + '; '.join(problems)} {name}")
    # Controls: each check must fire on a case it exists to catch, or it proves nothing.
    marks = company_high_water(BOOK)
    controls = (
        ("a pin, for a company one character different",
         hashlib.sha256(company_high_water(BOOK + " ").encode("utf-8")).hexdigest() != PINS["render_agent_company_high_water"]),
        ("the structure check, on the marks read with a date window added",
         bool(structure(wire(marks.replace("</STATICVARIABLES>",
                                           '<SVFROMDATE TYPE="Date">20250401</SVFROMDATE></STATICVARIABLES>'))))),
        ("the structure check, on the marks read made an Import",
         bool(structure(wire(marks.replace("<TALLYREQUEST>Export<", "<TALLYREQUEST>Import<"))))),
        ("the structure check, on the marks read with no byte-order mark",
         bool(structure(marks.encode("utf-16-le")))),
    )
    for what, fired in controls:
        wrong += not fired
        print(f"{'CONTROL FIRED' if fired else 'CONTROL DID NOT FIRE'}: {what}")
    print(f"{len(rendered)} pins equal; {len(book_requests())} book requests each a read with no date window; "
          f"{len(controls)} controls fired" if not wrong else f"FINDING: {wrong} difference(s)")
    return 1 if wrong else 0


def fixtures(root):
    root = pathlib.Path(root)
    wrong = 0
    for name, body in fixture_renders().items():
        what = dict(FIXTURES)[name]
        try:
            kept = (root / name).read_bytes()
        except OSError as error:
            raise FailedRead(f"{name}: unreadable ({type(error).__name__})")
        same = kept == body
        wrong += not same
        print(f"{'FIXTURE EQUAL' if same else 'FIXTURE DIFFERS'} {name} {len(kept)} B: {what}")
    print(f"{len(FIXTURES)} fixtures, all byte-equal" if not wrong else f"FINDING: {wrong} fixture(s) differ")
    return 1 if wrong else 0


def write(outdir):
    out = pathlib.Path(outdir)
    bodies = book_requests()
    for name in bodies:
        if (out / name).exists():
            raise FailedRead(f"{name} exists: never overwritten")
    for name, body in bodies.items():
        (out / name).parent.mkdir(parents=True, exist_ok=True)
        (out / name).write_bytes(body)
        print(line(name, body))
    return 0


def check(kitdir):
    kit = pathlib.Path(kitdir)
    wrong = 0
    for name, body in book_requests().items():
        try:
            kept = (kit / name).read_bytes()
        except OSError as error:
            raise FailedRead(f"{name}: unreadable ({type(error).__name__})")
        problems = structure(kept)
        if kept != body:
            problems.insert(0, "differs from the render")
        wrong += bool(problems)
        print(line(name, kept) + ("" if not problems else "  FINDING: " + "; ".join(problems)))
    print("5 requests equal to the render; each one Export of a Collection, no date window, no import"
          if not wrong else f"FINDING: {wrong} request(s)")
    return 1 if wrong else 0


def main(argv):
    try:
        if argv == ["--selftest"]:
            return selftest()
        if len(argv) == 2 and argv[0] in ("fixtures", "write", "check"):
            return {"fixtures": fixtures, "write": write, "check": check}[argv[0]](argv[1])
        raise FailedRead("usage: gen_ra.py --selftest | fixtures DIR | write OUTDIR | check KITDIR")
    except FailedRead as error:
        print(f"FAILED READ: {error}")
        return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
