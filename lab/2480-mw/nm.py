#!/usr/bin/env python3
# nm.py: row 2480-NM's counter. It is row 2346-PB's pb.py (lab/2346-pb-sheet at 53540665) with the one customer
# replaced by the eleven ledgers of names.tsv, and a Bills Receivable kind and two comparison modes added (the lines
# marked NM). The parsers (marks, the Ledger Outstandings grammar, the transport check) are PB's, copied unchanged.
# It reads each answer by an XML parser (LAB-COMMON C4), never a text search, and prints never more than CAP lines per
# call (AGENTS.md P5). The kind comes from the answer's name a-<label>-<K>-<request>:
#   M  marks: each COMPANY row's GUID, whether it is BOOK's, ALTVCHID and ALTMSTID.
#   S  master's List of Ledgers snapshot: the row count, then, for each of the eleven names (names.tsv), its row with
#      each field as sent, or "no row".                                                                     (NM)
#   B  the Bills Receivable report (B-bills.xml): the number of BILLFIXED groups, then for each of the eleven names its
#      groups (BILLREF, BILLDATE, BILLCL, BILLDUE, BILLOVERDUE), then the groups of any other party, counted.   (NM)
#   L  master's Ledger Outstandings answer for the ledger whose id the request name carries (a-<label>-L-lo-<id>-
#      <YYYYMMDD>.xml): what master's parse (parse_ledger_outstandings_answer, native_outstandings/party_bills.rs)
#      makes of it, as one ANSWER line, as PB's pb.py printed it:
#        ANSWER: NO BYTES                    the file holds 0 bytes, or curl wrote none (the request failed: recorded
#                                            first as that, by run.py's line and the headers file);
#        ANSWER: TALLY FAILURE               a STATUS element anywhere (party_bills_tally_failure);
#        ANSWER: MALFORMED (<rule>)          master's grammar refuses it (party_bills_answer_malformed);
#        ANSWER: ROWS <n> (<on account>)     n BILLFIXED rows with each row's BILLREF, BILLDATE, BILLCL, BILLDUE and
#                                            BILLOVERDUE and the sum of BILLCL; n may be 0.
#      The grammar followed is PB's (see its pb.py): a BILLFIXED holds only BILLDATE and BILLREF; outside it only
#      BILLOP, BILLCL, BILLDUE, BILLOVERDUE and the BILLVCH* scalars; no element is self-closed; text is trimmed.
# Modes:
#   nm.py ANSWER...              the lines above for each answer (an L answer also needs its .headers file beside it).
#   --same-row M0 M1             as PB's: exactly one COMPANY row, BOOK's, equal ALTVCHID and ALTMSTID, else MARKS DIFFER
#                                (exit 2); anything else is STOP.
#   --outcome ID SA SB LA LB     the refusal or the read fetch_party_bills (tally/runtime_party_bills.rs) gives for the
#                                ledger ID's name, the snapshot sent twice (SA, SB), then the report sent twice (LA, LB),
#                                in its order, as one last line OUTCOME: <code> (<why>). PB's, with the party from ID.
#                                Master would not send this name (LedgerScopeName refuses it, ledger_scope.rs:65 and
#                                :74); the line says what master's later steps would make of the answers.
#   --against ID LA B S          for ledger ID: its L answer's rows, the Bills Receivable groups named ID's name in B,
#                                and the snapshot's CLOSINGBALANCE for it in S; then EQUAL or DIFFER for the rows against
#                                the groups (BILLREF, BILLDATE, BILLCL, BILLDUE, BILLOVERDUE), whether the bill is the
#                                one this ledger's invoice made (BP/26-27/<its number>), and the sum against the closing.
#                                It records and never decides: it exits 0 whether they agree or not.             (NM)
#   --same A B                   the sizes and sha256 of two answers of this row, and EQUAL or DIFFER. A missing file is
#                                0 bytes. It records and never decides: it exits 0 whether they are equal or not.   (NM)
# In an M or S answer a character XML forbids, raw or as a reference, is kept as U+FFFD, as master's sanitizer keeps it.
# STOP, exit 1, never "0 rows", for an answer this script cannot read: an M, S or B answer that does not decode as
# UTF-16LE or parse, carries a LINEERROR, has no BODY/DATA/COLLECTION (M, S), or (S) has two rows with one name, a row
# whose BRIDGECOMPANYGUID is not BOOK, or a named row without one of its fields; a B answer whose root is not ENVELOPE
# or that holds an element this script does not know; an L answer that does not decode as UTF-16LE, or holds a
# character XML forbids; an L answer without its .headers file. An L answer that does not parse is ANSWER: MALFORMED,
# as master refuses it.
# Master's own parsers are run on the answers when they become fixtures, and that run is the one that counts.
# Example: python3 -I nm.py --outcome a-c1sa-S-snap-20260401-20260801.xml a-c1sb-S-snap-20260401-20260801.xml
#            a-c1la-L-lo-hy-20260801.xml a-c1lb-L-lo-hy-20260801.xml
import calendar, hashlib, pathlib, re, sys
from decimal import Decimal
import xml.etree.ElementTree as ET
BOOK = "6b43e498-430c-4d5c-bfef-d32e2ab93c85"  # BRIDGE PILOT LAB, as pilot-lab-register-book-extent reads it
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

