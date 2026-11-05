# Lab helper (1381-M, not product code): one write of step 5 in two phases, through send.py and chk.py (each stops on any miss).
#   send <row> <write file>          MARKS (<row>pre), checked: exactly one COMPANY row, THE COMPANY's; then the write, once; then a
#                                    capture of the whole Tally window (shots/<row>-after-write.png), to be looked at before 'check'
#   check <row> <write file> <prev>  check_1381.py import on the write's answer; MARKS (<row>post); items (<row>), a capture, rows;
#                                    diff <prev items label> -> <row>
# Usage: python -I -X utf8 write.py send r1 r1-name-unit.utf16le.xml | python -I -X utf8 write.py check r1 r1-name-unit.utf16le.xml base
import subprocess, sys
R = '<LAB_DIR>'
def run(*a):
    r = subprocess.run([sys.executable, '-I', '-X', 'utf8', *a], cwd=R, capture_output=True, text=True, encoding='utf-8')
    print(r.stdout, end=''); print(r.stderr, end='')
    if r.returncode: raise SystemExit(f'stop: {a[0]} exit {r.returncode}')
mode, row, wf = sys.argv[1], sys.argv[2], sys.argv[3]
if mode == 'send':
    run('send.py', f'{row}pre', 'marks.utf16le.xml'); run('chk.py', f'{row}pre-marks', 'marks', f'{row}pre-marks.utf16le.xml')
    run('send.py', row, wf)
    run('cap.py', f'{row}-after-write')
else:
    prev = sys.argv[4]
    run('chk.py', f'{row}-import', 'import', f'{row}-{wf}')
    run('send.py', f'{row}post', 'marks.utf16le.xml'); run('chk.py', f'{row}post-marks', 'marks', f'{row}post-marks.utf16le.xml')
    run('send.py', row, 'items.utf16le.xml'); run('cap.py', f'{row}-after-items')
    run('chk.py', f'{row}-items', 'rows', f'{row}-items.utf16le.xml')
    run('chk.py', f'{row}-diff', 'diff', f'{prev}-items.utf16le.xml', f'{row}-items.utf16le.xml')
