# Lab helper (#2016, not product code): raw sends on 9001, each once, saved whole; set-up only, never evidence of its own shape. Shapes copied from the #2016 capture fixtures of this book.
# Usage: python -I -X utf8 seed.py marks <label> | masters <name> | voucher <name> | window <label> <yyyymmdd>
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; C = 'BRIDGE SHAPE LAB'; D = '20260520'
IT, Q, R = 'Hydrochloric Acid 33pc', '1.000 Kgs', '30.00/Kgs'
BUY, SAL, SUP, PUR = 'Outstation Buyer 1', 'Sales - Exempt', 'Chem Supplier 1', 'ITC Test Ledger'
def send(label, body):
    raw = b'\xff\xfe' + body.encode('utf-16-le'); req = f'{L}reqs/{label}.xml'; ans = f'{L}reads/{label}.xml'
    if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
    open(req, 'wb').write(raw); t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
    r = subprocess.run(['curl.exe', '-sS', '--max-time', '300', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    a = open(ans, 'rb').read() if os.path.exists(ans) else b''
    line = f'{t0} {label} req {hashlib.sha256(raw).hexdigest()} exit {r.returncode} printed {r.stdout} {r.stderr.strip()} bytes {len(a)} sha256 {hashlib.sha256(a).hexdigest()}'
    open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line[:200])
    return a.decode('utf-16-le' if a[1:2] == b'\0' else 'utf-8', 'replace')
def counters(s): return {k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LINEERROR', 'LASTVCHID')}
def imp(report, msgs): return f'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>{report}</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA>{msgs}</REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
def batch(godown, name, qty, amount=''): return f'<BATCHALLOCATIONS.LIST><GODOWNNAME>{godown}</GODOWNNAME><BATCHNAME>{name}</BATCHNAME><DESTINATIONGODOWNNAME>{godown}</DESTINATIONGODOWNNAME>' + (f'<AMOUNT>{amount}</AMOUNT>' if amount else '') + f'<ACTUALQTY>{qty}</ACTUALQTY><BILLEDQTY>{qty}</BILLEDQTY></BATCHALLOCATIONS.LIST>'
def item(inward, ledger, godown, b, amount):
    d = 'Yes' if inward else 'No'
    return (f'<ALLINVENTORYENTRIES.LIST><STOCKITEMNAME>{IT}</STOCKITEMNAME><ISDEEMEDPOSITIVE>{d}</ISDEEMEDPOSITIVE><RATE>{R}</RATE><AMOUNT>{amount}</AMOUNT><ACTUALQTY>{Q}</ACTUALQTY><BILLEDQTY>{Q}</BILLEDQTY>'
            + batch(godown, b, Q, amount) + f'<ACCOUNTINGALLOCATIONS.LIST><LEDGERNAME>{ledger}</LEDGERNAME><ISDEEMEDPOSITIVE>{d}</ISDEEMEDPOSITIVE><AMOUNT>{amount}</AMOUNT></ACCOUNTINGALLOCATIONS.LIST></ALLINVENTORYENTRIES.LIST>')
def party(name, inward_stock, amount): return f'<LEDGERENTRIES.LIST><LEDGERNAME>{name}</LEDGERNAME><ISDEEMEDPOSITIVE>{"No" if inward_stock else "Yes"}</ISDEEMEDPOSITIVE><ISPARTYLEDGER>Yes</ISPARTYLEDGER><AMOUNT>{amount}</AMOUNT></LEDGERENTRIES.LIST>'
def inv(vtype, narr, p, ledger, inward, godown, b, invoice='No', extra=''):
    amt = '-30.00' if inward else '30.00'; pamt = '30.00' if inward else '-30.00'
    return (f'<TALLYMESSAGE xmlns:UDF="TallyUDF"><VOUCHER VCHTYPE="{vtype}" ACTION="Create" OBJVIEW="Invoice Voucher View"><DATE>{D}</DATE><EFFECTIVEDATE>{D}</EFFECTIVEDATE><VOUCHERTYPENAME>{vtype}</VOUCHERTYPENAME><PERSISTEDVIEW>Invoice Voucher View</PERSISTEDVIEW><ISINVOICE>{invoice}</ISINVOICE>{extra}'
            f'<PARTYLEDGERNAME>{p}</PARTYLEDGERNAME><NARRATION>{narr}</NARRATION>' + party(p, inward, pamt) + item(inward, ledger, godown, b, amt) + '</VOUCHER></TALLYMESSAGE>')
SJ = (f'<TALLYMESSAGE xmlns:UDF="TallyUDF"><VOUCHER VCHTYPE="Stock Journal" ACTION="Create" OBJVIEW="Consumption Voucher View"><DATE>{D}</DATE><EFFECTIVEDATE>{D}</EFFECTIVEDATE><VOUCHERTYPENAME>Stock Journal</VOUCHERTYPENAME><PERSISTEDVIEW>Consumption Voucher View</PERSISTEDVIEW><NARRATION>lab SJ</NARRATION>'
      f'<INVENTORYENTRIESIN.LIST><STOCKITEMNAME>{IT}</STOCKITEMNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><ACTUALQTY>2.000 Kgs</ACTUALQTY><BILLEDQTY>2.000 Kgs</BILLEDQTY>' + batch('Factory Floor', 'H-2504-A', '2.000 Kgs') + '</INVENTORYENTRIESIN.LIST>'
      f'<INVENTORYENTRIESOUT.LIST><STOCKITEMNAME>{IT}</STOCKITEMNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><RATE>{R}</RATE><AMOUNT>60.00</AMOUNT><ACTUALQTY>2.000 Kgs</ACTUALQTY><BILLEDQTY>2.000 Kgs</BILLEDQTY>' + batch('Main Location', 'H-2504-A', '2.000 Kgs', '60.00') + '</INVENTORYENTRIESOUT.LIST></VOUCHER></TALLYMESSAGE>')
CNA = (f'<TALLYMESSAGE xmlns:UDF="TallyUDF"><VOUCHER VCHTYPE="Credit Note" ACTION="Create" OBJVIEW="Accounting Voucher View"><DATE>{D}</DATE><EFFECTIVEDATE>{D}</EFFECTIVEDATE><VOUCHERTYPENAME>Credit Note</VOUCHERTYPENAME><PARTYLEDGERNAME>{BUY}</PARTYLEDGERNAME><NARRATION>lab CN accounts only</NARRATION>'
       f'<ALLLEDGERENTRIES.LIST><LEDGERNAME>{BUY}</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>30.00</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>{SAL}</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-30.00</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER></TALLYMESSAGE>')
VOUCHERS = {
    'sj': SJ,
    'dn': inv('Delivery Note', 'lab DN', BUY, SAL, False, 'Main Location', 'H-2504-A'),
    'rn': inv('Receipt Note', 'lab RN', SUP, PUR, True, 'Factory Floor', 'H-2504-B'),
    'cn': inv('Credit Note', 'lab CN', BUY, SAL, True, 'Main Location', 'H-2504-A', 'Yes'),
    'owndn': inv('Lab Own DN', 'lab own DN', BUY, SAL, False, 'Main Location', 'H-2504-A'),
    'owncn': inv('Lab Own CN', 'lab own CN', BUY, SAL, True, 'Main Location', 'H-2504-A', 'Yes'),
    'cnacc': CNA,
    'dn2': inv('Delivery Note', 'lab DN to cancel', BUY, SAL, False, 'Main Location', 'H-2504-A'),
    'dn3': inv('Delivery Note', 'lab DN optional', BUY, SAL, False, 'Main Location', 'H-2504-A', 'No', '<ISOPTIONAL>Yes</ISOPTIONAL>'),
    'sales': inv('Sales', 'lab Sales', BUY, SAL, False, 'Main Location', 'H-2504-A', 'Yes'),
    'purchase': inv('Purchase', 'lab Purchase', SUP, PUR, True, 'Factory Floor', 'H-2504-B', 'Yes'),
    'debit': inv('Debit Note', 'lab Debit Note', SUP, PUR, False, 'Factory Floor', 'H-2504-B', 'Yes'),
    'dn9': inv('Delivery Note', 'lab DN step 9', BUY, SAL, False, 'Main Location', 'Primary Batch').replace(IT, 'Descaler Concentrate'),
}
MASTERS = {
    'activate': '<TALLYMESSAGE><VOUCHERTYPE NAME="Delivery Note" ACTION="Alter"><NAME>Delivery Note</NAME><ISACTIVE>Yes</ISACTIVE></VOUCHERTYPE></TALLYMESSAGE><TALLYMESSAGE><VOUCHERTYPE NAME="Receipt Note" ACTION="Alter"><NAME>Receipt Note</NAME><ISACTIVE>Yes</ISACTIVE></VOUCHERTYPE></TALLYMESSAGE>',
    'owntypes': '<TALLYMESSAGE><VOUCHERTYPE NAME="Lab Own DN" ACTION="Create"><NAME>Lab Own DN</NAME><PARENT>Delivery Note</PARENT><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD><ISACTIVE>Yes</ISACTIVE></VOUCHERTYPE></TALLYMESSAGE><TALLYMESSAGE><VOUCHERTYPE NAME="Lab Own CN" ACTION="Create"><NAME>Lab Own CN</NAME><PARENT>Credit Note</PARENT><NUMBERINGMETHOD>Automatic</NUMBERINGMETHOD><ISACTIVE>Yes</ISACTIVE></VOUCHERTYPE></TALLYMESSAGE>',
}
kind = sys.argv[1]
if kind == 'marks':
    t = open('<lab root>/1331/sheet/requests/m-marks.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE PILOT LAB', C)
    s = send(sys.argv[2], t); print(re.findall(r'<(GUID|ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', s))
elif kind == 'masters':
    print(counters(send('m-' + sys.argv[2], imp('All Masters', MASTERS[sys.argv[2]]))))
elif kind == 'voucher':
    print(counters(send('v-' + sys.argv[2], imp('Vouchers', VOUCHERS[sys.argv[2]]))))
elif kind == 'window':
    t = open('<lab folder>/fx/vch-request.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff))
    t = re.sub(r'2026051\d', sys.argv[3], t)
    s = send(sys.argv[2], t); print('vouchers', len(re.findall(r'<VOUCHER ', s)))
