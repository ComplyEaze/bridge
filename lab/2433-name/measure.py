# Lab helper for item 2397 B1 (not product code). Reads files this sheet's steps saved and prints measurements.
# It never prints a company name, except the N3 pattern, which this sheet wrote and which holds letters only.
# Run with python -I. Exit 0 = measured; exit 3 = STOP (the file is not what the step expects); exit 2 = usage.
#   measure.py pattern                          the N3 name: 30 blocks of one letter repeated 10 times
#   measure.py native <answer.xml> <guid> [--pattern]   a raw Company collection answer (LAB-COMMON C4)
#   measure.py marks <answer.xml> <guid>        a raw marks answer: ALTVCHID and ALTMSTID of the GUID's row
#   measure.py tool <answer.json> [<guid>] [--pattern]  a client.py answer: refusal code, or the company's name
#   measure.py diff <answer-a.json> <answer-b.json>     two client.py answers, compared by structure
import hashlib
import json
import re
import sys
import xml.etree.ElementTree as ET

BOUND = 2_000_000
LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcd'
PATTERN = ''.join(letter * 10 for letter in LETTERS)
# Keys whose values differ between two runs of the same call: times, durations, process and request ids.
VOLATILE = {'id', 'read_at', 'duration_ms', 'call_sent_utc', 'round_trip_seconds', 'server_pid', 'server_info',
            'build', 'snapshot_id', 'today'}
CODE = re.compile(r'^[a-z][a-z0-9_]{0,79}$')
PLAIN_KEY = re.compile(r'^[A-Za-z0-9_]{1,64}$')
FORBIDDEN_RAW = re.compile('[\x00-\x08\x0b\x0c\x0e-\x1f]')
FORBIDDEN_REF = re.compile(r'&#(?:x0*(?:[0-8bBcCeEfF]|1[0-9a-fA-F])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));')


def stop(message):
    print(f'STOP: {message}')
    sys.exit(3)


def read_bounded(path):
    with open(path, 'rb') as f:
        data = f.read(BOUND + 1)
    if len(data) > BOUND:
        stop(f'{path} is above {BOUND} bytes')
    if not data:
        stop(f'{path} is empty')
    return data


def text_facts(label, value):
    raw = value.encode('utf-8')
    controls = sum(1 for ch in value if ch < ' ' or '\x7f' <= ch <= '\x9f')
    edge = value != value.strip()
    print(f'{label}: chars {len(value)}, utf8_bytes {len(raw)}, sha256 {hashlib.sha256(raw).hexdigest()}, '
          f'control_chars {controls}, leading_or_trailing_space {"yes" if edge else "no"}')


def pattern_facts(label, value):
    if not re.fullmatch(r'[A-Za-z]*', value):
        print(f'{label}: not letters only; not compared with the pattern, not printed')
        return
    common = 0
    while common < min(len(value), len(PATTERN)) and value[common] == PATTERN[common]:
        common += 1
    if common == len(value) and value:
        block, within = divmod(len(value) - 1, 10)
        print(f'{label}: a prefix of the pattern, {len(value)} of {len(PATTERN)} characters; ends in block '
              f'{block + 1} ({LETTERS[block]}), character {within + 1} of 10')
    else:
        print(f'{label}: NOT a prefix of the pattern; first difference at character {common + 1}')
    print(f'{label} as stored: {value}')


def parse_xml_answer(path):
    data = read_bounded(path)
    if data[:2] == b'\xff\xfe':
        data = data[2:]
    try:
        text = data.decode('utf-16-le')
    except UnicodeDecodeError:
        stop(f'{path} is not UTF-16LE')
    text = FORBIDDEN_REF.sub('', FORBIDDEN_RAW.sub('', text))
    try:
        return ET.fromstring(text)
    except ET.ParseError as error:
        stop(f'{path} does not parse as XML ({error})')


def collection_rows(root, path):
    if root.tag.upper() != 'ENVELOPE':
        stop(f'{path}: the root is not ENVELOPE')
    collections = [c for body in root if body.tag.upper() == 'BODY'
                   for data in body if data.tag.upper() == 'DATA'
                   for c in data if c.tag.upper() == 'COLLECTION']
    if len(collections) != 1:
        stop(f'{path}: {len(collections)} BODY/DATA/COLLECTION elements, not 1 (the request failed)')
    return [row for row in collections[0] if row.tag.upper() == 'COMPANY']


def row_for(rows, guid, path):
    print(f'COMPANY rows under BODY/DATA/COLLECTION: {len(rows)}')
    if len(rows) != 1:
        stop(f'{path}: {len(rows)} COMPANY rows; THE COMPANY alone gives 1')
    found = [c.text or '' for c in rows[0] if c.tag.upper() == 'GUID']
    if len(found) != 1 or found[0].strip().lower() != guid.lower():
        stop(f'{path}: the row\'s GUID is not {guid}')
    print(f'GUID: {found[0].strip()} (matches)')
    return rows[0]


def native(path, guid, with_pattern):
    row = row_for(collection_rows(parse_xml_answer(path), path), guid, path)
    attribute = next((v for k, v in row.attrib.items() if k.upper() == 'NAME'), None)
    elements = [c.text or '' for c in row if c.tag.upper() == 'NAME']
    print(f'NAME attribute present: {"yes" if attribute is not None else "no"}; NAME elements: {len(elements)}')
    if attribute is not None:
        text_facts('NAME attribute', attribute)
        if with_pattern:
            pattern_facts('NAME attribute', attribute)
    for i, element in enumerate(elements, 1):
        text_facts(f'NAME element {i}', element)
        if with_pattern:
            pattern_facts(f'NAME element {i}', element)
    if attribute is not None and elements:
        print(f'attribute equal to element 1: {"yes" if attribute == elements[0] else "no"}')


