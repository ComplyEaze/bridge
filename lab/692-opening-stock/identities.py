#!/usr/bin/env python3
"""Check of the three stock identities on Tally's own answers.
Usage: identities.py PL.xml BS.xml LEDGERS.xml GROUPS.xml   (answers as received, UTF-16LE)
Prints verdicts and counts only: no names, no amounts. Signs as Tally prints them (debit negative)."""
import re,sys
from decimal import Decimal as D
def text(p):
    b=open(p,'rb').read()
    return b.decode('utf-16-le' if not b.startswith(b'\xff\xfe') else 'utf-16')
def num(s): return D(s) if s.strip() else D(0)
def lines(t):
    names=[re.sub('&amp;','&',n) for n in re.findall(r'<DSPDISPNAME>(.*?)</DSPDISPNAME>',t)]
    subs=re.findall(r'<(?:PLSUBAMT|BSSUBAMT)>(.*?)</(?:PLSUBAMT|BSSUBAMT)>',t)
    mains=re.findall(r'<BSMAINAMT>(.*?)</BSMAINAMT>',t)
    return list(zip(names,subs,mains))
pl=lines(text(sys.argv[1])); bs=lines(text(sys.argv[2])); led=text(sys.argv[3]); grp=text(sys.argv[4])
once=lambda rows,n:[r for r in rows if r[0]==n]
o=once(pl,'Opening Stock'); c=once(pl,'Less: Closing Stock')
ok_shape=len(o)==1 and len(c)==1
O=num(o[0][1]) if ok_shape else None; C=num(c[0][1]) if ok_shape else None
L={}
for m in re.finditer(r'<LEDGER NAME="([^"]*)".*?</LEDGER>',led,re.S):
    b=m.group(0); g=lambda k:(re.search(r'<%s[^>]*>(.*?)</%s>'%(k,k),b,re.S) or [0,''])[1]
    L[m.group(1)]=(g('PARENT'),num(g('TBALOPENING')),num(g('TBALCLOSING')))
total=sum(v[2] for v in L.values())
print('opening/closing stock lines each present once:',ok_shape)
print('ledger rows:',len(L),' ledgers whose parent names Stock-in-Hand:',sum('Stock-in-Hand' in v[0] for v in L.values()))
if ok_shape:
    print('identity 3 (ledger closings sum == -O):',total==-O)
    bsm={r[0]:num(r[2]) for r in bs}
    ca=bsm.get('Current Assets'); print('Balance Sheet export balances (sum of its main amounts == 0 with assets negative):', sum(bsm.values())==0)
    print('opening stock line is a debit and closing a debit:',O<=0 and C<=0)
    parent={}
    for m in re.finditer(r'<GROUP NAME="([^"]*)".*?</GROUP>',grp,re.S):
        pm=re.search(r'<PARENT[^>]*>(.*?)</PARENT>',m.group(0),re.S); parent[m.group(1).replace('&amp;','&')]=(pm.group(1) if pm else '').replace('&amp;','&')
    def primary(g):
        seen=0
        while g in parent and parent[g] and 'Primary' not in parent[g] and seen<50: g=parent[g]; seen+=1
        return g
    nominal={'Sales Accounts','Purchase Accounts','Direct Incomes','Direct Expenses','Indirect Incomes','Indirect Expenses'}
    ca=D(0); nom=D(0); pl_led=D(0); unknown=0
    for name,(par,opn,cls) in L.items():
        par=par.replace('&amp;','&')
        if 'Primary' in par:
            if name.startswith('Profit'): pl_led+=cls
            else: unknown+=1
            continue
        pg=primary(par)
        if pg=='Current Assets': ca+=cls
        elif pg in nominal: nom+=cls
        elif pg not in ('Capital Account','Current Liabilities','Loans (Liability)','Fixed Assets','Investments','Suspense A/c','Misc. Expenses (ASSET)'): unknown+=1
    print('ledgers not classified by this script:',unknown)
    bsm2={r[0]:num(r[2]) for r in bs}
    t_ca=bsm2.get('Current Assets',D(0)); t_pl=bsm2.get('Profit & Loss A/c',D(0))
    print('identity 1 (Tally Current Assets - ledger Current Assets == C):',t_ca-ca==C)
    print('identity 2 (carried P&L line gap == O - C):',t_pl-(pl_led+nom)==O-C)
