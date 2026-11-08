#!/usr/bin/env python3
"""Writes row 1381-R1's read requests into requests/ and checks each one against its source.

  python3 -I gen_r1.py

Every read uses the envelope of render_native_masters_request (bridge-tally-protocol/src/native_masters.rs:241-246
at master 1f5dc024), copied below character for character. Each request's collection name, master type and fetch
list are the ones request_shape gives for its kind (native_masters.rs:95-135). The one exception is
a1-candidate-items, which adds four fields to the shipped stock-items fetch list.

The script asserts, and writes nothing if any assertion fails:
- a1-candidate-items is a0-shipped-items with the fetch list changed once, and nothing else;
- a1-candidate-items is row 1381-M's items request (m-items.utf16le.xml, byte-equal to
  lab/1381-stock-item-create requests/items.utf16le.xml at 5bbd6fb2) with that row's collection name
  replaced (twice) by the shipped one, and nothing else;
- requests/marks.utf16le.xml is that sheet's marks request, byte for byte.

Files are UTF-16LE with a BOM and no line end, as row 1381-M's requests are.
"""
import hashlib, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
COMPANY = 'BRIDGE STOCK CREATE LAB'
ENVELOPE = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
            '<ID>{collection}</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>'
            '<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE>'
            '<COLLECTION NAME="{collection}" ISMODIFY="{is_modify}"><TYPE>{master_type}</TYPE><FETCH>{fetch}</FETCH>'
            '<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION></TDLMESSAGE></TDL>'
            '</DESC></BODY></ENVELOPE>')
SHIPPED_ITEMS_FETCH = 'NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST'
CANDIDATE_ITEMS_FETCH = ('NAME, PARENT, GUID, MASTERID, ALTERID, BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, '
                         'HSNDETAILS.LIST, GSTDETAILS.LIST, LANGUAGENAME.LIST')
READS = {
    'a0-shipped-items': ('Bridge Master Stock Items', 'No', 'StockItem', SHIPPED_ITEMS_FETCH),
    'a1-candidate-items': ('Bridge Master Stock Items', 'No', 'StockItem', CANDIDATE_ITEMS_FETCH),
    'groups': ('Bridge Master Stock Groups', 'No', 'StockGroup', 'NAME, PARENT, GUID, MASTERID, ALTERID'),
    'units': ('Bridge Master Units', 'No', 'Unit',
              'NAME, GUID, MASTERID, ALTERID, ORIGINALNAME, DECIMALPLACES, ISSIMPLEUNIT'),
}
M_COLLECTION = 'Bridge Lab 1381 Stock Items'


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
    for name in ('m-items.utf16le.xml', 'requests/marks.utf16le.xml'):
        got = hashlib.sha256((HERE / name).read_bytes()).hexdigest()
        if got != kit_sha(name):
            sys.exit(f'not written: {name} is not the kit file (sha256 {got})')
    texts = {k: ENVELOPE.format(collection=c, company=COMPANY, is_modify=m, master_type=t, fetch=f)
             for k, (c, m, t, f) in READS.items()}
    a0, a1 = texts['a0-shipped-items'], texts['a1-candidate-items']
    assert a0.count(SHIPPED_ITEMS_FETCH) == 1 and a1 == a0.replace(SHIPPED_ITEMS_FETCH, CANDIDATE_ITEMS_FETCH)
    m = decode((HERE / 'm-items.utf16le.xml').read_bytes())
    assert m.count(M_COLLECTION) == 2 and m.replace(M_COLLECTION, 'Bridge Master Stock Items') == a1, \
        'a1 is not 1381-M items with the shipped collection name'
    for k, text in texts.items():
        out = HERE / 'requests' / f'{k}.utf16le.xml'
        out.write_bytes(encode(text))
        data = out.read_bytes()
        print(f'{hashlib.sha256(data).hexdigest()}  {len(data)}  requests/{out.name}')


if __name__ == '__main__':
    main()
