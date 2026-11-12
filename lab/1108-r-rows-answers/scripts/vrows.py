# Lab helper (rows 1108-R0 to R2, new; not product code): prints each voucher of a chk.py rows output (evidence/chk-<name>.txt):
# GUID tail, MASTERID, ALTERID, VOUCHERNUMBER, DATE, NARRATION and its ledger entries. Reads only. Usage: python -I -X utf8 vrows.py <name>
import re, sys
t = open(f'<LAB_DIR>/evidence/chk-{sys.argv[1]}.txt', encoding='utf-8').read()
for blk in re.split(r'\n(?=VOUCHER )', t):
    if not blk.startswith('VOUCHER'): continue
    g = re.search(r'GUID (\S+)', blk).group(1)[-8:]
    def f(k):
        m = re.search(rf"  {k} '([^']*)'", blk); return m.group(1) if m else None
    line = blk.split('ALLLEDGERENTRIES.LIST', 1)[1].split('\n')[0] if 'ALLLEDGERENTRIES.LIST' in blk else ''
    amts = re.findall(r'<LEDGERNAME[^>]*>([^<]*)</LEDGERNAME>.*?<AMOUNT[^>]*>([^<]*)</AMOUNT>', line)
    print(g, 'MASTERID', f('MASTERID'), 'ALTERID', f('ALTERID'), 'VOUCHERNUMBER', f('VOUCHERNUMBER'), 'DATE', f('DATE'), 'NARRATION', repr(f('NARRATION')), amts)
