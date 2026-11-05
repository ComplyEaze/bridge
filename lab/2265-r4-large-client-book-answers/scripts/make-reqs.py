# Lab helper (#2265 R4 on THE CLIENT BOOK, not product code): writes the size reads from R4b's templates
# (lab/register-large-book at 14cf41c4) with SVCURRENTCOMPANY removed, so no company name is typed anywhere: with THE CLIENT
# BOOK loaded alone (C1), Tally answers for the loaded company. extent drops Name, CompanyNumber and GUID from its FETCH and
# its GUID filter. The day census is Bridge's census shape (lab/1392-spanned-census-sheet) with FETCH DATE only and no ALTERID
# bound, one request per day. Usage: python -I -X utf8 make-reqs.py YYYYMM
import calendar, re, sys
T = '<LAB_DIR>/lab/register-large-book/reqs/'; O = '<LAB_DIR>/reqs/'
def rd(n):
    b = open(T + n, 'rb').read(); return b.decode('utf-16') if b[:2] in (b'\xff\xfe', b'\xfe\xff') else b.decode('utf-8-sig')
def wr(n, t): open(O + n, 'wb').write(b'\xff\xfe' + t.encode('utf-16-le')); print(n, len(t))
def nocomp(t): return re.sub(r'\s*<SVCURRENTCOMPANY>[^<]*</SVCURRENTCOMPANY>', '', t)
for n in ('marks.xml', 'catalogue.xml', 'groups.xml'):
    t = nocomp(rd(n)); assert 'CORPUS' not in t; wr(n, t)
t = nocomp(rd('extent.xml'))
t = re.sub(r'<SYSTEM TYPE="Formulae" NAME="BridgeCompanyGuidFilter">[^<]*</SYSTEM>', '', t).replace('<FILTERS>BridgeCompanyGuidFilter</FILTERS>', '')
t = t.replace('Name, GUID, CompanyNumber, BooksFrom', 'BooksFrom'); assert 'CORPUS' not in t and 'CompanyNumber' not in t and '6b43e498' not in t; wr('extent.xml', t)
ym = sys.argv[1]; y, m = int(ym[:4]), int(ym[4:])
for d in range(1, calendar.monthrange(y, m)[1] + 1):
    day = f'{y:04d}{m:02d}{d:02d}'
    t = ('<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Collection</TYPE><ID>Bridge Agent Voucher Census</ID></HEADER>'
         '<BODY><DESC><STATICVARIABLES><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>'
         f'<SVFROMDATE TYPE="Date">{day}</SVFROMDATE><SVTODATE TYPE="Date">{day}</SVTODATE></STATICVARIABLES><TDL><TDLMESSAGE>'
         f'<SYSTEM TYPE="Formulae" NAME="BridgeAgentCensus">$Date &gt;= $$Date:"{day}" AND $Date &lt;= $$Date:"{day}"</SYSTEM>'
         '<COLLECTION NAME="Bridge Agent Voucher Census" ISMODIFY="No"><TYPE>Voucher</TYPE><FETCH>DATE</FETCH><FILTERS>BridgeAgentCensus</FILTERS>'
         '</COLLECTION></TDLMESSAGE></TDL></DESC></BODY></ENVELOPE>')
    wr(f'census-{day}.xml', t)
