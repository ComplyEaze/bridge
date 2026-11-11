# Lab helper (row 2480-MX): step 9, part B's ledger reads, only after step 8 passed. For each id given (comma-separated, a part of B's ids, in table order), four sends, each once, each read by wx.py before the next request: q<n>a and q<n>b (Vm-v-<id>.xml), w<n>a and w<n>b (Vw-v-<id>.xml), each by vr.sh (the window captured at once after each, checked against evidence/gateway-refs.txt; the exception (b) tested on an empty answer); wx.py --same on each pair; then wx.py --match <id> movement and window against a-k7-Vb-book.xml. An answer wx.py reads is a result; exit 3 of --match is a result. A vr.sh exit other than 0 or 3 (7: a stop), a capture none of the list (6), or a wx.py exit 1 (its stop) ends it with nothing more sent. Usage: bash q9.sh <ids>
cd <LAB_DIR> || exit 9
IDS=$1; [ -n "$IDS" ] || { echo "usage: bash q9.sh <ids>"; exit 9; }
n=0
for id in pl ga ss ms qd pp hd ad ca xs al; do
  n=$((n+1)); case ",$IDS," in *",$id,"*) ;; *) continue;; esac
  for p in q:Vm w:Vw; do
    k=${p%%:*}; F=${p#*:}-v-$id.xml
    for s in a b; do
      L=$k$n$s
      bash vr.sh $L $F > evidence/vr-$L.out 2>&1; rc=$?
      echo "== vr.sh $L $F exit $rc: $(grep -ho 'ANSWER: .*' evidence/wx-$L.txt 2>/dev/null | head -1)"
      [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: vr.sh exit $rc: stop at step 9, id $id"; exit 7; }
      [ "$(tail -n 1 evidence/wx-$L.txt)" = "exit 1" ] && { echo "$L: a stop of wx.py, step 9, id $id"; exit 1; }
      h=$(sha256sum shots/$L-after.png | cut -c1-64)
      grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
    done
    python -I -X utf8 wx.py --same a-$k${n}a-$F a-$k${n}b-$F > evidence/same-$k$n.txt 2>&1; r=$?; echo "exit $r" >> evidence/same-$k$n.txt; echo "== $k$n $(head -1 evidence/same-$k$n.txt)"
    [ $r = 1 ] && { echo "$k$n: a stop of wx.py --same"; exit 1; }
  done
  python -I -X utf8 wx.py --match $id movement a-q${n}a-Vm-v-$id.xml a-k7-Vb-book.xml > evidence/match-q$n.txt 2>&1; r1=$?; echo "exit $r1" >> evidence/match-q$n.txt
  python -I -X utf8 wx.py --match $id window a-w${n}a-Vw-v-$id.xml a-k7-Vb-book.xml > evidence/match-w$n.txt 2>&1; r2=$?; echo "exit $r2" >> evidence/match-w$n.txt
  echo "n=$n $id: $(head -1 evidence/match-q$n.txt) | $(head -1 evidence/match-w$n.txt)"
  [ $r1 = 1 ] || [ $r2 = 1 ] && { echo "$id: a stop of wx.py --match"; exit 1; }
done
exit 0
