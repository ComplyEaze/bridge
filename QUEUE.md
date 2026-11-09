# Lab queue

The lab reads this file from `origin/lab/queue` at the end of each item, with `git fetch origin lab/queue` and `git show origin/lab/queue:QUEUE.md`. The newest block is at the bottom. Run the open items in order. Push answers branches only after the reviewer's yes.

## 9 Nov 2026

1. W4 correction: commit the note that the W4 export step ran the 30 Oct 0.6.1 desktop app, while the server rows used the bundle's server, to `lab/2074-2214-w4-answers`. Send its sha. A push is approved once the lab's own checks pass, because the book is synthetic.
2. Part W (W1 to W3, and O) is skipped. It needs the new desktop app installed, which needs the owner's hands. Record why.
3. Let A3 end at its cap. Then restore BATCH LAB from this morning's backup (C6) and check that the marks are back at 224/87.
4. `lab/1381-r1-sheet` at c76e23d6. Answers on `lab/1381-r1-answers`.
5. `lab/2162-class-sheet` at 8d6c7ab4. Answers on `lab/2162-class-answers`.
6. `lab/2119-p-sheet` at 742fc289 (reads only, DENSE). Build: artifact 13650831945 (unsigned-bundle-smoke-windows-latest) of run 42562853635, commit d04f67a9. Record the exe's sha256 yourself. Answers on `lab/2119-p-answers`.
7. `lab/2162-m1-sheet` at eaf2a4fc (reads only, raw requests, no Bridge build): parts A and B on `BRIDGE PILOT LAB`, part C on `BRIDGE EINVOICE LAB`. Check `SHA256.txt` first. Answers on `lab/2162-m1-answers`.

Item 1 is done: `lab/2074-2214-w4-answers` at a16b7cc7, gate PASS.
8. `lab/2162-bc-sheet` at e2b1ca82, right after item 7 in the same pilot sitting. It writes to `BRIDGE PILOT LAB` only: take BK first (C5), restore at the end (C6), and check the COMPANY row's marks equal step 1's. Answers on `lab/2162-bc-answers`.
Done so far: item 6 at a3611599, item 4 at e870ba8c. Both gate PASS.
Item 5: use the queue's branch name, `lab/2162-class-answers`. Where the sheet says otherwise, the queue wins.
