# Lab helper (row 2480-CV): step 7, the set-up of the ten, by wid.sh for each id in table order, skipping an id listed in evidence/step6-voucher-ids.txt. wid.sh exit 0: the next id; exit 4 (ledger refused) or 5 (invoice refused): recorded in evidence/step7-refused.txt, the next id; any other exit ends this script with nothing more sent (the stop of the set-up). Usage: bash s7.sh
cd <LAB_DIR> || exit 9
: > evidence/step7-refused.txt
for id in cm us dq ps at hs cl pc st ex; do
  grep -qx $id evidence/step6-voucher-ids.txt && { echo "== $id skipped: a voucher at step 6"; continue; }
  bash wid.sh $id > evidence/wid-$id.out 2>&1; r=$?
  echo "== wid.sh $id exit $r: $(grep -h 'IMPORT' evidence/wid-$id.out | tr '\n' ' ')"
  case $r in 0) ;; 4|5) echo "$id exit $r" >> evidence/step7-refused.txt;; *) echo "$id: wid.sh exit $r: the stop of the set-up"; exit $r;; esac
done
