#!/usr/bin/env python3
"""Report ledger-rate shapes and, for two saved answers, their structural diff.

Exit 2 means an input is not an answer at all: unreadable, not XML, a status
other than one STATUS of 1, or no ledger collection. Exit 0 means every input was
a ledger collection; what the build would make of it is in each VERDICT line:

  READ refused by the build's parser: <code>   the checks of parse_ledger_rates
      that end the read (company GUID, a repeated field, a repeated name, a
      rate detail with no head), agent_import_invoice_wire.rs:1537-1615
  SHAPE unmeasured: <what>                     the measured-element rule
  SHAPE measured                               neither of the above

SHAPE covers only that rule, not the later Sales checks of sales_rate_milli.
Not mirrored: the build ignores a CDATA section; this parser reads it as text.
"""

import hashlib
import pathlib
import re
import sys
import unicodedata
import xml.etree.ElementTree as ET

# Copied from src-tauri/src/agent_import_invoice_wire.rs:1467-1475; the rule is
# the loop at :1555-1575. A second GST row is not, by itself, an unknown shape.
GST_ROW_CHILDREN = (
    "APPLICABLEFROM",
    "GSTINELIGIBLEITC",
    "SRCOFGSTDETAILS",
    "STATEWISEDETAILS.LIST",
    "TAXABILITY",
)
STATE_BLOCK_CHILDREN = ("GSTSLABRATES.LIST", "RATEDETAILS.LIST", "STATENAME")
HEAD_RATE_CHILDREN = ("GSTRATE", "GSTRATEDUTYHEAD", "GSTRATEVALUATIONTYPE")
# The one synthetic lab book of this sheet; the build compares each row's
# BRIDGECOMPANYGUID with the company it was asked for, ignoring ASCII case (:1541).
COMPANY_GUID = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"

CONTROL_REFERENCE = re.compile(
    r"&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));",
    re.IGNORECASE,
)
RAW_CONTROL = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]")
XML_DECLARATION = re.compile(r"^\s*<\?xml[^>]*\?>")
LEDGER_CAP = 50
ROW_CAP = 40
STATE_CAP = 40
SLAB_CAP = 40
RATE_CAP = 40
CHILD_CAP = 100
REASON_CAP = 20
DIFF_CAP = 200


class AnswerError(Exception):
    pass


class Answer:
    def __init__(self, path, raw, text, encoding, references, controls, root):
        self.path = path
        self.raw = raw
        self.text = text
        self.encoding = encoding
        self.references = references
        self.controls = controls
        self.root = root


def direct_text(node):
    # Rust Node.text holds the node's own text chunks, not its descendants'.
    return (node.text or "") + "".join(child.tail or "" for child in node)


def clean(text):
    chars = list(text)
    left = 0
    right = len(chars)
    while left < right and (chars[left].isspace() or unicodedata.category(chars[left]) == "Cc"):
        left += 1
    while right > left and (chars[right - 1].isspace() or unicodedata.category(chars[right - 1]) == "Cc"):
        right -= 1
    return "".join(chars[left:right])


def field(node, tag, path, refused):
    """text_of, but a repeated field is recorded as the build's refusal."""
    found = [child for child in node if child.tag == tag]
    if len(found) > 1:
        refused.append(f"invoice_read_field_repeated: {tag}×{len(found)} below {path}")
        return None
    return None if not found else clean(direct_text(found[0]))


def decode(raw):
    if raw.startswith(b"\xff\xfe"):
        return raw[2:].decode("utf-16-le"), "UTF-16LE with BOM"
    if raw.startswith(b"\xef\xbb\xbf"):
        return raw[3:].decode("utf-8"), "UTF-8 with BOM"
    if raw.startswith(b"\xfe\xff"):
        raise AnswerError("UTF-16BE is outside this sheet's UTF-16LE/UTF-8 inputs")
    if len(raw) >= 2 and raw[1] == 0 and raw[0] in (9, 10, 13, 32, 60):
        return raw.decode("utf-16-le"), "UTF-16LE without BOM"
    return raw.decode("utf-8"), "UTF-8 without BOM"


