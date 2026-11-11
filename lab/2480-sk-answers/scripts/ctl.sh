# Lab helper (row 2480-SK, from row 2480-MW's wid.sh): step 3 for the control pl, by w.sh (kind ctl), in the recipe's order: s4plm, s4pl LM-pl.xml, s5plm, s5pl I-pl.xml. Ends at the first exit of w.sh other than 0 (a refusal or an alteration of the control is the stop of the set-up). Usage: bash ctl.sh
cd <LAB_DIR> || exit 9
for step in "m s4pl" "i s4pl LM-pl.xml ctl" "m s5pl" "i s5pl I-pl.xml ctl"; do
  bash w.sh $step; r=$?; echo "== w.sh $step exit $r"; [ $r = 0 ] || { echo "the control: the stop of the set-up"; exit 7; }
done
echo "the control: CREATED ONE twice"
