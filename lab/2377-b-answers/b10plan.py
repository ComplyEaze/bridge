# Lab helper (2377-L Part B, not product code): B10 (C6 before a restore). Checks the backup file's sha256 against B3's and
# that its folder holds that one file only; writes restore-plan.txt; lists THE COMPANY's own data folder (names, sizes,
# times) into evidence/<label>-company-folder.txt. The company number is found from the book's name in Company.1800, never
# typed. Sends nothing, presses no key. Usage: python -I -X utf8 b10plan.py <label> [--plan]
import datetime, hashlib, os, sys
R = '<LAB_DIR>'; B = '<Tally data folder>'; BK = 'C:/lab/2377-b/backup-1'
WANT = '1ca0ec4600368a0479d5d3cff8c235a7aca532565e93e39290575282b164cffe'; SIZE = 33477
NM = 'BRIDGE BILLTRAIL LAB'.encode('utf-16-le') + b'\x00\x00'
BS = chr(92)
lab = sys.argv[1]
nums = [d for d in os.listdir(B) if os.path.isfile(os.path.join(B, d, 'Company.1800')) and NM in open(os.path.join(B, d, 'Company.1800'), 'rb').read()]
if len(nums) != 1: raise SystemExit(f'expected one data folder for the book, found {len(nums)}')
num = nums[0]; now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
def ts(p): return datetime.datetime.fromtimestamp(os.path.getmtime(p), datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
def win(p): return p.replace('/', BS)
if '--plan' in sys.argv:
    files = os.listdir(BK); f = f'TDBK1800_{num}.001'; p = f'{BK}/{f}'
    h = hashlib.sha256(open(p, 'rb').read()).hexdigest() if os.path.isfile(p) else None
    ok = files == [f] and h == WANT and os.path.getsize(p) == SIZE
    lines = [f'restore-plan.txt, written {now}',
             f'Backup folder: {win(BK)} (entries: {len(files)})',
             f'Backup file: {f}, {os.path.getsize(p) if h else "missing"} B, sha256 {h} ({"equal to B3" if h == WANT else "NOT B3"}), written {ts(p) if h else "-"} (14:15 IST, 8 Nov 2026: the Restore list should show that backup)',
             f'THE COMPANY: BRIDGE BILLTRAIL LAB, number {num}, GUID 9ab5002c-e910-4242-83e4-a27cc490600b',
             f"Restore Data Path: Tally's data folder, {win(B)}",
             f'Source: Specify Path, {win(BK)}; select THE COMPANY only; screenshot before accepting',
             f'Prompts expected, and the only ones answered: "Accept ?" = Yes; "Company data folder ({num}) already exists ..." = Replace',
             'Check: ' + ('backup sha256 equal to B3 and the folder holds that one file only' if ok else 'FAILED: stop')]
    plan = '\n'.join(lines) + '\n'
    open(R + '/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write(plan); print(plan.replace(num, '<company number>'))
    if not ok: sys.exit(2)
d = f'{B}/{num}'; rows = []
for n in sorted(os.listdir(d)):
    p = f'{d}/{n}'; rows.append(f'{n}\t{os.path.getsize(p) if os.path.isfile(p) else "<dir>"}\t{ts(p)}')
head = f"time: {now}\nTHE COMPANY's data folder (<Tally data folder>{BS}<company number>), {len(rows)} entries: name, bytes, last written (UTC)\n"
open(f'{R}/evidence/{lab}-company-folder.txt', 'w', encoding='utf-8', newline='\n').write(head + '\n'.join(rows) + '\n')
print(f'{lab}: company folder listed, {len(rows)} entries'); print('\n'.join(rows).replace(num, '<company number>'))
