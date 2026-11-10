# Row 1381-R1P: row 1381-R1's seven kept-local answers checked and pushed, PUSH 7 of 7

Part of #1381 (plan v3, pull request 1a). File work only: no request was sent, Tally was not started and no book was opened. The sheet was `lab/1381-r1-push-sheet` (8f872368), its three files equal to its `SHA256.txt`. `check-push.py` as run: sha256 `bb6f21793219f18433549a61d386c478b0c46b1cd7b339a8ef0fc3d2d6eb5aad`, equal to `SHA256.txt`.

## The command as run

```
python -I check-push.py <row 1381-R1 folder> push-check.txt
```

Run from the sheet's folder on the lab PC, the row 1381-R1 folder given by its path there. Start 2026-11-10T20:50:42.922Z, end 2026-11-10T20:50:43.199Z, exit 0. No line began `REFUSED:`.

## Its whole output

```
Row 1381-R1P: row 1381-R1's kept-local answers checked before a push (check-push.py)
a-a1.xml: PUSH; 8270 B, sha256 92508c8e43b9ad14e75551219291b70d5f9db326ae0299b602dc4c128de683a4; registered identifier 2; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-a1b1.xml: PUSH; 10758 B, sha256 2ae12ef647f24b9645231b868f733f978b09d3a8508aab5bf6dfa2564ccbffbd; registered identifier 4; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-a1b2.xml: PUSH; 15922 B, sha256 f58ab8debba4d819c53e3d58df3d84b1bbfb050e35b78a9d506ee1cd74fc2ba3; registered identifier 6; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-a1c.xml: PUSH; 15998 B, sha256 3bdb313e443c8de87fa6f9c593611307833f93e29dc2f348133e76f00c5b800c; registered identifier 6; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-g1.xml: PUSH; 4170 B, sha256 b57cf4dc5311ee18bc767d9a3a8f6fa1220cdae9f72461066a85964b5d84602a; registered identifier 2; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-u1.xml: PUSH; 4040 B, sha256 cb9d693327fb6b8b27c9449ee9f841cc12f8a4afe2598b38692a0f825f0d4f39; registered identifier 2; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
a-m0.xml: PUSH; 3318 B, sha256 ed6554bcc9af34d808d6ee4df477803a8f22266c036aa9dd246abc909e4027e1; registered identifier 1; GSTIN outside the family 0, COMPANYNUMBER with text 0, other GUID 0, drive path 0, Users folder 0, email shape 0, 10+ digit run 0
PUSH 7 of 7
```

The same lines are in `push-check.txt` beside this file, as the script wrote them.

## The lab's own keep-local rules, run as well

Before staging, each of the seven was also read by row 2440-ED's keep-local rules, with the book being `BRIDGE STOCK CREATE LAB` (its data folder found by name and GUID, never typed): the book's GUID is in origin/master's `scripts/merge-gate-synthetic-uuids.json`; none holds row 2162-PB's second PAN part, the book's company number, the start-up company's number, the gate word, a GSTIN shape outside the family, or a digit run of 10 or more outside a VOUCHERKEY value, a UUID or a hash. All seven pass. That check script stays on the lab PC (it names lab paths): `check-1381r1p.py`, c4d0a23c02ff7471872f44215108786271e1e93de2c6499365bea0f0c7ef9837.

## What this commit adds

- `answers/a-a1.xml`, `answers/a-a1b1.xml`, `answers/a-a1b2.xml`, `answers/a-a1c.xml`, `answers/a-g1.xml`, `answers/a-u1.xml`, `answers/a-m0.xml`: byte for byte, each the size and sha256 above. Their rows are added to `SHA256.txt`.
- `1381-r1p/push-check.txt` and this file, with their rows in `SHA256.txt`.

**`kept-local.txt` is not edited.** Its entries now pushed are these seven, each listed there with the reason "company GUID": `answers/a-a1.xml`, `answers/a-a1b1.xml`, `answers/a-a1b2.xml`, `answers/a-a1c.xml`, `answers/a-g1.xml`, `answers/a-m0.xml`, `answers/a-u1.xml`. Every other entry there stays on the lab PC: the backup, the other answers, the restore plan, and the process and folder listings.

No answer was kept, and the branch gate refused none.
