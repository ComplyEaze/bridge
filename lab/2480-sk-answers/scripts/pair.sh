# Lab helper (row 2480-SK): step 3 for one pair (a, b), by w.sh, in ROW.md's order: s4<a>m, s4<a> LM-<a>.xml, s5<a>m, s5<a> I-<a>.xml (kind a); s4<b>m, s4<b> LM-<b>.xml (kind b: recipe (e), then (f)); only on CREATED ONE, s7<b> G-cat.xml by send.sh (window captured, checked against evidence/gateway-refs.txt) and sk.py --pair <a> <b>, recorded whole: exit 0 (BOTH) on, exit 3 the pair's result CREATED, NOT BOTH; only after BOTH, s5<b>m, s5<b> I-<b>.xml (kind a). The pair's result (BOTH, ALTERED, REFUSED at a label, CREATED, NOT BOTH) is appended to evidence/pair-results.txt and this exits 0; any other exit of w.sh or sk.py ends it with nothing more sent, the stop of the set-up (exit 7, or 6 for a capture none of the list). Usage: bash pair.sh <a> <b>
cd <LAB_DIR> || exit 9
a=$1; b=$2; P=${a%a}
case "$a:$b" in ka:kb|ra:rb|qa:qb|ua:ub|ea:eb) ;; *) echo "$a $b is not a pair of the table: nothing sent"; exit 9;; esac
res() { echo "$(date -u +%FT%TZ) pair $P ($a, $b): $1" >> evidence/pair-results.txt; echo "PAIR $P RESULT: $1"; exit 0; }
run() { bash w.sh "$@"; r=$?; echo "== w.sh $* exit $r"; }
for step in "m s4$a" "i s4$a LM-$a.xml a" "m s5$a" "i s5$a I-$a.xml a"; do
  run $step; L=$(echo $step | cut -d' ' -f2)
  case $r in 0) ;; 4) res "REFUSED at $L";; 6) exit 6;; *) echo "the stop of the set-up"; exit 7;; esac
done
run m s4$b; [ $r = 0 ] || { echo "the stop of the set-up"; exit 7; }
run i s4$b LM-$b.xml b
case $r in 0) ;; 4) res "REFUSED at s4$b";; 5) res "ALTERED";; 6) exit 6;; *) echo "the stop of the set-up"; exit 7;; esac
SHOT=s7$b-after bash send.sh s7$b G-cat.xml > evidence/send-s7$b.out 2>&1 || { echo "s7$b send.sh exit $?: the stop of the set-up"; exit 7; }
h=$(sha256sum shots/s7$b-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "s7$b capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }
python -I -X utf8 sk.py --pair $a $b a-s7$b-G-cat.xml > evidence/pair-s7$b.txt 2>&1; rp=$?; echo "exit $rp" >> evidence/pair-s7$b.txt; cat evidence/pair-s7$b.txt
case $rp in 0) ;; 3) res "CREATED, NOT BOTH";; *) echo "sk.py --pair exit $rp: the stop of the set-up"; exit 7;; esac
for step in "m s5$b" "i s5$b I-$b.xml a"; do
  run $step; L=$(echo $step | cut -d' ' -f2)
  case $r in 0) ;; 4) res "REFUSED at $L";; 6) exit 6;; *) echo "the stop of the set-up"; exit 7;; esac
done
res "BOTH"
