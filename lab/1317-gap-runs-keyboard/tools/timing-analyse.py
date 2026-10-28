# Lab helper (#1317 timing rule, not product code): reads one pass's timer folder (frames.tsv, inputs.tsv, the
# captures) and prints, per form action, t0 (the input time Windows recorded for the action's key or click), t1 (the
# time of the first capture showing the action's end state) and the latency t1 - t0, plus R (the median interval
# between captures). Read only. Usage: python -I -X utf8 timing-analyse.py <timer folder>
#
# The fixed input sequence of a pass (bursts closer than 600 ms count as one input):
#   g0 focus click, g1 Display More Reports click, g2 Down, g3 Enter = action 1 (Day Book), g4 Page Down x5 = action 2,
#   g5 Escape = action 3, g6 Escape, g7 Balance Sheet click = action 4, g8 Escape = action 5,
#   g9 Profit & Loss click = action 6, g10 Escape = action 7, g11 Display More Reports click, g12 Enter = action 8
#   (Trial Balance), g13 Escape = action 8b, g14 Escape.
# An action's end state is the screen as it settled before the next action's first input (for 8b, the last capture).
import csv
import datetime
import hashlib
import os
import statistics
import sys

d = sys.argv[1]
def ts(s):
    return datetime.datetime.strptime(s, '%H:%M:%S.%f')
frames = [r for r in csv.reader(open(os.path.join(d, 'frames.tsv')), delimiter='\t') if r and r[0].isdigit()]
frames = [(ts(t), hashlib.sha256(open(os.path.join(d, f), 'rb').read()).hexdigest(), f) for _, t, f in frames]
inputs = [ts(l.strip()) for l in open(os.path.join(d, 'inputs.tsv')).read().splitlines()[1:] if l.strip()]
inputs = [t for t in inputs if t >= frames[0][0]]
groups = []
for t in inputs:
    if groups and (t - groups[-1][-1]).total_seconds() < 0.6:
        groups[-1].append(t)
    else:
        groups.append([t])
gaps = [(frames[i][0] - frames[i - 1][0]).total_seconds() for i in range(1, len(frames))]
R = statistics.median(gaps)
print('frames', len(frames), 'R (median capture interval, s) %.3f' % R, 'largest interval %.3f' % max(gaps))
print('input groups', len(groups), [g[0].strftime('%H:%M:%S.%f')[:-3] for g in groups])
# The focus click (g0) may come before the first capture and is then not seen; the pass is anchored on its last
# fourteen groups (g1 to g14), which must all be present.
if len(groups) not in (14, 15):
    print('UNEXPECTED: 14 or 15 input groups expected; not analysed')
    sys.exit(2)
groups = [None] + groups[-14:]
def settled_before(t):
    prior = [f for f in frames if f[0] < t]
    return prior[-1][1]
# action: (label, t0 group, last input group of the action, group whose first input ends the settled window or None)
acts = [('1 Day Book', 3, 3, 4), ('2 Page Down x5', 4, 4, 5), ('3 Escape to Gateway', 5, 6, 7), ('4 Balance Sheet', 7, 7, 8),
        ('5 Escape to Gateway', 8, 8, 9), ('6 Profit & Loss', 9, 9, 10), ('7 Escape to Gateway', 10, 10, 11),
        ('8 Trial Balance', 12, 12, 13), ('8b Escape to Gateway', 13, 14, None)]
for label, g0, glast, gnext in acts:
    t0 = groups[g0][0]
    tlast = groups[glast][-1]
    end = settled_before(groups[gnext][0]) if gnext is not None else frames[-1][1]
    hit = [f for f in frames if f[0] >= tlast and f[1] == end]
    if not hit:
        print('%-22s t0 %s  end state not seen' % (label, t0.strftime('%H:%M:%S.%f')[:-3]))
        continue
    t1 = hit[0][0]
    print('%-22s t0 %s  t1 %s  latency %.3f s  (end capture %s)' % (label, t0.strftime('%H:%M:%S.%f')[:-3], t1.strftime('%H:%M:%S.%f')[:-3], (t1 - t0).total_seconds(), hit[0][2]))
