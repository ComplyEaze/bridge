# Lab helper (row 683-TN, from row 683-TA's copy, from row 683-NO's copy, from row 683-XA's copy, from row 683-XF second sitting, from its first sitting): checks the request file against SHA256.txt (C3), writes the send's UTC time to evidence/sends-utc.txt, sends by run.py; for the marks request prints lm.py's lines (tb.py is run by hand, as the sheet's steps say). Usage: bash send.sh <label> <file>
cd <LAB_DIR> || exit 9
L=$1; f=$2
h=$(sha256sum "$f" | cut -c1-64); grep -q "^$h  [0-9]*  $f\$" SHA256.txt || { echo "HASH DIFFERS $f: nothing sent"; exit 8; }
echo "$(date -u +%FT%T.%3NZ) $L $f (hash equal to the list)" >> evidence/sends-utc.txt
python -I run.py http://127.0.0.1:9001 "$L" "$f" || { echo "run.py exit $?"; exit 7; }
echo "$(date -u +%FT%T.%3NZ) $L answered" >> evidence/sends-utc.txt
case "$f" in M-*) python -I -X utf8 lm.py "a-$L-$f" > evidence/lm-$L.txt 2>&1; rc=$?; echo "exit $rc" >> evidence/lm-$L.txt; cat evidence/lm-$L.txt;; esac
