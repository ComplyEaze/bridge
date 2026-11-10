# Lab helper (row 2346-TD): one bills read (BR) in fetch_party_bills's order: <p>sa and <p>sb the snapshot, <p>la the report (the window captured at once after it, <p>la-after), <p>lb the report again. With SA_SHOT=1, the window is also captured at once after <p>sa (<p>sa-after). Each by send.sh; it stops at the first send that does not answer as run.py wants (the empty-report exception is then handled by hand). Then pb.py --outcome on the four answers, in master's order, to evidence/outcome-<p>.txt. Usage: [SA_SHOT=1] bash br-td.sh <p> <S file> <L file>
cd <LAB_DIR> || exit 9
P=$1; SF=$2; LF=$3
if [ -n "$SA_SHOT" ]; then SHOT=${P}sa-after bash send.sh ${P}sa $SF || exit 7; else bash send.sh ${P}sa $SF || exit 7; fi
bash send.sh ${P}sb $SF || exit 7
cmp -s a-${P}sa-$SF a-${P}sb-$SF && echo "${P}sa and ${P}sb byte-equal" | tee -a evidence/notes.txt || echo "${P}sa and ${P}sb NOT byte-equal: master would send nothing more; the row sends la and lb (sheet)" | tee -a evidence/notes.txt
SHOT=${P}la-after bash send.sh ${P}la $LF || exit 7
bash send.sh ${P}lb $LF || exit 7
python -I -X utf8 pb.py --outcome a-${P}sa-$SF a-${P}sb-$SF a-${P}la-$LF a-${P}lb-$LF > evidence/outcome-$P.txt 2>&1; echo "exit $?" >> evidence/outcome-$P.txt; cat evidence/outcome-$P.txt
