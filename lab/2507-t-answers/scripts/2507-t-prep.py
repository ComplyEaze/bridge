# Lab helper (row 2507-T, new; not product code): stages the row folder before the sitting. Copies the sheet (extracted from
# lab/2507-t-sheet at db2d6651) byte for byte and checks each file against its SHA256.txt; copies the requests to requests\; copies the
# stdio client, the call and show helpers and the C2 helper from row 1761-S4 byte for byte; writes shot.sh with the folder changed; writes
# the launch file by json.dump (no posting setting: Bridge only reads in this row); writes c3-send-mt.ps1 from the lab's c3-send.ps1 with
# --max-time 60 added to its curl line (the sheet's rule) and send.ps1, which sends one sheet request by it; writes the read arguments;
# makes the data and backups\B0 folders new and empty. Deletes nothing; stops if the row folder exists.
# Usage: python -I -X utf8 2507-t-prep.py
import hashlib, json, os, shutil
L = '<lab folder>'; R = L + '/2507-t'; SH = L + '/2507-t-sheet-db2d/lab/2507-t-sheet'; S4 = L + '/1761-s4'
GUID = '3a6bd6e1-b835-4bff-89dd-8a6af138c346'  # row 2016's recorded GUID for BRIDGE SHAPE LAB (lab/2016-stock-lines-answers at 186c39d6)
assert not os.path.exists(R), R
def sha(p): return hashlib.sha256(open(p, 'rb').read()).hexdigest()
for d in ('', 'requests', 'raw', 'answers', 'inbox', 'args', 'evidence', 'shots', 'step0', 'data', 'backups/B0', 'sheet', 'aside'):
    os.makedirs(f'{R}/{d}')
log = []
want = {}
for line in open(SH + '/SHA256.txt', encoding='utf-8'):
    parts = line.split()
    if parts: want[parts[-1]] = parts[0]
for n in sorted(os.listdir(SH)):
    shutil.copyfile(f'{SH}/{n}', f'{R}/sheet/{n}')
    if n != 'SHA256.txt': assert sha(f'{R}/sheet/{n}') == want[n], n
    log.append(f'sheet/{n} {os.path.getsize(f"{R}/sheet/{n}")} B sha256 {sha(f"{R}/sheet/{n}")} (byte copy' + ('' if n == 'SHA256.txt' else ', equal to SHA256.txt') + ')')
for n in ('t0.xml', 't1.xml', 't2.xml', 't3.xml', 'marks.xml'):
    shutil.copyfile(f'{SH}/{n}', f'{R}/requests/{n}'); assert sha(f'{R}/requests/{n}') == want[n], n
    log.append(f'requests/{n} sha256 {want[n]} (byte copy of the sheet file)')
for n in ('client.py', 'call.py', 'show.py', 'step0.py'):
    shutil.copyfile(f'{S4}/{n}', f'{R}/{n}'); log.append(f'{n} {os.path.getsize(f"{R}/{n}")} B sha256 {sha(f"{R}/{n}")} (byte copy from row 1761-S4)')
s = open(S4 + '/shot.sh', encoding='utf-8').read()
a, b = "(row 1761-S4, row 2346-N134's shot.sh with the folder changed;", "(row 2507-T, row 1761-S4's shot.sh with the folder changed;"
assert s.count(a) == 1 and s.count('<ROW_1761S4_DIR>/shots/') == 1
s = s.replace(a, b).replace('<ROW_1761S4_DIR>/shots/', '<LAB_DIR>/shots/')
open(R + '/shot.sh', 'w', encoding='utf-8', newline='\n').write(s); log.append(f'shot.sh sha256 {sha(R + "/shot.sh")} (row 1761-S4 copy, folder changed)')
env = {'BRIDGE_TALLY_HOST': '127.0.0.1', 'BRIDGE_TALLY_PORT': '9001', 'BRIDGE_AGENT_DATA_DIR': '<LAB_DIR>\\data', 'BRIDGE_TERMS_ACCEPTED': 'true'}
with open(R + '/launch-env.json', 'w', encoding='utf-8', newline='\n') as f: json.dump(env, f, indent=1); f.write('\n')
assert json.load(open(R + '/launch-env.json', encoding='utf-8')) == env
log.append(f'launch-env.json sha256 {sha(R + "/launch-env.json")} keys {sorted(env)} (no posting setting, no redaction, no byte limit)')
c = open(L + '/evidence-tools/c3-send.ps1', encoding='ascii').read()
reps = [('# Lab helper: LAB-COMMON v3 C3 send, one request, the exact curl line;', '# Lab helper (row 2507-T: the lab c3-send.ps1 with --max-time 60 on its curl line, the sheet\'s rule): LAB-COMMON v3 C3 send, one request, the exact curl line;'),
        ('& curl.exe -sS -w "%{http_code} %{size_download}"', '& curl.exe -sS --max-time 60 -w "%{http_code} %{size_download}"')]