NAMES = {}  # NM: id -> (invoice number, name), in names.tsv order; the names are never trimmed (C4)
for line in (pathlib.Path(__file__).resolve().parent / "names.tsv").read_text("utf-8").split("\n"):
    if line:
        ident, number, name = line.split("\t")
        NAMES[ident] = (number, name)
BILL_TAGS = ("BILLREF", "BILLDATE", "BILLCL", "BILLDUE", "BILLOVERDUE")  # NM: what is compared L against B

def kind(path):
    match = re.match(r"a-[^-]+-([MSBL])-", pathlib.Path(path).name)
    if not match:
        stop(f"{path}: not an answer name a-<label>-<M|S|B|L>-<request>")
    return match.group(1)

def ident_of(path):  # NM: the ledger an L answer is for, from its request's name
    match = re.match(r"a-[^-]+-L-lo-([a-z]+)-[0-9]{8}\.xml$", pathlib.Path(path).name)
    if not match or match.group(1) not in NAMES:
        stop(f"{path}: not an answer name a-<label>-L-lo-<id>-<YYYYMMDD>.xml with an id of names.tsv")
    return match.group(1)

def snapshot(path, show=True):
    """NM: {id: the ledger's fields or None} for the eleven names; PB's checks on every row."""
    rows = collection(path).findall("LEDGER")
    found = {ident: None for ident in NAMES}
    by_name = {name: ident for ident, (_, name) in NAMES.items()}
    for row in rows:
        guid = leaf(row, "BRIDGECOMPANYGUID")
        if guid is None or guid.strip().lower() != BOOK:
            stop(f"{path}: a LEDGER row's BRIDGECOMPANYGUID {guid!r} is not the book's")
        ident = by_name.get(row.get("NAME"))
        if ident is not None:
            if found[ident] is not None:
                stop(f"{path}: two rows named {row.get('NAME')!r}")
            found[ident] = {tag: leaf(row, tag) for tag in SNAP_FIELDS}
            missing = [tag for tag, value in found[ident].items() if value is None]
            if missing:
                stop(f"{path}: {row.get('NAME')!r} has no {', '.join(missing)}")
    if show:
        say(f"  LEDGER rows: {len(rows)}")
        for ident, fields in found.items():
            name = NAMES[ident][1]
            if fields is None:
                say(f"  {ident} {name!r}: no row")
                continue
            closing = fields["CLOSINGBALANCE"].strip()
            say(f"  {ident} {name!r}: " + "; ".join(f"{tag} {value!r}" for tag, value in fields.items()))
            say("    CLOSINGBALANCE reads " + ("EMPTY (master: None, not zero)" if closing == ""
                else f"{Decimal(closing)}" if PLAIN.fullmatch(closing) else "OTHER"))
    return found

def bills(path, show=True):
    """NM: the groups of a Bills Receivable answer, as dicts; STOP on anything this script does not know."""
    text = FORBIDDEN.sub("�", decode(path))
    try:
        root = ET.fromstring(text)
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")
    if root.tag.upper() != "ENVELOPE":
        stop(f"{path}: the root is {root.tag}, not ENVELOPE")
    groups = []
    for child in root:
        tag = child.tag.upper()
        if tag == "BILLFIXED":
            group = {}
            for part in child:
                name = part.tag.upper()
                if name not in ("BILLDATE", "BILLREF", "BILLPARTY") or len(part) or name in group:
                    stop(f"{path}: {part.tag} in a BILLFIXED, group {len(groups) + 1}: not a known element, or twice")
                group[name] = (part.text or "") if name == "BILLPARTY" else (part.text or "").strip()
            groups.append(group)
        elif tag in ("BILLCL", "BILLDUE", "BILLOVERDUE"):
            if not groups or tag in groups[-1] or len(child):
                stop(f"{path}: {child.tag} before a BILLFIXED, twice, or with content, group {len(groups)}")
            groups[-1][tag] = (child.text or "").strip()
        else:
            stop(f"{path}: {child.tag} is not an element of the Bills Receivable grammar")
    for index, group in enumerate(groups, 1):
        missing = [tag for tag in BILL_TAGS + ("BILLPARTY",) if tag not in group]
        if missing:
            stop(f"{path}: group {index} has no {', '.join(missing)}")
    if show:
        say(f"  BILLFIXED groups: {len(groups)}")
        named = set()
        for ident, (_, name) in NAMES.items():
            mine = [g for g in groups if g["BILLPARTY"] == name]
            named.update(id(g) for g in mine)
            say(f"  {ident} {name!r}: {len(mine)} groups")
            for g in mine:
                say("    " + "; ".join(f"{tag} {g[tag]!r}" for tag in BILL_TAGS))
        others = [g for g in groups if id(g) not in named]
        say(f"  groups of any other party: {len(others)}")
        for g in others[:20]:
            say(f"    {g['BILLPARTY']!r}: " + "; ".join(f"{tag} {g[tag]!r}" for tag in BILL_TAGS))
        if len(others) > 20:
            say(f"    ... {len(others) - 20} more not printed")
    return groups

