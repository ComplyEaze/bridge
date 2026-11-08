# Lab helper (#2162 index rows, not product code): B10 of the sheet. Checks the B1 backup, writes restore-plan.txt and lists
# THE COMPANY's data folder (evidence/b10-company-folder.txt, or the file named as the first argument). Usage: python -I -X utf8 b10.py [out]
import os, sys, hashlib, datetime
R = '<LAB_DIR>'
B = 'C:/lab/2162-ix/backup-1'
NUM = '<company number>'
D = '<Tally data folder>/' + NUM
BS = chr(92)
def U(t):
    return datetime.datetime.fromtimestamp(t, datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')
now = U(datetime.datetime.now().timestamp())
out = sys.argv[1] if len(sys.argv) > 1 else 'evidence/b10-company-folder.txt'
if out == 'evidence/b10-company-folder.txt':
    fs = os.listdir(B)
    f = os.path.join(B, fs[0])
    b = open(f, 'rb').read()
    h = hashlib.sha256(b).hexdigest()
    assert len(fs) == 1 and h == 'fbfa30cc33d90f3169b503e2c609452f3be3b47e4f2666c9410bb47f0d32a2c0', (fs, h)
    lines = [
        f'restore-plan.txt, written {now}',
        f'Backup folder: C:{BS}lab{BS}2162-ix{BS}backup-1 (entries: {len(fs)})',
        f'Backup file: {fs[0]}, {len(b)} B, sha256 {h} (equal to B1), written {U(os.path.getmtime(f))} (16:55 IST, 8 Nov 2026: the Restore list should show that backup)',
        f'THE COMPANY: BRIDGE BILLTRAIL LAB, number {NUM}, GUID 9ab5002c-e910-4242-83e4-a27cc490600b',
        f"Restore Data Path: Tally's data folder, C:{BS}Users{BS}Public{BS}TallyPrimeEditLog{BS}data",
        f'Source: Specify Path, C:{BS}lab{BS}2162-ix{BS}backup-1; select THE COMPANY only; screenshot before accepting',
        f'Prompts expected, and the only ones answered: "Accept ?" = Yes; "Company data folder ({NUM}) already exists ..." = Replace',
        'Check: backup sha256 equal to B1 and the folder holds that one file only',
    ]
    open(R + '/restore-plan.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
    print('plan', fs[0][:9], len(b), h[:8])
es = sorted(os.listdir(D))
with open(os.path.join(R, out), 'w', encoding='utf-8', newline='\n') as o:
    o.write(f'time: {now}\n')
    o.write(f"THE COMPANY's data folder (<Tally data folder>{BS}<company number>), {len(es)} entries: name, bytes, last written (UTC)\n")
    for e in es:
        p = os.path.join(D, e)
        o.write(f'{e}\t{os.path.getsize(p)}\t{U(os.path.getmtime(p))}\n')
print(now, out, 'entries', len(es))
