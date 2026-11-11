# Lab helper (row 2480-SC, from row 2480-SK): step 2's checks on the s2 answers, file work only, each recorded whole: st.py <the before option> and st.py --names on the snapshot (count20.py on the N and marks answers, nm.py on B-bills and the snapshot, cv.py on C-book and sk.py --names on G-cat were run by send.sh). Prints each record's last line. Usage: bash s2chk.sh
cd <LAB_DIR> || exit 9
S=a-s2-S-snap-20260401-20260801.xml
python -I -X utf8 st.py <the before option> $S > evidence/st-before-s2.txt 2>&1; echo "exit $?" >> evidence/st-before-s2.txt
python -I -X utf8 st.py --names $S > evidence/st-names-s2.txt 2>&1; echo "exit $?" >> evidence/st-names-s2.txt
for f in count-s2 nm-s2 cv-s2 names-s2 st-before-s2 st-names-s2; do echo "$f: $(tail -n 2 evidence/$f.txt | tr '\n' ' ')"; done
