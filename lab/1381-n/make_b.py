#!/usr/bin/env python3
"""Renders step 4's write on the lab PC: a Create of SC Copied Goods built from SC Sibling Goods's TYPED values.

  python3 -I make_b.py plain A1-ANSWER.xml     writes requests/b1-create.utf16le.xml, item SC Copied Goods
  python3 -I make_b.py marked A1-ANSWER.xml    writes requests/b2-create.utf16le.xml, item SC Copied Marked

A1-ANSWER is step 3's answer to requests/a1-candidate-items.utf16le.xml, as received. This is not row 1381-M's
make_r2.py, which copied the answer's own characters. Here the row is read as ComplyEaze Bridge's reader reads
it, and the request is written from the values alone, as a builder in Bridge would write it:

- READ: the answer is decoded and the references XML forbids are dropped (LAB-COMMON C4; Bridge's reader does
  the same, bridge-tally-protocol/src/tolerant_xml.rs:118). It is parsed as XML. Only the row at
  ./BODY/DATA/COLLECTION/STOCKITEM named SC Sibling Goods is used.
- VALUES: the fields master's stock-item parser models (stock_tax_details.rs:174-252), and the four scalars
  PARENT, BASEUNITS, GSTAPPLICABLE and GSTTYPEOFSUPPLY. Dates are trimmed. Each GSTRATE is trimmed and must be a
  plain non-negative decimal. Every other text is kept as received. Attributes (TYPE and the like) are values
  of no field and are not kept.
- REFUSED, writing nothing: the row missing or repeated; a scalar absent or repeated; no GST or HSN entry; an
  entry without a date; a field repeated inside an entry; any child element the parser does not model
  (master's other_content_skipped); a GSTSLABRATES.LIST with content; a rate that is not a plain decimal.
- WRITE: Bridge's import envelope (import_envelope.rs:27, All Masters), asserted equal, less its message, to
  row 1381-M's r1 write (m-r1-name-unit.utf16le.xml, byte-equal to that sheet's requests/r1-name-unit), one STOCKITEM with ACTION="Create",
  each value escaped once as escape_text does (xml_text.rs:41-50), elements in the parser's order. A present
  empty GSTSLABRATES.LIST is written empty, because the typed value records only its presence.

THE ONE DIFFERENCE between the two modes. Tally sends three of the sibling's values with a leading `&#4;`
reference, which C4 and Bridge's reader drop, leaving a leading space: GSTAPPLICABLE ` Applicable`, STATENAME
` Any` and GSTRATEVALUATIONTYPE ` Not Applicable` (seen in row 1381-M's r2 request at d8948467). `plain` writes
the values as read. `marked` writes exactly those three texts, and only those, as `&#4; Applicable`, `&#4; Any`
and `&#4; Not Applicable`: a closed set rendered in Tally's own form. Any other value is written as read in both.

It writes the file (UTF-16LE with a BOM, as row 1381-M's write) and prints the counts of what it wrote, how
many marked values it wrote, and the file's sha256. It prints no value.
"""
import hashlib, pathlib, re, sys
import xml.etree.ElementTree as ET

SOURCE, COMPANY = 'SC Sibling Goods', 'BRIDGE STOCK CREATE LAB'
MODES = {'plain': ('SC Copied Goods', 'b1-create'), 'marked': ('SC Copied Marked', 'b2-create')}
MARKED = {' Applicable', ' Any', ' Not Applicable'}
MARK = {'on': False, 'count': 0}
HERE = pathlib.Path(__file__).resolve().parent
FORBIDDEN = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:0*(?:[0-8]|1[124-9]|2\d|3[01])|x0*(?:[0-8bcef]|1[0-9a-f]));', re.I)
DECIMAL = re.compile(r'^\d+(?:\.\d+)?$')
SCALARS = ('PARENT', 'BASEUNITS', 'GSTAPPLICABLE', 'GSTTYPEOFSUPPLY')
GST_ENTRY = ('APPLICABLEFROM', 'TAXABILITY', 'SRCOFGSTDETAILS', 'GSTINELIGIBLEITC')
HSN_ENTRY = ('APPLICABLEFROM', 'HSNCODE', 'HSN', 'SRCOFHSNDETAILS')
RATE = ('GSTRATEDUTYHEAD', 'GSTRATEVALUATIONTYPE', 'GSTRATE')


def refuse(why):
    sys.exit(f'not rendered: {why}')


def escape_text(v):
    return (v.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;').replace('"', '&quot;')
            .replace("'", '&apos;').replace('\r', '&#13;').replace('\n', '&#10;'))


def text_of(el, where):
    if len(el):
        refuse(f'{where} holds markup')
    return el.text or ''


def fields(el, known, lists, where):
    """The element's scalar children named in `known`, once each; children named in `lists` are returned apart."""
    out, nested = {}, {k: [] for k in lists}
    for c in el:
        if c.tag in lists:
            nested[c.tag].append(c)
        elif c.tag in known:
            if c.tag in out:
                refuse(f'{where}: {c.tag} repeated')
            out[c.tag] = text_of(c, f'{where}/{c.tag}')
        else:
            refuse(f'{where}: a child the parser does not model ({c.tag})')
    return out, nested


def tag(name, value):
    if value is None:
        return ''
    if MARK['on'] and value in MARKED:
        MARK['count'] += 1
        return f'<{name}>&#4;{escape_text(value)}</{name}>'
    return f'<{name}>{escape_text(value)}</{name}>'


