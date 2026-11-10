# Lab helper (row 2480-CV): step 6, only after step 5 has passed. p4 to p13 (C-v-<id>.xml for cm to ex, table order) by cr.sh, each capture after it one of evidence/gateway-refs.txt; cv.py --same against p2 for each; for an answer with a voucher, cv.py --own pl as ROW.md says, and the id listed in evidence/step6-voucher-ids.txt (step 7 skips it). A voucher for one name does not end the loop. Then p14 (M-marks.xml) and count20.py --same-row against c5. Ends at a cr.sh exit other than 0 or 3 (7: a stop), or a capture none of the list. Usage: bash p6.sh
cd <LAB_DIR> || exit 9
: > evidence/step6-voucher-ids.txt
n=3
for id in cm us dq ps at hs cl pc st ex; do
  n=$((n+1)); L=p$n; F=C-v-$id.xml
  bash cr.sh $L $F > evidence/cr-$L.out 2>&1; rc=$?
  a=$(grep -ho 'ANSWER: .*' evidence/cv-$L.txt 2>/dev/null | head -1); echo "== cr.sh $L $F exit $rc: $a"
  [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: cr.sh exit $rc: stop"; exit 7; }
  h=$(sha256sum shots/$L-after.png | cut -c1-64)
  grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
  python -I -X utf8 cv.py --same a-p2-C-v-pl.xml a-$L-$F > evidence/same-$L.txt 2>&1; echo "exit $?" >> evidence/same-$L.txt; head -1 evidence/same-$L.txt
  case "$a" in "ANSWER: VOUCHERS 0"|"ANSWER: NO BYTES") ;;
    "ANSWER: VOUCHERS "*) python -I -X utf8 cv.py --own pl a-$L-$F a-s2-C-book.xml a-c4-C-book.xml pl > evidence/own-$L.txt 2>&1; echo "exit $?" >> evidence/own-$L.txt; cat evidence/own-$L.txt; echo $id >> evidence/step6-voucher-ids.txt; echo "$id: a voucher before its ledger exists; step 7 skips it";;
    *) echo "$id: $a, recorded; it is still set up at step 7";;
  esac
done
bash send.sh p14 M-marks.xml > /dev/null || { echo "p14 send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-c5-M-marks.xml a-p14-M-marks.xml > evidence/same-p14.txt 2>&1; echo "exit $?" >> evidence/same-p14.txt; cat evidence/same-p14.txt
