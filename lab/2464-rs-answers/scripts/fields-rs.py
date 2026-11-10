# Prints the fields ROW.md step 5/7 asks to record, from one label's R-0105, RC-0105, V-0105 and M answers (first occurrence of each tag, attributes shown).
import re, sys, os
os.chdir('<LAB_DIR>'); lab = sys.argv[1]
def txt(f):
    d = open(f, 'rb').read()
    return d.decode('utf-16-le' if d[1:2] == b'\x00' or d[:2] == b'\xff\xfe' else 'utf-8', 'replace').lstrip('\ufeff')
def first(t, tag):
    m = re.search(rf'<{tag}(\s[^>]*)?>(.*?)</{tag}>|<{tag}(\s[^>]*)?/>', t, re.S)
    if not m: return 'absent'
    if m.group(2) is None: return f"empty element{(' ' + m.group(3).strip()) if m.group(3) else ''}"
    return repr(m.group(2)) + (f' (attr {m.group(1).strip()})' if m.group(1) else '')
out = []
for kind, tags in [('R-0105', ['VCHGSTSTATUSISINCLUDED', 'VCHGSTSTATUSISUNCERTAIN', 'VCHGSTSTATUSISEXCLUDED', 'ISGSTOVERRIDDEN', 'ALTERID', 'MASTERID']),
                   ('RC-0105', ['GSTREGISTRATION', 'CMPGSTIN']),
                   ('V-0105', ['GSTREGISTRATION', 'CMPGSTIN', 'CMPGSTSTATE', 'CMPGSTREGISTRATIONTYPE', 'PLACEOFSUPPLY', 'STATENAME', 'ALTERID', 'MASTERID']),
                   ('M-marks', ['ALTVCHID', 'ALTMSTID'])]:
    f = f'a-{lab}-{kind}.xml'
    if not os.path.exists(f): continue
    t = txt(f)
    out.append(f'{f}: ' + '; '.join(f'{g} {first(t, g)}' for g in tags))
print('\n'.join(out))