def outcome(ident, sa_path, sb_path, la_path, lb_path):  # PB's, with the party from ident
    name = NAMES[ident][1]
    def held(path):
        return pathlib.Path(path).read_bytes() if pathlib.Path(path).exists() else b""
    for path in (sa_path, sb_path):
        refused = transport(path)
        if refused:
            return refused
    if held(sa_path) != held(sb_path):
        return "native_ledger_collection_changed (the two snapshot answers are not byte-equal)"
    for path in (la_path, lb_path):
        refused = transport(path)
        if refused:
            return refused
    if held(la_path) != held(lb_path):
        return "party_bills_drifted (the two answers are not byte-equal)"
    klass, rows = answer(la_path)
    say(f"ANSWER: {klass}")
    party = snapshot(sa_path, show=False)[ident]
    if party is None:
        return "party_not_in_book (no snapshot row named it)"
    say(f"  snapshot row: " + "; ".join(f"{tag} {value!r}" for tag, value in party.items()))
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

def against(ident, la_path, b_path, s_path):  # NM
    number, name = NAMES[ident]
    klass, rows = answer(la_path)
    say(f"ANSWER: {klass}")
    groups = [g for g in bills(b_path, show=False) if g["BILLPARTY"] == name]
    say(f"  Bills Receivable groups named {name!r}: {len(groups)}")
    for g in groups:
        say("    " + "; ".join(f"{tag} {g[tag]!r}" for tag in BILL_TAGS))
    party = snapshot(s_path, show=False)[ident]
    closing = None if party is None else party["CLOSINGBALANCE"].strip()
    say(f"  snapshot CLOSINGBALANCE: {'no row' if party is None else repr(closing)}")
    if rows is None:
        say(f"AGAINST: DIFFER (the L answer has no rows: {klass})")
        return
    mine = sorted(tuple(row[tag] for tag in BILL_TAGS) for row in rows)
    theirs = sorted(tuple(g[tag] for tag in BILL_TAGS) for g in groups)
    own = {row["BILLREF"] for row in rows} == {f"BP/26-27/{number}"}
    total = sum((Decimal(row["BILLCL"]) for row in rows), Decimal(0))
    tied = closing is not None and PLAIN.fullmatch(closing or "") and total == Decimal(closing)
    say(f"  rows {len(rows)}, groups {len(groups)}; the one bill is this ledger's own (BP/26-27/{number}): "
        f"{'yes' if own else 'NO'}; sum of BILLCL {total}, " + ("equals the closing" if tied else "does not equal the closing"))
    say(f"AGAINST: {'EQUAL' if mine == theirs else 'DIFFER'} (L rows against the groups)")

def same(first, second):  # NM
    def held(path):
        return pathlib.Path(path).read_bytes() if pathlib.Path(path).exists() else b""
    a, b = held(first), held(second)
    for path, raw in ((first, a), (second, b)):
        say(f"  {path}: {len(raw)} bytes, sha256 {hashlib.sha256(raw).hexdigest()}")
    say("SAME: " + ("EQUAL" if a == b else "DIFFER"))

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
    if len(args) != 6 or [kind(a) for a in args[2:]] != ["S", "S", "L", "L"] or args[1] not in NAMES:
        stop("--outcome takes an id of names.tsv, the two S answers, then the two L answers")
    if ident_of(args[4]) != args[1] or ident_of(args[5]) != args[1]:
        stop("--outcome: each L answer must be for the id named")
    print(f"OUTCOME: {outcome(args[1], args[2], args[3], args[4], args[5])}")
elif args[0] == "--against":
    if len(args) != 5 or args[1] not in NAMES or [kind(a) for a in args[2:]] != ["L", "B", "S"]:
        stop("--against takes an id of names.tsv, an L answer, the B answer and the S answer")
    if ident_of(args[2]) != args[1]:
        stop("--against: the L answer must be for the id named")
    against(args[1], args[2], args[3], args[4])
elif args[0] == "--same":
    if len(args) != 3:
        stop("--same takes two answers")
    same(args[1], args[2])
else:
    for path in args:
        k = kind(path)
        say(f"== {path}")
        if k == "M":
            marks(path)
        elif k == "S":
            snapshot(path)
        elif k == "B":
            bills(path)
        else:
            ident_of(path)
            headers(path)
            say(f"ANSWER: {answer(path)[0]}")