def gst_entry(el, i):
    f, n = fields(el, GST_ENTRY, ('STATEWISEDETAILS.LIST',), f'GST entry {i}')
    if not f.get('APPLICABLEFROM', '').strip():
        refuse(f'GST entry {i} has no date')
    states = []
    for j, s in enumerate(n['STATEWISEDETAILS.LIST']):
        sf, sn = fields(s, ('STATENAME',), ('RATEDETAILS.LIST', 'GSTSLABRATES.LIST'), f'GST entry {i} state {j}')
        if len(sn['GSTSLABRATES.LIST']) > 1 or any(len(x) or (x.text or '').strip() for x in sn['GSTSLABRATES.LIST']):
            refuse(f'GST entry {i} state {j}: slab rates with content, or repeated')
        rates = []
        for k, r in enumerate(sn['RATEDETAILS.LIST']):
            rf, _ = fields(r, RATE, (), f'GST entry {i} state {j} rate {k}')
            if 'GSTRATE' in rf:
                rf['GSTRATE'] = rf['GSTRATE'].strip()
                if not DECIMAL.match(rf['GSTRATE']):
                    refuse(f'GST entry {i} state {j} rate {k}: not a plain decimal')
            rates.append('<RATEDETAILS.LIST>' + ''.join(tag(t, rf.get(t)) for t in RATE) + '</RATEDETAILS.LIST>')
        slab = '<GSTSLABRATES.LIST></GSTSLABRATES.LIST>' if sn['GSTSLABRATES.LIST'] else ''
        states.append('<STATEWISEDETAILS.LIST>' + tag('STATENAME', sf.get('STATENAME')) + ''.join(rates) + slab
                      + '</STATEWISEDETAILS.LIST>')
    f['APPLICABLEFROM'] = f['APPLICABLEFROM'].strip()
    body = ''.join(tag(t, f.get(t)) for t in GST_ENTRY) + ''.join(states)
    return '<GSTDETAILS.LIST>' + body + '</GSTDETAILS.LIST>', len(states)


def hsn_entry(el, i):
    f, _ = fields(el, HSN_ENTRY, (), f'HSN entry {i}')
    if not f.get('APPLICABLEFROM', '').strip():
        refuse(f'HSN entry {i} has no date')
    f['APPLICABLEFROM'] = f['APPLICABLEFROM'].strip()
    return '<HSNDETAILS.LIST>' + ''.join(tag(t, f.get(t)) for t in HSN_ENTRY) + '</HSNDETAILS.LIST>'


def main(mode, path):
    target, out_name = MODES[mode]
    MARK['on'] = mode == 'marked'
    raw = pathlib.Path(path).read_bytes()
    text = raw[2:].decode('utf-16-le') if raw[:2] == b'\xff\xfe' else raw.decode('utf-16-le')
    try:
        root = ET.fromstring(FORBIDDEN.sub('', text))
    except ET.ParseError as e:
        refuse(f'the answer does not parse: {e}')
    if root.tag != 'ENVELOPE':
        refuse('the answer is not an ENVELOPE')
    rows = [r for r in root.findall('./BODY/DATA/COLLECTION/STOCKITEM') if r.get('NAME') == SOURCE]
    if len(rows) != 1:
        refuse(f'{len(rows)} rows named {SOURCE}')
    row = rows[0]
    scalars = {}
    for t in SCALARS:
        els = row.findall(t)
        if len(els) != 1:
            refuse(f'{t}: {len(els)} elements')
        scalars[t] = text_of(els[0], t)
    gst = [gst_entry(e, i) for i, e in enumerate(row.findall('GSTDETAILS.LIST')) if len(e) or (e.text or '').strip()]
    hsn = [hsn_entry(e, i) for i, e in enumerate(row.findall('HSNDETAILS.LIST')) if len(e) or (e.text or '').strip()]
    if not gst or not hsn:
        refuse(f'{len(gst)} GST and {len(hsn)} HSN entries with content')
    message = (f'<TALLYMESSAGE><STOCKITEM NAME="{escape_text(target)}" ACTION="Create">{tag("NAME", target)}'
               + ''.join(tag(t, scalars[t]) for t in SCALARS) + ''.join(g for g, _ in gst) + ''.join(hsn)
               + '</STOCKITEM></TALLYMESSAGE>')
    envelope = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>'
                '</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES>'
                f'<SVCURRENTCOMPANY>{escape_text(COMPANY)}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC>'
                f'<REQUESTDATA>{message}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>')
    m_r1 = (HERE / 'm-r1-name-unit.utf16le.xml').read_bytes()
    m_r1 = m_r1[2:].decode('utf-16-le')
    r1_message = m_r1[m_r1.index('<REQUESTDATA>') + 13:m_r1.index('</REQUESTDATA>')]
    if envelope.replace(message, r1_message) != m_r1:
        refuse('the envelope is not row 1381-M r1\'s envelope (m-r1-name-unit.utf16le.xml) less its message')
    out = HERE / 'requests' / f'{out_name}.utf16le.xml'
    out.write_bytes(b'\xff\xfe' + envelope.encode('utf-16-le'))
    data = out.read_bytes()
    print(f'written: 4 scalars, {len(gst)} GST entries ({sum(n for _, n in gst)} state rows), {len(hsn)} HSN entries, '
          f'{MARK["count"]} marked values')
    print(f'{hashlib.sha256(data).hexdigest()}  {len(data)}  requests/{out.name}')


if __name__ == '__main__':
    if len(sys.argv) != 3 or sys.argv[1] not in MODES:
        sys.exit(__doc__)
    main(sys.argv[1], sys.argv[2])
