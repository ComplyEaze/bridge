# Lab helper (row 2480-MW, from row 2480-MX, from row 2480-NX, from row 2480-NC's wid.sh): step 3 for one id, by w.sh, in the recipe's order: s4<id>m, s4<id> LM-<id>.xml, s5<id>m, s5<id> I-<id>.xml. Ends at the first exit of w.sh other than 0. A refused ledger (w.sh exit 4 on s4<id>) ends the id with exit 4, its invoice never sent; a refused invoice (exit 4 on s5<id>) ends it with exit 5. A refusal of the control pl is a stop (exit 7). Any other exit is passed on: the stop of the set-up. Usage: bash wid.sh <id>
cd <LAB_DIR> || exit 9
id=$1
for step in "m s4$id" "i s4$id LM-$id.xml" "m s5$id" "i s5$id I-$id.xml"; do
  bash w.sh $step; r=$?; echo "== w.sh $step exit $r"
  if [ $r = 4 ]; then
    [ $id = pl ] && { echo "the control refused: stop"; exit 7; }
    case "$step" in "i s4"*) echo "$id: ledger refused; its invoice is never sent"; exit 4;; *) echo "$id: invoice refused"; exit 5;; esac
  fi
  [ $r = 0 ] || exit $r
done
