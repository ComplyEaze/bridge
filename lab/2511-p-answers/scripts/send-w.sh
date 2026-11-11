# Lab helper (row 2511-P, new; not product code): one import send by run8.py (the run lead's decision). Copies Bridge's saved
# file from data/imports/ into the row folder byte for byte (cp, no re-encoding) unless the copy exists, checks the copy's
# sha256 against the build result's sha256 given as the 3rd argument (a difference = nothing sent), writes the UTC times
# to evidence/sends-utc.txt, sends ONCE. Usage: bash send-w.sh <label> <file name> <sha256>
cd <LAB_DIR> || exit 9
L=$1; f=$2; want=$3
[ -e "a-$L-$f" ] && { echo "answer a-$L-$f exists: never resend"; exit 6; }
[ -e "$f" ] || cp "data/imports/$f" "$f"
h=$(sha256sum "$f" | cut -c1-64); [ "$h" = "$want" ] || { echo "HASH DIFFERS $f ($h): nothing sent"; exit 8; }
h2=$(sha256sum "data/imports/$f" | cut -c1-64); [ "$h2" = "$want" ] || { echo "SAVED FILE HASH DIFFERS ($h2): nothing sent"; exit 8; }
echo "$(date -u +%FT%T.%3NZ) $L $f (sha256 equal to the build result, by run8.py)" >> evidence/sends-utc.txt
python -I run8.py http://127.0.0.1:9001 "$L" "$f" || { echo "run8.py exit $?"; exit 7; }
echo "$(date -u +%FT%T.%3NZ) $L answered" >> evidence/sends-utc.txt
