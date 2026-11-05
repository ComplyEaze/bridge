# Lab helper (852-C2, from 1392-E, not product code): runs boxcheck.py against a copy of a saved box log in box/selftest/, so no answer
# file is written for a live box. Usage: python -I -X utf8 selftest-box.py <test tag> <batch_id> <count>
import sys
sys.argv = ['boxcheck.py'] + sys.argv[1:]
src = open('<LAB_DIR>/boxcheck.py', encoding='utf-8').read().replace("D = '<LAB_DIR>/box'", "D = '<LAB_DIR>/box/selftest'")
exec(compile(src, 'boxcheck.py', 'exec'))
