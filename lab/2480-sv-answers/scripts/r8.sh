# Lab helper (row 2480-SV, new; written whole by an editor): step 8 for one id, in ROW.md's order. Each send once by er.sh (which sends by send.sh, captures the whole window at once after it as <label>-after, and tests exception (a) on an empty answer), the capture checked against evidence/gateway-refs.txt, each answer read by its reader before the next request (send.sh runs cv.py, wx.py or nm.py on an answer with bytes; er.sh runs wx.py alone on an empty answer). The sends: r<n>ca, r<n>cb (C-v-<id>.xml); r<n>qa, r<n>qb (Vm-v-<id>.xml); r<n>wa, r<n>wb (Vw-v-<id>.xml); for ka and kb only, r<n>la, r<n>lb (L-lo-<id>-20260801.xml). After each pair: cv.py --same on a C pair with bytes in both, else wx.py --same (an empty answer's pair, the V pairs and the L-lo pair), recorded whole in evidence/same-r<n><x>.txt. Then sv.py --judge2 <id> <form> on each a answer against BOOK (census, movement, window, outstandings), recorded whole in evidence/judge2-r<n><x>.txt. A reader's answer is a result whatever it holds; the run ends with nothing more sent only at an er.sh exit 7 (exit 7), a capture none of the list (exit 6), or an exit 1 of a reader (exit 7): "a stop at step 8, id <id>". Usage: bash r8.sh <n> <id> <BOOK>
cd <LAB_DIR> || exit 9
N=$1; ID=$2; BOOK=$3
case "$ID" in qa|qb|ua|ub|ka|kb) ;; *) echo "$ID is not one of the six: nothing sent"; exit 9;; esac
[ -f "$BOOK" ] || { echo "$BOOK absent: nothing sent"; exit 9; }
st() { echo "a stop at step 8, id $ID: $1"; exit ${2:-7}; }
R=(); [ $ID = ka ] || [ $ID = kb ] && L=1 || L=0
EMPTY=" "
send() { # label file
  bash er.sh $1 $2 > evidence/er-$1.out 2>&1; r=$?; echo "== er.sh $1 $2 exit $r"; cat evidence/er-$1.out
  h=$(sha256sum shots/$1-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || st "capture $h of $1 is none of evidence/gateway-refs.txt, look at it" 6
  echo "capture $h, one of the Gateway captures in evidence/gateway-refs.txt"
  case $r in 3) ;; 0) EMPTY="$EMPTY$1 ";; *) st "er.sh $1 exit $r";; esac
  for f in evidence/cv-$1.txt evidence/wx-$1.txt evidence/nm-$1.txt; do [ -f $f ] && grep -qx "exit 1" $f && st "reader exit 1 in $f"; done
  return 0
}
pair() { # x file kind(c|v)
  a=r$N${1}a; b=r$N${1}b
  send $a $2; send $b $2
  if [ $3 = c ] && [[ "$EMPTY" != *" $a "* ]] && [[ "$EMPTY" != *" $b "* ]]; then
    python -I -X utf8 cv.py --same a-$a-$2 a-$b-$2 > evidence/same-r$N$1.txt 2>&1; rs=$?
  else
    python -I -X utf8 wx.py --same a-$a-$2 a-$b-$2 > evidence/same-r$N$1.txt 2>&1; rs=$?
  fi
  echo "exit $rs" >> evidence/same-r$N$1.txt; cat evidence/same-r$N$1.txt
  [ $rs = 1 ] && st "the --same line of r$N$1 exit 1"
  return 0
}
pair c C-v-$ID.xml c
pair q Vm-v-$ID.xml v
pair w Vw-v-$ID.xml v
[ $L = 1 ] && pair l L-lo-$ID-20260801.xml v
J() { # x form file
  python -I -X utf8 sv.py --judge2 $ID $2 a-r$N${1}a-$3 $BOOK > evidence/judge2-r$N$1.txt 2>&1; rj=$?
  echo "exit $rj" >> evidence/judge2-r$N$1.txt; cat evidence/judge2-r$N$1.txt
  [ $rj = 1 ] && st "sv.py --judge2 $2 exit 1"
  return 0
}
J c census C-v-$ID.xml
J q movement Vm-v-$ID.xml
J w window Vw-v-$ID.xml
[ $L = 1 ] && J l outstandings L-lo-$ID-20260801.xml
echo "$(date -u +%FT%TZ) step 8 id $ID (n=$N): every send answered; empty answers under (a):${EMPTY}" >> evidence/r8-results.txt
echo "STEP 8 $ID: DONE (empty under (a):${EMPTY})"; exit 0
