# Lab helper (row 2480-SK, from row 2480-MW's k9.sh): the end of step 7. k10 (Vb-book.xml, the file k8 sent) by send.sh with the window captured after it, then wx.py on it (send.sh) and wx.py --same against k8; then k11 (M-marks.xml) and count20.py --same-row against k9. On MARKS DIFFER: M-marks.xml once more as k11r, recorded, and the stop (exit 7). Usage: bash k10.sh
cd <LAB_DIR> || exit 9
SHOT=k10-after bash send.sh k10 Vb-book.xml > evidence/send-k10.out 2>&1 || { echo "k10 send.sh exit $?: stop"; exit 7; }
h=$(sha256sum shots/k10-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "k10 capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
echo "== k10 $(grep -ho 'ANSWER: .*' evidence/wx-k10.txt | head -1) $(tail -n 1 evidence/wx-k10.txt)"
python -I -X utf8 wx.py --same a-k8-Vb-book.xml a-k10-Vb-book.xml > evidence/same-k10.txt 2>&1; echo "exit $?" >> evidence/same-k10.txt; echo "== k10 against k8 $(head -1 evidence/same-k10.txt)"
SHOT=k11-after bash send.sh k11 M-marks.xml > /dev/null || { echo "k11 send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-k9-M-marks.xml a-k11-M-marks.xml > evidence/same-k11.txt 2>&1; r=$?; echo "exit $r" >> evidence/same-k11.txt; cat evidence/same-k11.txt
if [ $r != 0 ] || ! grep -qx "  equal" evidence/same-k11.txt; then bash send.sh k11r M-marks.xml > /dev/null; echo "k11r sent once: MARKS DIFFER, stop"; exit 7; fi
exit 0
