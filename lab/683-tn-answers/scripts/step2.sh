# Lab helper (row 683-TN, from row 683-TA's step2.sh with tb10 in place of tb3a and tb3a2, as the sheet orders it): step 2: tb10 straight
# after m0's counters, mv0 straight after tb10's; each answer's calls run and recorded whole (evidence/<name>.txt with its exit) before the
# next send. A failed request (send.sh's stop) or a stop of tb.py, rows.py or lm.py (exit 1) ends it: nothing more is sent. The EXPECTs
# are judged by the operator from the lines printed, before the BK.
cd <LAB_DIR> || exit 9
T1=T-tb1-20250401-20250430.xml; V=V-mvy-20250401-20260331.xml
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|SET [A-Z]+|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE|SAME MARKS|MARKS DIFFER|CAPTURE|vouchers:|  GUID|rows|LEDGER)' evidence/$o.txt | tr '\n' ' ')exit $e"; [ $e = 1 ] && { echo "stop: $o exit 1; nothing more sent"; exit 1; }; return 0; }
bash send.sh tb10 $T1 || { echo "tb10 send failed"; exit 7; }
r sets-ldtb10-tb10 tb.py --sets a-ldtb10-$T1 a-tb10-$T1 || exit 1
r rows-tb10-ldtb10 rows.py a-ldtb10-$T1 a-tb10-$T1 || exit 1
r cmp-tb10-ldtb10 cmpinfo.py a-ldtb10-$T1 a-tb10-$T1 || exit 1
bash send.sh mv0 $V || { echo "mv0 send failed"; exit 7; }
r lm-capture-mv0 lm.py --capture a-mv0-$V a-twmvy-$V || exit 1
cmp -s a-mv0-$V a-twmvy-$V && echo "a-mv0 byte-equal to a-twmvy" > evidence/eq-mv0.txt || echo "a-mv0 NOT byte-equal to a-twmvy" > evidence/eq-mv0.txt; cat evidence/eq-mv0.txt
echo "step 2 done"
