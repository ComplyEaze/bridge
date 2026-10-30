#!/bin/zsh
# Part A of lab/1964-mac-alert-caps: one cell at a time. Starts the child,
# reads the panel's bounds while it is open, waits for the person's Cancel,
# records the exit. No key is pressed and nothing is clicked by this script.
LAB=$1; BIN=$2; EV=$3; FROM=${4:-A1}; started=0
mkdir -p $EV
cd $LAB
for cell input in A1 a1-one-line.txt A2 a2-ten-lines.txt A3 a3-twenty-lines.txt \
    A4a a4a-capitals-26.txt A4b a4b-capitals-31.txt A4c a4c-capitals-36.txt \
    A4d a4d-capitals-41.txt A4e a4e-capitals-46.txt A5 a5-prose-100.txt \
    A6 a6-digits-100.txt A7 a7-smallest-voucher.txt; do
  [ "$cell" = "$FROM" ] && started=1
  [ $started = 1 ] || continue
  left=$(pgrep -fl -- '--confirm-' | grep -v pgrep)
  if [ -n "$left" ]; then echo "STOP leftover before $cell: $left"; exit 9; fi
  echo "CELL $cell start $(date -u +%H:%M:%S)"
  $BIN --confirm-journal < requests/$input > $EV/$cell.out 2> $EV/$cell.err &
  child=$!
  : > $EV/$cell.bounds
  ok=0
  for try in {1..20}; do
    sleep 0.5
    <LAB>/panel-bounds > $EV/$cell.bounds.try 2>&1; rc=$?
    if [ $rc = 0 ]; then cp $EV/$cell.bounds.try $EV/$cell.bounds; echo "bounds_exit=0 try=$try" >> $EV/$cell.bounds; ok=1; break; fi
  done
  rm -f $EV/$cell.bounds.try
  [ $ok = 1 ] || echo "bounds_exit=$rc after 20 tries" >> $EV/$cell.bounds
  echo "CELL $cell bounds: $(grep '^panel' $EV/$cell.bounds | tr '\n' ' ') ok=$ok"
  echo "CELL $cell waiting for Cancel"
  wait $child; echo "exit=$?" > $EV/$cell.exit
  echo "CELL $cell $(cat $EV/$cell.exit) out_bytes=$(wc -c < $EV/$cell.out | tr -d ' ')"
done
left=$(pgrep -fl -- '--confirm-' | grep -v pgrep)
echo "END leftovers=[${left}]"
