# Lab helper (row 2480-SC, from row 2480-SK, from row 2480-MW's s5.sh): step 4, the book-wide reads and the state. s6m (M-marks.xml); s6sa, s6sb (the snapshot); s6ba, s6bb (B-bills.xml); s6ga, s6gb (G-cat.xml); s6ka, s6kb (C-book.xml); one label a send by send.sh (which runs count20.py, nm.py, sk.py --names or cv.py on each answer), the window captured after each and checked against evidence/gateway-refs.txt, ending at the first send.sh exit other than 0. Then, file work only, each recorded whole: nm.py --same on each of the four pairs, st.py --names on s6sa, (row 2480-SC) sc.py --altered-state on s6ga. A script exit of 1 is printed as a stop of that script and ends this with exit 1. Usage: bash s6.sh
cd <LAB_DIR> || exit 9
S=S-snap-20260401-20260801.xml
for p in s6m:M-marks.xml s6sa:$S s6sb:$S s6ba:B-bills.xml s6bb:B-bills.xml s6ga:G-cat.xml s6gb:G-cat.xml s6ka:C-book.xml s6kb:C-book.xml; do
  L=${p%%:*}; F=${p#*:}
  SHOT=$L-after bash send.sh $L $F > /dev/null; r=$?; echo "== send.sh $L $F exit $r"; [ $r = 0 ] || exit $r
  h=$(sha256sum shots/$L-after.png | cut -c1-64); grep -q "^$h " evidence/gateway-refs.txt || { echo "$L capture $h is none of evidence/gateway-refs.txt: stop, look at it"; exit 6; }
done
for k in s:$S b:B-bills.xml g:G-cat.xml k:C-book.xml; do
  python -I -X utf8 nm.py --same a-s6${k%%:*}a-${k#*:} a-s6${k%%:*}b-${k#*:} > evidence/same-s6${k%%:*}.txt 2>&1; echo "exit $?" >> evidence/same-s6${k%%:*}.txt
done
python -I -X utf8 st.py --names a-s6sa-$S > evidence/st-names-s6.txt 2>&1; echo "exit $?" >> evidence/st-names-s6.txt
python -I -X utf8 sc.py --altered-state a-s6ga-G-cat.xml > evidence/altstate-s6ga.txt 2>&1; echo "exit $?" >> evidence/altstate-s6ga.txt
st=0
for f in count-s6m nm-s6sa nm-s6sb nm-s6ba nm-s6bb names-s6ga names-s6gb cv-s6ka cv-s6kb same-s6s same-s6b same-s6g same-s6k st-names-s6 altstate-s6ga; do
  x=$(tail -n 1 evidence/$f.txt); echo "$f: $x"; [ "$x" = "exit 1" ] && { echo "$f: a stop of its script"; st=1; }
done
exit $st
