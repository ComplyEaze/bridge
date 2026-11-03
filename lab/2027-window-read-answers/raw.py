# Lab helper (#2027 R1, not product code): raw sends on 9001, each once, saved whole. Usage: python -I -X utf8 raw.py marks <label> | day <label> | native <label> | seed <n>
import hashlib, re, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; C = 'BRIDGE NOTES LAB'; K = '<lab folder>/kit/'
ledger = lambda name, amount, deemed, bill='': f'<ALLLEDGERENTRIES.LIST><LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{deemed}</ISDEEMEDPOSITIVE><LEDGERFROMITEM>No</LEDGERFROMITEM><ISPARTYLEDGER>{"Yes" if bill else "No"}</ISPARTYLEDGER><AMOUNT>{amount}</AMOUNT>{bill}</ALLLEDGERENTRIES.LIST>'
def bill(name, amount): return f'<BILLALLOCATIONS.LIST><NAME>{name}</NAME><BILLTYPE>New Ref</BILLTYPE><AMOUNT>{amount}</AMOUNT></BILLALLOCATIONS.LIST>'
def voucher(vtype, date, party, gstin, ref, refdate, reason, note, notedate, entries):
    x = f'<VOUCHER VCHTYPE="{vtype}" ACTION="Create" OBJVIEW="Invoice Voucher View"><DATE>{date}</DATE><EFFECTIVEDATE>{date}</EFFECTIVEDATE><VOUCHERTYPENAME>{vtype}</VOUCHERTYPENAME><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><ISINVOICE>Yes</ISINVOICE>'
    x += f'<PARTYLEDGERNAME>{party}</PARTYLEDGERNAME><PARTYNAME>{party}</PARTYNAME><BASICBUYERNAME>{party}</BASICBUYERNAME><PLACEOFSUPPLY>Maharashtra</PLACEOFSUPPLY><STATENAME>Maharashtra</STATENAME><COUNTRYOFRESIDENCE>India</COUNTRYOFRESIDENCE><GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><VATDEALERTYPE>Regular</VATDEALERTYPE><PARTYGSTIN>{gstin}</PARTYGSTIN><CMPGSTIN>XXXXXXXXXXXXXXX</CMPGSTIN><GSTREGISTRATION>Maharashtra Registration</GSTREGISTRATION>'
    if ref: x += f'<REFERENCE>{ref}</REFERENCE><REFERENCEDATE>{refdate}</REFERENCEDATE>'
    if reason: x += f'<GSTNATUREOFRETURN>{reason}</GSTNATUREOFRETURN><VATPARTYTRANSRETURNNUMBER>{note}</VATPARTYTRANSRETURNNUMBER><VATPARTYTRANSRETURNDATE>{notedate}</VATPARTYTRANSRETURNDATE>'
    return x + entries + '</VOUCHER>'
