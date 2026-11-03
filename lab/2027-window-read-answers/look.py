# Lab helper (#2027 R1, not product code): reads a raw answer with an XML parser: for each VOUCHER row, each of the six elements as a direct child: count, TYPE and text, and the type, date, party, reference pair. Usage: python -I -X utf8 look.py <file>
import re, sys
import xml.etree.ElementTree as ET
SIX = ('PLACEOFSUPPLY', 'STATENAME', 'GSTREGISTRATIONTYPE', 'GSTNATUREOFRETURN', 'VATPARTYTRANSRETURNNUMBER', 'VATPARTYTRANSRETURNDATE')
def rows(path):
    b = open(path, 'rb').read(); s = b.decode('utf-16-le', 'replace').lstrip(chr(0xfeff))
    s = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', s); s = re.sub(r'&#(4|[0-9]);', '', s)
    return b, [v for v in ET.fromstring(s).iter('VOUCHER') if v.attrib or len(v)]
if __name__ == '__main__':
    b, vs = rows(sys.argv[1]); print('bytes', len(b), 'rows', len(vs))
    for v in vs:
        print(v.attrib.get('VCHTYPE'), v.findtext('DATE'), v.findtext('VOUCHERNUMBER'), v.findtext('PARTYLEDGERNAME'), 'REF', v.findtext('REFERENCE'), v.findtext('REFERENCEDATE'), 'ISINVOICE', v.findtext('ISINVOICE'), 'view', v.attrib.get('OBJVIEW'))
        for t in SIX:
            f = v.findall(t); print('   ', t, 'count', len(f), [(e.attrib.get('TYPE'), e.text) for e in f])
        print('    entries', [(e.findtext('LEDGERNAME'), e.findtext('AMOUNT')) for e in v.findall('ALLLEDGERENTRIES.LIST')] or [(e.findtext('LEDGERNAME'), e.findtext('AMOUNT')) for e in v.findall('LEDGERENTRIES.LIST')])
