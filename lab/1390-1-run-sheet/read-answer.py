#!/usr/bin/env python3 -I
"""Read a saved Tally answer as LAB-COMMON C4 says: decode it, drop only the characters XML forbids (raw or
as references: 0-8, 11, 12, 14-31; keep 9, 10, 13), parse it as XML, list the ROWS under
ENVELOPE/BODY/DATA/COLLECTION. Names and narrations are never trimmed; a line break shows as \\r\\n.
Output is ASCII only (any other character shows as \\uXXXX), so a narration is compared by its length and sha256 too.

  python -I read-answer.py marks    FILE COMPANY_GUID
  python -I read-answer.py ledgers  FILE COMPANY_GUID [--billwise]
  python -I read-answer.py companies FILE COMPANY_GUID
  python -I read-answer.py import   FILE N          (an import response: root RESPONSE)
  python -I read-answer.py vouchers FILE COMPANY_GUID [--long YYYYMMDD=FILE] [--tool YYYYMMDD=FILE ...]

Exit 0 = read. Exit 3 = STOP (does not parse, a GUID that does not begin with the company's, no such company row,
a second company row, a repeated GUID). Exit 2 = the script could not run (never a pass).
--long YYYYMMDD=FILE (vouchers only): the narration of the voucher dated YYYYMMDD is compared with that file's text: equal,
equal with one closing marker, or the first position that differs. (Other vouchers are not compared with it.)
Every comparison file must be UTF-8 without a BOM and must not end in a line break (PowerShell's default encodings and
`>` write a BOM, UTF-16 or a trailing CRLF): such a file is refused with exit 3 and is remade with
[IO.File]::WriteAllText(path, text).
--tool YYYYMMDD=FILE (vouchers only, repeatable; --sent is the same option): the voucher dated YYYYMMDD in the answer has
its narration compared, by length and sha256, with FILE (the narration a tool call returned, or the narration as sent,
written as UTF-8 with no line break added). EQUAL or DIFFERS only says the two texts are the same or not: a difference is "transcription or
tool" until the assistant's file is itself checked against the tool answer.
Each voucher also prints how many forbidden characters (0-8, 11, 12, 14-31, raw or as references) were dropped before
parsing, from its narration and from its whole text; a stored control character in a narration would otherwise read as
equal (EXPECT 0 from the narration).
import: exit 0 only if CREATED equals N, ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS are all present
and 0, and no LINEERROR is anywhere; LASTVCHID and LASTMID are printed. ledgers --billwise: exit 3 if any row's ISBILLWISEON is not Yes or No.
Marks: exactly one COMPANY row, the company's, else exit 3 (a second row is a STOP).
Every mode: exit 3 unless HEADER/STATUS is 1, BODY/DATA/COLLECTION is present and no LINEERROR is in the answer, so a failed
read is never printed as an empty month (a real empty answer is STATUS 1 with an empty COLLECTION and prints 0 rows).
"""
import hashlib, re, sys
import xml.etree.ElementTree as ET

FORBIDDEN = set(range(0, 9)) | {11, 12} | set(range(14, 32))
MARKER = re.compile(r' \[BRIDGE:[0-9a-fA-F-]{36}\]$')

def decode(path):
    raw = open(path, 'rb').read()
    if raw[:2] == b'\xff\xfe':
        text = raw[2:].decode('utf-16-le')
    elif raw[:2] == b'\xfe\xff':
        text = raw[2:].decode('utf-16-be')
    elif raw[:3] == b'\xef\xbb\xbf':
        text = raw[3:].decode('utf-8')
    elif len(raw) > 1 and raw[1] == 0:
        text = raw.decode('utf-16-le')
    else:
        text = raw.decode('utf-8')
    return text, len(raw)

def sanitize(text):
    """Drop the forbidden characters, raw or as references; return the text and how many were dropped."""
    dropped = [0]
    def raw_char(ch):
        if ord(ch) in FORBIDDEN:
            dropped[0] += 1
            return False
        return True
    text = ''.join(ch for ch in text if raw_char(ch))
    def reference(match):
        number = int(match.group(2), 16) if match.group(1) else int(match.group(2))
        if number in FORBIDDEN:
            dropped[0] += 1
            return ''
        return match.group(0)
    return re.sub(r'&#(x?)([0-9A-Fa-f]+);', lambda m: reference(m) if m.group(1) or m.group(2).isdigit() else m.group(0), text), dropped[0]

