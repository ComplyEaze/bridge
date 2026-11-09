# Lab helper (row 2162-HS): checks each request file against SHA256.txt (C3), writes the send's UTC time to evidence/sends-utc.txt, sends by run.py, counts by count15.py. Usage: bash send.sh <label> <files...>
cd <LAB_DIR> || exit 9
L=$1; shift
for f in "$@"; do h=$(sha256sum "$f" | cut -c1-64); grep -q "^$h  [0-9]*  $f\$" SHA256.txt || { echo "HASH DIFFERS $f: nothing sent"; exit 8; }; done
echo "$(date -u +%FT%T.%3NZ) $L $* (hashes equal to the lists)" >> evidence/sends-utc.txt
python -I run.py http://127.0.0.1:9001 "$L" "$@" || { echo "run.py exit $?"; exit 7; }
python -I count15.py a-$L-* > evidence/count-$L.txt 2>&1; rc=$?; cat evidence/count-$L.txt; echo "count15 exit $rc"
