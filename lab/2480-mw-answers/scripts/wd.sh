# Lab helper (row 2480-MW, from row 2480-MX, from row 2480-CV, from row 2480-NX): the wedge watch for one bg.sh run. Ends when evidence/run-<name>.rc exists, or after 300 s (the wedge rule's 5 minutes from the run's start) printing WEDGE. It sends nothing. Usage: bash wd.sh <name>
cd <LAB_DIR> || exit 9
for i in $(seq 1 300); do [ -e evidence/run-$1.rc ] && { echo "run-$1 ended, exit $(cat evidence/run-$1.rc)"; exit 0; }; sleep 1; done
echo "WEDGE: run-$1 has not ended 300 s after the watch began"; exit 1
