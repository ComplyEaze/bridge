# Lab helper (row 2480-SC, from row 2480-SK's k10.sh): the end of step 7 (labels a10, a11) or of step 8 (labels b12, b13). The book-wide read <first label> (Vb-book.xml, the file k8 sent) by send.sh with the window captured after it and checked against evidence/gateway-refs.txt, wx.py on it (send.sh) and wx.py --same against k8; then the marks read <second label> (M-marks.xml) and count20.py --same-row against <marks before> (k9 for a11, a11 for b13). On MARKS DIFFER: M-marks.xml once more as <second label>r, recorded, and the stop (exit 7). Usage: bash a10.sh a10 a11 k9 | bash a10.sh b12 b13 a11
cd <LAB_DIR> || exit 9
V=$1; M=$2; P=$3; case "$V:$M:$P" in a10:a11:k9|b12:b13:a11) ;; *) echo "not a10 a11 k9 or b12 b13 a11: nothing sent"; exit 9;; esac
SHOT=$V-after bash send.sh $V Vb-book.xml > evidence/send-$V.out 2>&1 || { echo "$V send.sh exit $?: stop"; exit 7; }
h=$(sha256sum shots/$V-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "$V capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
echo "== $V $(grep -ho 'ANSWER: .*' evidence/wx-$V.txt | head -1) $(tail -n 1 evidence/wx-$V.txt)"
python -I -X utf8 wx.py --same a-k8-Vb-book.xml a-$V-Vb-book.xml > evidence/same-$V.txt 2>&1; echo "exit $?" >> evidence/same-$V.txt; echo "== $V against k8 $(head -1 evidence/same-$V.txt)"
SHOT=$M-after bash send.sh $M M-marks.xml > /dev/null || { echo "$M send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-$P-M-marks.xml a-$M-M-marks.xml > evidence/same-$M.txt 2>&1; r=$?; echo "exit $r" >> evidence/same-$M.txt; cat evidence/same-$M.txt
if [ $r != 0 ] || ! grep -qx "  equal" evidence/same-$M.txt; then bash send.sh ${M}r M-marks.xml > /dev/null; echo "${M}r sent once: MARKS DIFFER, stop"; exit 7; fi
exit 0
