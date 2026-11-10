# Lab helper (row 683-PW, from row 683-XF's second sitting): C6's prep before the restore of a part. Re-hashes the BK, checks its folder holds only
# that file, writes restore-plan.txt (and a copy restore-plan-<part>.txt) and lists the data folder. Sends nothing to 9001 and presses no key.
# Usage: LAB_DIR=<the row's lab folder> TALLY_DATA=<Tally data folder> bash prep-c6.sh <part> <BK folder>   (row 683-PW: bash prep-c6.sh 8 bk-683pw)
# Changed from row 683-XF's copy: the BK folder is the second argument, never written here, and every line that names it uses that argument. The
# two lab paths come from LAB_DIR and TALLY_DATA, so this file is the kit's byte for byte on the lab PC. The folder must be bk-683 followed by
# lower-case letters or digits, and must sit directly in C:/lab.
P=$1; B=$2
[ -n "$P" ] || { echo "part missing"; exit 9; }
case "$B" in bk-683*) ;; *) echo "BK folder missing or not bk-683...: stop"; exit 9;; esac
[[ "$B" =~ ^bk-683[a-z0-9]+$ ]] || { echo "BK folder $B is not bk-683 followed by lower-case letters or digits: stop"; exit 9; }
[ -n "$LAB_DIR" ] && [ -n "$TALLY_DATA" ] || { echo "LAB_DIR or TALLY_DATA not set: stop"; exit 9; }
cd "$LAB_DIR" || exit 9
D=C:/lab/$B
[ -d "$D" ] || { echo "$D is not a folder: stop"; exit 8; }
n=$(ls -A "$D" | wc -l); [ "$n" = 1 ] || { echo "$B holds $n entries: stop"; exit 8; }
f=$(ls "$D"); num=$(echo "$f" | sed -E 's/TDBK1800_([0-9]+)\.001/\1/')
h=$(sha256sum "$D/$f" | cut -c1-64); sz=$(stat -c %s "$D/$f")
grep -q "sha256 $h " evidence/bk.txt || { echo "BK hash differs from bk.txt: stop"; exit 7; }
t=$(date -u +%FT%TZ)
ls -la --time-style=+%FT%T "$TALLY_DATA/$num" > evidence/$P-data-before-restore.txt
e=$(tail -n +4 evidence/$P-data-before-restore.txt | wc -l)
{
echo "Written $t, before the restore of step $P (C6)."
echo "folder C:\\lab\\$B (holds this one file), file $f, $sz B, sha256 $h (equal to evidence/bk.txt)"
echo "company number $num, BRIDGE FOREXGROUP LAB, GUID 1bb48be5-baba-45b2-90d8-7593244915a9"
echo "Restore Data Path <Tally data folder>; source: Specify Path, the $B folder"
lt=$(grep -o "will show [^)]*)" evidence/bk.txt | cut -c11-); echo "the Restore list must show BRIDGE FOREXGROUP LAB, its number, $lt, the BK's time"
echo "two prompts expected, the only ones answered: \"Accept ?\" = y; \"Company data folder ($num) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.\" = r (Replace)"
echo "data folder listing before the restore: evidence/$P-data-before-restore.txt ($e entries)"
} > restore-plan.txt
cp restore-plan.txt restore-plan-$P.txt
echo "$t step $P C6 prep ($B): BK re-hashed equal to evidence/bk.txt ($sz B), its folder holds only that file; restore-plan.txt written (copy restore-plan-$P.txt); the data folder listed (evidence/$P-data-before-restore.txt, $e entries, kept on the lab PC)" >> evidence/notes.txt
sed "s/$num/<N>/g" restore-plan.txt
