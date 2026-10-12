# Check that a filled 1050-B request equals its template byte for byte with only the placeholder replaced.
# Usage: python -I checkfill.py <template name> <filled name> <placeholder> <value>
import os
import sys

root = os.path.join('C:' + os.sep, 'bridge-lab', '1050-B')
t, f, p, v = sys.argv[1:5]
a = open(os.path.join(root, 'req', t), 'rb').read()
b = open(os.path.join(root, 'filled', f), 'rb').read()
print(t, 'bom', a[:2].hex(), len(a), '|', f, 'bom', b[:2].hex(), len(b), '| equal-with-only-placeholder-replaced:',
      b == a.replace(p.encode('utf-16-le'), v.encode('utf-16-le')))
