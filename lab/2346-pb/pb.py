#!/usr/bin/env python3
# pb.py: row 2377-NB's counter. It reads each answer by an XML parser (LAB-COMMON C4), never a text search, and prints
# never more than CAP lines per call (AGENTS.md P5). The kind comes from the answer's name a-<label>-<K>-<request>:
#   M  marks: each COMPANY row's GUID, whether it is BOOK's, ALTVCHID and ALTMSTID.
#   S  master's List of Ledgers snapshot: the row count, then PARTY's row with each field as sent.
#   L  master's Ledger Outstandings answer for PARTY: what master's parse (parse_ledger_outstandings_answer,
#      native_outstandings/party_bills.rs) makes of it, as one ANSWER line:
#        ANSWER: NO BYTES                    the file holds 0 bytes, or curl wrote none (the request failed: recorded
#                                            first as that, by run.py's line and the headers file);
#        ANSWER: TALLY FAILURE               a STATUS element anywhere (party_bills_tally_failure);
#        ANSWER: MALFORMED (<rule>)          master's grammar refuses it (party_bills_answer_malformed), with the
#                                            rule this script found, named as master names it where it can tell;
#        ANSWER: ROWS <n> (<on account>)     n BILLFIXED rows, with each row's BILLREF, BILLDATE and BILLCL and the
#                                            sum of BILLCL; n may be 0: an envelope with no row.
# The grammar followed (party_bills.rs): the root is ENVELOPE, and nothing follows it; inside it, a BILLFIXED holds only
# BILLDATE and BILLREF; outside a BILLFIXED only BILLOP, BILLCL, BILLDUE, BILLOVERDUE and the BILLVCH* scalars, each
# after a BILLFIXED; no element is self-closed; a row's BILLREF, BILLDATE, BILLCL, BILLDUE and BILLOVERDUE appear once
# each; BILLREF and BILLCL are not empty; BILLCL is an exact decimal; BILLOVERDUE is empty or an integer. Text is
# trimmed, as master trims it. Dates follow native_outstandings/date.rs: d-Mon-yy with one of the twelve month names
# and a real calendar day; a bill date's century is the one placing it after the later of 50 years before the books'
# first day (BOOKS_FROM) and 100 years before the report's to-date, and on or before that to-date, so a bill dated
# after the to-date is MALFORMED; a due date's is the one placing it after 10 years before its bill date and within 100
# years after that, and only a due date may carry a four-digit year, from 2100. The to-date is read from the request
# name, which ends -YYYYMMDD.xml. A row's BILLOP is not counted for duplicates, as master keeps no slot for it.
# Modes:
#   pb.py ANSWER...              the lines above for each answer (an L answer also needs its .headers file beside it).
#   --same-row M0 M1             each holds exactly one COMPANY row, BOOK's, with equal ALTVCHID and ALTMSTID, else
#                                MARKS DIFFER (exit 2); one row that is not BOOK's, or any other count, is STOP.
#   --outcome LA LB S            the refusal or the read fetch_party_bills (tally/runtime_party_bills.rs) gives for the
#                                report sent twice (LA, LB) and the snapshot S, in its order, as one last line
#                                OUTCOME: <code> (<why>):
#                                  response_content_type_unsupported   either answer has not exactly one Content-Type
#                                                                      header, or its value is not text/xml with at
#                                                                      most one charset of utf-8 or utf-16
#                                                                      (bridge-tally-transport lib.rs:1288-1295,
#                                                                      text_encoding.rs:451): the transport refuses it;
#                                  declared_encoding_mismatch          its charset is utf-8 where UTF-16 is asked for;
#                                  party_bills_drifted                 LA and LB are not byte-equal (require_stable,
#                                                                      runtime_party_bills.rs:101);
#                                  party_not_in_book                   no snapshot row is named PARTY (:116);
#                                  party_not_billwise                  its ISBILLWISEON is No, False or 0 (:118);
#                                  party_bills_tally_failure / party_bills_answer_malformed   the L answer's ANSWER;
#                                  party_ledger_closing_not_read       the snapshot's CLOSINGBALANCE is empty (:91);
#                                  party_bills_untied                  sum of BILLCL is not the closing (:98);
#                                  party_bills_no_rows_not_established  no row, the closing equal to zero (:104-107);
#                                  told                                rows that tie;
#                                  NOT READ BY THIS SCRIPT             a case this script does not decide (a
#                                                                      Content-Encoding header; a closing or bill-wise
#                                                                      flag that is not a plain form): master decides.
#                                Codes are master's (party_bills.rs, runtime_party_bills.rs); the order is the runtime's.
#                                Limits: master refuses the whole snapshot on any row it cannot read, before
#                                party_not_in_book; this script reads PARTY's row, every row's GUID and nothing else.
#                                The snapshot's own pair and Content-Type are not checked (it is sent once).
# In an M or S answer a character XML forbids, raw or as a reference, is kept as U+FFFD, as master's sanitizer keeps it.
# STOP, exit 1, never "0 rows", for an answer this script cannot read: an M or S answer that does not decode as UTF-16LE
# or parse, carries a LINEERROR, has no BODY/DATA/COLLECTION, or (S) has two rows named PARTY, a row whose
# BRIDGECOMPANYGUID is not BOOK, or PARTY's row without one of its fields; an L answer that does not decode as UTF-16LE,
# or holds a character XML forbids; an L answer without its .headers file. An L answer that does not parse is ANSWER:
# MALFORMED, as master refuses it.
# Master's own parsers are run on the answers when they become fixtures, and that run is the one that counts.
# Row 2346-PB: row 2377-NB's pb.py with PARTY alone changed (the line marked PB).
# Example: python3 -I pb.py --outcome a-l1a-L-lo-rj-20260804.xml a-l1b-L-lo-rj-20260804.xml a-s1-S-snap-20260401-20260804.xml
import calendar, pathlib, re, sys
from decimal import Decimal
import xml.etree.ElementTree as ET
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as pilot-lab-register-book-extent reads it
PARTY = "BRIDGE Walk-in BC"  # PB: row 2346-AR's customer, in place of row 2377-NB's BRIDGE Supplier RJ
BOOKS_FROM = (2026, 4, 1)  # the book's first day, pilot-lab-register-book-extent
CAP = 400
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
PLAIN = re.compile(r"-?[0-9]+(?:\.[0-9]+)?")
MONTHS = ("Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec")
ROW_SCALARS = ("BILLOP", "BILLCL", "BILLDUE", "BILLOVERDUE")
COUNTED = ("BILLCL", "BILLDUE", "BILLOVERDUE")  # master keeps no slot for BILLOP (party_bills.rs)
VOUCHER_SCALARS = ("BILLVCHDATE", "BILLVCHTYPE", "BILLVCHNUMBER", "BILLVCHAMOUNT", "BILLVCHQUALIFIER")
SNAP_FIELDS = ("PARENT", "CURRENCYNAME", "ISBILLWISEON", "OPENINGBALANCE", "CLOSINGBALANCE", "BRIDGECOMPANYGUID")
lines = 0

