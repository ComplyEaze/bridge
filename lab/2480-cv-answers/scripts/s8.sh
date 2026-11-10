# Lab helper (row 2480-CV): step 8, the book-wide reads and the state. s6m (M-marks.xml), s6sa and s6sb (the snapshot twice), s6ba and s6bb (B-bills.xml twice), s6ka and s6kb (C-book.xml twice), one label a send by send.sh (which runs count20.py, nm.py or cv.py on each answer), ending at the first send.sh exit other than 0. Then, file work only, each recorded whole: nm.py --same on the two snapshot and the two bills answers, st.py --names on s6sa, cv.py --same on the two census answers. Any script exit of 1 (a stop of that script) is printed as a stop and ends this script with exit 1. Usage: bash s8.sh
cd <LAB_DIR> || exit 9
S=S-snap-20260401-20260801.xml
for p in s6m:M-marks.xml s6sa:$S s6sb:$S s6ba:B-bills.xml s6bb:B-bills.xml s6ka:C-book.xml s6kb:C-book.xml; do
  L=${p%%:*}; F=${p#*:}
  bash send.sh $L $F > /dev/null; r=$?; echo "== send.sh $L $F exit $r"; [ $r = 0 ] || exit $r
done
python -I -X utf8 nm.py --same a-s6sa-$S a-s6sb-$S > evidence/same-s6s.txt 2>&1; echo "exit $?" >> evidence/same-s6s.txt
python -I -X utf8 nm.py --same a-s6ba-B-bills.xml a-s6bb-B-bills.xml > evidence/same-s6b.txt 2>&1; echo "exit $?" >> evidence/same-s6b.txt
python -I -X utf8 st.py --names a-s6sa-$S > evidence/st-names-s6.txt 2>&1; echo "exit $?" >> evidence/st-names-s6.txt
python -I -X utf8 cv.py --same a-s6ka-C-book.xml a-s6kb-C-book.xml > evidence/same-s6k.txt 2>&1; echo "exit $?" >> evidence/same-s6k.txt
st=0
for f in count-s6m nm-s6sa nm-s6sb nm-s6ba nm-s6bb cv-s6ka cv-s6kb same-s6s same-s6b st-names-s6 same-s6k; do
  x=$(tail -1 evidence/$f.txt); echo "$f: $x"; [ "$x" = "exit 1" ] && { echo "$f: a stop of its script"; st=1; }
done
exit $st