for x, y in reps:
    assert c.count(x) == 1, x; c = c.replace(x, y)
c.encode('ascii')
open(R + '/c3-send-mt.ps1', 'w', encoding='ascii', newline='\r\n').write(c); log.append(f'c3-send-mt.ps1 sha256 {sha(R + "/c3-send-mt.ps1")} (the lab c3-send.ps1 {sha(L + "/evidence-tools/c3-send.ps1")} with --max-time 60)')
send = r'''# Lab helper (row 2507-T, new; not product code): sends one of the sheet's requests (t0, t1, t2, t3 or marks) by c3-send-mt.ps1, its
# sha256 taken from sheet\SHA256.txt, bound 2,000,000, answer and headers to raw\<Ans>; logs to evidence\send-log.txt; stops on a non-zero exit.
# Usage: powershell -NoProfile -File send.ps1 -Name <t0|t1|t2|t3|marks> -Ans <answer file name>
param([Parameter(Mandatory = $true)][string]$Name, [Parameter(Mandatory = $true)][string]$Ans)
$R = '<LAB_DIR>'
$row = @(Get-Content "$R\sheet\SHA256.txt" | Where-Object { ($_ -split '\s+')[-1] -eq "$Name.xml" })
if ($row.Count -ne 1) { throw "HALT: $Name.xml not found once in SHA256.txt" }
$reqSha = ($row[0] -split '\s+')[0]
& "$R\c3-send-mt.ps1" -Req "$R\requests\$Name.xml" -Sha $reqSha -AnsDir "$R\raw" -Ans $Ans -Bound 2000000 -Log "$R\evidence\send-log.txt"
$code = $LASTEXITCODE
$t1 = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.fffZ')
Add-Content -Encoding ascii "$R\evidence\send-log.txt" "$t1 answered $Name as $Ans exit=$code"
if ($code -ne 0) { throw "HALT: c3-send-mt exit $code on $Name" }
'''
open(R + '/send.ps1', 'w', encoding='ascii', newline='\r\n').write(send); log.append(f'send.ps1 sha256 {sha(R + "/send.ps1")}')
args = {'none': {}, 'items': {'company_guid': GUID, 'kind': 'stock_items'},
        'v0520': {'company_guid': GUID, 'from': '2026-05-20', 'to': '2026-05-20'},
        'v0521': {'company_guid': GUID, 'from': '2026-05-21', 'to': '2026-05-21'},
        'v0522-31': {'company_guid': GUID, 'from': '2026-05-22', 'to': '2026-05-31'},
        'v0521-sl': {'company_guid': GUID, 'from': '2026-05-21', 'to': '2026-05-21', 'stock_lines': True}}
for k, v in args.items():
    with open(f'{R}/args/{k}.json', 'w', encoding='utf-8', newline='\n') as f: json.dump(v, f, indent=1)
    log.append(f'args/{k}.json sha256 {sha(f"{R}/args/{k}.json")}')
assert not os.listdir(R + '/data') and not os.listdir(R + '/backups/B0')
log.append('data and backups/B0 new and empty')
open(R + '/evidence/prep.txt', 'w', encoding='utf-8', newline='\n').write('\n'.join(log) + '\n')
print('\n'.join(log))
