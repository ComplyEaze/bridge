#!/usr/bin/env python3
"""Renders request r2 on the lab PC: a Create of SC Copied Goods carrying the keyed item's own GST details.

  python3 -I make_r2.py ITEMS-ANSWER.xml

ITEMS-ANSWER is step 2's answer to requests/items.utf16le.xml, as received. From the one STOCKITEM row
named SC Sibling Goods it copies, as the answer's own characters (TYPE attributes, spacing and the &#4;
references included), the elements PARENT, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, every
GSTDETAILS.LIST and every HSNDETAILS.LIST, and puts them in the envelope r1 uses
(requests/r1-name-unit.utf16le.xml), in place of r1's message. Nothing else is taken from the answer.

It writes requests/r2-copied.utf16le.xml and prints what it copied and the file's sha256. It refuses,
writing nothing, when the row is missing or repeated, PARENT or BASEUNITS is missing, any scalar element
is repeated, or no GSTDETAILS.LIST or HSNDETAILS.LIST is present. GSTAPPLICABLE or GSTTYPEOFSUPPLY
missing is printed, not refused: whether Tally answers them for a stock item is part of what step 2 shows.
"""
import hashlib, pathlib, re, sys

SOURCE = 'SC Sibling Goods'
TARGET = 'SC Copied Goods'
SCALARS = ('PARENT', 'BASEUNITS', 'GSTAPPLICABLE', 'GSTTYPEOFSUPPLY')
LISTS = ('GSTDETAILS.LIST', 'HSNDETAILS.LIST')
here = pathlib.Path(__file__).resolve().parent


def refuse(why):
    sys.exit(f'not rendered: {why}')


def text(path):
    raw = pathlib.Path(path).read_bytes()
    return raw[2:].decode('utf-16-le') if raw[:2] == b'\xff\xfe' else raw.decode('utf-16-le')


def spans(row, tag):
    """Each element of this tag at the row's top level, as its own characters."""
    found, depth, pos = [], 0, 0
    opener = re.compile(r'<(/?)([A-Z0-9._]+)\b[^>]*?(/?)>')
    start = None
    for m in opener.finditer(row):
        closing, name, empty = m.group(1), m.group(2), m.group(3)
        if empty:
            if depth == 1 and name == tag:
                found.append(row[m.start():m.end()])
            continue
        if not closing:
            depth += 1
            if depth == 2 and name == tag:
                start = m.start()
        else:
            if depth == 2 and name == tag and start is not None:
                found.append(row[start:m.end()])
                start = None
            depth -= 1
    return found


if len(sys.argv) != 2:
    sys.exit(__doc__)
out = here / 'requests' / 'r2-copied.utf16le.xml'
if out.exists():
    refuse(f'{out.name} exists already; a write file is rendered once')
answer = text(sys.argv[1])
rows = [m for m in re.finditer(r'<STOCKITEM NAME="([^"]*)"[^>]*>.*?</STOCKITEM>', answer, re.S)
        if m.group(1) == SOURCE]
if len(rows) != 1:
    refuse(f'{len(rows)} rows named {SOURCE}')
row = rows[0].group(0)

copied = []
for tag in SCALARS:
    found = spans(row, tag)
    if len(found) > 1:
        refuse(f'{tag} appears {len(found)} times')
    if not found:
        if tag in ('PARENT', 'BASEUNITS'):
            refuse(f'{tag} is missing')
        print(f'{tag}: not in the answer, so not sent')
        continue
    copied.append(found[0])
    print(f'{tag}: copied, {len(found[0])} characters')
for tag in LISTS:
    found = spans(row, tag)
    if not found:
        refuse(f'no {tag}')
    copied.extend(found)
    print(f'{tag}: {len(found)} copied, {sum(map(len, found))} characters')

raw = (here / 'requests' / 'r1-name-unit.utf16le.xml').read_bytes()
envelope = raw[2:].decode('utf-16-le')
head, rest = envelope.split('<REQUESTDATA>')
_, tail = rest.split('</REQUESTDATA>')
message = (f'<TALLYMESSAGE><STOCKITEM NAME="{TARGET}" ACTION="Create"><NAME>{TARGET}</NAME>'
           + ''.join(copied) + '</STOCKITEM></TALLYMESSAGE>')
body = b'\xff\xfe' + f'{head}<REQUESTDATA>{message}</REQUESTDATA>{tail}'.encode('utf-16-le')
out.write_bytes(body)
print(f'{out.name}: {len(body)} bytes, sha256 {hashlib.sha256(body).hexdigest()}')
