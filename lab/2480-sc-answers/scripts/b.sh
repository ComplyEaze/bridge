# Lab helper (row 2480-SC): step 8, Part B, only after Part A passed in full. "bash b.sh pre": sc.py --judge-b ka movement a-k8 a-k8, recorded whole (evidence/judge-b-pre.txt); it must print JUDGE-B ka movement: OWN YES, OTHERS <n> (exit 3), else exit 8 (Part B not sent). "bash b.sh <first> <last>": the table's rows first to last (1 to 11), each in order: b<i>a, b<i>b by er.sh (the window captured at once after each send and checked against evidence/gateway-refs.txt; exception (a) tested on an empty answer; an answer with bytes read by nm.py, cv.py or wx.py by send.sh, an empty one by wx.py alone), each read before the next request; then wx.py --same on the two (evidence/same-b<i>.txt) and sc.py --judge-b <spelling> <form> a-b<i>a-<file> a-k8-Vb-book.xml (evidence/judge-b<i>.txt), each recorded whole. "bash b.sh cross": the spellings against each other (b1/b8, b2/b9, b3/b10, b4/b11) and the forms against each other for ka (b2/b5, b3/b6, b4/b7), each wx.py --same on the a answers (evidence/sp-b<i>.txt, ff-b<i>.txt). An er.sh exit other than 0 or 3 (7), a capture none of the list (6) or a reader's exit 1 (its stop, Part B's) ends it with nothing more sent. Usage: bash b.sh pre | bash b.sh <first> <last> | bash b.sh cross
cd <LAB_DIR> || exit 9
VB=a-k8-Vb-book.xml
T=(x "L-lo-ka-20260801.xml ka outstandings" "C-v-ka.xml ka census" "Vm-v-ka.xml ka movement" "Vw-v-ka.xml ka window" "C-u-ka.xml ka census" "Vm-u-ka.xml ka movement" "Vw-u-ka.xml ka window" "L-lo-kb-20260801.xml kb outstandings" "C-v-kb.xml kb census" "Vm-v-kb.xml kb movement" "Vw-v-kb.xml kb window")
rec() { o=$1; shift; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; r=$?; echo "exit $r" >> evidence/$o.txt; echo "== $o: $(grep -v '^exit' evidence/$o.txt | tail -1) exit $r"; [ $r = 1 ] && { echo "$o: a stop of $1, step 8"; exit 1; }; return 0; }
if [ "$1" = pre ]; then
  python -I -X utf8 sc.py --judge-b ka movement $VB $VB > evidence/judge-b-pre.txt 2>&1; r=$?; echo "exit $r" >> evidence/judge-b-pre.txt; cat evidence/judge-b-pre.txt
  grep -qE '^JUDGE-B ka movement: OWN YES, OTHERS [0-9]+$' evidence/judge-b-pre.txt && exit 0; echo "not the line step 8 asks for: Part B is not sent"; exit 8
fi
if [ "$1" = cross ]; then
  for p in 1:8 2:9 3:10 4:11; do i=${p%%:*}; j=${p#*:}; set -- ${T[$i]}; Fi=$1; set -- ${T[$j]}; Fj=$1; rec sp-b$i wx.py --same a-b${i}a-$Fi a-b${j}a-$Fj; done
  for p in 2:5 3:6 4:7; do i=${p%%:*}; j=${p#*:}; set -- ${T[$i]}; Fi=$1; set -- ${T[$j]}; Fj=$1; rec ff-b$i wx.py --same a-b${i}a-$Fi a-b${j}a-$Fj; done
  exit 0
fi
A=$1; Z=$2; [ "$A" -ge 1 ] && [ "$Z" -le 11 ] && [ "$A" -le "$Z" ] || { echo "rows 1 to 11 only"; exit 9; }
for i in $(seq $A $Z); do
  set -- ${T[$i]}; F=$1; SP=$2; FM=$3
  case "$F" in L-lo-*) O=nm;; C-*) O=cv;; *) O=wx;; esac
  for s in a b; do
    L=b$i$s
    bash er.sh $L $F > evidence/er-$L.out 2>&1; rc=$?
    if [ $rc = 0 ]; then R=wx; else R=$O; fi
    echo "== er.sh $L $F exit $rc: $(grep -ho 'ANSWER: .*\|EMPTY.*' evidence/$R-$L.txt 2>/dev/null | head -1)"
    [ $rc = 0 ] || [ $rc = 3 ] || { echo "$L: er.sh exit $rc: stop at step 8, id $SP"; exit 7; }
    [ "$(tail -n 1 evidence/$R-$L.txt)" = "exit 1" ] && { echo "$L: a stop of its reader, step 8, id $SP"; exit 1; }
    h=$(sha256sum shots/$L-after.png | cut -c1-64)
    grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: look at it"; exit 6; }
  done
  rec same-b$i wx.py --same a-b${i}a-$F a-b${i}b-$F
  rec judge-b$i sc.py --judge-b $SP $FM a-b${i}a-$F $VB
done
exit 0
