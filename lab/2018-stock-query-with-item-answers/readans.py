# Lab helper (#2018 stock query with item, not product code): reads one raw answer by structure (LAB-COMMON C4: UTF-16LE,
# forbidden characters dropped, parsed as XML, never a text search on the bytes) and prints STATUS, any line error, each
# top-level element under ENVELOPE in order with its text (empty shown as <empty>), and every leaf under any compound
# element with its path. A 0-byte body is reported as such. Read only. Usage: python -I readans.py <answer> [<answer> ...]
import re
import sys
import xml.etree.ElementTree as ET

for path in sys.argv[1:]:
    raw = open(path, 'rb').read()
    print(f'== {path}: {len(raw)} B')
    if not raw:
        print('  0-byte body')
        continue
    text = raw.decode('utf-16-le')
    if text.startswith('﻿'):
        text = text[1:]
    text = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', text)
    text = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', text)
    root = ET.fromstring(text)
    print('  root', root.tag)
    for e in root.iter():
        if e.tag in ('STATUS', 'LINEERROR'):
            print(f'  {e.tag} {(e.text or "").strip()!r}')
    for e in root:
        kids = list(e)
        t = (e.text or '').strip()
        if kids:
            leaves = []
            def walk(n, pth):
                ch = list(n)
                if not ch:
                    leaves.append(f'{pth}={((n.text or "").strip() or "<empty>")!r}')
                for c in ch:
                    walk(c, f'{pth}/{c.tag}' if pth else c.tag)
            walk(e, '')
            print(f'  {e.tag}: ' + ', '.join(leaves))
        else:
            print(f'  {e.tag} {t or "<empty>"!r}')
