# Lab helper (row 2507-T, new; not product code): reads raw marks answers (the sheet's marks.xml request) and prints, for each, the
# company GUID, ALTVCHID and ALTMSTID, and whether its bytes equal the first file's once its read-time lines are set aside. Reads only.
# Usage: python -I -X utf8 marks.py <answer file> [<answer file> ...]
import re, sys
def dec(d):
    if d[:2] == b'\xff\xfe': return d[2:].decode('utf-16-le')
    if len(d) > 1 and d[1:2] == b'\x00': return d.decode('utf-16-le')
    return d.decode('utf-8')
def marks(t):
    g = re.findall(r'<GUID[^>]*>([^<]*)</GUID>', t)
    v = re.findall(r'<ALTVCHID[^>]*>([^<]*)</ALTVCHID>', t)
    m = re.findall(r'<ALTMSTID[^>]*>([^<]*)</ALTMSTID>', t)
    return g, v, m
def body(t):
    return '\n'.join(l for l in t.replace('><', '>\n<').split('\n') if not re.search(r'(?i)time|date', l))
first = None
for p in sys.argv[1:]:
    t = dec(open(p, 'rb').read()); g, v, m = marks(t)
    same = '' if first is None else (' | equal to the first apart from time lines' if body(t) == first else ' | DIFFERS from the first')
    if first is None: first = body(t)
    print(f'{p}: GUID {g} ALTVCHID {v} ALTMSTID {m}{same}')
