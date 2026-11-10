# Lab helper (row 683-TA, from row 683-NO's step2.sh with tb3a and tb3a2 added before mv0, as the sheet orders them): step 2: tb3a straight
# after m0's counters, tb3a2 straight after tb3a's, mv0 straight after tb3a2's; each answer's calls run and recorded whole (evidence/<name>.txt
# with its exit) before the next send. A failed request (send.sh's stop) or a stop of tb.py, rows.py or lm.py (exit 1) ends it: nothing more
# is sent. The EXPECTs are judged by the operator from the lines printed, before the BK.
cd <LAB_DIR> || exit 9
T=T-tb3-20250401-20250401.xml; V=V-mvy-20250401-20260331.xml
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|SET [A-Z]+|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE|SAME MARKS|MARKS DIFFER|CAPTURE|vouchers:|  GUID|rows|LEDGER)' evidence/$o.txt | tr '\n' ' ')exit $e"; [ $e = 1 ] && { echo "stop: $o exit 1; nothing more sent"; exit 1; }; return 0; }
bash send.sh tb3a $T || { echo "tb3a send failed"; exit 7; }
r sets-twtb3-tb3a tb.py --sets a-twtb3-$T a-tb3a-$T || exit 1
r rows-tb3a-twtb3 rows.py a-twtb3-$T a-tb3a-$T || exit 1
r cmp-tb3a-twtb3 cmpinfo.py a-twtb3-$T a-tb3a-$T || exit 1
bash send.sh tb3a2 $T || { echo "tb3a2 send failed"; exit 7; }
r sets-twtb3-tb3a2 tb.py --sets a-twtb3-$T a-tb3a2-$T || exit 1
r rows-tb3a2-twtb3 rows.py a-twtb3-$T a-tb3a2-$T || exit 1
r cmp-tb3a2-twtb3 cmpinfo.py a-twtb3-$T a-tb3a2-$T || exit 1
r rows-tb3a2-tb3a rows.py a-tb3a-$T a-tb3a2-$T || exit 1
r cmp-tb3a2-tb3a cmpinfo.py a-tb3a-$T a-tb3a2-$T || exit 1
bash send.sh mv0 $V || { echo "mv0 send failed"; exit 7; }
r lm-capture-mv0 lm.py --capture a-mv0-$V a-twmvy-$V || exit 1
cmp -s a-mv0-$V a-twmvy-$V && echo "a-mv0 byte-equal to a-twmvy" > evidence/eq-mv0.txt || echo "a-mv0 NOT byte-equal to a-twmvy" > evidence/eq-mv0.txt; cat evidence/eq-mv0.txt
echo "step 2 done"
