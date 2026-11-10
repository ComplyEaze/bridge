# Lab helper (row 2399-R, from row 2480-NM's send.sh): checks each request file against SHA256.txt (C3), refuses a label used before, writes the send's UTC time to evidence/sends-utc.txt, sends by run.py (one call, the files in the order given); with SHOT=<name> set, captures the whole Tally window at once after run.py returns, whatever it said. Then lab.py on each answer by its kind: M- marks (with the book's GUID), N- and D- rows, I- counters, each to evidence/lab-<label>.txt with its exit. Usage: [SHOT=name] bash send.sh <label> <files...>
cd <LAB_DIR> || exit 9
G=6b43e498-430c-4d5c-bfef-d32e2ab93c85
L=$1; shift
for f in "$@"; do h=$(sha256sum "$f" | cut -c1-64); grep -q "^$h  [0-9]*  $f\$" SHA256.txt || { echo "HASH DIFFERS $f: nothing sent"; exit 8; }; done
for f in "$@"; do [ -e "a-$L-$f" ] && { echo "a-$L-$f exists: a label is used once; nothing sent"; exit 9; }; done
echo "$(date -u +%FT%T.%3NZ) $L $* (hashes equal to the list)" >> evidence/sends-utc.txt
python -I run.py http://127.0.0.1:9001 "$L" "$@"; rc=$?
[ -n "$SHOT" ] && bash shot.sh "$SHOT"
if [ $rc != 0 ]; then echo "$(date -u +%FT%T.%3NZ) $L run.py exit $rc" >> evidence/sends-utc.txt; echo "run.py exit $rc"; exit 7; fi
echo "$(date -u +%FT%T.%3NZ) $L answered" >> evidence/sends-utc.txt
: > evidence/lab-$L.txt
for f in "$@"; do
  a=a-$L-$f
  case "$f" in M-*) set -- marks "$a" --guid $G;; N-*|D-*) set -- rows "$a";; I-*) set -- counters "$a";; *) echo "no kind for $f" >> evidence/lab-$L.txt; continue;; esac
  { echo "\$ lab.py $*"; python -I -X utf8 lab.py "$@"; echo "exit $?"; } >> evidence/lab-$L.txt 2>&1
done
cat evidence/lab-$L.txt
