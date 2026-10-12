#!/usr/bin/env bash
# Send ONE W21b request to Tally (port 9001) and log it. Usage: send.sh <request path under requests/> <answer file name>
# Checks the request's SHA-256 against requests/SHA256.txt (r files) or requests/workorder/SHA256.txt first; refuses on a mismatch.
set -u
cd /c/bridge-lab/w21b || exit 9
req="$1"; ans="$2"; base=$(basename "$req")
case "$req" in workorder/*) list=requests/workorder/SHA256.txt;; *) list=requests/SHA256.txt;; esac
exp=$(grep -h -E "^[0-9a-f]{64} +\*?$base( |\$)" $list | head -1 | cut -c1-64)
got=$(sha256sum "requests/$req" | cut -c1-64)
if [ -z "$exp" ] || [ "$exp" != "$got" ]; then
  echo "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) NOT SENT $req: hash $got expected '$exp'" | tee -a answers/log.txt
  exit 8
fi
t0=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
w=$(curl.exe -sS -H "Content-Type: text/xml; charset=utf-16" --data-binary "@requests/$req" -o "answers/$ans" -w "%{http_code} %{size_download} %{time_total}" http://127.0.0.1:9001)
rc=$?
t1=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
sz=$(stat -c %s "answers/$ans" 2>/dev/null || echo -)
sh=$(sha256sum "answers/$ans" 2>/dev/null | cut -c1-64)
echo "$t0 -> $t1 $req -> $ans curl exit $rc printed '$w' bytes $sz sha256 $sh" | tee -a answers/log.txt
