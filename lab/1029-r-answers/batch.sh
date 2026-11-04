# Lab helper (1029-R, not product code): step 3 a to f of one batch, stopping (exit 1) at the first thing that is not the
# plain path: a build with no batch id or another count; a post whose dispatch is not posted_verified with created equal
# to the count (the lab then follows the sheet by hand); a verify_import that is not posted_verified n of n; marks not
# V + n and M 211. Usage: bash batch.sh <b> <n> <V before> [built: a 4th argument skips the build and uses its saved answer]
set -u; R=<LAB_DIR>; cd $R; b=$1; n=$2; v0=$3
[ $# -ge 4 ] && echo "build already done (no rebuild): using its saved answer" || { bash step3.sh build $b || exit 1; }
f=$(ls run/answers/*-$b-build.json | tail -1)
read id cnt < <(python -I -X utf8 -c "import json,sys; r=json.load(open(sys.argv[1],encoding='utf-8'))['response']['result'].get('structuredContent',{}).get('result') or {}; print(r.get('batch_id'), r.get('voucher_count'))" $f | tr -d '\r')
python -I -X utf8 -c "import json,sys; r=json.load(open(sys.argv[1],encoding='utf-8'))['response']['result']['structuredContent']['result']; print('warnings', len(r.get('warnings') or []), [w[:60] for w in r.get('warnings') or []])" $f
echo "build: batch $id count $cnt"; [ "$cnt" = "$n" ] && [ "$id" != None ] || { echo "HALT at build"; exit 1; }
bash step3.sh post $b $id $n $b-p1
p=$(ls run/answers/*-$b-p1.json | tail -1); python -I -X utf8 fields.py $p
ok=$(python -I -X utf8 -c "import json,sys; d=json.load(open(sys.argv[1],encoding='utf-8'))['response']['result']['structuredContent']['result'].get('dispatch') or {}; print(d.get('state')=='posted_verified' and (d.get('counters') or {}).get('created')==int(sys.argv[2]))" $p $n)
[ "$ok" = True ] || { echo "HALT after post: not the plain path; follow the sheet by hand"; exit 1; }
python -I cap.py $b-after
printf '{"company_guid":"2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab","batch_id":"%s"}' $id > args-verify-$b.json
python -I -X utf8 call.py run $b-verify verify_import args-verify-$b.json > /dev/null
vf=$(ls run/answers/*-$b-verify.json | tail -1); python -I -X utf8 fields.py $vf
vok=$(python -I -X utf8 -c "import json,sys; r=json.load(open(sys.argv[1],encoding='utf-8'))['response']['result']['structuredContent']['result']; c=r.get('counts') or {}; print(r.get('verification_status')=='posted_verified' and c.get('posted_verified')==int(sys.argv[2]) and sum(c.values())==int(sys.argv[2]))" $vf $n)
python -I -X utf8 call.py run $b-egress egress_log args-egress.json > /dev/null
ef=$(ls run/answers/*-$b-egress.json | tail -1)
python -I -X utf8 -c "
import json,sys,re
r=json.load(open(sys.argv[1],encoding='utf-8'))['response']['result']['structuredContent']['result']; recs=[json.loads(s) if isinstance(s,str) else s for s in r['records']]
pp=[x for x in recs if x.get('record_type')=='response_prepared' and x.get('tool')=='post_import']
t=json.dumps(recs); print('egress records', len(recs), '| post_import response_prepared lines', len(pp), '| deadline/refus words in the log', len(re.findall(r'deadline|refus', t, re.I)))
for x in pp: tr=x.get('request_trail') or {}; print('  post', x['ts'], 'rows', x.get('rows_prepared'), 'trail sent', tr.get('sent'), 'failed', tr.get('failed'), 'outcomes', sorted({y.get('outcome') for y in tr.get('last') or []}))
print('kinds', sorted({x.get('record_type') for x in recs}))" $ef
m=$(python -I -X utf8 marks.py $b-marks); echo "$m"
echo "$m" | grep -q "V $((v0+n)) M 211" || { echo "HALT: marks not V $((v0+n)) M 211"; exit 1; }
[ "$vok" = True ] || { echo "HALT: verify_import not posted_verified $n of $n"; exit 1; }
echo "BATCH $b PLAIN PATH DONE"
