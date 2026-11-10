# Row 1381-R1P: push row 1381-R1's seven kept-local answers for #1381's fixtures

Part of #1381 (plan v3, pull request 1a).

**File work only.** No request is sent, Tally is not started, and no book is opened. The row checks seven files already on the lab PC and pushes those that pass.

## Why

Row 1381-R1 (`lab/1381-r1-answers` at 3b50a15e) kept its read answers on the lab PC. Each carries the company identifier of `BRIDGE STOCK CREATE LAB`, the synthetic company created for row 1381-M, and `kept-local.txt` gives that as the only reason. #2419 lists that identifier in `scripts/merge-gate-synthetic-uuids.json` on master, so the branch gate now admits it. #1381's fixtures pull request (1b) copies the pushed answers from that branch.

## The seven answers

From row 1381-R1's `kept-local.txt`, in that row's `answers/` folder on the lab PC:

| answer | what it is | bytes | sha256 |
| --- | --- | --- | --- |
| `a-a1.xml` | the candidate stock-item read, before the creates | 8,270 | `92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4` |
| `a-a1b1.xml` | the same read after b1 (the plain form) | 10,758 | `2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd` |
| `a-a1b2.xml` | the same read after b2 (the marked form) | 15,922 | `f58ab8debba4d819c53e3d58df3d84b1bbfb050e35b78a9d506ee1cd74fc2ba3` |
| `a-a1c.xml` | the same read with the alias keyed | 15,998 | `3bdb313e443c8de87fa6f9c593611307833f93e29dc2f348133e76f00c5b800c` |
| `a-g1.xml` | the stock groups read | 4,170 | `b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a` |
| `a-u1.xml` | the units read | 4,040 | `cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39` |
| `a-m0.xml` | the marks read before the row | 3,318 | `ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1` |

Everything else in that `kept-local.txt` stays on the lab PC: the backup, the other answers, the restore plan, and the process and folder listings.

## Run

From this folder, with the lab PC's `python` (there is no `python3` command):

```
python -I check-push.py <row 1381-R1 folder> push-check.txt
```

`check-push.py` checks each answer's size and sha256 against the table above, decodes it as UTF-16LE with no byte order mark, and counts, never printing a value:
- GSTIN-shaped values outside the synthetic identifier family;
- `COMPANYNUMBER` elements with any text;
- GUID-shaped values that are not `BRIDGE STOCK CREATE LAB`'s registered identifier (an item's GUID is that identifier followed by a hyphen and digits, and counts as it);
- drive-letter paths, home-folder paths (a Users folder between path separators), email shapes, and runs of 10 or more digits.

An answer is `PUSH` only if it matches and every count is 0. Otherwise it is `KEEP`, with the counts that are not 0.

## Expect

All seven `PUSH`, each with its registered identifier count above 0, and `PUSH 7 of 7`. Any `KEEP` is recorded, not a stop: that answer stays on the lab PC and the others go on.

## Push

To `lab/1381-r1-answers`, as a commit on 3b50a15e, after the branch gate passes on the new head:
- each `PUSH` answer into `lab/1381-r1-answers/answers/`, byte for byte (the folder's `.gitattributes` is `* -text`);
- `push-check.txt`, whole;
- a `RESULTS.md` for this row: the command as run, its whole output, the time, and the sha256 of `check-push.py` as run (it must match `SHA256.txt`);
- `kept-local.txt` itself is not edited. `RESULTS.md` lists which of its entries are now pushed.

If the branch gate refuses an answer that `check-push.py` passed, that answer stays local, `RESULTS.md` records the gate's line for it, and the rest are pushed.
