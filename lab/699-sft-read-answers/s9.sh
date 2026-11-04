#!/bin/bash
# Lab helper (699-R1, not product code): step 8's one-minute wait, then step 9's marks raw to 9001 by curl, and the comparison with s1.
cd <LAB_DIR> || exit 9
echo "wait start $(date -u +%FT%TZ)"; sleep 60
T=$(date -u +%FT%T.%3NZ)
curl.exe -sS -w "%{http_code} %{size_download} %{time_total}" --data-binary @sheet/requests/marks-sft.utf16le.xml -H "Content-Type: text/xml; charset=utf-16" -D reads/s9-marks.headers.txt http://127.0.0.1:9001 -o reads/s9-marks.xml; echo " sent $T"
sha256sum reads/s1-marks.xml reads/s9-marks.xml
cmp reads/s1-marks.xml reads/s9-marks.xml && echo "s9 BYTE-IDENTICAL to s1"
