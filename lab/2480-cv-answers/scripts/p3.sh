# Lab helper (row 2480-CV): step 3. p1 (C-u-pl.xml) and p2 (C-v-pl.xml) by cr.sh, each capture after it one of evidence/gateway-refs.txt; cv.py --same on the pair; then p3 (M-marks.xml) and count20.py --same-row against s1. Ends at a cr.sh exit other than 0 or 3 (7: a stop), or a capture none of the list. What the answers hold is judged by hand after it, as ROW.md step 3 says. Usage: bash p3.sh
cd <LAB_DIR> || exit 9
for p in p1:C-u-pl.xml p2:C-v-pl.xml; do
  L=${p%%:*}; F=${p#*:}
  bash cr.sh $L $F > evidence/cr-$L.out 2>&1; rc=$?; echo "== cr.sh $L $F exit $rc: $(grep -h '^ANSWER' evidence/cv-$L.txt 2>/dev/null | head -1)"
  [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: cr.sh exit $rc: stop"; exit 7; }
  h=$(sha256sum shots/$L-after.png | cut -c1-64)
  grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
done
python -I -X utf8 cv.py --same a-p1-C-u-pl.xml a-p2-C-v-pl.xml > evidence/same-p1p2.txt 2>&1; echo "exit $?" >> evidence/same-p1p2.txt; cat evidence/same-p1p2.txt
bash send.sh p3 M-marks.xml > /dev/null || { echo "p3 send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-s1-M-marks.xml a-p3-M-marks.xml > evidence/same-p3.txt 2>&1; echo "exit $?" >> evidence/same-p3.txt; cat evidence/same-p3.txt
