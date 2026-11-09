# Lab helper (2390-CRT, not product code): lists the entries of the 0.7.0 .mcpb (a zip) by name and size, reading it in place
# (nothing is unpacked to disk), and the sha256 of each entry the sheet compares. Usage: python -I -X utf8 mcpb-list.py <mcpb> <out>
import hashlib, sys, zipfile
src, out = sys.argv[1], sys.argv[2]
d = open(src, 'rb').read()
lines = [f'mcpb {len(d)} B sha256 {hashlib.sha256(d).hexdigest()}']
with zipfile.ZipFile(src) as z:
    infos = z.infolist()
    lines.append(f'entries {len(infos)}')
    for i in infos:
        lines.append(f'{i.file_size:>10}  {i.compress_size:>10}  {i.filename}')
    for i in infos:
        if i.filename.endswith(('bridge_mcp.exe', 'pdfium.dll', 'bridge.exe')):
            lines.append(f'entry sha256 {hashlib.sha256(z.read(i)).hexdigest()}  {i.file_size}  {i.filename}')
open(out, 'w', encoding='utf-8', newline='\n').write('\n'.join(lines) + '\n')
print('\n'.join(lines))
