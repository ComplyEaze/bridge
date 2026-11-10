# Lab helper (row 683-NO, from row 683-XA's copy, from row 683-PW's step6.sh with tb1p's last --sets against a-ldtb10 as the sheet gives it; first from row 683-XF second sitting's step6.sh): steps 5 and 6 as the sheet orders them: m1 and its two compares, then
# x1, tb3p, tb3p2 and tb1p, one send at a time, each answer's calls run and recorded whole (evidence/<name>.txt with its exit) before the next
# send; the Tally window captured at once after tb3p and after tb1p. A failed request (send.sh's stop) or a stop of lm.py, tb.py or rows.py
# (exit 1) ends it: nothing more is sent. tb.py's SET UNREAD is a result, not a stop.
cd <LAB_DIR> || exit 9
T=T-tb3-20250401-20250401.xml; T1=T-tb1-20250401-20250430.xml; X=X-lx1-20250401.xml
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|SET [A-Z]+|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE|SAME MARKS|MARKS DIFFER|  GUID|MIXED|PLAIN|UNREAD)' evidence/$o.txt | tr '\n' ' ')exit $e"; [ $e = 1 ] && { echo "stop: $o exit 1; nothing more sent"; exit 1; }; return 0; }
bash send.sh m1 M-marks.xml > /dev/null || { echo "m1 send failed"; exit 7; }
r lm-same-m1-mc lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml || exit 1
r lm-same-m1-ldm1 lm.py --same-row a-ldm1-M-marks.xml a-m1-M-marks.xml || exit 1
bash send.sh x1 $X || { echo "x1 send failed"; exit 7; }
r rows-x1-ldx1 rows.py a-ldx1-$X a-x1-$X || exit 1
r cmp-x1-ldx1 cmpinfo.py a-ldx1-$X a-x1-$X || exit 1
r lm-x1 lm.py a-x1-$X || exit 1
bash send.sh tb3p $T || { echo "tb3p send failed"; exit 7; }
bash shot.sh tb3p-window | tail -1
r rows-tb3p-ldtb3 rows.py a-ldtb3-$T a-tb3p-$T || exit 1
r rows-tb3p-vdtb3w rows.py a-vdtb3w-$T a-tb3p-$T || exit 1
r rows-tb3p-twtb3 rows.py a-twtb3-$T a-tb3p-$T || exit 1
r cmp-tb3p-ldtb3 cmpinfo.py a-ldtb3-$T a-tb3p-$T || exit 1
r cmp-tb3p-vdtb3w cmpinfo.py a-vdtb3w-$T a-tb3p-$T || exit 1
r tb-tb3p tb.py a-tb3p-$T || exit 1
r sets-ldtb3-tb3p tb.py --sets a-ldtb3-$T a-tb3p-$T || exit 1
r sets-vdtb3w-tb3p tb.py --sets a-vdtb3w-$T a-tb3p-$T || exit 1
bash send.sh tb3p2 $T || { echo "tb3p2 send failed"; exit 7; }
r rows-tb3p2-tb3p rows.py a-tb3p-$T a-tb3p2-$T || exit 1
r cmp-tb3p2-tb3p cmpinfo.py a-tb3p-$T a-tb3p2-$T || exit 1
r sets-tb3p-tb3p2 tb.py --sets a-tb3p-$T a-tb3p2-$T || exit 1
bash send.sh tb1p $T1 || { echo "tb1p send failed"; exit 7; }
bash shot.sh tb1p-window | tail -1
r tb-tb1p tb.py a-tb1p-$T1 || exit 1
r sets-ldtb1-tb1p tb.py --sets a-ldtb1-$T1 a-tb1p-$T1 || exit 1
r rows-tb1p-ldtb1 rows.py a-ldtb1-$T1 a-tb1p-$T1 || exit 1
r cmp-tb1p-ldtb1 cmpinfo.py a-ldtb1-$T1 a-tb1p-$T1 || exit 1
r sets-ldtb10-tb1p tb.py --sets a-ldtb10-$T1 a-tb1p-$T1 || exit 1
echo "step 6 done"
