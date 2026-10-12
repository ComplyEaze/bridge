#!/usr/bin/env python3 -I
"""Write the W15 request: the vouchers whose AlterID lies in (BEFORE, AFTER], and refuse a malformed one.

usage: python3 -I make-w15-request.py BEFORE AFTER FROM TO OUTFILE
  BEFORE, AFTER: the company's ALTVCHID read before and after the export (integers, BEFORE < AFTER, AFTER - BEFORE <= 20)
  FROM, TO: the date window, YYYYMMDD, FROM <= TO (a collection with no dates returns only the display period)
The output is a BOM-prefixed UTF-16LE Collection request whose header ID equals its COLLECTION NAME; the
script checks that, and prints the file's SHA-256 and byte count. Exit 0 only when the file was written
and re-read to the same hash; any other exit writes nothing.
"""
import hashlib, re, sys

def main(argv):
    if len(argv) != 6:
        print(__doc__); return 2
    before, after = argv[1], argv[2]
    d_from, d_to, out = argv[3], argv[4], argv[5]
    for v in (before, after):
        if not re.fullmatch(r'[0-9]{1,9}', v):
            print('not an integer:', v); return 2
    b, a = int(before), int(after)
    if not (b < a <= b + 20):
        print('need BEFORE < AFTER <= BEFORE + 20, got', b, a); return 2
    for v in (d_from, d_to):
        if not re.fullmatch(r'20[0-9]{2}(0[1-9]|1[0-2])(0[1-9]|[12][0-9]|3[01])', v):
            print('not a date YYYYMMDD:', v); return 2
    if d_from > d_to:
        print('FROM after TO'); return 2
    name = 'Bridge Altered Since W15'
    xml = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE>'
           '<ID>%s</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>'
           '<SVCURRENTCOMPANY>BRIDGE PILOT LAB</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">%s</SVFROMDATE>'
           '<SVTODATE TYPE="Date">%s</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE>'
           '<SYSTEM TYPE="Formulae" NAME="BridgeAlteredSince">$AlterID &gt; %d AND $AlterID &lt;= %d</SYSTEM>'
           '<COLLECTION NAME="%s" ISMODIFY="No"><TYPE>Voucher</TYPE>'
           '<FETCH>DATE, VOUCHERNUMBER, VOUCHERTYPENAME, MASTERID, ALTERID, GUID, NARRATION, ISCANCELLED, ISOPTIONAL</FETCH>'
           '<FILTERS>BridgeAlteredSince</FILTERS></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>'
           % (name, d_from, d_to, b, a, name))
    ident = re.search(r'<ID>([^<]*)</ID>', xml).group(1)
    names = re.findall(r'<COLLECTION\b[^>]*\bNAME="([^"]*)"', xml)
    if ident not in names:
        print('ID names no defined collection'); return 2
    data = b'\xff\xfe' + xml.encode('utf-16-le')
    open(out, 'wb').write(data)
    back = open(out, 'rb').read()
    if back != data:
        print('re-read differs'); return 2
    print('%s  %d bytes  %s' % (hashlib.sha256(back).hexdigest(), len(back), out))
    return 0

sys.exit(main(sys.argv))
