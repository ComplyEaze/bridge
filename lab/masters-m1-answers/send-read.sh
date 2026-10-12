#!/bin/bash
# usage: [PH=label] m1send.sh <step> <ID>  (one send, never repeated)
step="$1"; id="$2"; R=C:/bridge-lab/masters-m1/requests; E=C:/bridge-lab/masters-m1/evidence
f="$id.utf16le.xml"
want=$(grep "  $f\$" "$R/SHA256.txt" | cut -d' ' -f1)
have=$(sha256sum "$R/$f" | cut -d' ' -f1)
if [ -z "$want" ] || [ "$want" != "$have" ]; then echo "HASH MISMATCH $f want=$want have=$have"; exit 9; fi
cd "$R"
t0=$(date -u +%Y-%m-%dT%H:%M:%SZ)
w=$(curl.exe -sS --data-binary "@$f" -H "Content-Type: text/xml; charset=utf-16" -o "../evidence/$step-$id.response" -D "../evidence/$step-$id.headers" -w "%{http_code} %{size_download}\n" http://localhost:9001/)
rc=$?
t1=$(date -u +%Y-%m-%dT%H:%M:%SZ)
echo "$t1 ${PH:-PHASE 2} $step $id: request sha256 $have OK; sent $t0; curl exit $rc; -w: $w" >> "$E/log.txt"
echo "rc=$rc w=$w t0=$t0 t1=$t1"
