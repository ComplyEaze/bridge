#!/usr/bin/env python3
"""Writes row 1381-N's fourteen name-and-unit Create requests into requests/.

  python3 -I gen_n.py

Every request is m-r1-name-unit.utf16le.xml with SC Name Unit Only replaced by the row's name, and nothing else.
The template is checked against SHA256.txt before anything is written. Files are UTF-16LE with a BOM and no line end.
"""
import hashlib, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
OLD_NAME = 'SC Name Unit Only'
REQUESTS = (
    ('t1-hyphen-target', 'SN Hyphen Item'),
    ('t2-slash-target', 'SN Slash Item'),
    ('t3-space-target', 'SN Space Item'),
    ('t4-stops-target', 'S.N. Stops Item'),
    ('t5-alias-target', 'SN Alias Item'),
    ('n1-hyphen', 'SN Hyphen-Item'),
    ('n2-slash', 'SN Slash/Item'),
    ('n3-space', 'SN SpaceItem'),
    ('n4-stops', 'SN Stops Item'),
    ('n5-case-gst', 'sc sibling goods'),
    ('n6-spacing-gst', 'SC Copied  Marked'),
    ('n7-group-name', 'SC Goods Group'),
    ('n8-unit-name', 'Nos'),
    ('n9-alias', 'SN Alias Other'),
)


def encode(text):
    return b'\xff\xfe' + text.encode('utf-16-le')


def decode(raw):
    assert raw[:2] == b'\xff\xfe', 'no BOM'
    return raw[2:].decode('utf-16-le')


def kit_sha(name):
    for line in (HERE / 'SHA256.txt').read_text().splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[2] == name:
            return parts[0]
    sys.exit(f'not written: {name} has no line in SHA256.txt')


def main():
    template_path = HERE / 'm-r1-name-unit.utf16le.xml'
    got = hashlib.sha256(template_path.read_bytes()).hexdigest()
    if got != kit_sha(template_path.name):
        sys.exit(f'not written: {template_path.name} is not the kit file (sha256 {got})')
    template = decode(template_path.read_bytes())
    assert template.count(OLD_NAME) == 2, 'template name count is not 2'
    outputs = []
    for stem, name in REQUESTS:
        assert not any(c in name for c in '&<>"\''), f'XML escaping needed: {name}'
        assert name == name.strip(' '), f'name starts or ends with a space: {name}'
        text = template.replace(OLD_NAME, name)
        assert text.count(f'NAME="{name}"') == 1, f'NAME attribute count is not 1: {name}'
        assert text.count(f'<NAME>{name}</NAME>') == 1, f'NAME element count is not 1: {name}'
        assert OLD_NAME not in text, f'old name remains: {name}'
        outputs.append((stem, encode(text)))
    for stem, data in outputs:
        out = HERE / 'requests' / f'{stem}.utf16le.xml'
        out.write_bytes(data)
        print(f'{hashlib.sha256(data).hexdigest()}  {len(data)}  requests/{out.name}')


if __name__ == '__main__':
    main()
