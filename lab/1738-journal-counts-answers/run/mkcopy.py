# Lab helper (#1738-COUNTS, not product code): writes run\copy.cmd (ASCII) for the WMI out step: the environment line,
# then for each label (Q, R, L1...) and each of its two files: size and sha256 of the source, copy /b into
# copies\<label>\, size and sha256 of the source again and of the copy; a file absent is recorded as absent (if exist).
# Every path is absolute; Q is named by its full Packages path. The label -> folder map goes to kept\labels.txt (local).
# Usage: python -I -X utf8 mkcopy.py
import os
W = 'C:\\bridge-lab\\1738c'
LA = os.environ['LOCALAPPDATA']
FAM = '<family>'
L = [r'C:\bridge-lab\<folder-1>\egress-copy', r'C:\bridge-lab\<folder-2>\data', r'C:\bridge-lab\<folder-3>\data', r'C:\bridge-lab\<folder-4>\data-before',
     r'C:\bridge-lab\1215', r'C:\bridge-lab\1386', r'C:\bridge-lab\1390', r'C:\bridge-lab\1393', r'C:\bridge-lab\1461', r'C:\bridge-lab\<folder-10>\data',
     r'C:\bridge-lab\<folder-11>\data', r'C:\bridge-lab\<folder-12>\data', r'C:\bridge-lab\<folder-13>\data', r'C:\bridge-lab\<folder-14>\data', r'C:\bridge-lab\<folder-15>\data',
     r'C:\bridge-lab\<folder-16>\data', r'C:\bridge-lab\<folder-17>\data', r'C:\bridge-lab\<folder-18>\data', r'C:\bridge-lab\<folder-19>\data', r'C:\bridge-lab\<folder-20>\data',
     r'C:\bridge-lab\<folder-21>\data', r'C:\bridge-lab\<folder-22>\data', r'C:\bridge-lab\<folder-23>\data', r'C:\bridge-lab\<folder-24>\data',
     r'C:\bridge-lab\<folder-25>\data', r'C:\bridge-lab\<folder-26>\r1\data', r'C:\bridge-lab\<folder-27>\data']
labels = [('Q', os.path.join(LA, 'Packages', FAM, 'LocalCache', 'Local', 'Bridge', 'agent')), ('R', os.path.join(LA, 'Bridge', 'agent'))]
labels += [(f'L{i}', p) for i, p in enumerate(L, 1)]
lines = ['@echo off', f'(echo LOCALAPPDATA=%LOCALAPPDATA%& echo USERPROFILE=%USERPROFILE%) > {W}\\out\\env-out.txt']
for lab, folder in labels:
    log = f'{W}\\out\\copy-{lab}.txt'
    lines.append(f'mkdir {W}\\copies\\{lab} 2>nul')
    lines.append(f'echo label {lab} > {log}')
    for name in ('agent-import-ledger.jsonl', 'agent-import-statement-skips.jsonl'):
        src = os.path.join(folder, name); dst = f'{W}\\copies\\{lab}\\{name}'
        lines.append(f'if exist "{src}" (')
        lines.append(f'  echo == {name} source before >> {log}')
        lines.append(f'  for %%A in ("{src}") do echo size %%~zA >> {log}')
        lines.append(f'  certutil -hashfile "{src}" SHA256 >> {log}')
        lines.append(f'  copy /b "{src}" "{dst}" >> {log} 2>&1')
        lines.append(f'  echo == {name} source after >> {log}')
        lines.append(f'  for %%A in ("{src}") do echo size %%~zA >> {log}')
        lines.append(f'  certutil -hashfile "{src}" SHA256 >> {log}')
        lines.append(f'  echo == {name} copy >> {log}')
        lines.append(f'  for %%A in ("{dst}") do echo size %%~zA >> {log}')
        lines.append(f'  certutil -hashfile "{dst}" SHA256 >> {log}')
        lines.append(f') else (echo == {name} absent >> {log})')
lines.append(f'echo %ERRORLEVEL% > {W}\\out\\copy.exit')
open(f'{W}\\run\\copy.cmd', 'w', encoding='ascii', newline='\r\n').write('\n'.join(lines) + '\n')
with open(f'{W}\\kept\\labels.txt', 'w', encoding='utf-8', newline='\n') as f:
    for lab, folder in labels: f.write(f'{lab} {folder}\n')
print(len(labels), 'labels;', len(lines), 'lines')
