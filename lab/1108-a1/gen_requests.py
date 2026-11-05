#!/usr/bin/env python3
"""Writes requests/ for row 1108-A1, changing only the company name and the import's messages.

Run from this folder: python3 -I gen_requests.py

templates/ holds three byte copies:
- marks-l0.utf16le.xml and create-l0.utf16le.xml are copied from lab/690-batch-ledger-create-l0 at
  83a5c989, by way of lab/1381-stock-item-create at 5bbd6fb2. ComplyEaze Bridge's own code rendered
  both, and the lab sent them live on 29 Oct 2026.
- post-span-journal-import.xml is master's captured fixture of the same name. It is a Journal batch
  that ComplyEaze Bridge rendered and the lab sent live, recorded in POST_SPAN_CAPTURE_PROVENANCE.md
  with sha256 69c8aa02.

Each write keeps its template's envelope byte for byte; only the messages inside REQUESTDATA are this
row's. Each Journal is that fixture's first voucher with a new date, new ledgers and a new amount: the
shape agent_import.rs:5618 renders, and the REMOTEID of the fixture's first or second voucher (that
company is not this one). Every file is written as BOM-prefixed UTF-16LE, as the fixture was sent.
"""
import pathlib

L0_COMPANY = 'BRIDGE LEDGERBOX CI LAB'
FIXTURE_COMPANY = 'BRIDGE AMEND LAB'
COMPANY = 'BRIDGE STOCK CREATE LAB'
here = pathlib.Path(__file__).resolve().parent


def read(name):
    raw = (here / 'templates' / name).read_bytes()
    if raw[:2] == b'\xff\xfe':
        return raw[2:].decode('utf-16-le')
    return raw.decode('utf-8')


def write(name, text):
    assert L0_COMPANY not in text and FIXTURE_COMPANY not in text, name
    (here / 'requests' / name).write_bytes(b'\xff\xfe' + text.encode('utf-16-le'))


def collection(ident, kind, fetch, dates=''):
    return ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            f'<ID>{ident}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>'
            f'<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY>{dates}</STATICVARIABLES><TDL><TDLMESSAGE>'
            f'<COLLECTION NAME="{ident}" ISMODIFY="No"><TYPE>{kind}</TYPE><FETCH>{fetch}</FETCH>'
            '<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION>'
            '</TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')


def replace_data(template, message):
    head, rest = template.split('<REQUESTDATA>')
    _, tail = rest.split('</REQUESTDATA>')
    return f'{head}<REQUESTDATA>{message}</REQUESTDATA>{tail}'


def ledger(name, parent):
    return (f'<TALLYMESSAGE><LEDGER NAME="{name}" ACTION="Create"><NAME>{name}</NAME><PARENT>{parent}</PARENT>'
            '<ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE>')


def journal(remote_id, narration, debit, credit, amount):
    def entry(name, positive, value):
        return (f'<ALLLEDGERENTRIES.LIST><LEDGERNAME>{name}</LEDGERNAME>'
                f'<ISDEEMEDPOSITIVE>{positive}</ISDEEMEDPOSITIVE><AMOUNT>{value}</AMOUNT></ALLLEDGERENTRIES.LIST>')
    return (f'<TALLYMESSAGE xmlns:UDF="TallyUDF"><VOUCHER REMOTEID="{remote_id}" VCHTYPE="Journal" '
            'ACTION="Create" OBJVIEW="Accounting Voucher View"><DATE>20250410</DATE>'
            f'<VOUCHERTYPENAME>Journal</VOUCHERTYPENAME><NARRATION>{narration}</NARRATION>'
            + entry(debit, 'Yes', f'-{amount}') + entry(credit, 'No', amount) + '</VOUCHER></TALLYMESSAGE>')


fixture = read('post-span-journal-import.xml')
remote_ids = [part.split('"', 1)[0] for part in fixture.split('REMOTEID="')[1:3]]
assert len(remote_ids) == 2 and remote_ids[0] != remote_ids[1], remote_ids

masters = read('create-l0.utf16le.xml').replace(L0_COMPANY, COMPANY)
vouchers = fixture.replace(FIXTURE_COMPANY, COMPANY)
WRITES = {
    'w1-stock-ledger': replace_data(masters, ledger('A1 Closing Stock', 'Stock-in-Hand')),
    'w2-capital-ledger': replace_data(masters, ledger('A1 Capital', 'Capital Account')),
    'a1-stock-journal': replace_data(vouchers, journal(remote_ids[0], 'Row 1108-A1 stock leg',
                                                       'A1 Closing Stock', 'A1 Capital', '100.00')),
    'b-cash-journal': replace_data(vouchers, journal(remote_ids[1], 'Row 1108-B cash leg',
                                                     'Cash', 'A1 Capital', '200.00')),
}

out = here / 'requests'
out.mkdir(exist_ok=True)
write('marks.utf16le.xml', read('marks-l0.utf16le.xml').replace(L0_COMPANY, COMPANY))
write('flags.utf16le.xml', collection('Bridge Lab 1108 Company Flags', 'Company',
                                      'NAME, GUID, ISINVENTORYON, ISINTEGRATED'))
write('ledgers.utf16le.xml', collection('Bridge Lab 1108 Ledgers', 'Ledger',
                                        'NAME, PARENT, GUID, MASTERID, ALTERID'))
write('vouchers.utf16le.xml', collection(
    'Bridge Lab 1108 Vouchers', 'Voucher',
    'DATE, VOUCHERTYPENAME, NARRATION, GUID, MASTERID, ALTERID, '
    'ALLLEDGERENTRIES.LEDGERNAME, ALLLEDGERENTRIES.AMOUNT',
    # the book's whole first year, the dates the verification read of agent_import.rs:5704 also sends
    '<SVFROMDATE TYPE="Date">20250401</SVFROMDATE><SVTODATE TYPE="Date">20260331</SVTODATE>'))
for name, text in WRITES.items():
    write(f'{name}.utf16le.xml', text)
print(len(list(out.iterdir())), 'request files')
