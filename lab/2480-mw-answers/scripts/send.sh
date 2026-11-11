# Lab helper (row 2480-MW, from row 2480-MX, from row 2480-NX, from row 2480-NC, from row 2480-NS, from row 2480-NM's send.sh, from row 2346-TD's; LM- and I- answers are left to st.py --import, run by hand): checks each request file against SHA256.txt (C3), writes the send's UTC time to evidence/sends-utc.txt, sends by run.py (one call, the files in the order given); with SHOT=<name> set, captures the whole Tally window at once after run.py returns, whatever it said. Then counts: S-, B- and L-lo- answers by nm.py (evidence/nm-<label>.txt), M and N answers by count20.py (evidence/count-<label>.txt), and (row 2480-MX) a V answer by wx.py (evidence/wx-<label>.txt; a label carries at most one V file; LM- and I- answers are left to st.py). They share no answer file. Usage: [SHOT=name] bash send.sh <label> <files...>
cd <LAB_DIR> || exit 9
L=$1; shift
for f in "$@"; do h=$(sha256sum "$f" | cut -c1-64); grep -q "^$h  [0-9]*  $f\$" SHA256.txt || { echo "HASH DIFFERS $f: nothing sent"; exit 8; }; done
for f in "$@"; do [ -e "a-$L-$f" ] && { echo "a-$L-$f exists: a label is used once; nothing sent"; exit 9; }; done
echo "$(date -u +%FT%T.%3NZ) $L $* (hashes equal to the list)" >> evidence/sends-utc.txt
python -I run.py http://127.0.0.1:9001 "$L" "$@"; rc=$?
[ -n "$SHOT" ] && bash shot.sh "$SHOT"
if [ $rc != 0 ]; then echo "$(date -u +%FT%T.%3NZ) $L run.py exit $rc" >> evidence/sends-utc.txt; echo "run.py exit $rc"; exit 7; fi
echo "$(date -u +%FT%T.%3NZ) $L answered" >> evidence/sends-utc.txt
P=(); C=()
for f in "$@"; do case "$f" in S-*|B-*|L-lo-*) P+=("a-$L-$f");; M-*|N-*) C+=("a-$L-$f");; esac; done
if [ ${#P[@]} -gt 0 ]; then python -I -X utf8 nm.py "${P[@]}" > evidence/nm-$L.txt 2>&1; echo "exit $?" >> evidence/nm-$L.txt; cat evidence/nm-$L.txt; fi
if [ ${#C[@]} -gt 0 ]; then python -I -X utf8 count20.py "${C[@]}" > evidence/count-$L.txt 2>&1; echo "exit $?" >> evidence/count-$L.txt; cat evidence/count-$L.txt; fi
for f in "$@"; do case "$f" in V*) python -I -X utf8 wx.py "a-$L-$f" > evidence/wx-$L.txt 2>&1; echo "exit $?" >> evidence/wx-$L.txt; cat evidence/wx-$L.txt;; esac; done
