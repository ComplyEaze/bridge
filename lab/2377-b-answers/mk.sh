# Lab helper (2377-L Part B, not product code): one marks send by send1.sh, then marks.py on its answer.
# Usage: bash mk.sh <label>
cd <LAB_DIR> || exit 9
bash send1.sh "$1" marks || exit $?
python -I marks.py "ans/$1.answer.xml"
