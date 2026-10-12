#!/usr/bin/env bash
# Usage: ./send.sh <checkpoint> <q file name> <out dir>
# Checks the request against q/SHA256.txt, sends it once with curl, records status, size and sha256 in log.txt.
set -u
cp_=$1; req=$2; out=$3
cd /c/bridge-lab/1262
exp=$(grep -E "^[0-9a-f]{64} +\*?$req( |\$)" q/SHA256.txt | head -1 | cut -c1-64)
got=$(sha256sum "q/$req" | cut -c1-64)
if [ -z "$exp" ] || [ "$exp" != "$got" ]; then echo "HASH MISMATCH $req exp=$exp got=$got" | tee -a log.txt; exit 3; fi
ans="$out/$cp_-$req"
t0=$(date -u +%FT%T.%3NZ)
code=$(curl.exe -sS -m 600 -w "%{http_code}" --data-binary "@q/$req" -H "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o "$ans"); rc=$?
t1=$(date -u +%FT%T.%3NZ)
size=$(stat -c %s "$ans" 2>/dev/null || echo 0)
sha=$(sha256sum "$ans" 2>/dev/null | cut -c1-64)
echo "$t0 -> $t1 $cp_ $req -> $ans curl_exit=$rc status=$code bytes=$size sha256=$sha" | tee -a log.txt
