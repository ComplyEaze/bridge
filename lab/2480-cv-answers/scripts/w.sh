# Lab helper (row 2480-CV, from row 2480-NX, from row 2480-NC's w.sh, with ROW.md step 4 (e) added): one label of step 4's write recipe a call. "bash w.sh m <label>": (a) the marks read <label>m by send.sh, then count20.py <the before-write option> against a-s1. "bash w.sh i <label> <file>": (b) the import alone by send.sh with the window captured at once after it (<label>-after), (c) the capture compared with the Gateway captures listed in evidence/gateway-refs.txt (a capture that differs ends here, exit 6, with nothing more sent), (d) st.py --import, recorded whole with its exit: exit 0 on CREATED ONE. (e) only on st.py exit 1 after a run.py exit 0: rf.py, recorded whole; on its exit 0 (REFUSED), the marks read <label>x by send.sh and count20.py --same-row against <label>m: on equal marks, exit 4 (the import refused). Every other outcome is the stop of the set-up, exit 7.
cd <LAB_DIR> || exit 9
if [ "$1" = m ]; then
  bash send.sh ${2}m M-marks.xml > /dev/null || { echo "${2}m send failed: stop"; exit 7; }
  python -I -X utf8 count20.py <the before-write option> a-s1-M-marks.xml a-${2}m-M-marks.xml > evidence/bw-${2}m.txt 2>&1; r=$?; echo "exit $r" >> evidence/bw-${2}m.txt; cat evidence/bw-${2}m.txt; exit $r
fi
L=$2; F=$3
SHOT=$L-after bash send.sh $L $F; rc=$?
h=$(sha256sum shots/$L-after.png | cut -c1-64)
grep -q "^$h " evidence/gateway-refs.txt || { echo "capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }
echo "capture $h, one of the Gateway captures in evidence/gateway-refs.txt"
[ $rc = 0 ] || { echo "run.py on $F failed: the stop of the set-up"; exit 7; }
python -I -X utf8 st.py --import a-$L-$F > evidence/import-$L.txt 2>&1; r=$?; echo "exit $r" >> evidence/import-$L.txt; cat evidence/import-$L.txt
[ $r = 0 ] && exit 0
[ $r = 1 ] || { echo "st.py exit $r: the stop of the set-up"; exit 7; }
python -I -X utf8 rf.py a-$L-$F > evidence/rf-$L.txt 2>&1; rr=$?; echo "exit $rr" >> evidence/rf-$L.txt; cat evidence/rf-$L.txt
[ $rr = 0 ] || { echo "rf.py exit $rr: the stop of the set-up"; exit 7; }
bash send.sh ${L}x M-marks.xml > /dev/null || { echo "${L}x send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-${L}m-M-marks.xml a-${L}x-M-marks.xml > evidence/same-${L}x.txt 2>&1; rs=$?; echo "exit $rs" >> evidence/same-${L}x.txt; cat evidence/same-${L}x.txt
[ $rs = 0 ] && grep -qx "  equal" evidence/same-${L}x.txt && { echo "REFUSED, marks equal: $L refused"; exit 4; }
echo "marks after the refusal not equal: the stop of the set-up"; exit 7
