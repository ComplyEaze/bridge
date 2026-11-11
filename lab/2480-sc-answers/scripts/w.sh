# Lab helper (row 2480-SC, from row 2480-SK's w.sh, without recipe (f)): one label of step 3's write recipe a call. "bash w.sh m <label>": (a) the marks read <label>m by send.sh, then count20.py <the before-write option> against a-s1. "bash w.sh i <label> <file> <kind>": (b) the import alone by send.sh with the window captured at once after it (<label>-after), (c) the capture compared with the Gateway captures in evidence/gateway-refs.txt (one that differs ends here, exit 6, nothing more sent), (d) st.py --import, recorded whole. Kinds ctl and a: exit 0 on CREATED ONE; any other answer is the stop of the set-up (exit 7). Kind kb (s4kb alone): st.py exit 0 is the stop of the set-up; only on st.py exit 1 after a run.py exit 0, (e) sk.py --altered, recorded whole; on its exit 0 (ALTERED ONE) the marks read <label>x, count20.py <the before-write option> against a-s1 and --same-row against <label>m, each recorded whole (the --same-row line a record): <the before-write option> passing, exit 5 (ALTERED); anything else, the stop of the set-up (exit 7). rf.py is never run: a refusal is the stop of the set-up. Usage: bash w.sh m <label> | bash w.sh i <label> <file> <ctl|a|kb>
cd <LAB_DIR> || exit 9
if [ "$1" = m ]; then
  bash send.sh ${2}m M-marks.xml > /dev/null || { echo "${2}m send failed: stop"; exit 7; }
  python -I -X utf8 count20.py <the before-write option> a-s1-M-marks.xml a-${2}m-M-marks.xml > evidence/bw-${2}m.txt 2>&1; r=$?; echo "exit $r" >> evidence/bw-${2}m.txt; cat evidence/bw-${2}m.txt; exit $r
fi
L=$2; F=$3; K=$4
case "$K" in ctl|a|kb) ;; *) echo "kind $K is not ctl, a or kb: nothing sent"; exit 9;; esac
SHOT=$L-after bash send.sh $L $F; rc=$?
h=$(sha256sum shots/$L-after.png | cut -c1-64)
grep -q "^$h " evidence/gateway-refs.txt || { echo "capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }
echo "capture $h, one of the Gateway captures in evidence/gateway-refs.txt"
[ $rc = 0 ] || { echo "run.py on $F failed: the stop of the set-up"; exit 7; }
python -I -X utf8 st.py --import a-$L-$F > evidence/import-$L.txt 2>&1; r=$?; echo "exit $r" >> evidence/import-$L.txt; cat evidence/import-$L.txt
if [ $K != kb ]; then [ $r = 0 ] && exit 0; echo "st.py exit $r on $L: the stop of the set-up"; exit 7; fi
[ $r = 0 ] && { echo "s4kb CREATED ONE: the stop of the set-up (the row is built on SK's alteration)"; exit 7; }
[ $r = 1 ] || { echo "st.py exit $r: the stop of the set-up"; exit 7; }
python -I -X utf8 sk.py --altered a-$L-$F > evidence/alt-$L.txt 2>&1; ra=$?; echo "exit $ra" >> evidence/alt-$L.txt; cat evidence/alt-$L.txt
[ $ra = 0 ] || { echo "sk.py --altered exit $ra: the stop of the set-up"; exit 7; }
bash send.sh ${L}x M-marks.xml > /dev/null || { echo "${L}x send failed: stop"; exit 7; }
python -I -X utf8 count20.py <the before-write option> a-s1-M-marks.xml a-${L}x-M-marks.xml > evidence/bw-${L}x.txt 2>&1; rb=$?; echo "exit $rb" >> evidence/bw-${L}x.txt; cat evidence/bw-${L}x.txt
python -I -X utf8 count20.py --same-row a-${L}m-M-marks.xml a-${L}x-M-marks.xml > evidence/same-${L}x.txt 2>&1; echo "exit $?" >> evidence/same-${L}x.txt; cat evidence/same-${L}x.txt
[ $rb = 0 ] && { echo "ALTERED: $L taken as the first ledger on import"; exit 5; }
echo "<the before-write option> did not pass after the alteration: the stop of the set-up"; exit 7
