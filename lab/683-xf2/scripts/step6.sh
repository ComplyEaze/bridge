# Lab helper (row 683-XF second sitting): steps 5 and 6 as the sheet orders them: m1 and its two compares, then x1, tb3f and tb3f2, one send at a time,
# each answer's calls run and recorded whole (evidence/<name>.txt with its exit) before the next send; the Tally window captured at once after tb3f.
# A failed request (send.sh's stop) or a stop of lm.py, tb.py or rows.py (exit 1) ends it: nothing more is sent.
cd <LAB_DIR> || exit 9
T=T-tb3-20250401-20250401.xml; X=X-lx1-20250401.xml
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|SET [A-Z]+|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE|SAME MARKS|MARKS DIFFER|  GUID)' evidence/$o.txt | tr '\n' ' ')exit $e"; [ $e = 1 ] && { echo "stop: $o exit 1; nothing more sent"; exit 1; }; return 0; }
bash send.sh m1 M-marks.xml > /dev/null || { echo "m1 send failed"; exit 7; }
r lm-same-m1-mc lm.py --same-row a-mc-M-marks.xml a-m1-M-marks.xml || exit 1
r lm-same-m1-ldm1 lm.py --same-row a-ldm1-M-marks.xml a-m1-M-marks.xml || exit 1
bash send.sh x1 $X || { echo "x1 send failed"; exit 7; }
r rows-x1-ldx1 rows.py a-ldx1-$X a-x1-$X || exit 1
r cmp-x1-ldx1 cmpinfo.py a-ldx1-$X a-x1-$X || exit 1
r lm-x1 lm.py a-x1-$X || exit 1
bash send.sh tb3f $T || { echo "tb3f send failed"; exit 7; }
bash shot.sh tb3f-window | tail -1
r rows-tb3f-ldtb3 rows.py a-ldtb3-$T a-tb3f-$T || exit 1
r rows-tb3f-vdtb3w rows.py a-vdtb3w-$T a-tb3f-$T || exit 1
r rows-tb3f-twtb3 rows.py a-twtb3-$T a-tb3f-$T || exit 1
r cmp-tb3f-ldtb3 cmpinfo.py a-ldtb3-$T a-tb3f-$T || exit 1
r cmp-tb3f-vdtb3w cmpinfo.py a-vdtb3w-$T a-tb3f-$T || exit 1
r tb-tb3f tb.py a-tb3f-$T || exit 1
r sets-ldtb3-tb3f tb.py --sets a-ldtb3-$T a-tb3f-$T || exit 1
r sets-vdtb3w-tb3f tb.py --sets a-vdtb3w-$T a-tb3f-$T || exit 1
bash send.sh tb3f2 $T || { echo "tb3f2 send failed"; exit 7; }
r rows-tb3f2-tb3f rows.py a-tb3f-$T a-tb3f2-$T || exit 1
r cmp-tb3f2-tb3f cmpinfo.py a-tb3f-$T a-tb3f2-$T || exit 1
r sets-tb3f-tb3f2 tb.py --sets a-tb3f-$T a-tb3f2-$T || exit 1
echo "step 6 done"
