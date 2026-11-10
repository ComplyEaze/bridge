# Lab helper (row 2480-NM, from row 2346-TD's wr.sh): one write by the sheet's "Each write": <s>m M-marks.xml, then count20.py <the before-write option> against s1; only if that passes, the import <s> <file> alone, the whole window captured at once after it (<s>-after); count20.py on the answer (by send.sh). Each file is sent at most once (C3): this script refuses a label whose answer already exists, and an import file already sent under any label. Usage: bash wr.sh <s> <file>
cd <LAB_DIR> || exit 9
S=$1; F=$2
[ -e a-$S-$F ] && { echo "a-$S-$F exists: never sent twice"; exit 9; }
ls a-*-$F >/dev/null 2>&1 && { echo "$F was sent before: never sent twice"; exit 9; }
[ -e a-${S}m-M-marks.xml ] && { echo "a-${S}m-M-marks.xml exists: stop"; exit 9; }
bash send.sh ${S}m M-marks.xml || exit 7
python -I -X utf8 count20.py <the before-write option> a-s1-M-marks.xml a-${S}m-M-marks.xml > evidence/bw-$S.txt 2>&1; rc=$?; echo "exit $rc" >> evidence/bw-$S.txt; cat evidence/bw-$S.txt
[ $rc = 0 ] || { echo "before-write did not pass: nothing imported"; exit 6; }
SHOT=$S-after bash send.sh $S $F || exit 7
