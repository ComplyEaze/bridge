#!/usr/bin/env python3
"""Writes row 1381-O's six wider stock-item Create requests into requests/.

  python3 -I gen_o.py

Every request is requests/b2-create.utf16le.xml with SC Copied Marked replaced by the row's name and one counted
change to its GST details, HSN details or parent. The template is checked against SHA256.txt before anything is
written. Files are UTF-16LE with a BOM and no line end.
"""
import hashlib, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
OLD_NAME = 'SC Copied Marked'
REQUESTS = (
    ('o1-two-gst-dates', 'SO Two Gst Dates'),
    ('o2-two-hsn-dates', 'SO Two Hsn Dates'),
    ('o3-cess-rate', 'SO Cess Rate'),
    ('o4-other-rates', 'SO Other Rates'),
    ('o5-root-parent', 'SO Root Parent'),
    ('o6-root-marked', 'SO Root Marked'),
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


def replace_once(text, old, new):
    assert text.count(old) == 1, f'anchor count is not 1: {old}'
    return text.replace(old, new)


def entry(text, tag):
    start_tag, end_tag = f'<{tag}>', f'</{tag}>'
    assert text.count(start_tag) == 1, f'{tag} start count is not 1'
    assert text.count(end_tag) == 1, f'{tag} end count is not 1'
    start = text.index(start_tag)
    end = text.index(end_tag, start) + len(end_tag)
    return text[start:end]


def rate(text, head, old, new):
    old_rate = (f'<GSTRATEDUTYHEAD>{head}</GSTRATEDUTYHEAD>'
                f'<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>{old}</GSTRATE>')
    new_rate = (f'<GSTRATEDUTYHEAD>{head}</GSTRATEDUTYHEAD>'
                f'<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>{new}</GSTRATE>')
    return replace_once(text, old_rate, new_rate)


def main():
    template_path = HERE / 'requests' / 'b2-create.utf16le.xml'
    relative_name = f'requests/{template_path.name}'
    got = hashlib.sha256(template_path.read_bytes()).hexdigest()
    if got != kit_sha(relative_name):
        sys.exit(f'not written: {relative_name} is not the kit file (sha256 {got})')
    template = decode(template_path.read_bytes())
    assert template.count(OLD_NAME) == 2, 'template name count is not 2'
    gst = entry(template, 'GSTDETAILS.LIST')
    hsn = entry(template, 'HSNDETAILS.LIST')

    second_gst = replace_once(gst, '<APPLICABLEFROM>20250401</APPLICABLEFROM>',
                              '<APPLICABLEFROM>20251001</APPLICABLEFROM>')
    for head, old, new in (('CGST', '9', '6'), ('SGST/UTGST', '9', '6'), ('IGST', '18', '12')):
        second_gst = rate(second_gst, head, old, new)
    second_hsn = replace_once(hsn, '<APPLICABLEFROM>20250401</APPLICABLEFROM>',
                              '<APPLICABLEFROM>20251001</APPLICABLEFROM>')

    variants = {}
    variants['o1-two-gst-dates'] = replace_once(template, gst, gst + second_gst)
    variants['o2-two-hsn-dates'] = replace_once(template, hsn, hsn + second_hsn)
    cess = ('<RATEDETAILS.LIST><GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>'
            '<GSTRATEVALUATIONTYPE>&#4; Not Applicable</GSTRATEVALUATIONTYPE></RATEDETAILS.LIST>')
    cess_with_rate = ('<RATEDETAILS.LIST><GSTRATEDUTYHEAD>Cess</GSTRATEDUTYHEAD>'
                      '<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE>'
                      '<GSTRATE>12</GSTRATE></RATEDETAILS.LIST>')
    variants['o3-cess-rate'] = replace_once(template, cess, cess_with_rate)
    other_rates = template
    for head, old, new in (('CGST', '9', '2.5'), ('SGST/UTGST', '9', '2.5'), ('IGST', '18', '5')):
        other_rates = rate(other_rates, head, old, new)
    variants['o4-other-rates'] = other_rates
    parent = '<PARENT>SC Goods Group</PARENT>'
    variants['o5-root-parent'] = replace_once(template, parent, '')
    variants['o6-root-marked'] = replace_once(template, parent, '<PARENT>&#4; Primary</PARENT>')

    outputs = []
    for stem, name in REQUESTS:
        assert not any(c in name for c in '&<>"\''), f'XML escaping needed: {name}'
        assert name == name.strip(' '), f'name starts or ends with a space: {name}'
        text = variants[stem]
        assert text.count(OLD_NAME) == 2, f'template name count is not 2: {name}'
        outputs.append((stem, name, encode(text.replace(OLD_NAME, name))))
    for stem, name, data in outputs:
        out = HERE / 'requests' / f'{stem}.utf16le.xml'
        out.write_bytes(data)
        written = decode(out.read_bytes())
        assert written.count(name) == template.count(OLD_NAME), f'new name count is wrong: {name}'
        assert OLD_NAME not in written, f'old name remains: {name}'
        print(f'{hashlib.sha256(data).hexdigest()}  {len(data)}  requests/{out.name}')


if __name__ == '__main__':
    main()
