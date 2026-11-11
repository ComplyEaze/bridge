# Lab helper (row 2480-MX): step 8, part B's shape step, the control alone, only after part A passed in full. k0 to k8 in ROW.md's table order, one send at a time by send.sh, the window captured at once after each (<label>-after) and checked against evidence/gateway-refs.txt, then the check ROW.md names for it, run before the next send; the next request is sent only if that check gave what it names. Anything else ends this script with nothing more sent (the stop of the shape step), but MARKS DIFFER at k0 or k8, after which M-marks.xml is sent once more as k0r or k8r and recorded first. Each check's output goes to evidence/k-<label>.txt. Usage: bash k8.sh
cd <LAB_DIR> || exit 9
stop() { echo "$1: the stop of the shape step; nothing more sent"; exit 7; }
snd() { SHOT=$1-after bash send.sh $1 $2 > evidence/send-$1.out 2>&1 || stop "$1 send.sh exit $?"
  h=$(sha256sum shots/$1-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || stop "$1 capture $h is none of evidence/gateway-refs.txt (look at it)"; }
marks() { L=$1; B=$2; snd $L M-marks.xml
  python -I -X utf8 count20.py --same-row a-$B-M-marks.xml a-$L-M-marks.xml > evidence/k-$L.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-$L.txt; echo "== $L marks against $B: $(grep -h '^  equal' evidence/k-$L.txt) exit $r"
  if [ $r != 0 ] || ! grep -qx "  equal" evidence/k-$L.txt; then bash send.sh ${L}r M-marks.xml > /dev/null; echo "${L}r sent once: $(grep -h 'ALTVCHID' evidence/count-${L}r.txt)"; stop "$L MARKS DIFFER"; fi; }
ans() { python -I -X utf8 wx.py a-$1-$2 > evidence/k-$1.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-$1.txt; a=$(grep -ho 'ANSWER: .*' evidence/k-$1.txt | head -1); echo "== $1 $a exit $r"
  [ $r = 0 ] && echo "$a" | grep -qE '^ANSWER: VOUCHERS [0-9]+$' || stop "$1 not ANSWER: VOUCHERS <n>"; }
same() { python -I -X utf8 wx.py --same a-$1 a-$2 > evidence/k-$3.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-$3.txt; echo "== $3 $(head -1 evidence/k-$3.txt) exit $r"
  [ $r = 0 ] && grep -q '^SAME: EQUAL' evidence/k-$3.txt || stop "$3 not SAME: EQUAL"; }
match() { python -I -X utf8 wx.py --match pl $1 a-$2 a-k7-Vb-book.xml > evidence/k-k7-$1.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-k7-$1.txt; echo "== k7 $(head -1 evidence/k-k7-$1.txt) exit $r"
  [ $r = 0 ] && grep -q "^MATCH pl $1: EQUAL" evidence/k-k7-$1.txt || stop "k7 MATCH pl $1 not EQUAL"; }
marks k0 m1
snd k1 Vm-u-pl.xml; ans k1 Vm-u-pl.xml
snd k2 Vm-uv-pl.xml; same k1-Vm-u-pl.xml k2-Vm-uv-pl.xml k2
snd k3 Vm-v-pl.xml; same k1-Vm-u-pl.xml k3-Vm-v-pl.xml k3
snd k4 Vw-u-pl.xml; ans k4 Vw-u-pl.xml
snd k5 Vw-uv-pl.xml; same k4-Vw-u-pl.xml k5-Vw-uv-pl.xml k5
snd k6 Vw-v-pl.xml; same k4-Vw-u-pl.xml k6-Vw-v-pl.xml k6
snd k7 Vb-book.xml; ans k7 Vb-book.xml; match movement k1-Vm-u-pl.xml; match window k4-Vw-u-pl.xml
marks k8 k0
for x in k2:Vm-uv-pl.xml k3:Vm-v-pl.xml k5:Vw-uv-pl.xml k6:Vw-v-pl.xml; do echo "== record $x: $(grep -ho 'ANSWER: .*' evidence/wx-${x%%:*}.txt | head -1)"; done
echo "the shape step passes"
