# Lab helper (1108-R3; new, not run on Windows; not product code). Makes and checks requests/e1-equal-first.utf16le.xml, the
# one raw write of this row: a Journal equal in date, type, ledgers, amounts and sides to the first voucher of
# batches/r3.json (b1108-p2-1), with that voucher's narration. The template is the seed Journal's request of rows 1108-R0 to
# R2 (same element order and shape; its file is not in this kit), with the six values of FIELDS put in; the REMOTEID is one
# no other request of the lab uses. It was made once on macOS and its shape compared with that seed's after dropping those
# six values; it has not met Tally.
# Modes: `check` only reads: it rebuilds the file in memory and compares it with the committed one byte for byte, and
# prints "e1 request: ok" (exit 0), "FAIL ..." (exit 1) or "NOT READ ..." (exit 2). `make` writes the file, and refuses
# to overwrite one; the lab runs `check` only.
# Usage: python -I make_e1.py check|make
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, '..', 'requests', 'e1-equal-first.utf16le.xml')
SEED = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA>'
        '<REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>BRIDGE STOCK CREATE LAB</SVCURRENTCOMPANY>'
        '</STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF"><VOUCHER REMOTEID="{rid}" VCHTYPE="Journal" '
        'ACTION="Create" OBJVIEW="Accounting Voucher View"><DATE>{date}</DATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME>'
        '<NARRATION>{narr}</NARRATION><ALLLEDGERENTRIES.LIST><LEDGERNAME>{dr}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE>'
        '<AMOUNT>-{amt}</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>{cr}</LEDGERNAME>'
        '<ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>{amt}</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE>'
        '</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>')
# FIELDS: the values of b1108-p2-1 in batches/r3.json; rid is a REMOTEID this request alone uses.
FIELDS = dict(rid='a3e53387-16d8-4002-b46b-a4dc9bce70d4', date='20250414', narr='Row 1108-P2 first',
              dr='A2 Expense', cr='A1 Capital', amt='21.00')
def build(): return b'\xff\xfe' + SEED.format(**FIELDS).encode('utf-16-le')
if len(sys.argv) != 2 or sys.argv[1] not in ('check', 'make'): raise SystemExit('usage: make_e1.py check|make')
want = build()
if sys.argv[1] == 'make':
    if os.path.exists(OUT): raise SystemExit('STOP: ' + OUT + ' exists; nothing written')
    open(OUT, 'wb').write(want); print('written', len(want), 'B')
else:
    try: have = open(OUT, 'rb').read()
    except OSError as e: print('NOT READ: %s: %s' % (OUT, e.strerror)); sys.exit(2)
    if have != want: print('FAIL: the committed request differs from the rebuilt one (%d B against %d B)' % (len(have), len(want))); sys.exit(1)
    print('e1 request: ok (%d B)' % len(have))
