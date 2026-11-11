# Lab helper (row 2480-MX): the end of step 9. k9 (Vb-book.xml, the file k7 sent) by send.sh with the window captured after it, then wx.py on it (send.sh) and wx.py --same against k7; then k10 (M-marks.xml) and count20.py --same-row against k8. On MARKS DIFFER: M-marks.xml once more as k10r, recorded, and the stop (exit 7). Usage: bash k9.sh
cd <LAB_DIR> || exit 9
SHOT=k9-after bash send.sh k9 Vb-book.xml > evidence/send-k9.out 2>&1 || { echo "k9 send.sh exit $?: stop"; exit 7; }
h=$(sha256sum shots/k9-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "k9 capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
echo "== k9 $(grep -ho 'ANSWER: .*' evidence/wx-k9.txt | head -1) $(tail -n 1 evidence/wx-k9.txt)"
python -I -X utf8 wx.py --same a-k7-Vb-book.xml a-k9-Vb-book.xml > evidence/same-k9.txt 2>&1; echo "exit $?" >> evidence/same-k9.txt; echo "== k9 against k7 $(head -1 evidence/same-k9.txt)"
bash send.sh k10 M-marks.xml > /dev/null || { echo "k10 send failed: stop"; exit 7; }
python -I -X utf8 count20.py --same-row a-k8-M-marks.xml a-k10-M-marks.xml > evidence/same-k10.txt 2>&1; r=$?; echo "exit $r" >> evidence/same-k10.txt; cat evidence/same-k10.txt
if [ $r != 0 ] || ! grep -qx "  equal" evidence/same-k10.txt; then bash send.sh k10r M-marks.xml > /dev/null; echo "k10r sent once: MARKS DIFFER, stop"; exit 7; fi
exit 0
