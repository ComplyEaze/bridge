#!/usr/bin/env python3
# n.py: row N's request writer (lab item #2433, not product code). Run with python -I. Exit 3 = STOP, 2 = usage.
#   n.py gen   writes, beside this script, from L-ledgers-shape-lab.utf16le.xml:
#              L-ledgers-long.utf16le.xml       SVCURRENTCOMPANY holds the 300-character pattern (measure.py pattern)
#              L-ledgers-prefix255.utf16le.xml  SVCURRENTCOMPANY holds the pattern's first 255 characters
#              It asserts, for each, that the company name is the only change, byte for byte, and prints the
#              pattern's sha256 and each file's bytes and sha256. The pattern is ASCII letters only, so nothing in
#              it needs XML escaping.
import hashlib
import pathlib
import sys

SYNTHETIC = 'BRIDGE SHAPE LAB'
SOURCE = 'L-ledgers-shape-lab.utf16le.xml'
SOURCE_SHA = '61c9b17f265e3f1e9dc9d6997488dc41e65ae49a2850233e16df523abd041373'
LETTERS = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcd'
PATTERN = ''.join(letter * 10 for letter in LETTERS)  # the same as measure.py's PATTERN
PATTERN_SHA = '1d6366e1ccaa762b2da904319dc3a5385513fc401f1c814bb7a16bb0a03e433e'
TARGETS = (('L-ledgers-long.utf16le.xml', PATTERN), ('L-ledgers-prefix255.utf16le.xml', PATTERN[:255]))
here = pathlib.Path(__file__).resolve().parent


def stop(message):
    print(f'STOP: {message}')
    sys.exit(3)


def gen():
    if hashlib.sha256(PATTERN.encode()).hexdigest() != PATTERN_SHA or len(PATTERN) != 300:
        stop('the pattern is not measure.py\'s')
    raw = (here / SOURCE).read_bytes()
    if hashlib.sha256(raw).hexdigest() != SOURCE_SHA:
        stop(f'{SOURCE} is not the file this script was written for')
    assert raw[:2] == b'\xff\xfe', 'the source starts with a UTF-16LE byte-order mark'
    text = raw[2:].decode('utf-16-le')
    old = f'<SVCURRENTCOMPANY>{SYNTHETIC}</SVCURRENTCOMPANY>'
    assert text.count(old) == 1, 'the source names the company once'
    assert text.count(SYNTHETIC) == 1, 'the name appears nowhere else'
    assert text.count('SVCURRENTCOMPANY') == 2, 'the source has one SVCURRENTCOMPANY element'
    name_at = raw.index(old.encode('utf-16-le')) + len('<SVCURRENTCOMPANY>'.encode('utf-16-le'))
    name_end = name_at + len(SYNTHETIC.encode('utf-16-le'))
    print(f'pattern: {len(PATTERN)} characters, sha256 {PATTERN_SHA}')
    print(f'{SOURCE}: {len(raw)} bytes, sha256 {SOURCE_SHA}')
    for target, name in TARGETS:
        assert name.isascii() and name.isalpha(), 'letters only: no escaping'
        out = b'\xff\xfe' + text.replace(old, f'<SVCURRENTCOMPANY>{name}</SVCURRENTCOMPANY>').encode('utf-16-le')
        # The only change: the name's bytes, and nothing else.
        assert out == raw[:name_at] + name.encode('utf-16-le') + raw[name_end:], 'only the name differs'
        assert len(out) == len(raw) + 2 * (len(name) - len(SYNTHETIC))
        (here / target).write_bytes(out)
        print(f'{target}: {len(out)} bytes, sha256 {hashlib.sha256(out).hexdigest()}, name {len(name)} characters')


if __name__ == '__main__':
    if sys.argv[1:] != ['gen']:
        print('usage: n.py gen')
        sys.exit(2)
    gen()