def load_answer(name):
    path = pathlib.Path(name)
    try:
        raw = path.read_bytes()
    except OSError as error:
        raise AnswerError(f"cannot read {name}: {error}") from error
    if not raw:
        raise AnswerError("empty answer: the request failed")
    try:
        text, encoding = decode(raw)
    except UnicodeError as error:
        raise AnswerError(f"answer does not decode as UTF-16LE or UTF-8: {error}") from error
    text, references = CONTROL_REFERENCE.subn("", text)
    text, controls = RAW_CONTROL.subn("", text)
    text = XML_DECLARATION.sub("", text, count=1)
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        raise AnswerError(f"XML does not parse after control removal: {error}") from error
    if root.tag != "ENVELOPE":
        raise AnswerError(f"root is {root.tag}, not ENVELOPE")
    header = root.find("HEADER")
    statuses = [] if header is None else [child for child in header if child.tag == "STATUS"]
    if len(statuses) != 1 or clean(direct_text(statuses[0])) != "1":
        raise AnswerError("HEADER/STATUS is absent, repeated or is not 1")
    collection = root.find("BODY/DATA/COLLECTION")
    if collection is None:
        raise AnswerError("no BODY/DATA/COLLECTION: the request failed")
    ledgers = [child for child in collection if child.tag == "LEDGER"]
    if not ledgers:
        raise AnswerError("the collection has no LEDGER rows")
    for ledger in ledgers:
        if ledger.get("NAME") is None:
            raise AnswerError("a LEDGER row has no NAME attribute")
    return Answer(path, raw, text, encoding, references, controls, root)


def child_counts(node, admitted, list_name):
    counts = {}
    for child in node:
        counts[child.tag] = counts.get(child.tag, 0) + 1
    parts = [
        f"{tag}×{count}" + ("" if tag in admitted else f" [OUTSIDE {list_name}]")
        for tag, count in counts.items()
    ]
    omitted = max(0, len(node) - CHILD_CAP)
    shown = []
    remaining = CHILD_CAP
    for child in node:
        if remaining <= 0:
            break
        if child.tag not in counts:
            continue
        shown.append(child.tag)
        remaining -= 1
    by_tag = []
    seen = set()
    for tag in shown:
        if tag not in seen:
            seen.add(tag)
            by_tag.append(next(part for part in parts if part.startswith(tag + "×")))
    if omitted:
        by_tag.append(f"... {omitted} children not printed")
    return ", ".join(by_tag) if by_tag else "<none>"


def unexpected(node, admitted, path, reasons):
    for child in node:
        if child.tag not in admitted:
            reasons.append(f"{path} has outside child {child.tag}")


