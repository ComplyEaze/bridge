#!/usr/bin/env bash
# Send ONE 1050-C request to Tally (port 9001) and log it. Usage: send.sh <request file in req/ or filled/> <answer name>
# Static requests (b1, b2) are hash-checked against req/SHA256.txt; filled templates are checked against filled/FILLED-SHA256.txt.
set -u
cd /c/bridge-lab/1050-C || exit 9
req="$1"; ans="$2"
base=$(basename "$req")
if [ -f "req/$base" ] && [ "$req" = "req/$base" ]; then list=req/SHA256.txt; else list=filled/FILLED-SHA256.txt; fi
exp=$(grep -h -E "^[0-9a-f]{64} +\*?$base( |\$)" "$list" | head -1 | cut -c1-64)
got=$(sha256sum "$req" | cut -c1-64)
if [ -z "$exp" ] || [ "$exp" != "$got" ]; then
  echo "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) NOT SENT $req: hash $got expected '$exp'" | tee -a answers/log.txt
  exit 8
fi
t0=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
w=$(curl.exe -sS -w "%{http_code} %{size_download}" --data-binary "@$req" -H "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o "answers/$ans")
rc=$?
t1=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
sz=$(stat -c %s "answers/$ans" 2>/dev/null || echo -)
sh=$(sha256sum "answers/$ans" 2>/dev/null | cut -c1-64)
echo "$t0 -> $t1 $req -> $ans curl exit $rc printed '$w' bytes $sz sha256 $sh" | tee -a answers/log.txt
