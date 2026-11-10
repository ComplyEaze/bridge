# Lab helper (row 2480-NM, a read of answer files only, never a send): for every LEDGER row of each snapshot answer given whose NAME
# attribute or name list holds "BRIDGE Nm ", prints the NAME attribute and each NAME of its LANGUAGENAME.LIST, by an XML parser (C4).
import re, sys, xml.etree.ElementTree as ET
BAD = re.compile(r'[\x00-\x08\x0b\x0c\x0e-\x1f]|&#(?:x0*(?:[0-8bce]|1[0-9a-f])|0*(?:[0-8]|1[124-9]|2[0-9]|3[01]));', re.I)
for p in sys.argv[1:]:
    d = open(p, 'rb').read(); t = (d[2:] if d[:2] == b'\xff\xfe' else d).decode('utf-16-le')
    root = ET.fromstring(BAD.sub('', t)); n = 0
    print(f'== {p}')
    for led in root.iter('LEDGER'):
        a = led.get('NAME', ''); names = [x.text or '' for x in led.iter('NAME')]
        if 'BRIDGE Nm ' in a or any('BRIDGE Nm ' in x for x in names):
            n += 1; print(f'  NAME attribute {a!r}; name list {names!r}; ' + ('the same' if names == [a] else 'DIFFERS'))
    print(f'  rows with "BRIDGE Nm ": {n}')
