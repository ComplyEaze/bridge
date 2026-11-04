# Lab helper (2119-B, not product code): one step's send. Sends req/<name>.utf16le.xml once by the sheet's C3 line through
# lab/2119-l-answers' scripts/c3-send-t.ps1 (unchanged), with the sha256 tie.py printed for it, into answers/<label>.xml
# (bound 2,000,000 B; the helper never overwrites an answer), logs to evidence/send-log.txt, then captures Tally's whole
# window (cap.py) before anything else is sent. Usage: bash send.sh <request name> <answer label>
set -u; R=<LAB_DIR>; cd $R; name=$1; label=$2
sha=$(grep "^$name.utf16le.xml " evidence/fixed.txt | awk '{print $3}')
[ -n "$sha" ] || { echo "HALT: no sha line for $name"; exit 2; }
powershell -NoProfile -ExecutionPolicy Bypass -File "<LAB_DIR>\\scripts\\c3-send-t.ps1" -Req "<LAB_DIR>\\req\\$name.utf16le.xml" -Sha $sha -AnsDir "<LAB_DIR>\\answers" -Ans "$label.xml" -Bound 2000000 -Log "<LAB_DIR>\\evidence\\send-log.txt"
ec=$?; python -I cap.py after-$label; exit $ec