def say(text):
    global lines
    lines += 1
    if lines == CAP:
        print(f"... more than {CAP} lines: the rest is not printed (STOP for this call)")
        raise SystemExit(1)
    print(text)

def stop(text):
    print(f"STOP: {text}")
    raise SystemExit(1)

def kind(path):
    match = re.match(r"a-[^-]+-([MSL])-", pathlib.Path(path).name)
    if not match:
        stop(f"{path}: not an answer name a-<label>-<M|S|L>-<request>")
    return match.group(1)

def to_date(path):
    match = re.search(r"-([0-9]{4})([0-9]{2})([0-9]{2})\.xml$", pathlib.Path(path).name)
    if not match:
        stop(f"{path}: the request name does not end -YYYYMMDD.xml")
    return tuple(int(part) for part in match.groups())

def native_date(raw, after, through, full_year):
    """The one (y, m, d) in (after, through] that d-Mon-yy names, as date.rs resolves it, else None."""
    parts = raw.split("-")
    if len(parts) != 3 or not re.fullmatch(r"[0-9]{1,2}", parts[0]) or parts[1] not in MONTHS:
        return None
    day, month, year = int(parts[0]), MONTHS.index(parts[1]) + 1, parts[2]
    if not year.isdigit() or not year.isascii():
        return None
    if len(year) == 2:
        years = [century + int(year) for century in range(after[0] // 100 * 100, through[0] // 100 * 100 + 1, 100)]
    elif len(year) == 4 and full_year and int(year) >= 2100:
        years = [int(year)]
    else:
        return None
    found = [(y, month, day) for y in years
             if 1 <= day <= calendar.monthrange(y, month)[1] and after < (y, month, day) <= through]
    return found[0] if len(found) == 1 else None

def decode(path):
    raw = pathlib.Path(path).read_bytes()
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        return raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")

def collection(path):
    # A character XML forbids is kept as U+FFFD, as master's sanitizer keeps it (tolerant_xml.rs), and as op.py and
    # tb.py of row 683-OP do: the snapshot's Profit & Loss A/c sends one in its PARENT.
    text = FORBIDDEN.sub("\ufffd", decode(path))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    status = root.find("HEADER/STATUS")
    print(f"{path}: HEADER/STATUS {status.text.strip() if status is not None and status.text else '<absent>'}")
    if list(root.iter("LINEERROR")):
        stop(f"{path} carries a LINEERROR: the request failed")
    found = root.find("BODY/DATA/COLLECTION")
    if found is None:
        stop(f"{path} has no BODY/DATA/COLLECTION: the request failed")
    return found

def leaf(row, tag):
    found = row.findall(tag)
    if len(found) > 1:
        stop(f"{tag} twice in one row")
    return None if not found else (found[0].text or "")

def marks(path):
    rows = collection(path).findall("COMPANY")
    out = [(leaf(r, "GUID"), (leaf(r, "ALTVCHID") or "").strip(), (leaf(r, "ALTMSTID") or "").strip()) for r in rows]
    say(f"  COMPANY rows: {len(rows)}")
    for guid, v, m in out:
        say(f"  GUID {guid} (the book's: {'yes' if guid == BOOK else 'NO'}) ALTVCHID {v} ALTMSTID {m}")
    return out

def snapshot(path, show=True):
    rows = collection(path).findall("LEDGER")
    party = None
    for row in rows:
        guid = leaf(row, "BRIDGECOMPANYGUID")
        if guid is None or guid.strip().lower() != BOOK:
            stop(f"{path}: a LEDGER row's BRIDGECOMPANYGUID {guid!r} is not the book's")
        name = row.get("NAME")
        if name == PARTY:
            if party is not None:
                stop(f"{path}: two rows named {PARTY!r}")
            party = {tag: leaf(row, tag) for tag in SNAP_FIELDS}
            missing = [tag for tag, value in party.items() if value is None]
            if missing:
                stop(f"{path}: {PARTY!r} has no {', '.join(missing)}")
    if show:
        say(f"  LEDGER rows: {len(rows)}")
        if party is None:
            say(f"  {PARTY!r}: no row")
        else:
            say(f"  {PARTY!r}: " + "; ".join(f"{tag} {value!r}" for tag, value in party.items()))
            closing = party["CLOSINGBALANCE"]
            say(f"    CLOSINGBALANCE reads " + ("EMPTY (master: None, not zero)" if closing.strip() == ""
                else f"{Decimal(closing.strip())}" if PLAIN.fullmatch(closing.strip()) else "OTHER"))
    return party

def headers(path):
    found = pathlib.Path(str(path) + ".headers")
    if not found.exists():
        stop(f"{path}: its headers file {found.name} is missing")
    text = found.read_text("latin-1")
    blocks = [b for b in re.split(r"\r?\n\r?\n", text) if b.strip()]
    last = blocks[-1].splitlines() if blocks else []
    status = last[0].strip() if last else "<none>"
    types = [l.split(":", 1)[1].strip() for l in last[1:] if l.lower().startswith("content-type:")]
    length = [l.split(":", 1)[1].strip() for l in last[1:] if l.lower().startswith("content-length:")]
    coding = [l.split(":", 1)[1].strip() for l in last[1:] if l.lower().startswith("content-encoding:")]
    say(f"  headers: {status!r}; Content-Type {types if types else 'absent'}; CONTENT-LENGTH {length if length else 'absent'}"
        + (f"; Content-Encoding {coding}" if coding else ""))
    return types, coding

def transport(path):
    """The transport's refusal of an answer's headers (lib.rs:1288-1295, text_encoding.rs:451), or None."""
    types, coding = headers(path)
    if len(types) != 1:
        return f"response_content_type_unsupported ({len(types)} Content-Type headers)"
    value = types[0]
    parts = value.split(";")
    if not value.isascii() or any(ord(c) < 32 or ord(c) == 127 for c in value) or parts[0].strip().lower() != "text/xml":
        return f"response_content_type_unsupported (Content-Type {value!r})"
    charsets = []
    for parameter in parts[1:]:
        name, eq, charset = parameter.strip().partition("=")
        if not eq or name.strip().lower() != "charset" or charsets:
            return f"response_content_type_unsupported (Content-Type {value!r})"
        charsets.append(charset.strip().lower())
    if charsets and charsets[0] not in ("utf-8", "utf-16"):
        return f"response_content_type_unsupported (Content-Type {value!r})"
    if charsets == ["utf-8"]:
        return f"declared_encoding_mismatch (Content-Type {value!r}, UTF-16 asked for)"
    if coding:
        return f"NOT READ BY THIS SCRIPT (Content-Encoding {coding}: the transport decides)"
    return None

def answer(path, show=True):
    """(class, rows): what master's parse makes of an L answer."""
    if not pathlib.Path(path).exists():  # curl may write no file for an empty body; its headers file says why
        if show:
            say(f"{path}: no file written")
        return "NO BYTES", None
    raw = pathlib.Path(path).read_bytes()
    if len(raw) == 0:
        if show:
            say(f"{path}: 0 bytes")
        return "NO BYTES", None
    text = decode(path)
    if FORBIDDEN.search(text):
        stop(f"{path} holds a character XML forbids")
    if re.search(r"<\s*STATUS[\s>/]", text, re.I):
        return "TALLY FAILURE", None
    if re.search(r"<[A-Za-z][^<>]*/\s*>", text):
        return "MALFORMED (party_bills_empty_element)", None
    if re.search(r"<!DOCTYPE|<!\[CDATA\[|<\?(?!xml\s)", text, re.I):
        return "MALFORMED (party_bills_unexpected_content)", None
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        return f"MALFORMED (party_bills_xml_malformed: {error})", None
    if root.tag.upper() != "ENVELOPE":
        return "MALFORMED (party_bills_root_not_envelope)", None
    if (root.text or "").strip():
        return "MALFORMED (party_bills_unexpected_content)", None
    rows = []
    for child in root:
        tag = child.tag.upper()
        if (child.tail or "").strip():
            return "MALFORMED (party_bills_unexpected_content)", None
        if tag == "BILLFIXED":
            if (child.text or "").strip():
                return "MALFORMED (party_bills_unexpected_content)", None
            row = {}
            for part in child:
                name = part.tag.upper()
                if name not in ("BILLDATE", "BILLREF") or len(part) or (part.tail or "").strip():
                    return f"MALFORMED (party_bills_unknown_element or nested content, row {len(rows) + 1})", None
                if name in row:
                    return f"MALFORMED (party_bills_duplicate_element, row {len(rows) + 1})", None
                row[name] = (part.text or "").strip()
            rows.append(row)
            continue
        if tag not in ROW_SCALARS and tag not in VOUCHER_SCALARS:
            return f"MALFORMED (party_bills_unknown_element, row {len(rows)})", None
        if len(child):
            return f"MALFORMED (party_bills_xml_malformed: content inside {tag}, row {len(rows)})", None
        if not rows:
            return "MALFORMED (party_bills_scalar_before_fixed)", None
        if tag in COUNTED:
            if tag in rows[-1]:
                return f"MALFORMED (party_bills_duplicate_element, row {len(rows)})", None
            rows[-1][tag] = (child.text or "").strip()
    for index, row in enumerate(rows, 1):
        for tag, rule in (("BILLREF", "reference_missing"), ("BILLDATE", "bill_date_missing"),
                          ("BILLCL", "closing_missing"), ("BILLDUE", "due_date_missing"),
                          ("BILLOVERDUE", "overdue_missing")):
            if tag not in row:
                return f"MALFORMED (party_bills_{rule}, row {index})", None
        if row["BILLREF"] == "":
            return f"MALFORMED (party_bills_reference_empty, row {index})", None
        end = to_date(path)
        after = max((BOOKS_FROM[0] - 50,) + BOOKS_FROM[1:], (end[0] - 100,) + end[1:])
        bill = native_date(row["BILLDATE"], after, end, False)
        if bill is None:
            return f"MALFORMED (party_bills_bill_date_invalid, row {index})", None
        if row["BILLCL"] == "":
            return f"MALFORMED (party_bills_closing_empty, row {index})", None
        if not PLAIN.fullmatch(row["BILLCL"]):
            return f"MALFORMED (party_bills_closing_invalid, row {index})", None
        due_after = (bill[0] - 10,) + bill[1:]
        if row["BILLDUE"] != "" and native_date(row["BILLDUE"], due_after, (due_after[0] + 100,) + due_after[1:], True) is None:
            return f"MALFORMED (party_bills_due_date_invalid, row {index})", None
        if row["BILLOVERDUE"] != "" and (not re.fullmatch(r"[+-]?[0-9]+", row["BILLOVERDUE"])
                                         or not -2**63 <= int(row["BILLOVERDUE"]) < 2**63):
            return f"MALFORMED (party_bills_overdue_invalid, row {index})", None
    on_account = sum(1 for row in rows if row["BILLREF"] == "On Account")
    if show:
        for row in rows:
            say(f"  BILLREF {row['BILLREF']!r} BILLDATE {row['BILLDATE']!r} BILLCL {row['BILLCL']!r} "
                f"BILLDUE {row['BILLDUE']!r} BILLOVERDUE {row['BILLOVERDUE']!r}")
        say(f"  sum of BILLCL: {sum((Decimal(row['BILLCL']) for row in rows), Decimal(0))}")
    return f"ROWS {len(rows)} ({on_account} On Account)", rows

def outcome(la_path, lb_path, s_path):
    for path in (la_path, lb_path):
        refused = transport(path)
        if refused:
            return refused
    def held(path):
        return pathlib.Path(path).read_bytes() if pathlib.Path(path).exists() else b""
    if held(la_path) != held(lb_path):
        return "party_bills_drifted (the two answers are not byte-equal)"
    klass, rows = answer(la_path)
    say(f"ANSWER: {klass}")
    party = snapshot(s_path)
    if party is None:
        return "party_not_in_book (no snapshot row named it)"
    flag = party["ISBILLWISEON"].strip().lower()
    if flag in ("no", "false", "0"):
        return f"party_not_billwise (ISBILLWISEON {party['ISBILLWISEON']!r})"
    if flag not in ("yes", "true", "1"):
        return f"NOT READ BY THIS SCRIPT (ISBILLWISEON {party['ISBILLWISEON']!r}: the snapshot parse refuses it)"
    if klass == "NO BYTES":
        return "party_bills_answer_malformed (an empty text is an unclosed envelope: party_bills_envelope_unclosed)"
    if klass == "TALLY FAILURE":
        return "party_bills_tally_failure"
    if klass.startswith("MALFORMED"):
        return f"party_bills_answer_malformed ({klass})"
    closing = party["CLOSINGBALANCE"].strip()
    if closing == "":
        return "party_ledger_closing_not_read (CLOSINGBALANCE empty)"
    if not PLAIN.fullmatch(closing):
        return f"NOT READ BY THIS SCRIPT (CLOSINGBALANCE {closing!r}: the snapshot parse decides it)"
    total = sum((Decimal(row["BILLCL"]) for row in rows), Decimal(0))
    if total != Decimal(closing):
        return f"party_bills_untied (sum of BILLCL {total}, closing {closing})"
    if not rows:
        return f"party_bills_no_rows_not_established (no row, closing {closing})"
    return f"told ({klass}, sum of BILLCL {total} equals closing {closing})"

args = sys.argv[1:]
if not args:
    stop("no answer named")
if args[0] == "--same-row":
    if len(args) != 3 or kind(args[1]) != "M" or kind(args[2]) != "M":
        stop("--same-row takes two marks answers")
    a, b = marks(args[1]), marks(args[2])
    if len(a) != 1 or len(b) != 1 or a[0][0] != BOOK or b[0][0] != BOOK:
        stop("each marks answer must hold exactly one COMPANY row, the book's")
    if a != b:
        print(f"MARKS DIFFER: {a[0]} -> {b[0]}")
        raise SystemExit(2)
    print("SAME MARKS")
elif args[0] == "--outcome":
    if len(args) != 4 or kind(args[1]) != "L" or kind(args[2]) != "L" or kind(args[3]) != "S":
        stop("--outcome takes the two L answers, then an S answer")
    print(f"OUTCOME: {outcome(args[1], args[2], args[3])}")
else:
    for path in args:
        k = kind(path)
        if k == "M":
            marks(path)
        elif k == "S":
            snapshot(path)
        else:
            headers(path)
            say(f"ANSWER: {answer(path)[0]}")
