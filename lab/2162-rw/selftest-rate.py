#!/usr/bin/env python3
"""Exercise ratecmp.py with mutations of a captured rate answer.

The base is a byte-equal copy, in this folder, of the repository capture
src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/pilot-lab/pilot-lab-ledger-rates.utf16le.xml
(its SHA-256 is pinned below). Every other case is a mutation made at run time
in a temporary folder, which is removed afterwards.
"""

import contextlib
import hashlib
import importlib.util
import io
import pathlib
import sys
import tempfile

sys.dont_write_bytecode = True
HERE = pathlib.Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("ratecmp", HERE / "ratecmp.py")
ratecmp = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ratecmp)
FIXTURE = HERE / "pilot-lab-ledger-rates.utf16le.xml"
FIXTURE_SHA256 = "597f2b11728c39d6643ced43682b89143fce29481a803ad5f23433ee896c9847"
TARGET = "BRIDGE Svc 998313 5%"
OTHER = "BRIDGE Svc 998314 5%"
COMPANY_FIELD = '<BRIDGECOMPANYGUID TYPE="String">6b43e498-430c-4d5c-bfef-d32e2ab93c85</BRIDGECOMPANYGUID>'


def ledger_span(text, name):
    start = text.index(f'<LEDGER NAME="{name}"')
    end = text.index("</LEDGER>", start) + len("</LEDGER>")
    return start, end


def gst_span(text, name=TARGET):
    ledger_start, ledger_end = ledger_span(text, name)
    start = text.index("<GSTDETAILS.LIST>", ledger_start)
    end = text.index("</GSTDETAILS.LIST>", start) + len("</GSTDETAILS.LIST>")
    assert end < ledger_end
    return start, end


def change_gst(text, replacement):
    start, end = gst_span(text)
    return text[:start] + replacement(text[start:end]) + text[end:]


def add_gst_child(text):
    return change_gst(text, lambda row: row.replace(
        "<APPLICABLEFROM", '<EXTRA_GST_ROW TYPE="String">x</EXTRA_GST_ROW><APPLICABLEFROM', 1
    ))


def add_state_child(text):
    return change_gst(text, lambda row: row.replace(
        "<STATENAME", '<EXTRA_STATE_BLOCK TYPE="String">x</EXTRA_STATE_BLOCK><STATENAME', 1
    ))


def fill_slab(text):
    return change_gst(text, lambda row: row.replace(
        "<GSTSLABRATES.LIST>       </GSTSLABRATES.LIST>",
        "<GSTSLABRATES.LIST><FROMVALUE>1</FROMVALUE></GSTSLABRATES.LIST>", 1
    ))


def add_rate_child(text):
    return change_gst(text, lambda row: row.replace(
        "<GSTRATEDUTYHEAD", '<EXTRA_RATE_DETAIL TYPE="String">x</EXTRA_RATE_DETAIL><GSTRATEDUTYHEAD', 1
    ))


def second_gst_row(text):
    return change_gst(text, lambda row: row + row)


def remove_other_ledger(text):
    start, end = ledger_span(text, OTHER)
    return text[:start] + text[end:]


def change_rate_text(text):
    return change_gst(text, lambda row: row.replace(
        '<GSTRATE TYPE="Number"> 2.50</GSTRATE>',
        '<GSTRATE TYPE="Number"> 9.00</GSTRATE>', 1
    ))


def no_collection(text):
    return text.replace('<COLLECTION ISMSTDEPTYPE="Yes" MSTDEPTYPE="8">', "<OTHER>").replace(
        "</COLLECTION>", "</OTHER>"
    )


def failed_status(text):
    return text.replace("<STATUS>1</STATUS>", "<STATUS>0</STATUS>", 1)


def extra_control_reference(text):
    return change_gst(text, lambda row: row.replace(
        "<STATENAME TYPE=\"String\">&#4; Any",
        "<STATENAME TYPE=\"String\">&#4;&#4; Any", 1
    ))


def drop_company_guid(text):
    start, end = ledger_span(text, TARGET)
    row = text[start:end]
    assert row.count(COMPANY_FIELD) == 1
    return text[:start] + row.replace(COMPANY_FIELD, "") + text[end:]


def other_company_guid(text):
    start, end = ledger_span(text, TARGET)
    return text[:start] + text[start:end].replace(
        COMPANY_FIELD, COMPANY_FIELD.replace("6b43e498", "00000000")
    ) + text[end:]


def repeat_taxability(text):
    field = '<TAXABILITY TYPE="String">Taxable</TAXABILITY>'
    return change_gst(text, lambda row: row.replace(field, field + field, 1))


def repeat_ledger(text):
    start, end = ledger_span(text, TARGET)
    return text[:end] + text[start:end] + text[end:]


def repeat_status(text):
    return text.replace("<STATUS>1</STATUS>", "<STATUS>1</STATUS><STATUS>1</STATUS>", 1)


def utf16_bom(raw, text):
    return b"\xff\xfe" + raw


def utf8(raw, text):
    return text.encode("utf-8")


def same(raw, text):
    return raw


def encoded(change):
    return lambda raw, text: change(text).encode("utf-8")


