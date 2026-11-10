# Lab helper (row 2480-NX): step 3, p1 to p11 by lr.sh in table order; goes on only while lr.sh exits 0 (the empty-answer exception held); ends at any other exit (3: bytes came back, recorded by hand with nm.py --against; 7: a stop). Usage: bash p.sh
cd <LAB_DIR> || exit 9
n=0
for id in pl cm us dq ps at hs cl pc st ex; do
  n=$((n+1)); bash lr.sh p$n $id > evidence/lr-p$n.out 2>&1; r=$?
  echo "p$n $id exit $r: $(grep -h 'ANSWER' evidence/nm-p$n.txt 2>/dev/null | head -1)"
  [ $r = 0 ] || exit $r
done
