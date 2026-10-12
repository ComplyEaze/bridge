#!/usr/bin/env bash
# Send ONE W11 request to Tally (port 9001) and log it. Usage: send.sh <request file> <answer file name>
# Checks the request's SHA-256 against SHA256.txt / SHA256-top.txt first; refuses to send on a mismatch.
set -u
cd /c/bridge-lab/w11 || exit 9
req="$1"; ans="$2"
exp=$(grep -h -E "^[0-9a-f]{64} +\*?$req( |\$)" SHA256.txt SHA256-top.txt | head -1 | cut -c1-64)
got=$(sha256sum "$req" | cut -c1-64)
if [ -z "$exp" ] || [ "$exp" != "$got" ]; then
  echo "$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ) NOT SENT $req: hash $got expected '$exp'" | tee -a answers/log.txt
  exit 8
fi
t0=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
w=$(curl.exe -sS -H "Content-Type: text/xml; charset=utf-16" --data-binary "@$req" -o "answers/$ans" -w "%{http_code} %{size_download}" http://127.0.0.1:9001)
rc=$?
t1=$(date -u +%Y-%m-%dT%H:%M:%S.%3NZ)
sz=$(stat -c %s "answers/$ans" 2>/dev/null || echo -)
sh=$(sha256sum "answers/$ans" 2>/dev/null | cut -c1-64)
echo "$t0 -> $t1 $req -> $ans curl exit $rc printed '$w' bytes $sz sha256 $sh" | tee -a answers/log.txt
