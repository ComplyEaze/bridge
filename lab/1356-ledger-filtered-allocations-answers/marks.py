# Lab helper (#1356, not product code): reads a marks answer by structure (UTF-16LE, forbidden characters dropped,
# parsed as XML) and prints every ALTVCHID and ALTMSTID element's text with its path. Read only.
import re
import sys
import xml.etree.ElementTree as ET

for path in sys.argv[1:]:
    t = open(path, 'rb').read().decode('utf-16-le').lstrip('﻿')
    t = re.sub(r'&#(0*([0-8]|1[124]|1[4-9]|2[0-9]|3[01]));', '', t)
    t = re.sub(r'[\x00-\x08\x0b\x0c\x0e-\x1f]', '', t)
    root = ET.fromstring(t)

    def walk(e, p):
        for c in e:
            q = f'{p}/{c.tag}'
            if c.tag in ('ALTVCHID', 'ALTMSTID', 'NAME') and not list(c):
                print(path, q, repr((c.text or '').strip()))
            walk(c, q)
    walk(root, root.tag)
