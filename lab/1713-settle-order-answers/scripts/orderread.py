# Lab helper (#1713 order rows, not product code): the probe reading for this sheet's names. The earlier sheet's read.py
# lists only ledgers whose names begin "Lab Probe", so for a `Lab Order <row>` probe it always prints probe=absent. This
# reads a LEDGERS answer by the same structure and the same C4 decoding (it imports read.py's rows() and number()), and
# prints every ledger whose name begins "Lab Order" (repr, with GUID, PARENT and ALTERID), the row count, and
# probe=present / probe=absent / probe=TWICE for the name given.
# Usage: python -I orderread.py <answer> <probe name>
import sys
sys.path.insert(0, '<lab folder>/earlier/lab/1713-abandoned-ledger-create')
import read  # noqa: E402
path, probe = sys.argv[1], sys.argv[2]
count = 0; found = []
for row in read.rows(path, 'LEDGER'):
    count += 1; name = row.get('NAME')
    if name is not None and name.startswith('Lab Order'):
        found.append((name, read.number(row, 'GUID'), row.findtext('PARENT'), read.number(row, 'ALTERID')))
mine = [f for f in found if f[0] == probe]
state = 'absent' if not mine else ('present' if len(mine) == 1 else 'TWICE')
print(f'rows {count}; Lab Order ledgers: ' + (' '.join(f'{n!r} GUID {g} PARENT {p!r} ALTERID {a}' for n, g, p, a in found) or 'none') + f'; probe={state} {probe!r}')
