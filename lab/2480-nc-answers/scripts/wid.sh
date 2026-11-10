# Lab helper (row 2480-NC): step 4 for one id, by w.sh, in the recipe's order: s4<id>m, s4<id> LM-<id>.xml, s5<id>m, s5<id> I-<id>.xml. Ends at the first exit other than 0, and also at a capture that differs from p1-after's, sending nothing after it. Usage: bash wid.sh <id>
cd <LAB_DIR> || exit 9
id=$1
for step in "m s4$id" "i s4$id LM-$id.xml" "m s5$id" "i s5$id I-$id.xml"; do
  out=$(bash w.sh $step); r=$?; echo "$out" | grep -v "^  " | head -8; echo "== w.sh $step exit $r"
  [ $r = 0 ] || exit $r
  case "$step" in i*) echo "$out" | grep -q "capture equal to p1-after" || { echo "capture differs: stop"; exit 6; };; esac
done
