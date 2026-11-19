# Lab helper (2440-CP): prints snapshot master/voucher alter ids and reused from a saved ledger_masters answer.
import json,sys
j=json.load(open(sys.argv[1],encoding='utf-8')); sc=j['response']['result'].get('structuredContent',{})
def find(o):
  if isinstance(o,dict):
    for k,v in o.items():
      if k=='snapshot' and isinstance(v,dict): yield v
      yield from find(v)
  elif isinstance(o,list):
    for v in o: yield from find(v)
print([{k:s.get(k) for k in ('master_alter_id','voucher_alter_id','reused')} for s in find(sc)])
