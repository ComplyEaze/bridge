# Lab helper (row 2377-NB, from row 683-OP's send.sh): checks the request file against SHA256.txt (C3), writes the send's UTC time to evidence/sends-utc.txt, sends by run.py; with a third word, captures the whole Tally window at once after the send returns, under that name, whatever run.py said; for M and S answers prints pb.py's lines (pb.py on L answers and --outcome are run by hand, as the sheet's steps say). Usage: bash send.sh <label> <file> [capture-name]
cd <LAB_DIR> || exit 9
L=$1; f=$2; c=$3
h=$(sha256sum "$f" | cut -c1-64); grep -q "^$h  $f\$" SHA256.txt || { echo "HASH DIFFERS $f: nothing sent"; exit 8; }
echo "$(date -u +%FT%T.%3NZ) $L $f (hash equal to the list)" >> evidence/sends-utc.txt
python -I run.py http://127.0.0.1:9001 "$L" "$f"; rc=$?
[ -n "$c" ] && bash shot.sh "$c"
if [ $rc != 0 ]; then echo "$(date -u +%FT%T.%3NZ) $L run.py exit $rc" >> evidence/sends-utc.txt; echo "run.py exit $rc"; exit 7; fi
echo "$(date -u +%FT%T.%3NZ) $L answered" >> evidence/sends-utc.txt
case "$f" in M-*|S-*) python -I -X utf8 pb.py "a-$L-$f" > evidence/pb-$L.txt 2>&1; rc=$?; echo "exit $rc" >> evidence/pb-$L.txt; cat evidence/pb-$L.txt;; esac