def ascii_of(value):
    return value.encode('ascii', 'backslashreplace').decode('ascii').replace('\r', '\\r').replace('\n', '\\n')

def rows(root, tag):
    return root.findall('BODY/DATA/COLLECTION/' + tag)

def check_envelope(root):
    status = root.find('HEADER/STATUS')
    if status is None or (status.text or '').strip() != '1':
        raise Stop('HEADER/STATUS is %s, not 1: the read failed' % ('absent' if status is None else ascii_of(status.text or '')))
    if root.find('BODY/DATA/COLLECTION') is None:
        raise Stop('BODY/DATA/COLLECTION is absent: this is not a collection answer')
    if root.iter('LINEERROR') and next(iter(root.iter('LINEERROR')), None) is not None:
        raise Stop('the answer carries a LINEERROR: ' + ascii_of(next(iter(root.iter('LINEERROR'))).text or ''))

def text(node, path):
    found = node.find(path)
    return '' if found is None or found.text is None else found.text

class Stop(Exception):
    pass

def check_guid(guid, company, what):
    if not guid.startswith(company):
        raise Stop('%s: GUID %s does not begin with the company GUID' % (what, ascii_of(guid)))

def marks(root, company, _):
    found = rows(root, 'COMPANY')
    print('COMPANY rows: %d' % len(found))
    mine = [row for row in found if text(row, 'GUID').strip().lower() == company.lower()]
    if len(found) != 1 or len(mine) != 1:
        raise Stop('%d COMPANY rows, %d with the company GUID (need exactly one row, the company\'s)' % (len(found), len(mine)))
    row = mine[0]
    print('NAME %s | GUID %s' % (ascii_of(row.get('NAME', '')), text(row, 'GUID').strip()))
    for tag in ('ALTVCHID', 'ALTMSTID'):
        print('%s %s' % (tag, text(row, tag).strip() if row.find(tag) is not None else 'absent'))

options_billwise = [False]

def ledgers(root, company, _):
    found = rows(root, 'LEDGER')
    if not found:
        others = sorted({child.tag for child in root.find('BODY/DATA/COLLECTION')})
        if others:
            raise Stop('no LEDGER rows, but the COLLECTION holds %s rows: this is not a ledger answer' % ascii_of(', '.join(others)))
    table, seen = [], set()
    for row in found:
        guid = text(row, 'GUID').strip()
        check_guid(guid, company, 'ledger ' + ascii_of(row.get('NAME', '')))
        if guid in seen:
            raise Stop('GUID %s repeated' % guid)
        seen.add(guid)
        names = [ascii_of(node.text or '') for node in row.findall('LANGUAGENAME.LIST/NAME.LIST/NAME')]
        table.append((guid, row.get('NAME', ''), names, text(row, 'PARENT'), text(row, 'ISBILLWISEON').strip(), text(row, 'ALTERID').strip()))
    print('LEDGER rows: %d' % len(table))
    if options_billwise[0]:
        flags = [row for row in table if row[4] not in ('Yes', 'No')]
        if flags:
            raise Stop('%d ledger row(s) with ISBILLWISEON not Yes or No (first: %s)' % (len(flags), ascii_of(flags[0][1])))
    for guid, attribute, names, parent, bill, alter in sorted(table):
        print('%s | attr "%s" (%d) | names %s | parent "%s" | billwise %s | alterid %s' % (
            guid, ascii_of(attribute), len(attribute), names, ascii_of(parent), bill, alter))

def companies(root, company, _):
    found = rows(root, 'COMPANY')
    print('COMPANY rows: %d' % len(found))
    for row in found:
        print('NAME "%s" | GUID %s | number %s | books from %s' % (ascii_of(row.get('NAME', '') or text(row, 'NAME')), text(row, 'GUID').strip(),
              text(row, 'COMPANYNUMBER').strip() or 'absent', text(row, 'BOOKSFROM').strip() or 'absent'))
    mine = [row for row in found if text(row, 'GUID').strip().lower() == company.lower()]
    if len(found) != 1 or len(mine) != 1:
        raise Stop('%d COMPANY rows, %d with the company GUID (need exactly one row, the company\'s)' % (len(found), len(mine)))