def report(answer):
    print(
        f"ANSWER {answer.path}: {len(answer.raw)} bytes, sha256 {hashlib.sha256(answer.raw).hexdigest()}, "
        f"{answer.encoding}, control character references removed {answer.references}, "
        f"raw control characters removed {answer.controls}"
    )
    collection = answer.root.find("BODY/DATA/COLLECTION")
    ledgers = [child for child in collection if child.tag == "LEDGER"]
    name_counts = {}
    for ledger in ledgers:
        name_counts[ledger.get("NAME")] = name_counts.get(ledger.get("NAME"), 0) + 1
    shown = ", ".join(repr(ledger.get("NAME")) for ledger in ledgers[:LEDGER_CAP])
    print(f"LEDGER ROWS {len(ledgers)}: {shown}")
    for ledger in ledgers[:LEDGER_CAP]:
        name = ledger.get("NAME")
        print(f"LEDGER {name!r}")
        refused = []
        bound = field(ledger, "BRIDGECOMPANYGUID", name, refused)
        if bound is None or bound.lower() != COMPANY_GUID:
            refused.append(f"invoice_ledger_rates_company_mismatch: BRIDGECOMPANYGUID is {bound!r}")
        if name_counts[name] > 1:
            refused.append(f"invoice_ledger_rates_name_repeated: {name_counts[name]} rows carry this NAME")
        for tag in ("RATEOFTAXCALCULATION", "ROUNDINGMETHOD", "ROUNDINGLIMIT"):
            field(ledger, tag, name, refused)
        print(
            f"  GUID {field(ledger, 'GUID', name, refused)!r}; PARENT {field(ledger, 'PARENT', name, refused)!r}; "
            f"BRIDGECOMPANYGUID {bound!r}"
        )
        rows = [child for child in ledger if child.tag == "GSTDETAILS.LIST"]
        print(f"  GSTDETAILS.LIST rows: {len(rows)}")
        reasons = []
        for row_number, row in enumerate(rows, 1):
            row_path = f"{name}/GSTDETAILS.LIST[{row_number}]"
            unexpected(row, GST_ROW_CHILDREN, row_path, reasons)
            for tag in ("APPLICABLEFROM", "TAXABILITY", "SRCOFGSTDETAILS"):
                field(row, tag, row_path, refused)
            if row_number <= ROW_CAP:
                print(f"  GST row {row_number} children: {child_counts(row, GST_ROW_CHILDREN, 'GST_ROW_CHILDREN')}")
            states = [child for child in row if child.tag == "STATEWISEDETAILS.LIST"]
            for state_number, state in enumerate(states, 1):
                state_path = f"{row_path}/STATEWISEDETAILS.LIST[{state_number}]"
                unexpected(state, STATE_BLOCK_CHILDREN, state_path, reasons)
                field(state, "STATENAME", state_path, refused)
                if row_number <= ROW_CAP and state_number <= STATE_CAP:
                    print(
                        f"    STATE {state_number} children: "
                        f"{child_counts(state, STATE_BLOCK_CHILDREN, 'STATE_BLOCK_CHILDREN')}"
                    )
                slabs = [child for child in state if child.tag == "GSTSLABRATES.LIST"]
                for slab_number, slab in enumerate(slabs, 1):
                    nonempty = bool(len(slab)) or bool(clean(direct_text(slab)))
                    if nonempty:
                        reasons.append(f"{state_path}/GSTSLABRATES.LIST[{slab_number}] is non-empty")
                    if row_number <= ROW_CAP and state_number <= STATE_CAP and slab_number <= SLAB_CAP:
                        print(
                            f"      GSTSLABRATES.LIST {slab_number}: "
                            f"{'NON-EMPTY [UNMEASURED]' if nonempty else 'empty'}"
                        )
                if len(slabs) > SLAB_CAP and row_number <= ROW_CAP and state_number <= STATE_CAP:
                    print(f"      ... {len(slabs) - SLAB_CAP} GSTSLABRATES.LIST rows not printed")
                rates = [child for child in state if child.tag == "RATEDETAILS.LIST"]
                for rate_number, rate in enumerate(rates, 1):
                    rate_path = f"{state_path}/RATEDETAILS.LIST[{rate_number}]"
                    unexpected(rate, HEAD_RATE_CHILDREN, rate_path, reasons)
                    for tag in ("GSTRATEVALUATIONTYPE", "GSTRATE"):
                        field(rate, tag, rate_path, refused)
                    head = field(rate, "GSTRATEDUTYHEAD", rate_path, refused)
                    if not any(child.tag == "GSTRATEDUTYHEAD" for child in rate):
                        refused.append(f"invoice_ledger_rates_head_unnamed: {rate_path}")
                    if row_number <= ROW_CAP and state_number <= STATE_CAP and rate_number <= RATE_CAP:
                        print(
                            f"      RATEDETAILS.LIST {rate_number} children: "
                            f"{child_counts(rate, HEAD_RATE_CHILDREN, 'HEAD_RATE_CHILDREN')}; "
                            f"GSTRATEDUTYHEAD={head!r}; "
                            f"GSTRATEVALUATIONTYPE={field(rate, 'GSTRATEVALUATIONTYPE', rate_path, [])!r}; "
                            f"GSTRATE={field(rate, 'GSTRATE', rate_path, [])!r}"
                        )
                if len(rates) > RATE_CAP and row_number <= ROW_CAP and state_number <= STATE_CAP:
                    print(f"      ... {len(rates) - RATE_CAP} RATEDETAILS.LIST rows not printed")
            if len(states) > STATE_CAP and row_number <= ROW_CAP:
                print(f"    ... {len(states) - STATE_CAP} STATEWISEDETAILS.LIST rows not printed")
        if len(rows) > ROW_CAP:
            print(f"  ... {len(rows) - ROW_CAP} GSTDETAILS.LIST rows not printed")
        if refused:
            detail = "; ".join(refused[:REASON_CAP])
            if len(refused) > REASON_CAP:
                detail += f"; ... {len(refused) - REASON_CAP} reasons not printed"
            if reasons:
                detail += f"; and {len(reasons)} shape reason(s), the first: {reasons[0]}"
            print(f"VERDICT {name!r}: READ refused by the build's parser: {detail}")
        elif reasons:
            detail = "; ".join(reasons[:REASON_CAP])
            if len(reasons) > REASON_CAP:
                detail += f"; ... {len(reasons) - REASON_CAP} reasons not printed"
            print(f"VERDICT {name!r}: SHAPE unmeasured: {detail}")
        else:
            print(f"VERDICT {name!r}: SHAPE measured")
    if len(ledgers) > LEDGER_CAP:
        print(f"... {len(ledgers) - LEDGER_CAP} ledgers and their verdicts not printed")


