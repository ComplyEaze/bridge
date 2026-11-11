# Lab helper (row 2480-MX, from row 2480-CV's s8.sh): step 5, the book-wide reads and the state. s6m (M-marks.xml), s6sa and s6sb (the snapshot twice), s6ba and s6bb (B-bills.xml twice), one label a send by send.sh (which runs count20.py or nm.py on each answer), ending at the first send.sh exit other than 0. Then, file work only, each recorded whole: nm.py --same on each pair, st.py --names on s6sa. Any script exit of 1 is printed as a stop of that script and ends this script with exit 1. Usage: bash s5.sh
cd <LAB_DIR> || exit 9
S=S-snap-20260401-20260801.xml
for p in s6m:M-marks.xml s6sa:$S s6sb:$S s6ba:B-bills.xml s6bb:B-bills.xml; do
  L=${p%%:*}; F=${p#*:}
  bash send.sh $L $F > /dev/null; r=$?; echo "== send.sh $L $F exit $r"; [ $r = 0 ] || exit $r
done
python -I -X utf8 nm.py --same a-s6sa-$S a-s6sb-$S > evidence/same-s6s.txt 2>&1; echo "exit $?" >> evidence/same-s6s.txt
python -I -X utf8 nm.py --same a-s6ba-B-bills.xml a-s6bb-B-bills.xml > evidence/same-s6b.txt 2>&1; echo "exit $?" >> evidence/same-s6b.txt
python -I -X utf8 st.py --names a-s6sa-$S > evidence/st-names-s6.txt 2>&1; echo "exit $?" >> evidence/st-names-s6.txt
st=0
for f in count-s6m nm-s6sa nm-s6sb nm-s6ba nm-s6bb same-s6s same-s6b st-names-s6; do
  x=$(tail -n 1 evidence/$f.txt); echo "$f: $x"; [ "$x" = "exit 1" ] && { echo "$f: a stop of its script"; st=1; }
done
exit $st
