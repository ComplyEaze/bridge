# Lab helper (2377-L Part B, not product code): writes B6's notes line, read from sendlog.txt and from the kit's scripts
# run again on the kept answers. Sends nothing and presses no key. Usage: python -I -X utf8 b6line.py
import subprocess, sys
R='<LAB_DIR>'; K=R+'/kit-99c3/lab/2377-b'
log={l.split('\t')[2]: l.rstrip('\n').split('\t') for l in open(R+'/sendlog.txt',encoding='utf-8') if l.count('\t')>=9}
def run(*a):
    r=subprocess.run([sys.executable,'-I','-X','utf8',*a],capture_output=True,text=True,encoding='utf-8'); return (r.stdout+r.stderr).strip()
def ans(lb): f=log[lb]; return f'{f[0][11:19]}Z {f[8].split()[1]} B {f[8].split()[3][:8]}...'
def mk(lb): o=run(R+'/marks.py',f'{R}/ans/{lb}.answer.xml').splitlines(); return f'{log[lb][0][11:19]}Z '+o[0].split(', ',2)[2]+('' if o[-1]=='COMPANY rows: 1' else ' '+o[-1])
out=['B6 L7.']
for w,n in (('w1','w1-party-l7'),('w2','w2-bill-l7'),('w3','w3-onacct-l7')):
    lb='b6-'+w
    out.append(f'{n}: marks {mk(lb+"-marks-1")}; sent {ans(lb)}; shots/{lb}-after.png f280852a... (no dialog); counters "{run(K+"/l2377b.py","counters",f"{R}/ans/{lb}.answer.xml")}"; marks {mk(lb+"-marks-2")}.')
out.append(f'r-lo-l7 {ans("b6-r-lo")}; lo "{run(K+"/l2377.py","lo",R+"/ans/b6-r-lo.answer.xml")}"; screen: '+' | '.join(run(K+'/bills.py','screen',R+'/ans/b6-r-lo.answer.xml').splitlines())+'.')
out.append('BILLFIXED groups as answered: (1) BILLDATE 1-Oct-25, BILLREF On Account; BILLOP -100.00, BILLCL -100.00, BILLDUE 1-Oct-25, BILLOVERDUE 181, BILLVCHDATE 1-Oct-25, BILLVCHTYPE Journal, BILLVCHNUMBER 16, BILLVCHAMOUNT -100.00, BILLVCHQUALIFIER empty. (2) BILLDATE 31-Mar-26, BILLREF On Account; BILLOP -50.00, BILLCL -50.00, BILLDUE empty, BILLOVERDUE empty, BILLVCHDATE 1-Oct-25, BILLVCHTYPE Journal, BILLVCHNUMBER 17, BILLVCHAMOUNT -50.00, BILLVCHQUALIFIER empty. The two rows differ in BILLDATE, BILLDUE, BILLOVERDUE, BILLVCHNUMBER and the amounts; BILLREF, BILLVCHDATE and BILLVCHTYPE are the same.')
pr=run(K+'/l2377b.py','present',R+'/ans/b6-snap.answer.xml').splitlines()
out.append(f'snap {ans("b6-snap")}; present: l7 '+[l for l in pr if l.startswith('l7 ')][0].split()[1]+'; not exact: '+'; '.join(l for l in pr if l.split()[1] != 'exact')+f'; tie "{run(K+"/l2377b.py","tie","l7",R+"/ans/b6-r-lo.answer.xml",R+"/ans/b6-snap.answer.xml")}". EXPECT -150.00: met.')
line=' '.join(out); open(R+'/notes.txt','a',encoding='utf-8',newline='\n').write(line+'\n'); print(line)
