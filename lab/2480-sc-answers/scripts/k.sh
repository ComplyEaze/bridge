# Lab helper (row 2480-SC, from row 2480-SK, from row 2480-MW's k8.sh): step 6, the shape step, the control alone, only after the set-up passed. k0 to k9 in ROW.md's table order, one send at a time by send.sh, the window captured at once after each (<label>-after) and checked against evidence/gateway-refs.txt, then the checks ROW.md names for it, each recorded whole in evidence/k-<label>*.txt, run before the next send; the next request is sent only if each check gave what it names. sk.py --ref is required EQUAL only when ALLBOTH is 1 (every pair's result BOTH); with 0 it is run and recorded. Anything else ends this with nothing more sent (the stop of the shape step), but MARKS DIFFER at k0 or k9, after which M-marks.xml is sent once more as k0r or k9r and recorded first. Usage: bash k.sh <ALLBOTH 1|0> <INVOICED, comma-separated>
cd <LAB_DIR> || exit 9
AB=$1; INV=$2; case "$AB" in 0|1) ;; *) echo "ALLBOTH not 0 or 1"; exit 9;; esac; [ -n "$INV" ] || { echo "INVOICED missing"; exit 9; }
stop() { echo "$1: the stop of the shape step; nothing more sent"; exit 7; }
snd() { SHOT=$1-after bash send.sh $1 $2 > evidence/send-$1.out 2>&1 || stop "$1 send.sh exit $?"
  h=$(sha256sum shots/$1-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || stop "$1 capture $h is none of evidence/gateway-refs.txt (look at it)"; }
chk() { # chk <out> <wanted line, an extended regex> <python args...>
  o=$1; w=$2; shift 2; python -I -X utf8 "$@" > evidence/$o.txt 2>&1; r=$?; echo "exit $r" >> evidence/$o.txt
  echo "== $o: $(grep -E -- "$w" evidence/$o.txt | head -1) exit $r"; [ $r = 0 ] && grep -qE -- "$w" evidence/$o.txt; }
ref() { python -I -X utf8 sk.py --ref a-$1 > evidence/k-$2-ref.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-$2-ref.txt; echo "== k-$2-ref: $(head -1 evidence/k-$2-ref.txt) exit $r"
  [ $r = 1 ] && stop "$2 a stop of sk.py --ref"; [ $AB = 1 ] && [ $r != 0 ] && stop "$2 REF not EQUAL with every pair BOTH"; return 0; }
marks() { L=$1; B=$2; snd $L M-marks.xml
  python -I -X utf8 count20.py --same-row a-$B-M-marks.xml a-$L-M-marks.xml > evidence/k-$L.txt 2>&1; r=$?; echo "exit $r" >> evidence/k-$L.txt; echo "== $L marks against $B: $(grep -h '^  equal' evidence/k-$L.txt) exit $r"
  if [ $r != 0 ] || ! grep -qx "  equal" evidence/k-$L.txt; then bash send.sh ${L}r M-marks.xml > /dev/null; echo "${L}r sent once: $(grep -h 'ALTVCHID' evidence/count-${L}r.txt)"; stop "$L MARKS DIFFER"; fi; }
marks k0 m1
snd k1 Vm-u-pl.xml; chk k-k1 ': ANSWER: VOUCHERS 1$' wx.py a-k1-Vm-u-pl.xml || stop "k1"; ref k1-Vm-u-pl.xml k1
snd k2 Vm-v-pl.xml; chk k-k2 '^SAME: EQUAL' wx.py --same a-k1-Vm-u-pl.xml a-k2-Vm-v-pl.xml || stop "k2"; ref k2-Vm-v-pl.xml k2
snd k3 Vw-u-pl.xml; chk k-k3 ': ANSWER: VOUCHERS 1$' wx.py a-k3-Vw-u-pl.xml || stop "k3"; ref k3-Vw-u-pl.xml k3
snd k4 Vw-v-pl.xml; chk k-k4 '^SAME: EQUAL' wx.py --same a-k3-Vw-u-pl.xml a-k4-Vw-v-pl.xml || stop "k4"; ref k4-Vw-v-pl.xml k4
snd k5 L-lo-pl-20260801.xml; chk k-k5 '^ANSWER: ROWS 1 [(]0 On Account[)]$' nm.py a-k5-L-lo-pl-20260801.xml || stop "k5"; ref k5-L-lo-pl-20260801.xml k5
snd k6 C-u-pl.xml; chk k-k6 ': ANSWER: VOUCHERS 1$' cv.py a-k6-C-u-pl.xml || stop "k6"
snd k7 C-v-pl.xml; chk k-k7 '^SAME: EQUAL' cv.py --same a-k6-C-u-pl.xml a-k7-C-v-pl.xml || stop "k7"
snd k8 Vb-book.xml; chk k-k8 ': ANSWER: VOUCHERS [0-9]+$' wx.py a-k8-Vb-book.xml || stop "k8"
chk k-k8-movement '^MATCH pl movement: EQUAL' wx.py --match pl movement a-k1-Vm-u-pl.xml a-k8-Vb-book.xml || stop "k8 movement"
chk k-k8-window '^MATCH pl window: EQUAL' wx.py --match pl window a-k3-Vw-u-pl.xml a-k8-Vb-book.xml || stop "k8 window"
chk k-k8-own '^OWN: YES' cv.py --own pl a-k6-C-u-pl.xml a-s2-C-book.xml a-s6ka-C-book.xml $INV || stop "k8 own"
marks k9 k0
echo "the shape step passes"
