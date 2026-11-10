# Lab helper (row 2480-NX): step 5's five reads in order, one label a send, ending at the first send.sh exit other than 0. Usage: bash s6.sh
cd <LAB_DIR> || exit 9
for p in s6m:M-marks.xml s6sa:S-snap-20260401-20260801.xml s6sb:S-snap-20260401-20260801.xml s6ba:B-bills.xml s6bb:B-bills.xml; do
  L=${p%%:*}; F=${p#*:}
  bash send.sh $L $F > /dev/null; r=$?; echo "== send.sh $L $F exit $r"; [ $r = 0 ] || exit $r
done