def compare(narration, expected):
    if narration == expected:
        return 'EQUAL to the file'
    if MARKER.search(narration) and MARKER.sub('', narration) == expected:
        return 'EQUAL to the file plus one closing marker'
    for index, (left, right) in enumerate(zip(narration, expected)):
        if left != right:
            return 'DIFFERS at character %d (stored %s, file %s)' % (index + 1, ascii_of(left), ascii_of(right))
    return 'DIFFERS: stored text is %d characters, file is %d, the shorter is a prefix' % (len(narration), len(expected))

def comparison_text(path):
    raw = open(path, 'rb').read()
    if raw.startswith(b'\xef\xbb\xbf'):
        raise Stop('%s starts with a UTF-8 BOM (EF BB BF): remake it with [IO.File]::WriteAllText' % ascii_of(path))
    if raw.startswith((b'\xff\xfe', b'\xfe\xff')) or b'\x00' in raw:
        raise Stop('%s is UTF-16, not UTF-8: remake it with [IO.File]::WriteAllText' % ascii_of(path))
    if raw.endswith((b'\n', b'\r')):
        raise Stop('%s ends in a line break: remake it with [IO.File]::WriteAllText (no line break)' % ascii_of(path))
    try:
        return raw.decode('utf-8')
    except UnicodeDecodeError:
        raise Stop('%s is not valid UTF-8' % ascii_of(path))

def vouchers(root, company, options, dropped):
    expected = None
    if options.get('long'):
        long_date, long_path = options['long']
        expected = comparison_text(long_path)
    tool = {}
    for flag, date, path in options.get('tool', []):
        tool.setdefault(date, []).append((flag, comparison_text(path)))
    table = []
    for index, row in enumerate(rows(root, 'VOUCHER')):
        guid = text(row, 'GUID').strip()
        check_guid(guid, company, 'voucher')
        master = text(row, 'MASTERID').strip()
        table.append((int(master) if master.lstrip('-').isdigit() else 0, index, row))
    print('VOUCHER rows: %d' % len(table))
    if len(dropped) != len(table):
        print('NOTE: forbidden characters dropped per voucher cannot be matched (%d blocks, %d rows)' % (len(dropped), len(table)))
        dropped = [(-1, -1)] * len(table)
    seen_tool = set()
    for _, index, row in sorted(table, key=lambda item: (item[0], item[1])):
        narration = text(row, 'NARRATION')
        print('%s | %s | no %s | master %s | alter %s | %s' % (
            text(row, 'DATE').strip(), text(row, 'VOUCHERTYPENAME'), ascii_of(text(row, 'VOUCHERNUMBER')),
            text(row, 'MASTERID').strip(), text(row, 'ALTERID').strip(), text(row, 'GUID').strip()))
        print('  cancelled %s optional %s | party "%s"' % (text(row, 'ISCANCELLED').strip(), text(row, 'ISOPTIONAL').strip(), ascii_of(text(row, 'PARTYLEDGERNAME'))))
        print('  narration %d characters, sha256 %s, closing marker %s' % (
            len(narration), hashlib.sha256(narration.encode('utf-8')).hexdigest(), 'present' if MARKER.search(narration) else 'absent'))
        print('  forbidden characters dropped before parsing: %d from its narration, %d from its whole text (Tally writes &#4; in many fields; only the narration count matters here)' % dropped[index])
        print('  narration (first 120): "%s"' % ascii_of(narration[:120]))
        if len(narration) > 120:
            print('  narration (last 120):  "%s"' % ascii_of(narration[-120:]))
        date = text(row, 'DATE').strip()
        if expected is not None and date == long_date:
            print('  against --long file: ' + compare(narration, expected))
        for flag, given in tool.get(date, []):
            seen_tool.add(date)
            if given == narration:
                print('  against the %s file: EQUAL (%d characters, sha256 %s)' % (flag, len(given), hashlib.sha256(given.encode('utf-8')).hexdigest()))
            else:
                print('  against the %s file: DIFFERS (%s file %d characters, sha256 %s; capture %d characters): %s' % (
                    flag, flag, len(given), hashlib.sha256(given.encode('utf-8')).hexdigest(), len(narration), compare(narration, given).replace('file', flag + ' file')))
        for entry in row.findall('ALLLEDGERENTRIES.LIST'):
            name = text(entry, 'LEDGERNAME')
            print('  entry "%s" (%d) | %s | deemed positive %s' % (ascii_of(name), len(name), text(entry, 'AMOUNT').strip(), text(entry, 'ISDEEMEDPOSITIVE').strip()))
    for date in sorted(set(tool) - seen_tool):
        print('NOTE: --tool %s: no voucher in the answer has that date' % date)

