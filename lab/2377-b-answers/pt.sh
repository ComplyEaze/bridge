# Lab helper (2377-L Part B, not product code): l2377b.py present on a kept snapshot answer, then tie for one class.
# Sends nothing. Usage: bash pt.sh <class> <lo label> <snap label>
cd <LAB_DIR> || exit 9
python -I -X utf8 kit-99c3/lab/2377-b/l2377b.py present "ans/$3.answer.xml"; echo "present exit $?"
python -I -X utf8 kit-99c3/lab/2377-b/l2377b.py tie "$1" "ans/$2.answer.xml" "ans/$3.answer.xml"; echo "tie exit $?"
