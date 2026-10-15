#!/usr/bin/env python3 -I
"""Re-encode the import file ComplyEaze Bridge's build_import_xml saved (UTF-8) as UTF-16LE with a BOM, the encoding the
lab sends every request in (LAB-COMMON C3). Nothing else about the file changes.

Run: python -I to-utf16.py IN.xml OUT.xml "COMPANY NAME" --sha256 HEX64 --bank "BANK LEDGER" --suspense "SUSPENSE LEDGER"
       --narrations DIR --voucher YYYYMMDD:Receipt|Payment:N.NN [--voucher ...] [--text YYYYMMDD=FILE ...]
The file is admitted only when it IS the build's file and holds exactly the vouchers the step names: IN's sha256 equals
--sha256 (either case; the value the build answered and the program recorded for the batch); one voucher for each
--voucher and no other, each ACTION="Create", of that type and date, with exactly two entries, the bank ledger and the
suspense ledger, of that amount, debit on the suspense ledger for a Payment and on the bank ledger for a Receipt; every
narration ends in ` [BRIDGE:<its REMOTEID>]`. Dates must differ from one another. DIR must be absent or empty: each
voucher's narration as sent (UTF-8, no line break added) is written to DIR/<date>.txt for the comparison with the stored
one. With --text DATE=FILE (UTF-8, no BOM, no final line break) that voucher's narration without its marker must equal the
file's text (for a narration the assistant typed).
Exit 0: OUT written and checked. Exit 3: the file is not what the step expects (nothing written).
Exit 2: the script could not run (never a pass). Output is ASCII only.
"""
import hashlib, os, re, sys
from decimal import Decimal, InvalidOperation
import xml.etree.ElementTree as ET

HEAD = b'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE>'
MARKER = re.compile(r' \[BRIDGE:([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})\]$')

def stop(why):
    print('STOP: ' + why)
    return 3

def parse_args(rest):
    one = {}
    vouchers, texts = [], {}
    index = 0
    while index < len(rest):
        flag = rest[index]
        if index + 1 >= len(rest):
            return None
        value = rest[index + 1]
        if flag in ('--sha256', '--bank', '--suspense', '--narrations') and flag not in one:
            one[flag] = value
        elif flag == '--voucher':
            found = re.fullmatch(r'([0-9]{8}):(Receipt|Payment):([0-9]+\.[0-9]{2})', value)
            if not found:
                return None
            vouchers.append((found.group(1), found.group(2), Decimal(found.group(3))))
        elif flag == '--text':
            found = re.fullmatch(r'([0-9]{8})=(.+)', value)
            if not found or found.group(1) in texts:
                return None
            texts[found.group(1)] = found.group(2)
        else:
            return None
        index += 2
    if sorted(one) != ['--bank', '--narrations', '--sha256', '--suspense'] or not vouchers:
        return None
    one['--sha256'] = one['--sha256'].lower()  # Get-FileHash prints upper case
    if not re.fullmatch(r'[0-9a-f]{64}', one['--sha256']):
        return None
    if len({d for d, _, _ in vouchers}) != len(vouchers) or not set(texts) <= {d for d, _, _ in vouchers}:
        return None
    return one, vouchers, texts

