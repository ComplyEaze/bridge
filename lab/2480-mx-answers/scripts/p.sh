# Lab helper (row 2480-MX, from row 2480-NX's p.sh): step 3. p1 to p11 (L-lo-<id>-20260801.xml, table order) by lr.sh, each capture after it one of evidence/gateway-refs.txt. lr.sh exit 0: the empty-answer exception held. Exit 3 (bytes came back): that name's result, recorded with nm.py --against as ROW.md says, and the id listed in evidence/step3-bytes-ids.txt (step 4 and part B skip it); the loop goes on, but if the id is the control pl, p12 is sent and the script ends with exit 5 (the stop after p12). Any other lr.sh exit (7: a stop) or a capture none of the list ends it with nothing more sent. Then p12 (M-marks.xml) and count20.py --same-row against s1. Usage: bash p.sh
cd <LAB_DIR> || exit 9
: > evidence/step3-bytes-ids.txt
n=0; ctl=0
for id in pl ga ss ms qd pp hd ad ca xs al; do
  n=$((n+1)); L=p$n; F=L-lo-$id-20260801.xml
  bash lr.sh $L $id > evidence/lr-$L.out 2>&1; r=$?
  echo "$L $id exit $r: $(grep -h 'ANSWER' evidence/nm-$L.txt 2>/dev/null | head -1)"
  [ $r = 0 ] || [ $r = 3 ] || { echo "$L: lr.sh exit $r: stop"; exit 7; }
  h=$(sha256sum shots/$L-after.png | cut -c1-64)
  grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
  if [ $r = 3 ]; then
    python -I -X utf8 nm.py --against $id a-$L-$F a-s2-B-bills.xml a-s2-S-snap-20260401-20260801.xml > evidence/against-$L.txt 2>&1; echo "exit $?" >> evidence/against-$L.txt; cat evidence/against-$L.txt
    echo $id >> evidence/step3-bytes-ids.txt; echo "$id: bytes before its ledger exists; step 4 and part B skip it"
    [ $id = pl ] && { ctl=1; break; }
  fi
done
bash send.sh p12 M-marks.xml > /dev/null || { echo "p12 send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-s1-M-marks.xml a-p12-M-marks.xml > evidence/same-p12.txt 2>&1; echo "exit $?" >> evidence/same-p12.txt; cat evidence/same-p12.txt
[ $ctl = 1 ] && { echo "the control answered with bytes: stop after p12"; exit 5; }
exit 0
