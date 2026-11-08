# Lab helper (2377-L Part B, not product code): act 4 of a write, l2377b.py counters on one kept answer. Sends nothing.
# Usage: bash cnt.sh <label>
cd <LAB_DIR> || exit 9
python -I kit-99c3/lab/2377-b/l2377b.py counters "ans/$1.answer.xml"; echo "exit $?"
