# Lab helper (row 2440-ED, not product code): one timed.py send. Checks the request file's sha256 against the sheet's SHA256.txt first
# (a different hash or size: nothing is sent), records the wall clock before and after beside timed.py's own line, in evidence/wall-<label>.txt.
# Usage: bash send.sh <label> <request file>
cd <LAB_DIR> || exit 1
L=$1; F=$2; W=evidence/wall-$L.txt
[ -e "$W" ] && { echo "$W exists: stop"; exit 1; }
want=$(awk -v f="$F" '$3==f{print $1" "$2}' SHA256.txt); got="$(sha256sum "$F" | cut -d' ' -f1) $(stat -c %s "$F")"
echo "request $F: sheet $want; file $got" > "$W"
[ -n "$want" ] && [ "$want" = "$got" ] || { echo "request hash or size differs: nothing sent" | tee -a "$W"; exit 1; }
echo "wall clock before the send $(date -u +%FT%T.%3NZ)" >> "$W"
python -I -X utf8 timed.py http://127.0.0.1:9001 "$L" "$F"; rc=$?
echo "wall clock after the send $(date -u +%FT%T.%3NZ); timed.py exit $rc" >> "$W"
cat "$W"; exit $rc
