#!/usr/bin/env python3
"""Writes requests/ and batches/ for rows 1108-P1, P2 and P3 (A1-post, A2 and A2').

Run from this folder: python3 -I gen_requests.py

requests/ is row 1108-A1's (lab/1108-a1-stock-in-hand-journal at 72b54de8) without its two raw
Journals, plus the ledger Create W3. templates/ holds A1's two byte copies, which came from
lab/690-batch-ledger-create-l0 at 83a5c989 by way of lab/1381-stock-item-create at 5bbd6fb2:
ComplyEaze Bridge's own code rendered both, and the lab sent them live on 29 Oct 2026. Each Create
keeps its template's envelope byte for byte; only the message inside REQUESTDATA is this row's.
Every request is written as BOM-prefixed UTF-16LE.

templates/b-cash-journal.utf16le.xml is row 1108-B's request byte for byte (Cash Dr, A1 Capital Cr,
200.00, 10-Apr-2025), which this book answered CREATED 1 on 5 Nov 2026. It is copied unchanged to
requests/s0-seed-journal.utf16le.xml: the seed that gives the book a voucher mark, without which the
build refuses as empty_book_first_import.

batches/ holds the three `vouchers` arrays for build_import_xml, written as UTF-8 JSON. No voucher
here is sent raw: each goes to Tally only through post_import.
"""
import json
import pathlib

L0_COMPANY = 'BRIDGE LEDGERBOX CI LAB'
COMPANY = 'BRIDGE STOCK CREATE LAB'
STOCK, CAPITAL, EXPENSE = 'A1 Closing Stock', 'A1 Capital', 'A2 Expense'
here = pathlib.Path(__file__).resolve().parent


def read(name):
    raw = (here / 'templates' / name).read_bytes()
    if raw[:2] == b'\xff\xfe':
        return raw[2:].decode('utf-16-le')
    return raw.decode('utf-8')


def write(name, text):
    assert L0_COMPANY not in text, name
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


def journal(txn, date, narration, debit, credit, amount):
    return {'bridge_txn_id': txn, 'date': date, 'voucher_type': 'Journal', 'narration': narration,
            'entries': [{'ledger': debit, 'amount': amount, 'side': 'Dr'},
                        {'ledger': credit, 'amount': amount, 'side': 'Cr'}]}


# One date per post, all in the book's first year. Amounts differ across posts, so no voucher of one
# post has the date and figures of another's, and every bridge_txn_id is new.
TWIN = ('2025-04-16', 'Row 1108-P3 twin', EXPENSE, CAPITAL, '31.00')
BATCHES = {
    'p1': [journal('b1108-p1-1', '2025-04-12', 'Row 1108-P1 stock leg', STOCK, CAPITAL, '10.00')],
    'p2': [journal('b1108-p2-1', '2025-04-14', 'Row 1108-P2 first', EXPENSE, CAPITAL, '21.00'),
           journal('b1108-p2-2', '2025-04-14', 'Row 1108-P2 stock leg', STOCK, CAPITAL, '22.00'),
           journal('b1108-p2-3', '2025-04-14', 'Row 1108-P2 third', EXPENSE, CAPITAL, '23.00')],
    'p3': [journal('b1108-p3-1', *TWIN),
           journal('b1108-p3-2', '2025-04-16', 'Row 1108-P3 stock leg', STOCK, CAPITAL, '32.00'),
           journal('b1108-p3-3', *TWIN)],
}
txns = [v['bridge_txn_id'] for b in BATCHES.values() for v in b]
assert len(txns) == len(set(txns)), txns
p3 = BATCHES['p3']
assert {k: v for k, v in p3[0].items() if k != 'bridge_txn_id'} == \
       {k: v for k, v in p3[2].items() if k != 'bridge_txn_id'}

masters = read('create-l0.utf16le.xml').replace(L0_COMPANY, COMPANY)
for folder in ('requests', 'batches'):
    (here / folder).mkdir(exist_ok=True)
write('marks.utf16le.xml', read('marks-l0.utf16le.xml').replace(L0_COMPANY, COMPANY))
write('flags.utf16le.xml', collection('Bridge Lab 1108 Company Flags', 'Company',
                                      'NAME, GUID, ISINVENTORYON, ISINTEGRATED'))
write('ledgers.utf16le.xml', collection('Bridge Lab 1108 Ledgers', 'Ledger',
                                        'NAME, PARENT, GUID, MASTERID, ALTERID'))
write('vouchers.utf16le.xml', collection(
    'Bridge Lab 1108 Vouchers', 'Voucher',
    'DATE, VOUCHERTYPENAME, NARRATION, GUID, MASTERID, ALTERID, '
    'ALLLEDGERENTRIES.LEDGERNAME, ALLLEDGERENTRIES.AMOUNT',
    '<SVFROMDATE TYPE="Date">20250401</SVFROMDATE><SVTODATE TYPE="Date">20260331</SVTODATE>'))
write('w1-stock-ledger.utf16le.xml', replace_data(masters, ledger(STOCK, 'Stock-in-Hand')))
write('w2-capital-ledger.utf16le.xml', replace_data(masters, ledger(CAPITAL, 'Capital Account')))
write('w3-expense-ledger.utf16le.xml', replace_data(masters, ledger(EXPENSE, 'Indirect Expenses')))
(here / 'requests' / 's0-seed-journal.utf16le.xml').write_bytes(
    (here / 'templates' / 'b-cash-journal.utf16le.xml').read_bytes())
for name, vouchers in BATCHES.items():
    (here / 'batches' / f'{name}.json').write_text(json.dumps(vouchers, separators=(',', ':')) + '\n',
                                                     encoding='utf-8')
print(len(list((here / 'requests').iterdir())), 'request files,',
      len(list((here / 'batches').iterdir())), 'batches')
