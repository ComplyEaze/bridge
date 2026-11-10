# Lab helper (row 683-NO, from row 683-XA's step2.sh less x0): step 2 as the sheet orders it: mv0 alone, straight after m0's counters,
# its calls run and recorded whole (evidence/<name>.txt with its exit). A failed request (send.sh's stop) or a stop of lm.py (exit 1) ends it:
# nothing more is sent. The EXPECT is judged by the operator from the lines printed.
cd <LAB_DIR> || exit 9
V=V-mvy-20250401-20260331.xml
r() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; e=$?; echo "exit $e" >> evidence/$o.txt; echo "$o: $(grep -E '^(SETS|SET [A-Z]+|CONTENT|ROW ORDER|ELEMENT ORDER [A-Z]|NOT BYTE|BYTE|SAME MARKS|MARKS DIFFER|CAPTURE|vouchers:|  GUID|rows|LEDGER)' evidence/$o.txt | tr '\n' ' ')exit $e"; [ $e = 1 ] && { echo "stop: $o exit 1; nothing more sent"; exit 1; }; return 0; }
bash send.sh mv0 $V || { echo "mv0 send failed"; exit 7; }
r lm-capture-mv0 lm.py --capture a-mv0-$V a-twmvy-$V || exit 1
cmp -s a-mv0-$V a-twmvy-$V && echo "a-mv0 byte-equal to a-twmvy" > evidence/eq-mv0.txt || echo "a-mv0 NOT byte-equal to a-twmvy" > evidence/eq-mv0.txt; cat evidence/eq-mv0.txt
echo "step 2 done"
