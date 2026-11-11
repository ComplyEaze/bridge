# Lab helper (row 2480-SK, from row 2480-MW's q9.sh): step 7, the pair reads, only after step 6 passed. For each id given (comma-separated, a part of READ, in table order), eight sends, each once, each by er.sh (the window captured at once after each and checked against evidence/gateway-refs.txt; the exception (a) tested on an empty answer), each read by its reader before the next request: r<n>a, r<n>b (L-lo-<id>-20260801.xml, nm.py), c<n>a, c<n>b (C-v-<id>.xml, cv.py), q<n>a, q<n>b (Vm-v-<id>.xml, wx.py), w<n>a, w<n>b (Vw-v-<id>.xml, wx.py), with the form's --same on each two; then nm.py --against, cv.py --own (INVOICED), wx.py --match movement and window, and sk.py --judge for the four forms, each one record. A reader's exit 3 is a result; an er.sh exit other than 0 or 3 (7: a stop), a capture none of the list (6) or a reader's exit 1 (its stop) ends it with nothing more sent. Usage: bash r.sh <ids> <INVOICED>
cd <LAB_DIR> || exit 9
IDS=$1; INV=$2; [ -n "$IDS" ] && [ -n "$INV" ] || { echo "usage: bash r.sh <ids> <INVOICED>"; exit 9; }
S=a-s6sa-S-snap-20260401-20260801.xml; B=a-s6ba-B-bills.xml; CB=a-s2-C-book.xml; CA=a-s6ka-C-book.xml; VB=a-k8-Vb-book.xml
rec() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; r=$?; echo "exit $r" >> evidence/$o.txt; echo "== $o: $(grep -v '^exit' evidence/$o.txt | tail -1) exit $r"; [ $r = 1 ] && { echo "$o: a stop of $1, step 7, id $id"; exit 1; }; return 0; }
n=0
for id in pl ka kb ra rb qa qb ua ub ea eb; do
  n=$((n+1)); case ",$IDS," in *",$id,"*) ;; *) continue;; esac
  for p in r:L-lo-$id-20260801.xml:nm c:C-v-$id.xml:cv q:Vm-v-$id.xml:wx w:Vw-v-$id.xml:wx; do
    k=${p%%:*}; rest=${p#*:}; F=${rest%%:*}; O=${rest#*:}
    for s in a b; do
      L=$k$n$s
      bash er.sh $L $F > evidence/er-$L.out 2>&1; rc=$?
      echo "== er.sh $L $F exit $rc: $(grep -ho 'ANSWER: .*' evidence/$O-$L.txt 2>/dev/null | head -1)"
      [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: er.sh exit $rc: stop at step 7, id $id"; exit 7; }
      [ "$(tail -n 1 evidence/$O-$L.txt)" = "exit 1" ] && { echo "$L: a stop of its reader, step 7, id $id"; exit 1; }
      h=$(sha256sum shots/$L-after.png | cut -c1-64)
      grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
    done
    rec same-$k$n $O.py --same a-$k${n}a-$F a-$k${n}b-$F
  done
  rec against-$n nm.py --against $id a-r${n}a-L-lo-$id-20260801.xml $B $S
  rec own-$n cv.py --own $id a-c${n}a-C-v-$id.xml $CB $CA $INV
  rec match-q$n wx.py --match $id movement a-q${n}a-Vm-v-$id.xml $VB
  rec match-w$n wx.py --match $id window a-w${n}a-Vw-v-$id.xml $VB
  rec judge-r$n sk.py --judge $id outstandings a-r${n}a-L-lo-$id-20260801.xml $VB
  rec judge-c$n sk.py --judge $id census a-c${n}a-C-v-$id.xml $VB
  rec judge-q$n sk.py --judge $id movement a-q${n}a-Vm-v-$id.xml $VB
  rec judge-w$n sk.py --judge $id window a-w${n}a-Vw-v-$id.xml $VB
done
exit 0