def main(argv):
    parsed = parse_args(argv[4:]) if len(argv) >= 4 else None
    if parsed is None:
        print(__doc__)
        return 2
    options, specs, texts = parsed
    src, dst, company = argv[1], argv[2], argv[3]
    if os.path.exists(dst):
        return stop('OUT exists; a file is never overwritten')
    narrations = options['--narrations']
    if os.path.exists(narrations) and (not os.path.isdir(narrations) or os.listdir(narrations)):
        return stop('--narrations DIR is not absent or empty')
    raw = open(src, 'rb').read()
    if hashlib.sha256(raw).hexdigest() != options['--sha256']:
        return stop('IN sha256 %s is not the sha256 the build recorded (%s)' % (hashlib.sha256(raw).hexdigest(), options['--sha256']))
    if not raw.startswith(HEAD):
        return stop('the file does not start with the UTF-8 declaration and ENVELOPE a Bridge build writes (a BOM or another encoding?)')
    text = raw.decode('utf-8')
    root = ET.fromstring(text.split('?>', 1)[1])
    def one(path):
        found = root.findall(path)
        return found[0].text if len(found) == 1 else None
    if one('HEADER/TALLYREQUEST') != 'Import Data':
        return stop('TALLYREQUEST is not Import Data')
    if one('BODY/IMPORTDATA/REQUESTDESC/REPORTNAME') != 'Vouchers':
        return stop('REPORTNAME is not Vouchers')
    if one('BODY/IMPORTDATA/REQUESTDESC/STATICVARIABLES/SVCURRENTCOMPANY') != company:
        return stop('SVCURRENTCOMPANY is not ' + company)
    vouchers = root.findall('BODY/IMPORTDATA/REQUESTDATA/TALLYMESSAGE/VOUCHER')
    if len(vouchers) != len(specs) or len(root.findall('.//VOUCHER')) != len(specs):
        return stop('voucher count %d, expected %d' % (len(root.findall('.//VOUCHER')), len(specs)))
    bank, suspense = options['--bank'], options['--suspense']
    by_date = {}
    for voucher in vouchers:
        by_date.setdefault(voucher.findtext('DATE'), []).append(voucher)
    written = []
    for date, want_type, amount in specs:
        found = by_date.get(date, [])
        if len(found) != 1:
            return stop('%d vouchers dated %s, expected 1' % (len(found), date))
        voucher = found[0]
        debit_ledger, credit_ledger = (suspense, bank) if want_type == 'Payment' else (bank, suspense)
        if voucher.get('ACTION') != 'Create':
            return stop('voucher %s is not ACTION=Create' % date)
        if voucher.get('VCHTYPE') != want_type or voucher.findtext('VOUCHERTYPENAME') != want_type:
            return stop('voucher %s is not a %s' % (date, want_type))
        entries = voucher.findall('ALLLEDGERENTRIES.LIST')
        if len(entries) != 2:
            return stop('voucher %s has %d entries, not 2' % (date, len(entries)))
        try:
            debit = [e for e in entries if e.findtext('ISDEEMEDPOSITIVE') == 'Yes']
            credit = [e for e in entries if e.findtext('ISDEEMEDPOSITIVE') == 'No']
            if len(debit) != 1 or len(credit) != 1 or debit[0].findtext('LEDGERNAME') != debit_ledger or credit[0].findtext('LEDGERNAME') != credit_ledger \
                    or Decimal(debit[0].findtext('AMOUNT')) != -amount or Decimal(credit[0].findtext('AMOUNT')) != amount:
                raise ValueError
        except (ValueError, InvalidOperation, TypeError):
            return stop('voucher %s: the entries are not Dr %s / Cr %s of %s' % (date, ascii(debit_ledger), ascii(credit_ledger), amount))
        narration = voucher.findtext('NARRATION') or ''
        found_marker = MARKER.search(narration)
        if not found_marker or found_marker.group(1) != voucher.get('REMOTEID'):
            return stop('voucher %s: the narration does not end in [BRIDGE:<its REMOTEID>]' % date)
        if date in texts:
            expected = open(texts[date], 'rb').read()
            if expected.startswith((b'\xef\xbb\xbf', b'\xff\xfe', b'\xfe\xff')) or b'\x00' in expected or expected.endswith((b'\n', b'\r')):
                return stop('--text file for %s is not UTF-8 without a BOM and without a final line break' % date)
            if MARKER.sub('', narration) != expected.decode('utf-8'):
                return stop('voucher %s: the narration is not the text of its --text file; do NOT send' % date)
        print('voucher %s: narration %d characters, ends in its marker, REMOTEID %s' % (date, len(narration), voucher.get('REMOTEID')))
        written.append((date, narration))
    out = b'\xff\xfe' + text.encode('utf-16-le')
    os.makedirs(narrations, exist_ok=True)
    with open(dst, 'xb') as handle:
        handle.write(out)
    for date, narration in written:
        with open(os.path.join(narrations, date + '.txt'), 'xb') as handle:
            handle.write(narration.encode('utf-8'))
    back = open(dst, 'rb').read()
    if back != out or back[2:].decode('utf-16-le').encode('utf-8') != raw:
        return stop('the written file does not decode back to the input; delete OUT')
    print('IN  %d bytes sha256 %s' % (len(raw), hashlib.sha256(raw).hexdigest()))
    print('OUT %d bytes sha256 %s' % (len(back), hashlib.sha256(back).hexdigest()))
    return 0

if __name__ == '__main__':
    try:
        sys.exit(main(sys.argv))
    except Exception as error:
        print('could not run: %s: %s' % (type(error).__name__, str(error).encode('ascii', 'backslashreplace').decode('ascii')))
        sys.exit(2)
