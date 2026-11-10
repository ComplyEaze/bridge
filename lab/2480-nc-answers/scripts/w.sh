# Lab helper (row 2480-NC, from row 2480-NS): one label of step 4's write recipe a call. "bash w.sh m <label>": (a) the marks read <label>m by send.sh, then count20.py <the before-write option> against a-s1. "bash w.sh i <label> <file>": (b) the import alone by send.sh with the window captured at once after it (<label>-after), (c) the capture compared with p1-after's, (d) st.py --import, recorded whole with its exit.
cd <LAB_DIR> || exit 9
if [ "$1" = m ]; then
  bash send.sh ${2}m M-marks.xml > /dev/null || { echo "${2}m send failed: stop"; exit 7; }
  python -I -X utf8 count20.py <the before-write option> a-s1-M-marks.xml a-${2}m-M-marks.xml > evidence/bw-${2}m.txt 2>&1; r=$?; echo "exit $r" >> evidence/bw-${2}m.txt; cat evidence/bw-${2}m.txt; exit $r
fi
L=$2; F=$3
SHOT=$L-after bash send.sh $L $F; rc=$?
h=$(sha256sum shots/$L-after.png | cut -c1-64); ref=$(sha256sum shots/p1-after.png | cut -c1-64)
[ "$h" = "$ref" ] && echo "capture equal to p1-after" || echo "capture DIFFERS from p1-after: look at it"
[ $rc = 0 ] || { echo "run.py on $F failed: the stop of the set-up"; exit 7; }
python -I -X utf8 st.py --import a-$L-$F > evidence/import-$L.txt 2>&1; r=$?; echo "exit $r" >> evidence/import-$L.txt; cat evidence/import-$L.txt; exit $r
