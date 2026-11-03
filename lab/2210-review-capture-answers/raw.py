# Lab helper (#2210, not product code): raw sends on 9001, each once, saved whole. Usage: python -I -X utf8 raw.py marks <label> | ledger
import hashlib, os, re, subprocess, sys
from datetime import datetime, timezone
L = '<lab folder>/'; C = 'BRIDGE REVIEW LAB'
def send(label, reqbytes):
    req = f'{L}reqs/{label}.xml'; ans = f'{L}reads/{label}.xml'; open(req, 'wb').write(reqbytes)
    if os.path.exists(ans): sys.exit(f'{label} exists; never resent')
    t0 = datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.%f')[:-3] + 'Z'
    r = subprocess.run(['curl.exe', '-sS', '--max-time', '120', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', '-D', ans[:-4] + '.headers.txt', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
    a = open(ans, 'rb').read() if os.path.exists(ans) else b''
    line = f'{t0} {label} req {hashlib.sha256(reqbytes).hexdigest()} exit {r.returncode} printed {r.stdout} {r.stderr.strip()} sha256 {hashlib.sha256(a).hexdigest()}'
    open(L + 'evidence/send-log.txt', 'a', encoding='utf-8', newline='\n').write(line + '\n'); print(line[:170])
    return a.decode('utf-16-le', 'replace')
kind = sys.argv[1]
if kind == 'marks':
    t = open('<lab root>/1331/sheet/requests/m-marks.xml', 'rb').read().decode('utf-16-le').lstrip(chr(0xfeff)).replace('BRIDGE PILOT LAB', C)
    s = send(sys.argv[2], b'\xff\xfe' + t.encode('utf-16-le')); print(re.findall(r'<(GUID|ALTVCHID|ALTMSTID)[^>]*>\s*([^<]*)', s))
elif kind == 'setup':
    v = '<VOUCHER VCHTYPE="Journal" ACTION="Create" OBJVIEW="Accounting Voucher View"><DATE>20260401</DATE><EFFECTIVEDATE>20260401</EFFECTIVEDATE><VOUCHERTYPENAME>Journal</VOUCHERTYPENAME><NARRATION>set-up</NARRATION><ALLLEDGERENTRIES.LIST><LEDGERNAME>Test Expense B</LEDGERNAME><ISDEEMEDPOSITIVE>Yes</ISDEEMEDPOSITIVE><AMOUNT>-0.01</AMOUNT></ALLLEDGERENTRIES.LIST><ALLLEDGERENTRIES.LIST><LEDGERNAME>Cash</LEDGERNAME><ISDEEMEDPOSITIVE>No</ISDEEMEDPOSITIVE><AMOUNT>0.01</AMOUNT></ALLLEDGERENTRIES.LIST></VOUCHER>'
    body = f'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>Vouchers</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE xmlns:UDF="TallyUDF">{v}</TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
    s = send('setup-journal', b'\xff\xfe' + body.encode('utf-16-le'))
    print({k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LINEERROR', 'LASTVCHID')})
elif kind == 'ledger':
    body = f'<?xml version="1.0" encoding="UTF-8"?><ENVELOPE><HEADER><TALLYREQUEST>Import Data</TALLYREQUEST></HEADER><BODY><IMPORTDATA><REQUESTDESC><REPORTNAME>All Masters</REPORTNAME><STATICVARIABLES><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY></STATICVARIABLES></REQUESTDESC><REQUESTDATA><TALLYMESSAGE><LEDGER NAME="Test Expense B" ACTION="Create"><NAME>Test Expense B</NAME><PARENT>Indirect Expenses</PARENT><ISBILLWISEON>No</ISBILLWISEON></LEDGER></TALLYMESSAGE></REQUESTDATA></IMPORTDATA></BODY></ENVELOPE>'
    s = send('ledger', b'\xff\xfe' + body.encode('utf-16-le'))
    print({k: re.findall(rf'<{k}>([^<]*)', s) for k in ('CREATED', 'ALTERED', 'ERRORS', 'EXCEPTIONS', 'LINEERROR')})
