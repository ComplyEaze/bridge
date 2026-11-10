# Lab helper (row 683-LD): compares two answers byte for byte; if they differ, says whether every difference lies inside the CMPINFO count block (C4: not rows) and lists those count lines. Usage: python -I -X utf8 cmpinfo.py <control> <answer>
import sys, hashlib, re
a, b = (open(f, 'rb').read() for f in sys.argv[1:3])
for f, d in zip(sys.argv[1:3], (a, b)):
    print(f'{f}: {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}')
if a == b:
    print('BYTE-EQUAL'); sys.exit(0)
def parts(d):
    s = d[2:].decode('utf-16-le') if d[:2] == b'\xff\xfe' else d.decode('utf-16-le')
    j = s.index('<CMPINFO>'); i = s.index('</CMPINFO>')
    return s[:j], s[j:i], s[i:]
pa, pb = parts(a), parts(b)
same_out = pa[0] == pb[0] and pa[2] == pb[2]
ca = dict(re.findall(r'<(\w+)>([^<]*)</\1>', pa[1])); cb = dict(re.findall(r'<(\w+)>([^<]*)</\1>', pb[1]))
print('NOT BYTE-EQUAL; outside CMPINFO:', 'EQUAL' if same_out else 'DIFFERS')
for k in ca:
    if ca[k] != cb.get(k): print(f'  CMPINFO {k}: {ca[k]} -> {cb.get(k)}')
for k in cb:
    if k not in ca: print(f'  CMPINFO {k}: (none) -> {cb[k]}')
sys.exit(1 if same_out else 2)