A, S, SS, PP = 'BRIDGE Customer A', 'BRIDGE Supplier A', 'BRIDGE Svc Sales 5%', 'BRIDGE Svc Purchase 5%'
G1, G2 = 'XXXXXXXXXXXXXXX', 'XXXXXXXXXXXXXXX'
SEEDS = {
 1: voucher('Sales', '20250603', A, G1, '', '', '', '', '', ledger(A, '-1050.00', 'Yes', bill('SYN-BILL-S2', '-1050.00')) + ledger(SS, '1000.00', 'No') + ledger('CGST', '25.00', 'No') + ledger('SGST', '25.00', 'No')),
 2: voucher('Credit Note', '20250604', A, G1, 'SYN-ORIG-1', '20250512', '01-Sales Return', 'SYN-NOTE-1', '20250513', ledger(A, '420.00', 'No', bill('SYN-BILL-C2', '420.00')) + ledger(SS, '-400.00', 'Yes') + ledger('CGST', '-10.00', 'Yes') + ledger('SGST', '-10.00', 'Yes')),
 3: voucher('Debit Note', '20250605', S, G2, 'SYN-ORIG-2', '20250514', '04-Correction in Invoice', 'SYN-NOTE-2', '20250515', ledger(S, '-420.00', 'Yes', bill('SYN-BILL-D2', '-420.00')) + ledger(PP, '400.00', 'No') + ledger('CGST', '10.00', 'No') + ledger('SGST', '10.00', 'No')),
 4: voucher('Purchase', '20250606', S, G2, 'SYN-SUP-1', '20250516', '', '', '', ledger(S, '1050.00', 'No', bill('SYN-BILL-P2', '1050.00')) + ledger(PP, '-1000.00', 'Yes') + ledger('CGST', '-25.00', 'Yes') + ledger('SGST', '-25.00', 'Yes')),
}
KA, GKA = 'BRIDGE Customer KA', 'XXXXXXXXXXXXXXX'
SEEDS[5] = voucher('Sales', '20250607', KA, GKA, '', '', '', '', '', ledger(KA, '-1050.00', 'Yes', bill('SYN-BILL-S5', '-1050.00')) + ledger(SS, '1000.00', 'No') + ledger('IGST', '50.00', 'No')).replace('<PLACEOFSUPPLY>Maharashtra', '<PLACEOFSUPPLY>Karnataka').replace('<STATENAME>Maharashtra', '<STATENAME>Karnataka')
MASTERS = (f'<TALLYMESSAGE><LEDGER NAME="{KA}" ACTION="Create"><NAME>{KA}</NAME><PARENT>Sundry Debtors</PARENT><ISBILLWISEON>Yes</ISBILLWISEON><LEDSTATENAME>Karnataka</LEDSTATENAME><LEDGSTREGDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><GSTREGISTRATIONTYPE>Regular</GSTREGISTRATIONTYPE><PLACEOFSUPPLY>Karnataka</PLACEOFSUPPLY><GSTIN>{GKA}</GSTIN></LEDGSTREGDETAILS.LIST><LEDMAILINGDETAILS.LIST><APPLICABLEFROM>20250401</APPLICABLEFROM><MAILINGNAME>{KA}</MAILINGNAME><STATE>Karnataka</STATE><COUNTRY>India</COUNTRY></LEDMAILINGDETAILS.LIST></LEDGER></TALLYMESSAGE>'
           '<TALLYMESSAGE><LEDGER NAME="IGST" ACTION="Create"><NAME>IGST</NAME><PARENT>Duties &amp; Taxes</PARENT><TAXTYPE>GST</TAXTYPE><GSTDUTYHEAD>IGST</GSTDUTYHEAD><ISBILLWISEON>No</ISBILLWISEON><RATEOFTAXCALCULATION>5</RATEOFTAXCALCULATION></LEDGER></TALLYMESSAGE>')
def send(label, reqbytes):
    req = f'{L}reqs/{label}.xml'; ans = f'{L}reads/{label}.xml'; open(req, 'wb').write(reqbytes)
    import os
    if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
    t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
    r = subprocess.run(['curl.exe', '-sS', '--max-time', '120', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    a = open(ans, 'rb').read() if os.path.exists(ans) else b''
    line = f'{t0} {label} req {hashlib.sha256(reqbytes).hexdigest()} exit {r.returncode} printed {r.stdout} {r.stderr.strip()} sha256 {hashlib.sha256(a).hexdigest()}'
    open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line[:170])
    return a.decode('utf-16-le', 'replace')
kind, arg = sys.argv[1], sys.argv[2]
if kind == 'marks':
    t = open('<lab root>/1331/sheet/requests/m-marks.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE PILOT LAB', C)
    s = send(arg, b'\xff\xfe' + t.encode('utf-16-le')); print(re.findall(r'<(GUID|ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', s))
elif kind == 'masters':
    body = f'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{MASTERS}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
    s = send(arg, b'\xff\xfe' + body.encode('utf-16-le'))
    print({k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LINEERROR')})
elif kind == 'native7':
    t = open(K + 'r-native-20250603-20250606.utf16le.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('20250603', '20250607').replace('20250606', '20250607')
    send(arg, b'\xff\xfe' + t.encode('utf-16-le'))
elif kind == 'catalogue':
    t = open('<lab root>/regl/reqs/catalogue.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE CORPUS DENSE', C)
    send(arg, b'\xff\xfe' + t.encode('utf-16-le'))
elif kind == 'extent':
    t = open('<lab root>/regl/reqs/extent.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE CORPUS DENSE', C)
    send(arg, b'\xff\xfe' + t.encode('utf-16-le'))
elif kind == 'native':
    send(arg, open(K + 'r-native-20250603-20250606.utf16le.xml', 'rb').read())
elif kind == 'seed':
    n = int(arg); variant = sys.argv[3] if len(sys.argv) > 3 else ''; v = SEEDS[n]
    if 'L' in variant: v = v.replace('ALLLEDGERENTRIES.LIST', 'LEDGERENTRIES.LIST')
    if 'A' in variant: v = v.replace('Invoice Voucher View', 'Accounting Voucher View')
    if 'N' in variant: v = v.replace('<ISINVOICE>Yes</ISINVOICE>', '')
    if 'B' in variant: v = re.sub(r'<BILLALLOCATIONS\.LIST>.*?</BILLALLOCATIONS\.LIST>', '', v)
    body = f'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF">{v}</TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
    s = send(f'seed{n}{variant}', b'\xff\xfe' + body.encode('utf-16-le'))
    print({k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LINEERROR', 'LASTVCHID')})
