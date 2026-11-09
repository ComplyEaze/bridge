# Lab helper (row 2162-GY): prints two answers' size and sha256 and every line that differs between them (unified, no context).
# Answers are UTF-16LE (with or without a BOM); text files UTF-8. Sends nothing and presses no key.
# Usage: python -I -X utf8 linediff-gy.py <answer A> <answer B>
import sys, difflib, hashlib
a, b = sys.argv[1], sys.argv[2]
ra, rb = open(a, 'rb').read(), open(b, 'rb').read()
def dec(r):
    if r[:2] == b'\xff\xfe': return r[2:].decode('utf-16-le')
    if b'\x00' in r[:400]: return r.decode('utf-16-le')
    return r.decode('utf-8')
ta, tb = dec(ra).splitlines(), dec(rb).splitlines()
print(f'== {a} ({len(ra)} B, sha256 {hashlib.sha256(ra).hexdigest()}) vs {b} ({len(rb)} B, sha256 {hashlib.sha256(rb).hexdigest()})')
n = 0
for l in difflib.unified_diff(ta, tb, lineterm='', n=0):
    if l.startswith(('---', '+++')): continue
    print('  ' + l.strip()[:200]); n += 1
print(f'  {n} diff lines')
