# Lab helper (#2162 index rows, not product code): sends M-billtrail.xml by the kit's timed.py under <label>, then prints
# the one COMPANY row's GUID, ALTVCHID and ALTMSTID, and their difference from <previous label>'s answer if given.
# Sends nothing else; presses no key. Usage: python -I -X utf8 mk.py <label> [<previous label>]
import os, subprocess, sys, xml.etree.ElementTree as ET
os.chdir('<LAB_DIR>/run')
lab = sys.argv[1]
r = subprocess.run([sys.executable, '-I', '-X', 'utf8', 'timed.py', 'http://localhost:9001', lab, 'M-billtrail.xml'], capture_output=True, text=True, encoding='utf-8')
print(open(f'log-{lab}.txt', encoding='utf-8').read().strip() if os.path.exists(f'log-{lab}.txt') else '(no log line)')
if r.returncode != 0: print('timed.py exit', r.returncode, (r.stdout + r.stderr).strip()[-300:]); sys.exit(2)
def marks(l):
    d = open(f'a-{l}-M-billtrail.xml', 'rb').read(); t = d.decode('utf-16') if d[:2] == b'\xff\xfe' or d[1:2] == b'\x00' else d.decode('utf-8')
    cs = ET.fromstring(t.lstrip('\ufeff')).findall('./BODY/DATA/COLLECTION/COMPANY')
    if len(cs) != 1: print(f'{l}: COMPANY rows {len(cs)}: stop'); sys.exit(3)
    c = cs[0]; return (c.findtext('GUID') or '').strip(), int((c.findtext('ALTVCHID') or '').strip()), int((c.findtext('ALTMSTID') or '').strip())
g, v, m = marks(lab)
line = f'{lab}: GUID {g}, V {v}, M {m}'
if len(sys.argv) > 2:
    g0, v0, m0 = marks(sys.argv[2]); line += f'; from {sys.argv[2]}: V {v - v0:+d}, M {m - m0:+d}' + ('' if g0 == g else ' GUID DIFFERS')
print(line)