def marks(path, guid):
    row = row_for(collection_rows(parse_xml_answer(path), path), guid, path)
    for tag in ('ALTVCHID', 'ALTMSTID'):
        values = [(c.text or '').strip() for c in row if c.tag.upper() == tag]
        if len(values) != 1 or not values[0].isdigit():
            stop(f'{path}: {tag} is not one number')
        print(f'{tag}: {values[0]}')


def expand(value):
    """A JSON value with every string that holds a JSON object or array parsed in place."""
    if isinstance(value, dict):
        return {k: expand(v) for k, v in value.items()}
    if isinstance(value, list):
        return [expand(v) for v in value]
    if isinstance(value, str) and value[:1] in '{[':
        try:
            return expand(json.loads(value))
        except ValueError:
            return value
    return value


def load_answer(path):
    try:
        body = json.loads(read_bounded(path).decode('utf-8'))
    except ValueError:
        stop(f'{path} is not a client.py answer')
    if 'response' not in body:
        stop(f'{path} has no response')
    return expand(body)


def walk(value, path=()):
    yield path, value
    if isinstance(value, dict):
        for k, v in value.items():
            yield from walk(v, path + (k,))
    elif isinstance(value, list):
        for i, v in enumerate(value):
            yield from walk(v, path + (i,))


def show(path):
    parts = []
    for part in path:
        if isinstance(part, int) or PLAIN_KEY.match(part):
            parts.append(str(part))
        else:
            parts.append('<key ' + hashlib.sha256(part.encode('utf-8')).hexdigest()[:8] + '>')
    return '/' + '/'.join(parts)


def tool(path, guid, with_pattern):
    body = load_answer(path)
    response = body['response']
    print(f'tool: {body.get("tool")}; JSON-RPC error: {"yes" if response.get("error") else "no"}; '
          f'isError: {(response.get("result") or {}).get("isError")}')
    for where, value in walk(response):
        if where and where[-1] in ('code', 'cause', 'refusal_reason', 'reason_code', 'identity_state') \
                and isinstance(value, str) and CODE.match(value):
            print(f'{show(where)}: {value}')
    if guid is None:
        return
    lists = [v for where, v in walk(response) if where and where[-1] == 'companies' and isinstance(v, list)]
    if not lists:
        print('no companies list in this answer')
        return
    rows = [c for c in lists[0] if isinstance(c, dict)]
    print(f'companies listed: {len(rows)}')
    match = [c for c in rows if str(c.get('guid', '')).lower() == guid.lower()]
    if len(match) != 1:
        stop(f'{len(match)} companies with GUID {guid}')
    name = match[0].get('name')
    if not isinstance(name, str):
        stop('the company has no name string')
    text_facts('name', name)
    if with_pattern:
        pattern_facts('name', name)


def diff(path_a, path_b):
    a, b = load_answer(path_a), load_answer(path_b)
    ignored, differs = [], []

    def compare(x, y, where):
        if where and where[-1] in VOLATILE:
            if x != y:
                ignored.append(show(where))
            return
        if isinstance(x, dict) and isinstance(y, dict):
            for k in sorted(set(x) | set(y), key=str):
                if k not in x or k not in y:
                    differs.append(f'{show(where + (k,))} present only in {"a" if k in x else "b"}')
                else:
                    compare(x[k], y[k], where + (k,))
        elif isinstance(x, list) and isinstance(y, list):
            if len(x) != len(y):
                differs.append(f'{show(where)} has {len(x)} items in a, {len(y)} in b')
            for i, (u, v) in enumerate(zip(x, y)):
                compare(u, v, where + (i,))
        elif x != y:
            differs.append(f'{show(where)} differs')

    compare(a, b, ())
    print(f'compared {path_a} with {path_b}')
    for line in ignored:
        print(f'ignored (time or id): {line}')
    for line in differs:
        print(f'DIFFERS: {line}')
    print(f'result: {"EQUAL apart from time and id keys" if not differs else f"{len(differs)} differences"}')


def main(argv):
    with_pattern = '--pattern' in argv
    args = [a for a in argv if a != '--pattern']
    if args[:1] == ['pattern'] and len(args) == 1:
        print(PATTERN)
        print(f'chars {len(PATTERN)}, utf8_bytes {len(PATTERN.encode())}, '
              f'sha256 {hashlib.sha256(PATTERN.encode()).hexdigest()}')
    elif args[:1] == ['native'] and len(args) == 3:
        native(args[1], args[2], with_pattern)
    elif args[:1] == ['marks'] and len(args) == 3 and not with_pattern:
        marks(args[1], args[2])
    elif args[:1] == ['tool'] and len(args) in (2, 3):
        tool(args[1], args[2] if len(args) == 3 else None, with_pattern)
    elif args[:1] == ['diff'] and len(args) == 3 and not with_pattern:
        diff(args[1], args[2])
    else:
        print(__doc__ or 'usage: see the head of measure.py')
        sys.exit(2)


main(sys.argv[1:])
