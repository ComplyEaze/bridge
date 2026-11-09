# Lab helper (row 703-G, not product code): the sheet's closed set of calls (rule 2) through the running client in run-$1, in order, then egress_log 50.
set -u; export PRUN=run-$1; cd <row folder>
python -I -X utf8 pcall.py marks-a ledger_masters args/basic.json || exit 1
python -I -X utf8 pcall.py vouchers vouchers args/v.json --shots || exit 1
for i in 1 2 3 4 5 6 7 8; do python -I -X utf8 pcall.py gst$i gst_status args/g$i.json --answer-shot || exit 1; done
python -I -X utf8 pcall.py marks-b ledger_masters args/basic.json || exit 1
python -I -X utf8 pcall.py egress egress_log args/egress50.json || exit 1
