# Lab helper (row 2162-FY14; not product code, from 2162-S9b's pre.sh): the sheet's check before each write. Marks read in its own
# run.py call as s<n>m, then count7.py's before-write check against s1. Exit 0 only if it passes. Times in UTC to
# evidence/sends-utc.txt.
# Usage: bash pre.sh <label>   (for example: bash pre.sh s3)
set -u; L=$1; cd <LAB_DIR>
T=$(date -u +%FT%T.%3NZ); python -I run.py http://localhost:9001 ${L}m M-marks.xml || exit 1; echo "$T ${L}m M-marks.xml" >> evidence/sends-utc.txt
python -I count7.py a-${L}m-* > evidence/count-${L}m.txt 2>&1 || { cat evidence/count-${L}m.txt; echo COUNT-FAILED; exit 1; }
python -I count7.py <the before-write option> a-s1-M-marks.xml a-${L}m-M-marks.xml > evidence/check-${L}m.txt 2>&1 || { cat evidence/check-${L}m.txt; echo CHECK-FAILED; exit 1; }
cat evidence/count-${L}m.txt evidence/check-${L}m.txt
