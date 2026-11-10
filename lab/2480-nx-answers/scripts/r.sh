# Lab helper (row 2480-NX): step 6, for each id in the table order, LR (lr.sh r<n>a, then r<n>b, the same file), each capture after a read one of evidence/gateway-refs.txt; then, file work only, nm.py on r<n>a, --same, --outcome and --against, each recorded whole. Ends at the first lr.sh exit other than 0 or 3, or a capture none of the list. Usage: bash r.sh
cd <LAB_DIR> || exit 9
SA=a-s6sa-S-snap-20260401-20260801.xml; SB=a-s6sb-S-snap-20260401-20260801.xml; BA=a-s6ba-B-bills.xml; n=0
for id in pl cm us dq ps at hs cl pc st ex; do
  n=$((n+1)); F=L-lo-$id-20260801.xml
  for s in a b; do
    bash lr.sh r$n$s $id > evidence/lr-r$n$s.out 2>&1; rc=$?; echo "== lr.sh r$n$s $id exit $rc"
    [ $rc = 0 ] || [ $rc = 3 ] || { echo "r$n$s: lr.sh exit $rc: stop"; exit 7; }
    h=$(sha256sum shots/r$n$s-after.png | cut -c1-64)
    grep -q "^$h " evidence/gateway-refs.txt || { echo "r$n$s capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }
  done
  python -I -X utf8 nm.py a-r${n}a-$F > evidence/nm-r$n.txt 2>&1; echo "exit $?" >> evidence/nm-r$n.txt
  python -I -X utf8 nm.py --same a-r${n}a-$F a-r${n}b-$F > evidence/same-r$n.txt 2>&1; echo "exit $?" >> evidence/same-r$n.txt
  python -I -X utf8 nm.py --outcome $id $SA $SB a-r${n}a-$F a-r${n}b-$F > evidence/outcome-r$n.txt 2>&1; echo "exit $?" >> evidence/outcome-r$n.txt
  python -I -X utf8 nm.py --against $id a-r${n}a-$F $BA $SA > evidence/against-r$n.txt 2>&1; echo "exit $?" >> evidence/against-r$n.txt
  echo "r$n $id: $(grep -h '^ANSWER\|^SAME\|^OUTCOME\|^AGAINST' evidence/nm-r$n.txt evidence/same-r$n.txt evidence/outcome-r$n.txt evidence/against-r$n.txt | tr '\n' '|')"
done