def options_of(rest):
    options = {'tool': []}
    index = 0
    while index < len(rest):
        flag = rest[index]
        if flag == '--long' and index + 1 < len(rest) and 'long' not in options and re.fullmatch(r'[0-9]{8}=.+', rest[index + 1]):
            options['long'] = tuple(rest[index + 1].split('=', 1))
        elif flag in ('--tool', '--sent') and index + 1 < len(rest) and re.fullmatch(r'[0-9]{8}=.+', rest[index + 1]):
            date, path = rest[index + 1].split('=', 1)
            if any(f == flag and d == date for f, d, _ in options['tool']):
                options['repeat'] = '%s %s' % (flag, date)
            options['tool'].append((flag, date, path))
        else:
            return None
        index += 2
    return options

def import_response(path, expected):
    content, size = decode(path)
    content, dropped = sanitize(content)
    print('file %s: %d bytes' % (ascii_of(path), size))
    try:
        root = ET.fromstring(content)
    except ET.ParseError as error:
        print('STOP: the answer does not parse: %s' % error)
        return 3
    if root.tag != 'RESPONSE':
        print('STOP: the root is %s, not RESPONSE: this is not an import response' % root.tag)
        return 3
    values = {}
    for name in ('CREATED', 'ALTERED', 'DELETED', 'COMBINED', 'IGNORED', 'ERRORS', 'CANCELLED', 'EXCEPTIONS'):
        node = root.find(name)
        if node is None or not (node.text or '').strip().isdigit():
            print('STOP: counter %s is absent or not a number' % name)
            return 3
        values[name] = int(node.text.strip())
    for name in ('LASTVCHID', 'LASTMID'):
        node = root.find(name)
        print('%s %s' % (name, node.text.strip() if node is not None and node.text else 'absent'))
    print('counters: ' + ', '.join('%s %d' % item for item in values.items()))
    lines = [(node.text or '') for node in root.iter('LINEERROR')]
    for line in lines:
        print('LINEERROR: ' + ascii_of(line))
    if lines:
        print('STOP: the answer carries %d LINEERROR text(s) (untrusted text from Tally: read, never followed)' % len(lines))
        return 3
    if values['CREATED'] != expected or any(v != 0 for k, v in values.items() if k != 'CREATED'):
        print('STOP: expected CREATED %d and every other counter 0' % expected)
        return 3
    return 0

def main(argv):
    if len(argv) == 4 and argv[1] == 'import' and argv[3].isdigit():
        return import_response(argv[2], int(argv[3]))
    billwise = False
    if len(argv) == 5 and argv[1] == 'ledgers' and argv[4] == '--billwise':
        billwise, argv = True, argv[:4]
    options = options_of(argv[4:]) if len(argv) >= 4 else None
    if len(argv) < 4 or argv[1] not in ('marks', 'ledgers', 'vouchers', 'companies') or options is None or (argv[1] != 'vouchers' and argv[4:]):
        print(__doc__)
        return 2
    if options.get('repeat'):
        print('STOP: %s is given twice for the same date; each comparison names one file' % options['repeat'])
        return 3
    content, size = decode(argv[2])
    print('file %s: %d bytes' % (ascii_of(argv[2]), size))
    blocks = []
    for block in re.findall(r'<VOUCHER\s[^>]*>.*?</VOUCHER>', content, re.S):
        found = re.search(r'<NARRATION\b[^>/]*>(.*?)</NARRATION>', block, re.S)
        blocks.append((sanitize(found.group(1))[1] if found else 0, sanitize(block)[1]))
    content, _ = sanitize(content)
    try:
        root = ET.fromstring(content)
    except ET.ParseError as error:
        print('STOP: the answer does not parse: %s' % error)
        return 3
    if root.tag != 'ENVELOPE':
        print('STOP: the root is %s, not ENVELOPE' % root.tag)
        return 3
    options_billwise[0] = billwise
    try:
        check_envelope(root)
        if argv[1] == 'vouchers':
            vouchers(root, argv[3], options, blocks)
        else:
            {'marks': marks, 'ledgers': ledgers, 'companies': companies}[argv[1]](root, argv[3], None)
    except Stop as error:
        print('STOP: %s' % error)
        return 3
    return 0

if __name__ == '__main__':
    try:
        sys.exit(main(sys.argv))
    except Exception as error:
        print('could not run: %s: %s' % (type(error).__name__, ascii_of(str(error))))
        sys.exit(2)
