# Lab helper (1381-O, not product code): one write of S4/S5, split at C3's screenshot.
#   python -I -X utf8 probe.py write <w> <request file>   marks <w>-pre (one COMPANY row, the book's 8, equal to the
#       previous marks answer given as PREV in evidence/last-marks.txt), then the write once by send.py, then the
#       Tally window capture shots/<w>-after.png. Stops at any miss.
#   python -I -X utf8 probe.py after <w> <previous a1 answer>   check_r1.py import, marks <w>-post, a1-<w>,
#       check_r1.py diff <previous a1> a1-<w>, check_r1.py rows a1-<w>; everything to evidence/<w>-checks.txt.
import os, subprocess, sys, xml.etree.ElementTree as ET, re
R = '<LAB_DIR>'; BOOK8 = 'bac2b5e9'
def run(args):
    r = subprocess.run(['python', '-I', '-X', 'utf8'] + args, cwd=R, capture_output=True, text=True, encoding='utf-8')
    return r.returncode, r.stdout + r.stderr
def marks(path):
    t = open(path, 'rb').read().decode('utf-16-le').lstrip('\ufeff')
    t = re.sub(r'&#([0-8]|1[124-9]|2[0-9]|3[01]);', '', t); t = re.sub('[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    rows = ET.fromstring(t).findall('./BODY/DATA/COLLECTION/COMPANY')
    out = []
    for c in rows:
        g = (c.findtext('GUID') or '').strip(); v = (c.findtext('ALTVCHID') or '').strip() or 'absent'; m = (c.findtext('ALTMSTID') or '').strip()
        out.append((g, v, m))
    return out
mode, w = sys.argv[1], sys.argv[2]
log = open(f'{R}/evidence/{w}-checks.txt', 'a', encoding='utf-8', newline='\n')
def say(s): print(s); log.write(s + '\n'); log.flush()
if mode == 'write':
    req = sys.argv[3]
    prev = open(f'{R}/evidence/last-marks.txt', encoding='utf-8').read().split()
    rc, out = run(['send.py', f'{w}-pre', 'requests/marks.utf16le.xml']); say(out.strip())
    if rc: sys.exit('stop: marks before failed')
    mk = marks(f'{R}/answers/a-{w}-pre.xml'); say(f'{w}-pre marks: {[(g[:8], v, m) for g, v, m in mk]}')
    if len(mk) != 1 or not mk[0][0].startswith(BOOK8): sys.exit('stop: marks before is not one COMPANY row with the book\'s 8')
    if [mk[0][1], mk[0][2]] != prev: sys.exit(f'stop: marks before V/M {mk[0][1]}/{mk[0][2]} differ from the step before {prev}')
    rc, out = run(['send.py', w, req]); say(out.strip())
    if rc: sys.exit('stop: the write send failed')
    r = subprocess.run(['powershell', '-NoProfile', '-File', f'{R}/capnow.ps1', '-Name', f'{w}-after'], capture_output=True, text=True)
    say('capture ' + r.stdout.strip())
elif mode == 'after':
    preva1 = sys.argv[3]
    for args in (['check_r1.py', 'import', f'answers/a-{w}.xml'],):
        rc, out = run(args); say('> ' + ' '.join(args) + '\n' + out.strip() + f'\nexit {rc}')
    rc, out = run(['send.py', f'{w}-post', 'requests/marks.utf16le.xml']); say(out.strip())
    if rc: sys.exit('stop: marks after failed')
    mk = marks(f'{R}/answers/a-{w}-post.xml'); say(f'{w}-post marks: {[(g[:8], v, m) for g, v, m in mk]}')
    if len(mk) != 1 or not mk[0][0].startswith(BOOK8): sys.exit('stop: marks after is not one COMPANY row with the book\'s 8')
    pre = marks(f'{R}/answers/a-{w}-pre.xml')[0]
    say(f'marks difference: V {pre[1]} -> {mk[0][1]}, M {pre[2]} -> {mk[0][2]}')
    open(f'{R}/evidence/last-marks.txt', 'w', encoding='utf-8').write(f'{mk[0][1]} {mk[0][2]}')
    rc, out = run(['send.py', f'a1-{w}', 'requests/a1-candidate-items.utf16le.xml']); say(out.strip())
    if rc: sys.exit('stop: a1 read failed')
    for args in (['check_r1.py', 'diff', preva1, f'answers/a-a1-{w}.xml'], ['check_r1.py', 'rows', f'answers/a-a1-{w}.xml']):
        rc, out = run(args); say('> ' + ' '.join(args) + '\n' + out.strip() + f'\nexit {rc}')
