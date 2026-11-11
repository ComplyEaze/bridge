# Lab helper (row 2507-T, new; not product code): the empty-day check as the run lead decided it (deviation 1), on answer files.
# 1. the 2026-05-21 read: isError false, no refusal, total 0, every part served, state partial with reason
#    empty_window_wider_read_nonempty or empty_uncorroborated;
# 2. the 2026-05-20 control read: state complete, total 13;
# 3. the marks answer: GUID 3a6bd6e1-..., ALTVCHID 126, ALTMSTID 300 (step 0's).
# Prints each check and PASS or stop. Reads only.
# Usage: python -I -X utf8 check-empty.py <0521 answer> <0520 answer> <marks raw answer>
import json, re, sys
def res(p):
    a = json.load(open(p, encoding='utf-8'))['response']; r = a.get('result') or {}
    return a, r, (r.get('structuredContent') or {})
ok = True
a, r, sc = res(sys.argv[1]); x = sc.get('result') or {}
parts = (x.get('window') or {}).get('parts') or []
c1 = (not a.get('error') and r.get('isError') is False and 'error' not in x and x.get('total') == 0 and parts and all(p.get('served') for p in parts)
      and x.get('state') == 'partial' and x.get('reason') in ('empty_window_wider_read_nonempty', 'empty_uncorroborated'))
print(f'1. 2026-05-21: isError {r.get("isError")}, state {x.get("state")}, reason {x.get("reason")}, total {x.get("total")}, parts served {[p.get("served") for p in parts]}:', 'pass' if c1 else 'FAIL')
a, r, sc = res(sys.argv[2]); x = sc.get('result') or {}
c2 = r.get('isError') is False and x.get('state') == 'complete' and x.get('total') == 13
print(f'2. 2026-05-20 control: isError {r.get("isError")}, state {x.get("state")}, total {x.get("total")}:', 'pass' if c2 else 'FAIL')
d = open(sys.argv[3], 'rb').read(); t = d[2:].decode('utf-16-le') if d[:2] == b'\xff\xfe' else (d.decode('utf-16-le') if d[1:2] == b'\x00' else d.decode('utf-8'))
g = re.findall(r'<GUID[^>]*>([^<]*)</GUID>', t); v = [s.strip() for s in re.findall(r'<ALTVCHID[^>]*>([^<]*)</ALTVCHID>', t)]; m = [s.strip() for s in re.findall(r'<ALTMSTID[^>]*>([^<]*)</ALTMSTID>', t)]
c3 = g == ['3a6bd6e1-b835-4bff-89dd-8a6af138c346'] and v == ['126'] and m == ['300']
print(f'3. marks: GUID {g}, ALTVCHID {v}, ALTMSTID {m}:', 'pass' if c3 else 'FAIL')
print('empty-day check:', 'PASS' if (c1 and c2 and c3) else 'stop')
