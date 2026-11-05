#!/usr/bin/env python3
"""Writes requests/ from templates/, changing only the company name and the dates.

Run from this folder: python3 -I gen_requests.py
The templates are byte copies of lab/692-identity-check-requests at 15c2802f; with BRIDGE TIEOUT D
and that book's dates filled in, each equals the request the #692 D3 sitting sent live
(lab/692-opening-stock-d3-requests at 6f4f7869). The marks request keeps @@GUID@@, which the lab
fills on the lab PC (LAB-COMMON C2 d), keeping the filled file there.
"""
import pathlib

COMPANY = 'BRIDGE TIEOUT E'
WINDOWS = {
    'w1': ('20240401', '20250331'),
    'w2': ('20250401', '20260331'),
    'w3': ('20250401', '20250930'),
    'w4': ('20251001', '20260331'),
    'w5': ('20240401', '20260331'),
}
DATED = {'t1-pl': 'pl', 't2-bs': 'bs', 't3-ledgers': 'ledgers', 't5-negative-stock': 'negative-stock',
         't6-stock-summary': 'stock-summary'}
here = pathlib.Path(__file__).resolve().parent


def fill(name, **values):
    raw = (here / 'templates' / f'{name}-TEMPLATE.xml').read_bytes()
    assert raw[:2] == b'\xff\xfe', name
    text = raw[2:].decode('utf-16-le')
    for key, value in values.items():
        token = f'@@{key}@@'
        assert token in text, (name, token)
        text = text.replace(token, value)
    return b'\xff\xfe' + text.encode('utf-16-le')


out = here / 'requests'
out.mkdir(exist_ok=True)
(out / 'm-marks-TEMPLATE.utf16le.xml').write_bytes(fill('t-flags', COMPANY=COMPANY))
(out / 'groups.utf16le.xml').write_bytes(fill('t4-groups', COMPANY=COMPANY))
for window, (start, end) in WINDOWS.items():
    for template, short in DATED.items():
        body = fill(template, COMPANY=COMPANY, FROM=start, TO=end)
        assert '@@' not in body[2:].decode('utf-16-le'), (window, template)
        (out / f'{window}-{short}.utf16le.xml').write_bytes(body)
print(len(list(out.iterdir())), 'request files')
