#!/usr/bin/env python3
"""Writes row 2162-L2b's two new requests into this folder and checks the copied ones.

  python3 -I gen_c.py

- vt-read.xml: the voucher-type read of render_native_masters_request for NativeMasterKind::VoucherTypes
  (bridge-tally-protocol/src/native_masters.rs:95-102 and :241-246 at master 1f5dc024), for BRIDGE BILLTRAIL
  LAB. The envelope is copied character for character, as lab/1381-r1/gen_r1.py copies it.
- c2-alter.xml: one All Masters import in Bridge's import envelope (import_envelope.rs:27), the envelope
  byte-equal to lab/1381-r1's, with one VOUCHERTYPE message: NAME="IX SALES" ACTION="Alter", and its PARENT
  set to Journal. Nothing else is in the message. Every value is fixed here; none comes from an answer.

It asserts that M-billtrail.xml and Y0-bt-20251001.xml are lab/2162-ix's files (by sha256, from this
folder's SHA256.txt), and that each new file names the company exactly once. It writes nothing if any assertion fails. UTF-16LE with a BOM.
"""
import hashlib, pathlib, sys

HERE = pathlib.Path(__file__).resolve().parent
COMPANY = 'BRIDGE BILLTRAIL LAB'
READ = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
        '<ID>List of VoucherTypes</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML'
        '</SVEXPORTFORMAT><SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES><TDL><TDLMESSAGE>'
        '<COLLECTION NAME="List of VoucherTypes" ISMODIFY="Yes"><TYPE>VoucherType</TYPE><FETCH>NAME, PARENT, GUID, '
        'MASTERID, ALTERID, ISACTIVE, ISOPTIONAL, NUMBERINGMETHOD</FETCH>'
        '<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE></COLLECTION></TDLMESSAGE></TDL>'
        '</DESC></BODY></ENVELOPE>')
ALTER = ('<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST>'
         '</HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES>'
         '<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>'
         '<TALLYMESSAGE><VOUCHERTYPE NAME="IX SALES" ACTION="Alter"><NAME>IX SALES</NAME><PARENT>Journal</PARENT>'
         '</VOUCHERTYPE></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>')


def kit_sha(name):
    for line in (HERE / 'SHA256.txt').read_text().splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[2] == name:
            return parts[0]
    sys.exit(f'not written: {name} has no line in SHA256.txt')


def main():
    for name in ('M-billtrail.xml', 'Y0-bt-20251001.xml'):
        got = hashlib.sha256((HERE / name).read_bytes()).hexdigest()
        if got != kit_sha(name):
            sys.exit(f'not written: {name} is not the kit file (sha256 {got})')
    texts = {'vt-read.xml': READ.format(company=COMPANY), 'c2-alter.xml': ALTER.format(company=COMPANY)}
    for name, text in texts.items():
        if text.count(COMPANY) != 1:
            sys.exit(f'not written: {name} names the company {text.count(COMPANY)} times')
    for name, text in texts.items():
        out = HERE / name
        out.write_bytes(b'\xff\xfe' + text.encode('utf-16-le'))
        data = out.read_bytes()
        print(f'{hashlib.sha256(data).hexdigest()}  {len(data)}  {name}')


if __name__ == '__main__':
    main()
