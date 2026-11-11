# Lab helper (row 2480-MX, from row 2480-CV's s7.sh): step 4, the set-up, by wid.sh for each id in table order, skipping an id listed in evidence/step3-bytes-ids.txt. wid.sh exit 0: the next id; exit 4 (ledger refused) or 5 (invoice refused): recorded in evidence/step4-refused.txt, the next id (a refusal of the control is wid.sh's exit 7); any other exit ends this script with nothing more sent (the stop of the set-up). Usage: bash s4.sh
cd <LAB_DIR> || exit 9
: > evidence/step4-refused.txt
for id in pl ga ss ms qd pp hd ad ca xs al; do
  grep -qx $id evidence/step3-bytes-ids.txt && { echo "== $id skipped: bytes at step 3"; continue; }
  bash wid.sh $id > evidence/wid-$id.out 2>&1; r=$?
  echo "== wid.sh $id exit $r: $(grep -h 'IMPORT' evidence/wid-$id.out | tr '\n' ' ')"
  case $r in 0) ;; 4|5) echo "$id exit $r" >> evidence/step4-refused.txt;; *) echo "$id: wid.sh exit $r: the stop of the set-up"; exit $r;; esac
done