def cases():
    return [
        ("captured base", "both targets SHAPE measured; 14 rows; 15 references removed", 0,
         ["LEDGER ROWS 14", "control character references removed 15", f"VERDICT {TARGET!r}: SHAPE measured",
          f"VERDICT {OTHER!r}: SHAPE measured"], [], same, False),
        ("extra child in GST row", "SHAPE unmeasured: EXTRA_GST_ROW", 0,
         ["EXTRA_GST_ROW", "OUTSIDE GST_ROW_CHILDREN", "SHAPE unmeasured"], [], encoded(add_gst_child), False),
        ("extra child in state block", "SHAPE unmeasured: EXTRA_STATE_BLOCK", 0,
         ["EXTRA_STATE_BLOCK", "OUTSIDE STATE_BLOCK_CHILDREN", "SHAPE unmeasured"], [], encoded(add_state_child), False),
        ("non-empty slab list", "SHAPE unmeasured: non-empty GSTSLABRATES.LIST", 0,
         ["NON-EMPTY [UNMEASURED]", "SHAPE unmeasured"], [], encoded(fill_slab), False),
        ("extra child in rate detail", "SHAPE unmeasured: EXTRA_RATE_DETAIL", 0,
         ["EXTRA_RATE_DETAIL", "OUTSIDE HEAD_RATE_CHILDREN", "SHAPE unmeasured"], [], encoded(add_rate_child), False),
        ("second GSTDETAILS.LIST row", "two rows; still SHAPE measured under the Rust loop", 0,
         ["GSTDETAILS.LIST rows: 2", f"VERDICT {TARGET!r}: SHAPE measured"],
         ["SHAPE unmeasured"], encoded(second_gst_row), False),
        ("missing ledger", "valid diff with removed ledger paths, not a failed read", 0,
         ["removed /ENVELOPE", OTHER, "STRUCTURAL DIFF added 0, removed"], [], encoded(remove_other_ledger), True),
        ("answer is not a collection", "exit 2: request failed, not no difference", 2,
         ["ERROR", "no BODY/DATA/COLLECTION"], [], encoded(no_collection), False),
        ("UTF-16LE with BOM", "SHAPE measured; encoding decoded", 0,
         ["UTF-16LE with BOM", f"VERDICT {TARGET!r}: SHAPE measured"], [], utf16_bom, False),
        ("UTF-8", "SHAPE measured; encoding decoded", 0,
         ["UTF-8 without BOM", f"VERDICT {TARGET!r}: SHAPE measured"], [], utf8, False),
        ("extra &#4; reference", "16 references removed; shape unchanged", 0,
         ["control character references removed 16", f"VERDICT {TARGET!r}: SHAPE measured"],
         ["SHAPE unmeasured"], encoded(extra_control_reference), False),
        ("text changed", "structural diff names the changed GSTRATE text", 0,
         ["text changed", "2.50", "9.00"], [], encoded(change_rate_text), True),
        ("identical answers", "no structural difference, exit 0", 0,
         ["STRUCTURAL DIFF added 0, removed 0, text changed 0", "no structural difference"], [], same, True),
        ("Tally status is failure", "exit 2: unsuccessful answer", 2,
         ["ERROR", "HEADER/STATUS"], [], encoded(failed_status), False),
        ("company GUID absent from one row", "READ refused: company mismatch, on that ledger only", 0,
         [f"VERDICT {TARGET!r}: READ refused by the build's parser: invoice_ledger_rates_company_mismatch",
          f"VERDICT {OTHER!r}: SHAPE measured"], [f"VERDICT {TARGET!r}: SHAPE"], encoded(drop_company_guid), False),
        ("another company's GUID on one row", "READ refused: company mismatch", 0,
         [f"VERDICT {TARGET!r}: READ refused by the build's parser: invoice_ledger_rates_company_mismatch",
          "00000000-430c"], [f"VERDICT {TARGET!r}: SHAPE"], encoded(other_company_guid), False),
        ("TAXABILITY repeated in the GST row", "READ refused: field repeated", 0,
         [f"VERDICT {TARGET!r}: READ refused by the build's parser: invoice_read_field_repeated: TAXABILITY"],
         [f"VERDICT {TARGET!r}: SHAPE"], encoded(repeat_taxability), False),
        ("one ledger's row twice", "READ refused: name repeated; 15 rows", 0,
         ["LEDGER ROWS 15", "invoice_ledger_rates_name_repeated: 2 rows"],
         [f"VERDICT {TARGET!r}: SHAPE"], encoded(repeat_ledger), False),
        ("STATUS repeated in the header", "exit 2: not one STATUS of 1", 2,
         ["ERROR", "HEADER/STATUS"], [], encoded(repeat_status), False),
    ]


def main():
    raw = FIXTURE.read_bytes()
    if hashlib.sha256(raw).hexdigest() != FIXTURE_SHA256:
        print(f"FIXTURE STOP: {FIXTURE} does not have the captured SHA-256")
        return 2
    text = raw.decode("utf-16-le")
    right = 0
    all_cases = cases()
    with tempfile.TemporaryDirectory(prefix="2162-rw-") as folder:
        folder = pathlib.Path(folder)
        base = folder / "base.xml"
        base.write_bytes(raw)
        for number, (name, expectation, expected_exit, needles, forbidden, mutate, compare_base) in enumerate(all_cases, 1):
            candidate = folder / f"case-{number:02}.xml"
            candidate.write_bytes(mutate(raw, text))
            args = [str(base), str(candidate)] if compare_base else [str(candidate)]
            output = io.StringIO()
            with contextlib.redirect_stdout(output):
                actual_exit = ratecmp.main(args)
            printed = output.getvalue()
            good = actual_exit == expected_exit
            good = good and all(needle in printed for needle in needles)
            good = good and all(needle not in printed for needle in forbidden)
            if good:
                right += 1
                print(f"OK {name}: {expectation}; exit {expected_exit}")
            else:
                print(f"WRONG {name}: expected {expectation}, exit {expected_exit}; got exit {actual_exit}")
                print("  missing:", [needle for needle in needles if needle not in printed])
                print("  unexpectedly present:", [needle for needle in forbidden if needle in printed])
                print("  output start:", printed[:1200].replace("\n", " | "))
    print(f"{len(all_cases)} cases, {right} right")
    return 0 if right == len(all_cases) else 1


if __name__ == "__main__":
    raise SystemExit(main())
