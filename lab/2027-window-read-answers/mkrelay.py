# Lab helper (#2027 R1, not product code): the R0 relay, its sha256 checked, with one addition: each request and answer's bytes saved beside the log, named by connection and request number. Nothing else changes.
import hashlib
R = open('<lab root>/2176r0/relay.py', 'rb').read()
assert hashlib.sha256(R).hexdigest() == '38a2115baaf6d3a3327af4a49a1b0c76d862f04048724040e2bc75e4c783e23b'
s = R.decode('utf-8')
a = '                drop = self.take_drop(body)\n'
b = a + '                with open(os.path.join(self.outdir, "req-%d-%04d.bin" % (conn, served)), "wb") as saved:\n                    saved.write(body)\n                with open(os.path.join(self.outdir, "ans-%d-%04d.bin" % (conn, served)), "wb") as saved:\n                    saved.write(answer)\n'
assert s.count(a) == 1
open('<lab folder>/relay.py', 'w', encoding='utf-8', newline='\n').write(s.replace(a, b)); print('relay written')
