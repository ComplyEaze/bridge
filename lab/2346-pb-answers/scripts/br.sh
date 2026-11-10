# Lab helper (row 2346-PB): one read of the bills (BR) after step <step>, in the sheet's order: <step>sa and <step>sb the snapshot, <step>la the report (the window captured at once after it, <step>la-after), <step>lb the report again, <step>r B-bills, B-payable, M-marks and the step's N or R reads. Each by send.sh; it stops at the first send that does not answer as run.py wants (the empty-report exception is then handled by hand). Then pb.py --outcome on the four answers, in master's order, to evidence/outcome-<step>.txt. Usage: bash br.sh <step> [N or R files...]
cd <LAB_DIR> || exit 9
S=$1; shift
bash send.sh ${S}sa S-snap-20260401-20260801.xml || exit 7
bash send.sh ${S}sb S-snap-20260401-20260801.xml || exit 7
SHOT=${S}la-after bash send.sh ${S}la L-lo-bc-20260801.xml || exit 7
bash send.sh ${S}lb L-lo-bc-20260801.xml || exit 7
bash send.sh ${S}r B-bills.xml B-payable.xml M-marks.xml "$@" || exit 7
python -I -X utf8 pb.py --outcome a-${S}sa-S-snap-20260401-20260801.xml a-${S}sb-S-snap-20260401-20260801.xml a-${S}la-L-lo-bc-20260801.xml a-${S}lb-L-lo-bc-20260801.xml > evidence/outcome-$S.txt 2>&1; echo "exit $?" >> evidence/outcome-$S.txt; cat evidence/outcome-$S.txt
