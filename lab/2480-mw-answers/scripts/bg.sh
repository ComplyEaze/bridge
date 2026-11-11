# Lab helper (row 2480-MW, from row 2480-MX, from row 2480-CV, from row 2480-NX): runs one command for the row in the background of the operator's shell and writes its output and exit to evidence/run-<name>.out and .rc, so a send that waits is never ended by a tool's time limit (the wedge rule: do not end curl). Usage: bash bg.sh <name> <command...>
cd <LAB_DIR> || exit 9
n=$1; shift
[ -e evidence/run-$n.rc ] && { echo "run-$n already used"; exit 9; }
echo "$(date -u +%FT%T.%3NZ) start $*" > evidence/run-$n.out
"$@" >> evidence/run-$n.out 2>&1; r=$?
echo "$(date -u +%FT%T.%3NZ) end, exit $r" >> evidence/run-$n.out; echo $r > evidence/run-$n.rc
echo "run-$n exit $r"
