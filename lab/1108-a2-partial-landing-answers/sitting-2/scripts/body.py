# Lab helper (1108-A2 sitting 2, not product code; written whole by an editor): the proxy keeps an import's whole HTTP
# answer, its header block included, as proxy/import-N.answer. This writes the body alone (the bytes after the first blank
# line) as answers/proxy-import-N-body.xml, after checking that the body's length equals the header block's CONTENT-LENGTH
# (else a stop, nothing written). The proxy's file is not changed. Prints both sizes and sha256s.
# Usage: python -I -X utf8 body.py <N>
import hashlib, re, sys
R = '<LAB_DIR>'; N = sys.argv[1]
raw = open(f'{R}/proxy/import-{N}.answer', 'rb').read()
cut = raw.find(b'\r\n\r\n')
if cut < 0: raise SystemExit('stop: no header block end in the proxy answer')
head, body = raw[:cut].decode('ascii'), raw[cut + 4:]
m = re.search(r'(?im)^CONTENT-LENGTH:\s*(\d+)\s*$', head)
if not m or int(m.group(1)) != len(body): raise SystemExit(f'stop: CONTENT-LENGTH {m.group(1) if m else None} against a body of {len(body)} B')
out = f'{R}/answers/proxy-import-{N}-body.xml'
open(out, 'wb').write(body)
print(f'proxy/import-{N}.answer {len(raw)} B sha256 {hashlib.sha256(raw).hexdigest()}; header block {cut + 4} B; '
      f'body {len(body)} B = CONTENT-LENGTH, sha256 {hashlib.sha256(body).hexdigest()} -> answers/proxy-import-{N}-body.xml')
