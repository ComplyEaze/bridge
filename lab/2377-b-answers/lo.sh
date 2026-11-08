# Lab helper (2377-L Part B, not product code): l2377.py lo on one kept answer. Sends nothing.
# Usage: bash lo.sh <label>
cd <LAB_DIR> || exit 9
python -I -X utf8 kit-99c3/lab/2377-b/l2377.py lo "ans/$1.answer.xml"; echo "exit $?"
