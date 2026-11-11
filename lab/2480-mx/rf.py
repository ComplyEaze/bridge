#!/usr/bin/env python3
# rf.py: row 2480-NX's refusal check. New for this row. st.py --import (NS's, byte-equal) passes only CREATED ONE;
# this script decides, for an import answer st.py did not pass, whether Tally refused the import and changed nothing.
# It reads the answer by an XML parser (LAB-COMMON C4), never a text search, prints never more than CAP lines per call
# (AGENTS.md P5), and STOPs (exit 1) on an answer it cannot read, never "nothing found".
# Mode:
#   rf.py ANSWER   an import answer (an a-*-LM-* or a-*-I-* file). IMPORT: REFUSED (exit 0) only if all hold:
#                  - the answer holds exactly one RESPONSE;
#                  - it carries each of st.py's COUNTERS exactly once, as a whole number;
#                  - each of CHANGE, and IGNORED, is 0 (an IGNORED above 0 may be Tally matching another ledger);
#                  - ERRORS or EXCEPTIONS is above 0.
#                  Anything else is IMPORT: NOT A REFUSAL (exit 1): a counter missing, twice or not a whole number;
#                  any of CHANGE or IGNORED above 0; or neither ERRORS nor EXCEPTIONS above 0, a LINEERROR alone
#                  included. ROW.md makes that a STOP.
#                  A refusal is still proved only by the marks read after it (ROW.md, step 4 (e)), which must equal
#                  the marks read before it.
# Each LINEERROR is printed whole, as Tally sent it.
# Example: python3 -I rf.py a-s4cm-LM-cm.xml
import pathlib, re, sys
import xml.etree.ElementTree as ET
CAP = 200
COUNTERS = ("CREATED", "ALTERED", "DELETED", "LASTVCHID", "LASTMID", "COMBINED", "IGNORED", "ERRORS", "CANCELLED",
            "EXCEPTIONS")  # st.py's
CHANGE = ("CREATED", "ALTERED", "DELETED", "COMBINED", "CANCELLED")
SIGNS = ("ERRORS", "EXCEPTIONS")
# st.py's and nm.py's: a character XML forbids, raw or as a reference, kept as U+FFFD, as master's sanitizer keeps it.
FORBIDDEN = re.compile(r"[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bcef]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));", re.I)
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

def root_of(path):
    raw = pathlib.Path(path).read_bytes() if pathlib.Path(path).is_file() else b""
    if not raw:
        stop(f"{path} is missing or empty")
    if raw[:2] == b"\xff\xfe":
        raw = raw[2:]
    try:
        text = raw.decode("utf-16-le")
    except UnicodeDecodeError as error:
        stop(f"{path} does not decode as UTF-16LE: {error}")
    try:
        return ET.fromstring(FORBIDDEN.sub("�", text))
    except ET.ParseError as error:
        stop(f"{path} does not parse: {error}")

def whole(found):
    """The counter's value, or None if it is absent, there twice, or not a whole number."""
    text = (found[0].text or "").strip() if len(found) == 1 else ""
    return int(text) if text.isdigit() and text.isascii() else None

args = sys.argv[1:]
if len(args) != 1:
    stop("rf.py takes one ANSWER")
path = args[0]
root = root_of(path)
responses = [root] if root.tag == "RESPONSE" else list(root.iter("RESPONSE"))
errors = [" ".join((e.text or "").split()) for e in root.iter("LINEERROR")]
if len(responses) != 1:
    stop(f"{path}: {len(responses)} RESPONSE elements, not one")
read = {tag: whole(responses[0].findall(tag)) for tag in COUNTERS}
say(f"{path}: " + ", ".join(f"{tag} {read[tag]}" for tag in COUNTERS) + f", LINEERROR {len(errors)}")
for text in errors:
    say(f"  LINEERROR: {text}")
wrong = [f"{tag} missing, twice or not a whole number" for tag in COUNTERS if read[tag] is None]
wrong += [f"{tag} {read[tag]}" for tag in CHANGE + ("IGNORED",) if read[tag] not in (None, 0)]
if not wrong and not any(read[tag] > 0 for tag in SIGNS):
    wrong.append("neither ERRORS nor EXCEPTIONS above 0")
if wrong:
    say("IMPORT: NOT A REFUSAL (" + "; ".join(wrong) + ")")
    raise SystemExit(1)
say("IMPORT: REFUSED")
