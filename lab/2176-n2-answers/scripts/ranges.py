# Lab helper (#2176 N2, not product code): reads every voucher's narration by an unfiltered Collection and counts any index inside 107800 to 113199; N1's range is the positive control. Narration, since Tally keeps no seed REMOTEID: the book's 54,600 vouchers read back GUID-based ones, and a filter on it answered empty. Reads only. Writes evidence/ranges.txt.
import re, subprocess
W = '<lab folder>/'; C = 'BRIDGE ABANDON LAB'
t = f'''<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Lab Ranges</ID></HEADER><BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT><SVCURRENTCOMPANY>{C}</SVCURRENTCOMPANY><SVFROMDATE TYPE="Date">20250401</SVFROMDATE><SVTODATE TYPE="Date">20260331</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE><COLLECTION NAME="Bridge Lab Ranges" ISMODIFY="No"><TYPE>Voucher</TYPE><FETCH>NARRATION</FETCH></COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>'''
req = W + 'evidence/ranges-request.xml'; ans = W + 'evidence/ranges-answer.xml'
open(req, 'wb').write(b'\xff\xfe' + t.encode('utf-16-le'))
r = subprocess.run(['curl.exe', '-sS', '--max-time', '300', '-w', '%{http_code} %{size_download} %{time_total}', '--data-binary', '@' + req, '-H', 'Content-Type: text/xml; charset=utf-16', 'http://127.0.0.1:9001', '-o', ans], capture_output=True, text=True)
s = open(ans, 'rb').read().decode('utf-16-le', 'replace')
ids = [int(x) for x in re.findall(r'Lab bulk (\d+)', s)]
ids = sorted(set(ids)); inside = [i for i in ids if 107800 <= i <= 113199]
line = f'printed {r.stdout} N1 control 105100 to 107799 found {sum(1 for i in ids if 105100 <= i <= 107799)} ids found {len(ids)} lowest {ids[0] if ids else None} highest {ids[-1] if ids else None} inside 107800 to 113199: {len(inside)}'
print(line); open(W + 'evidence/ranges.txt', 'w', encoding='utf-8', newline='\n').write(line + '\n')
