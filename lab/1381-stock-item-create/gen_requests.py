#!/usr/bin/env python3
"""Writes requests/ for row 1381-M, changing only the company name and the import's messages.

Run from this folder: python3 -I gen_requests.py
templates/ holds byte copies of two request files of lab/690-batch-ledger-create-l0 at 83a5c989
(marks.utf16le.xml and create-a.utf16le.xml), which ComplyEaze Bridge's own code rendered at master
5b2a352d and the lab sent live on 29 Oct 2026. The marks request is used as it is, with this book's name.
The import keeps that envelope byte for byte; only the messages inside REQUESTDATA are this row's.
The STOCKITEM messages are hand-written candidates: no stock-item Create builder exists yet. The shape
each one answers is meant to become the stock-item Create message of #1381, beside the ledger Create
of src-tauri/src/tally/ledger_create_item.rs, in the same masters import envelope.

Request r2 is not written here: make_r2.py renders it on the lab PC from the keyed item's own answer.
"""
import pathlib

L0_COMPANY = 'BRIDGE LEDGERBOX CI LAB'
COMPANY = 'BRIDGE STOCK CREATE LAB'
here = pathlib.Path(__file__).resolve().parent


def read(name):
    raw = (here / 'templates' / name).read_bytes()
    assert raw[:2] == b'\xff\xfe', name
    return raw[2:].decode('utf-16-le')


def write(name, text):
    assert L0_COMPANY not in text, name
    (here / 'requests' / name).write_bytes(b'\xff\xfe' + text.encode('utf-16-le'))


def collection(ident, kind, fetch):
    return ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            f'<ID>{ident}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>'
            f'<SVCURRENTCOMPANY>{COMPANY}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE>'
            f'<COLLECTION NAME="{ident}" ISMODIFY="No"><TYPE>{kind}</TYPE><FETCH>{fetch}</FETCH>'
            '<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION>'
            '</TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')


def item(name, body):
    return (f'<TALLYMESSAGE><STOCKITEM NAME="{name}" ACTION="Create"><NAME>{name}</NAME>{body}'
            '</STOCKITEM></TALLYMESSAGE>')


# One GST rate row as Tally's own answer arranges it (lab/hsn-masters-answers, ans-h03b5.xml), written
# without the TYPE attributes, as the ledger Create of L0 was. &#4; is the reserved-name mark Tally
# prints before Any and Not Applicable.
R3_GST = ('<HSNDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><HSNCODE>39269099</HSNCODE>'
          '<HSN>Other articles of plastics</HSN><SRCOFHSNDETAILS>Specify Details Here</SRCOFHSNDETAILS>'
          '</HSNDETAILS.LIST>'
          '<GSTDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><TAXABILITY>Taxable</TAXABILITY>'
          '<SRCOFGSTDETAILS>Specify Details Here</SRCOFGSTDETAILS><STATEWISEDETAILS.LIST>'
          '<STATENAME>&#4; Any</STATENAME>'
          '<RATEDETAILS.LIST><GSTRATEDUTYHEAD>CGST</GSTRATEDUTYHEAD>'
          '<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>2.50</GSTRATE></RATEDETAILS.LIST>'
          '<RATEDETAILS.LIST><GSTRATEDUTYHEAD>SGST/UTGST</GSTRATEDUTYHEAD>'
          '<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>2.50</GSTRATE></RATEDETAILS.LIST>'
          '<RATEDETAILS.LIST><GSTRATEDUTYHEAD>IGST</GSTRATEDUTYHEAD>'
          '<GSTRATEVALUATIONTYPE>Based on Value</GSTRATEVALUATIONTYPE><GSTRATE>5</GSTRATE></RATEDETAILS.LIST>'
          '</STATEWISEDETAILS.LIST></GSTDETAILS.LIST>')

WRITES = {
    'r1-name-unit': item('SC Name Unit Only', '<BASEUNITS>Nos</BASEUNITS>'),
    'r3-existing-name': item('SC Copied Goods',
                             '<PARENT>SC Goods Group</PARENT><BASEUNITS>Nos</BASEUNITS>' + R3_GST),
    'r4a-case': item('sc name unit only', '<BASEUNITS>Nos</BASEUNITS>'),
    'r4b-spacing': item('SC Name  Unit Only', '<BASEUNITS>Nos</BASEUNITS>'),
    'r5a-absent-unit': item('SC Absent Unit Item', '<BASEUNITS>SCX</BASEUNITS>'),
    'r5b-absent-group': item('SC Absent Group Item', '<PARENT>SC Absent Group</PARENT><BASEUNITS>Nos</BASEUNITS>'),
}

out = here / 'requests'
out.mkdir(exist_ok=True)
write('marks.utf16le.xml', read('marks-l0.utf16le.xml').replace(L0_COMPANY, COMPANY))
write('items.utf16le.xml', collection(
    'Bridge Lab 1381 Stock Items', 'StockItem',
    'NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, '
    'HSNDETAILS.LIST, GSTDETAILS.LIST, LANGUAGENAME.LIST'))
write('groups.utf16le.xml', collection('Bridge Lab 1381 Stock Groups', 'StockGroup',
                                       'NAME, PARENT, GUID, MASTERID, ALTERID'))
write('units.utf16le.xml', collection('Bridge Lab 1381 Units', 'Unit', 'NAME, GUID, MASTERID, ALTERID'))

envelope = read('create-l0.utf16le.xml').replace(L0_COMPANY, COMPANY)
head, rest = envelope.split('<REQUESTDATA>')
_, tail = rest.split('</REQUESTDATA>')
for name, message in WRITES.items():
    write(f'{name}.utf16le.xml', f'{head}<REQUESTDATA>{message}</REQUESTDATA>{tail}')
print(len(list(out.iterdir())), 'request files')
