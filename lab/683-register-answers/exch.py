# Lab helper (683-R1, not product code): reads relay/relay.jsonl and the kept exchanges and writes one line per
# exchange (number, connection, first request line, request and answer bytes and sha256, the request's TYPE and
# collection ID), then, for every answer that holds VOUCHER elements or LEDGER rows, counts with an XML parser:
# VOUCHER elements; AMOUNT and VATEXPAMOUNT elements whose text holds " @ "; per voucher its VOUCHERTYPENAME,
# VOUCHERNUMBER and ALLLEDGERENTRIES.LIST count; voucher type class elements; ledger rows, non-base currency rows
# and opening/closing texts holding " @ ". For parsing only, the text is stripped at both ends, &#4; references and
# raw U+0003 removed and literal CR replaced by &#13;; the kept bytes are never changed. Output:
# evidence/exchanges.txt (no figures).
# Usage: python -I -X utf8 exch.py
import hashlib, json, re, sys
import xml.etree.ElementTree as ET
R = '<lab folder>/'
def body(raw):
    i = raw.find(b'\r\n\r\n'); return raw[:i], raw[i + 4:]
def text_of(b):
    if not b: return ''
    if b[:2] == b'\xff\xfe': return b[2:].decode('utf-16-le')
    if len(b) > 1 and b[1:2] == b'\0': return b.decode('utf-16-le')
    return b.decode('utf-8')
def parse(t):
    t = t.strip().replace('&#4;', '').replace('\x03', '').replace('\r', '&#13;')
    t = re.sub(r'<\?xml[^>]*\?>', '', t)
    return ET.fromstring(t)
out = []
for line in open(R + 'relay/relay.jsonl', encoding='utf-8'):
    e = json.loads(line)
    if 'event' in e:
        if e['event'] not in ('sent',): out.append(f"event {e['event']} conn {e.get('conn')} {({k: v for k, v in e.items() if k in ('requests', 'number', 'error', 'cause')})}")
        continue
    n = e['number']
    rq = open(f'{R}relay/exchange-{n}.request', 'rb').read(); an = open(f'{R}relay/exchange-{n}.answer', 'rb').read()
    rh, rb = body(rq); ah, ab = body(an)
    assert hashlib.sha256(rb).hexdigest() == e['sent_sha256'] and hashlib.sha256(ab).hexdigest() == e['answer_sha256']
    rt = text_of(rb)
    kind = re.findall(r'<TYPE>([^<]*)</TYPE>', rt)[:1]; cid = re.findall(r'<ID>([^<]*)</ID>', rt)[:1]
    tr = re.findall(r'<TALLYREQUEST>([^<]*)</TALLYREQUEST>', rt)[:1]
    out.append(f"ex {n} conn {e['conn']} line '{e['line']}' req {e['sent']} B {e['sent_sha256']} ans {e['answered']} B {e['answer_sha256']} request {tr} {kind} {cid} first_byte_s {e['first_byte_s']}")
    if not ab or b'<' not in ab[:200] and b'<\0' not in ab[:400]: continue
    try: root = parse(text_of(ab))
    except ET.ParseError as x: out.append(f'   parse error {x}'); continue
    vs = [v for v in root.iter('VOUCHER') if len(v) or v.attrib]
    if vs:
        amt = [a for a in root.iter('AMOUNT') if a.text and ' @ ' in a.text]
        vat = [a for a in root.iter('VATEXPAMOUNT') if a.text and ' @ ' in a.text]
        out.append(f'   VOUCHER {len(vs)}; AMOUNT with " @ " {len(amt)}; VATEXPAMOUNT with " @ " {len(vat)}; AMOUNT elements {sum(1 for _ in root.iter("AMOUNT"))}')
        cls = {}
        for v in vs:
            ty = (v.findtext('VOUCHERTYPENAME') or '').strip(); no = (v.findtext('VOUCHERNUMBER') or '').strip()
            out.append(f'   voucher {ty} {no}: ALLLEDGERENTRIES.LIST {len(v.findall("ALLLEDGERENTRIES.LIST"))}, LEDGERENTRIES.LIST {len(v.findall("LEDGERENTRIES.LIST"))}, children {sorted(set(c.tag for c in v))[:0]}')
            for c in v:
                if 'CLASS' in c.tag.upper() or c.tag.upper().startswith(('VCHTYPE', 'BRIDGEVCH')): cls[c.tag] = cls.get(c.tag, 0) + 1
        carry = sum(1 for v in vs if any(c.tag.upper().startswith('BRIDGEVCH') for c in v))
        out.append(f'   voucher-level class elements (tag: rows carrying it): {cls}; vouchers carrying any BRIDGEVCH*: {carry}')
        out.append(f'   voucher child tags: {sorted(set(c.tag for v in vs for c in v))}')
    ls = [l for l in root.iter('LEDGER') if len(l) or l.attrib]
    if ls:
        cur = [l for l in ls if (l.findtext('CURRENCYNAME') or '').strip() not in ('', '\u20b9')]
        at = [l for l in ls if any(' @ ' in (l.findtext(t) or '') for t in ('OPENINGBALANCE', 'CLOSINGBALANCE'))]
        cn = {}
        for l in ls:
            k = (l.findtext('CURRENCYNAME') or '').strip() if l.find('CURRENCYNAME') is not None else '(no element)'; cn[k] = cn.get(k, 0) + 1
        out.append(f'   LEDGER rows {len(ls)}; CURRENCYNAME values {cn}; CURRENCYNAME not base {len(cur)}; OPENING/CLOSING with " @ " {len(at)}; tags {sorted(set(c.tag for l in ls for c in l))}')
open(R + 'evidence/exchanges.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(out) + '\n')
print(len(out), 'lines')
