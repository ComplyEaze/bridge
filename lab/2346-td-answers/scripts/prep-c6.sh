# Lab helper (row 2346-TD, from row 2346-PB's, itself from row 2162-ST's): C6's prep before the restore of a part. Re-hashes the BK, checks its folder holds only that file,
# writes restore-plan.txt (and a copy restore-plan-<part>.txt) and lists the data folder. Sends nothing to 9001 and presses no key.
# Usage: bash prep-c6.sh <part>   (9: the restore of step 9)
cd <LAB_DIR> || exit 9
P=$1; [ -n "$P" ] || { echo "part missing"; exit 9; }
D=C:/lab/2346-td/backup-1
n=$(ls -A $D | wc -l); [ "$n" = 1 ] || { echo "backup-1 holds $n entries: stop"; exit 8; }
f=$(ls $D); num=$(echo "$f" | sed -E 's/TDBK1800_([0-9]+)\.001/\1/')
h=$(sha256sum $D/$f | cut -c1-64); sz=$(stat -c %s $D/$f)
grep -q "sha256 $h " evidence/bk.txt || { echo "BK hash differs from bk.txt: stop"; exit 7; }
t=$(date -u +%FT%TZ)
ls -la --time-style=+%FT%T <Tally data folder>/$num > evidence/$P-data-before-restore.txt
e=$(tail -n +4 evidence/$P-data-before-restore.txt | wc -l)
{
echo "Written $t, before the restore after step $P (C6)."
echo "folder C:\\lab\\2346-td\\backup-1 (holds this one file), file $f, $sz B, sha256 $h (equal to evidence/bk.txt)"
echo "company number $num, BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85"
echo "Restore Data Path <Tally data folder>; source: Specify Path, the backup-1 folder"
lt=$(grep -o "will show [^)]*)" evidence/bk.txt | cut -c11-); echo "the Restore list must show BRIDGE PILOT LAB, its number, $lt, the BK's time"
echo "two prompts expected, the only ones answered: \"Accept ?\" = y; \"Company data folder ($num) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder.\" = r (Replace)"
echo "data folder listing before the restore: evidence/$P-data-before-restore.txt ($e entries)"
} > restore-plan.txt
cp restore-plan.txt restore-plan-$P.txt
echo "$t step $P C6 prep: BK re-hashed equal to evidence/bk.txt ($sz B), its folder holds only that file; restore-plan.txt written (copy restore-plan-$P.txt); the data folder listed (evidence/$P-data-before-restore.txt, $e entries, kept on the lab PC)" >> evidence/notes.txt
sed "s/$num/<N>/g" restore-plan.txt
