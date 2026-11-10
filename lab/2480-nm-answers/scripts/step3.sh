# Lab helper (row 2480-NM): step 3 for the ids given, in the order given, each ledger then its invoice, by wr.sh. After each import the
# whole-window capture (<s>-after) must be byte-equal to the Gateway capture s1-gateway (no dialog, the same screen); if not, it stops at once
# for the operator's eyes (the prompt rule), sending nothing more. A ledger answer that is not CREATED 1, ERRORS 0 is that name's result:
# recorded, its invoice never sent, and the next id goes on. An invoice answer that is not CREATED 1, ERRORS 0: no further import, exit 5 (step 7 next).
# Any wr.sh failure (a stop of run.py, count20.py or the before-write gate): exit at once. Usage: bash step3.sh <id>...
cd <LAB_DIR> || exit 9
REF=$(sha256sum shots/s1-gateway.png | cut -c1-64)
ok() { grep -q "CREATED '1'" "$1" && grep -q "ERRORS '0'" "$1" && grep -q "EXCEPTIONS '0'" "$1" && grep -q "^exit 0" "$1"; }
for I in "$@"; do
  bash wr.sh s3$I LM-$I.xml > evidence/wr-s3$I.txt 2>&1; rc=$?; tail -14 evidence/wr-s3$I.txt
  [ -e a-s3$I-LM-$I.xml ] || { echo "s3$I: no answer (wr.sh exit $rc): stop"; exit 7; }
  [ "$(sha256sum shots/s3$I-after.png | cut -c1-64)" = "$REF" ] || { echo "s3$I-after differs from the Gateway capture: look now, press nothing"; exit 4; }
  if ! ok evidence/count-s3$I.txt; then echo "$(date -u +%FT%TZ) s3$I LM-$I.xml: not CREATED 1, ERRORS 0 (evidence/count-s3$I.txt); the window as the Gateway (s3$I-after); the name's result; I-$I.xml never sent" | tee -a evidence/notes.txt; continue; fi
  bash wr.sh s4$I I-$I.xml > evidence/wr-s4$I.txt 2>&1; rc=$?; tail -14 evidence/wr-s4$I.txt
  [ -e a-s4$I-I-$I.xml ] || { echo "s4$I: no answer (wr.sh exit $rc): stop"; exit 7; }
  [ "$(sha256sum shots/s4$I-after.png | cut -c1-64)" = "$REF" ] || { echo "s4$I-after differs from the Gateway capture: look now, press nothing"; exit 4; }
  if ! ok evidence/count-s4$I.txt; then echo "$(date -u +%FT%TZ) s4$I I-$I.xml: not CREATED 1, ERRORS 0 (evidence/count-s4$I.txt): no further import; to step 7" | tee -a evidence/notes.txt; exit 5; fi
  echo "$(date -u +%FT%TZ) $I: ledger and invoice each CREATED 1, ERRORS 0, EXCEPTIONS 0; the window as the Gateway after each (s3$I-after, s4$I-after)" | tee -a evidence/notes.txt
done
