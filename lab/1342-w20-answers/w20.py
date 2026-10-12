# W20 expectations (b), (c), (d): compare aw20-a with W19's aw19-e-1, and aw20-b with aw20-a (uses w19.py's parser).
import sys
sys.path.insert(0, 'C:/bridge-lab/w20')
from w19 import rows, key
FLAGS = ['DATE', 'VOUCHERTYPENAME', 'VOUCHERNUMBER', 'ALTERID', 'MASTERID', 'ISGSTOVERRIDDEN', 'VCHGSTSTATUSISINCLUDED',
         'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISDELETED']
def cmp(a, b, fields, la, lb):
    A = {key(d): d for d in rows(a)}; B = {key(d): d for d in rows(b)}
    print(f'{la}: {len(A)} vouchers, {lb}: {len(B)} vouchers; only in {la}: {sorted(k[1] for k in A.keys() - B.keys())}; only in {lb}: {sorted(k[1] for k in B.keys() - A.keys())}')
    n = 0
    for k in sorted(A.keys() & B.keys(), key=lambda x: x[1]):
        ch = [f'{f}: {A[k][f]} -> {B[k][f]}' for f in fields if A[k][f] != B[k][f]]
        if ch: n += 1; print('  changed', k[1], '; '.join(ch))
    print(f'  vouchers with a difference in {fields[3:]}: {n}')
print('(b) aw20-a vs W19 aw19-e-1 (status fields only)')
cmp('answers/aw20-a.xml', 'C:/bridge-lab/w19/answers/aw19-e-1.xml', FLAGS, 'aw20-a', 'aw19-e-1')
print('(d) aw20-a vs aw20-b (status fields)')
cmp('answers/aw20-a.xml', 'answers/aw20-b.xml', FLAGS, 'aw20-a', 'aw20-b')
print('(c) aw20-b ISCANCELLED / ISOPTIONAL per voucher (count of each tag in the voucher text)')
import re
from w19 import txt
t = txt('answers/aw20-b.xml')
for m in re.finditer(r'<VOUCHER [^>]*>.*?</VOUCHER>', t, re.S):
    v = m.group(0); num = re.search(r'<VOUCHERNUMBER>([^<]*)<', v).group(1)
    c = re.findall(r'<ISCANCELLED[^>]*>([^<]*)<', v); o = re.findall(r'<ISOPTIONAL[^>]*>([^<]*)<', v)
    flag = '' if (c, o) == (['No'], ['No']) else '   <--'
    print(f'  {num:16s} ISCANCELLED {c} ISOPTIONAL {o}{flag}')
ta = txt('answers/aw20-a.xml')
print('aw20-a ISCANCELLED tags:', len(re.findall('<ISCANCELLED', ta)), 'ISOPTIONAL tags:', len(re.findall('<ISOPTIONAL', ta)))