def normalized_text(node):
    text = direct_text(node)
    return "" if not text.strip() else text


def flatten(root):
    values = {}

    def visit(node, path):
        values[path] = normalized_text(node)
        for key, value in node.attrib.items():
            values[f"{path}/@{key}"] = value
        counts = {}
        for child in node:
            identity = (child.tag, child.get("NAME")) if child.tag == "LEDGER" else (child.tag, None)
            counts[identity] = counts.get(identity, 0) + 1
            suffix = child.tag
            if child.tag == "LEDGER":
                suffix += f"[NAME={child.get('NAME')!r}]"
                if counts[identity] > 1:
                    suffix += f"[{counts[identity]}]"
            else:
                suffix += f"[{counts[identity]}]"
            visit(child, f"{path}/{suffix}")

    visit(root, f"/{root.tag}")
    return values


def structural_diff(first, second):
    a = flatten(first.root)
    b = flatten(second.root)
    added = sorted(set(b) - set(a))
    removed = sorted(set(a) - set(b))
    changed = sorted(path for path in set(a) & set(b) if a[path] != b[path])
    print(
        f"STRUCTURAL DIFF added {len(added)}, removed {len(removed)}, text changed {len(changed)}"
    )
    lines = [f"added {path}" for path in added]
    lines += [f"removed {path}" for path in removed]
    lines += [f"text changed {path}: {a[path]!r} -> {b[path]!r}" for path in changed]
    for line in lines[:DIFF_CAP]:
        print("  " + line)
    if len(lines) > DIFF_CAP:
        print(f"  ... {len(lines) - DIFF_CAP} structural-diff lines not printed")
    if not lines:
        print("no structural difference")


def main(argv=None):
    args = sys.argv[1:] if argv is None else list(argv)
    if len(args) not in (1, 2):
        print("usage: python3 -I ratecmp.py ANSWER [OTHER_ANSWER]", file=sys.stderr)
        return 2
    answers = []
    for name in args:
        try:
            answer = load_answer(name)
            report(answer)
            answers.append(answer)
        except AnswerError as error:
            print(f"ERROR {name}: request failed or answer is not a ledger collection: {error}")
            return 2
    if len(answers) == 2:
        structural_diff(*answers)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
