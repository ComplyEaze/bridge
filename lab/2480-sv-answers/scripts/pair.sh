# Lab helper (row 2480-SC, from row 2480-SK's pair.sh): step 3 for one pair, by w.sh, in ROW.md's order. Pair k: s4kam, s4ka LM-ka.xml, s5kam, s5ka I-ka.xml (kind a); s4kbm, s4kb LM-kb.xml (kind kb: ALTERED by recipe (e), w.sh exit 5); then s7kb G-cat.xml by send.sh (window captured, checked against evidence/gateway-refs.txt) and sc.py --altered-state, recorded whole: exit 0 (AS SK RECORDED). Pairs q and u (a, b): s4<a>m, s4<a> LM-<a>.xml, s5<a>m, s5<a> I-<a>.xml, s4<b>m, s4<b> LM-<b>.xml (kind a); s7<b> G-cat.xml by send.sh (the same capture check) and sk.py --pair <a> <b>, recorded whole: exit 0 (BOTH); then s5<b>m, s5<b> I-<b>.xml (kind a). Any other outcome ends it with nothing more sent: the stop of the set-up (exit 7, or 6 for a capture none of the list). The pair's line is appended to evidence/pair-results.txt. Usage: bash pair.sh <k|q|u>
cd <LAB_DIR> || exit 9
P=$1; a=${P}a; b=${P}b
case "$P" in k|q|u) ;; *) echo "$P is not k, q or u: nothing sent"; exit 9;; esac
st() { echo "the stop of the set-up, pair $P: $1"; exit 7; }
run() { bash w.sh "$@"; r=$?; echo "== w.sh $* exit $r"; [ $r = 6 ] && exit 6; }
for step in "m s4$a" "i s4$a LM-$a.xml a" "m s5$a" "i s5$a I-$a.xml a"; do run $step; [ $r = 0 ] || st "w.sh $step exit $r"; done
run m s4$b; [ $r = 0 ] || st "w.sh m s4$b exit $r"
SHOTB() { SHOT=s7$b-after bash send.sh s7$b G-cat.xml > evidence/send-s7$b.out 2>&1 || st "s7$b send.sh exit $?"
  h=$(sha256sum shots/s7$b-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "s7$b capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }; }
if [ $P = k ]; then
  run i s4kb LM-kb.xml kb; [ $r = 5 ] || st "w.sh i s4kb exit $r, not ALTERED"
  SHOTB
  python -I -X utf8 sc.py --altered-state a-s7kb-G-cat.xml > evidence/altstate-s7kb.txt 2>&1; rp=$?; echo "exit $rp" >> evidence/altstate-s7kb.txt; cat evidence/altstate-s7kb.txt
  [ $rp = 0 ] || st "sc.py --altered-state exit $rp"
  echo "$(date -u +%FT%TZ) pair k (ka, kb): ALTERED, AS SK RECORDED" >> evidence/pair-results.txt; echo "PAIR k: ALTERED, AS SK RECORDED"; exit 0
fi
run i s4$b LM-$b.xml a; [ $r = 0 ] || st "w.sh i s4$b exit $r"
SHOTB
python -I -X utf8 sk.py --pair $a $b a-s7$b-G-cat.xml > evidence/pair-s7$b.txt 2>&1; rp=$?; echo "exit $rp" >> evidence/pair-s7$b.txt; cat evidence/pair-s7$b.txt
[ $rp = 0 ] || st "sk.py --pair exit $rp, not BOTH"
for step in "m s5$b" "i s5$b I-$b.xml a"; do run $step; [ $r = 0 ] || st "w.sh $step exit $r"; done
echo "$(date -u +%FT%TZ) pair $P ($a, $b): BOTH" >> evidence/pair-results.txt; echo "PAIR $P: BOTH"; exit 0
