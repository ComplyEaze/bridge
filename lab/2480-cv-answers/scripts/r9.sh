# Lab helper (row 2480-CV): step 9, the ledger reads. For each id of CREATED (the argument, comma-separated, pl first) in the table order, r<n>a and r<n>b (C-v-<id>.xml twice) by cr.sh, each capture after it one of evidence/gateway-refs.txt; then, file work only, each recorded whole: cv.py --same on the pair (same-r<n>.txt) and cv.py --own <id> r<n>a against s2's and s6ka's book census (own-r<n>.txt); cv.py on r<n>a is cr.sh's (cv-r<n>a.txt). OWN: NO or NOT JUDGED (exit 3) is a result, not a stop. Ends at a cr.sh exit other than 0 or 3 (7: a stop), a capture none of the list (6), or a cv.py stop (exit 1). Usage: bash r9.sh <CREATED>
cd <LAB_DIR> || exit 9
CR=$1; [ -n "$CR" ] || { echo "usage: bash r9.sh <CREATED>"; exit 9; }
n=0
for id in pl cm us dq ps at hs cl pc st ex; do
  n=$((n+1)); case ",$CR," in *",$id,"*) ;; *) continue;; esac
  F=C-v-$id.xml
  for s in a b; do
    L=r$n$s
    bash cr.sh $L $F > evidence/cr-$L.out 2>&1; rc=$?
    a=$(grep -ho 'ANSWER: .*' evidence/cv-$L.txt 2>/dev/null | head -1); echo "== cr.sh $L $F exit $rc: $a"
    [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: cr.sh exit $rc: stop"; exit 7; }
    [ "$(tail -1 evidence/cv-$L.txt)" = "exit 1" ] && { echo "$L: a stop of cv.py"; exit 1; }
    h=$(sha256sum shots/$L-after.png | cut -c1-64)
    grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
  done
  python -I -X utf8 cv.py --same a-r${n}a-$F a-r${n}b-$F > evidence/same-r$n.txt 2>&1; r1=$?; echo "exit $r1" >> evidence/same-r$n.txt
  python -I -X utf8 cv.py --own $id a-r${n}a-$F a-s2-C-book.xml a-s6ka-C-book.xml $CR > evidence/own-r$n.txt 2>&1; r2=$?; echo "exit $r2" >> evidence/own-r$n.txt
  echo "r$n $id: $(head -1 evidence/same-r$n.txt) | $(grep -h '^OWN' evidence/own-r$n.txt | head -1)"
  [ $r1 = 1 ] || [ $r2 = 1 ] && { echo "r$n: a stop of cv.py"; exit 1; }
done
exit 0
