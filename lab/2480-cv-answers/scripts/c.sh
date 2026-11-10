# Lab helper (row 2480-CV): step 5, the shape step, the control alone. c0 M-marks.xml; c1 C-u-pl.xml, c2 C-uv-pl.xml, c3 C-v-pl.xml, each with the window captured at once after it (one of evidence/gateway-refs.txt); c4 C-book.xml; c5 M-marks.xml; one label a send, ending at the first send.sh exit other than 0 (no empty-answer exception at this step) or a capture none of the list. Then, file work only, each recorded whole: cv.py --same c1/c3 and c1/c2, cv.py --own pl, count20.py --same-row c0/c5. Whether the step passes is judged by hand after it, as ROW.md step 5 says. Usage: bash c.sh
cd <LAB_DIR> || exit 9
for p in c0:M-marks.xml c1:C-u-pl.xml c2:C-uv-pl.xml c3:C-v-pl.xml c4:C-book.xml c5:M-marks.xml; do
  L=${p%%:*}; F=${p#*:}
  case $L in c1|c2|c3) S=$L-after;; *) S=;; esac
  SHOT=$S bash send.sh $L $F > evidence/send-$L.out 2>&1; r=$?; echo "== send.sh $L $F exit $r: $(grep -h '^a-.*ANSWER\|COMPANY .ALT' evidence/send-$L.out | head -1)"
  [ $r = 0 ] || { echo "$L: send.sh exit $r: stop of the shape step"; exit 7; }
  if [ -n "$S" ]; then h=$(sha256sum shots/$S.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }; fi
done
python -I -X utf8 cv.py --same a-c1-C-u-pl.xml a-c3-C-v-pl.xml > evidence/same-c1c3.txt 2>&1; echo "exit $?" >> evidence/same-c1c3.txt
python -I -X utf8 cv.py --same a-c1-C-u-pl.xml a-c2-C-uv-pl.xml > evidence/same-c1c2.txt 2>&1; echo "exit $?" >> evidence/same-c1c2.txt
python -I -X utf8 cv.py --own pl a-c1-C-u-pl.xml a-s2-C-book.xml a-c4-C-book.xml pl > evidence/own-c1.txt 2>&1; echo "exit $?" >> evidence/own-c1.txt
python -I -X utf8 count20.py --same-row a-c0-M-marks.xml a-c5-M-marks.xml > evidence/same-c5.txt 2>&1; echo "exit $?" >> evidence/same-c5.txt
for f in cv-c1 cv-c2 cv-c3 same-c1c3 same-c1c2 own-c1 same-c5; do echo "-- $f"; cat evidence/$f.txt; done
