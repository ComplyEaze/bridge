# Row 2162-RC: results

Sheet `lab/2162-rc-sheet` at 5242ca2571ca428530906777090b75b0f9968099. Run 24 Nov 2026, 15:47Z to 16:17Z, in one sitting, on TallyPrime 7.2 Silver (Tally's current date Tuesday 15-Sep-2026, period 1-Apr-26 to 31-Mar-27, no date or period key pressed), book BRIDGE PILOT LAB, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (synthetic), loaded alone. Each launch by `client.py` with the sheet's committed launch file, started whole as a background run of the operator's shell tool (Git Bash; no window, standard input from /dev/null).

- **BUILD 8c3fb9f2** (without the change): exe sha256=1e3dd5402178d6ae5576d7c410cc54247e2faf0864d291f6b56cbdc91ff76b59, launch LG1.
- **BUILD 6f541ae4** (with the change, #2593): exe sha256=9096eeedab9180d19c80b62ebe6c50917ddf7c9e9302a8e026f1b6b6679cc9cc, from the CI artifact the sheet names (zip digest and MCPB archive_sha256 as the sheet names, both checked `: OK`), launches LP, LG2 and LC. Both builds report bridge-tally 0.7.0.

## G1 and G2: the answer

**G1: BUILD 8c3fb9f2 refused the test invoice in the grown state.** `VERDICT refused invoice_not_admitted: invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998313 5%), invoice_sales_ledger_rate_shape_unmeasured (BRIDGE Svc 998314 5%)`. Confidence: partial (one book, one release, one sitting, one invoice).

**G2: BUILD 6f541ae4 admitted the same invoice in the same grown state, with no reload, load or key in Tally between.** `VERDICT admitted voucher_count 1, no refusal`. Confidence: partial (one book, one release, one sitting, one invoice).

The state both launches read was grown and byte-equal before and after each: G, G1-post and G2-post are each 30,626 B, sha256=d7f5db99170ea9a643fab195a9465bd20d1f1af1102b76fbefaefe6673a12257, with no structural difference between G and either post read. From G's read (16:08Z) to G2-post's read (16:13Z) Tally had no key, load, Shut Company or screen act; screenshots only. So by the sheet's Decides: **the change admits the measured shape on a live Tally: in one grown state, with no reload between, the build without it refused and the build with it admitted.**

## P, C and importeq

- **P: BUILD 6f541ae4 admitted the invoice in the plain state.** `VERDICT admitted voucher_count 1, no refusal`; P-post plain. Confidence: partial.
- **C: BUILD 6f541ae4 admitted it again in the plain state after a reload that cleared G.** `VERDICT admitted voucher_count 1, no refusal`; C plain. Confidence: partial.
- **importeq P against G2: `EQUAL once the identity is set aside`, exit 0.** Both batch files are 2,421 B, UTF-8; the identity derived from each answer's batch id and `bridge_txn_id` was found once as `REMOTEID` and once in the narration tag in each; with it set aside both are sha256=1c5ec4092ce745e70eb28288ae6af16703faad4fe41231659391ce577eaac706. By the Decides: in this sitting the grown state did not change the file BUILD 6f541ae4 built. Confidence: partial.

Every plain answer in the sitting is 29,114 B, sha256=a6197d9082955c5b3fc8fe9a95d2edf385b801b51a6f756c463b6cd3af215b62, all six ledgers measured. The build's rate request is `rates.xml`, 2,874 B, sha256=182f598aafaf333bc8284b6976e664cee00af1eadb5a29bd4f73c295f8601e08, the same bytes as rows 2162-RW and 2162-RX.

## The arm table

| arm | state before | build | `VERDICT` | state after | marks before and after |
|---|---|---|---|---|---|
| A0 | A0-first and A0-second plain, byte-equal (A0-third not sent) | none | not launched | n/a | 68/245, 68/245 |
| P | plain (A0-second) | 6f541ae4 (LP) | admitted voucher_count 1, no refusal | P-post plain | 68/245, 68/245 |
| G | Shut Company and load; G-pre and G-pre2 plain, byte-equal; then the walk | none | not launched | G grown (sha256=d7f5db99…) | 68/245, 68/245 |
| G1 | grown (G) | 8c3fb9f2 (LG1) | refused invoice_not_admitted, the two codes above | G1-post grown, byte-equal to G | 68/245, 68/245 |
| G2 | grown (G1-post) | 6f541ae4 (LG2) | admitted voucher_count 1, no refusal | G2-post grown, byte-equal to G | 68/245, 68/245 |
| C | Shut Company and load; C-pre and C-pre2 plain, byte-equal | 6f541ae4 (LC) | admitted voucher_count 1, no refusal | C plain | 68/245, 68/245 |

Marks are ALTVCHID/ALTMSTID by `count19.py --same-row` against `start`. No arm was skipped. Plain means 29,114 B with sha256=a6197d90…; grown means 30,626 B with sha256=d7f5db99…, each checked `: OK` against the sheet's full hash.

**G's growth.** `ratecmp.py` A0-second against G: added 28, removed 0, text changed 0. Every added path is under `BRIDGE Svc 998313 5%` or `BRIDGE Svc 998314 5%` (`GSTRATEPERUNIT` five times in each, and `GSTRATE` in rate rows 4 and 5, with their `TYPE` attributes); those two ledgers read unmeasured and the other four measured, as in rows 2162-RW and 2162-RX. The walk: Go To, GSTR-1 (period 1-Aug-26 to 31-Aug-26, no period key), 24 lines down to HSN Summary - 12 (B2B - B2C Supplies), the line 9983, then the voucher register of 998313 and of 998314, each left by one Escape, then GSTR-1 and the Gateway by one Escape each.

## The builds

| launch | build | `05` time | `05` | evidence bytes | request sha256 | response sha256 | batch file |
|---|---|---|---|---|---|---|---|
| LP (P) | 6f541ae4 | 15.164 s | admitted, voucher_count 1 | 1,974,198 | 73e152f0d2a4… | 598e857614d9… | bridge-80f628cc-e275-43ea-956a-72b8bcc5987a, 2,421 B, sha256=c2ded851d9ffc19f314ac7daaa3ea67d126f19187a688719893584faa7e04c64 |
| LG1 (G1) | 8c3fb9f2 | 11.470 s | refused, invoice_not_admitted, 2 refusals | 62,071 | 6d6404a2e22a… | 5db1811db4c0… | none; no `.xml` in data-g1 |
| LG2 (G2) | 6f541ae4 | 15.150 s | admitted, voucher_count 1 | 1,983,638 | 73e152f0d2a4… | b409ba85c0f2… | bridge-24bdd2f8-1e68-422c-858e-7c0584d0a980, 2,421 B, sha256=1f27d015d0ed85e7678ecc141f954426062c83671168c7850a7db89824f7f9a8 |
| LC (C) | 6f541ae4 | 15.313 s | admitted, voucher_count 1 | 1,974,198 | 73e152f0d2a4… | 598e857614d9… | bridge-86a964ef-8945-44b5-989e-ae2ffeae6ddd, 2,421 B, sha256=b6899b2741eaf3c600039d9ff24f4aea75e4a5df51751beb9a39c19c679d755b |

The evidence sizes and digests are recorded, not graded. LG1's are the same as row 2162-RX's refused launch; LP's and LC's are the same as each other and as row 2162-RX's admitted launch. LG2's request is the same as LP's; its evidence and response differ from LP's, as the grown rate answer it read is larger. Each batch file was equal to its answer's sha256 (`calls.py batch`, exit 0).

In every launch: `01` read `tools: 37; missing of this row's: []`; `02` one company, BRIDGE PILOT LAB, the pinned GUID; `03` and `07` M 245, V 68, reused False; the log's last line `session ended; server pid <pid> exit code 0`; client exit 0; the server error file 0 bytes; no `bridge_mcp.exe` before or after. The launch files' sha256: LP ab994a78…, LG1 9eb7d0fb…, LG2 b68a0437…, LC 933eaecc…, each equal to the sheet's line.

## Each step's EXPECT

| step | EXPECT | met |
|---|---|---|
| 0 | the sheet's commit fetched and copied (the copy line's two exits both 0); self-checks; BUILD 8c3fb9f2's exe `: OK`; the one download, its zip, MCPB and exe `: OK` (installers never extracted, opened or run); C2 with no company loaded: one tally.exe, pid 23160, on 9001, `/status` status=200, bytes=51 | met (deviation 1) |
| 1 | the Gateway, 15-Sep-2026; `start` one row, the pinned GUID, V 68, M 245; BK1 one file (90,051 B, sha256=d5fed9164066260967caf5aae98310ae3a0e32853d0bd9f86784be3ddb13f791); `setup` marks equal; `LEDGER ROWS 6`, the six names; `rates.xml` made | met (deviations 2, 3 and 4) |
| 2 | A0 first and second byte-equal and plain; LP admitted; batch equal to the answer; P-post plain, marks equal | met |
| 3 | Shut Company and load; G-pre and G-pre2 equal and plain; the walk; G grown, marks equal | met |
| 4 | LG1 refused with the two codes; no `.xml` written; G1-post grown and byte-equal to G | met |
| 5 | LG2 admitted, voucher_count 1; batch equal to the answer; G2-post grown and byte-equal to G; importeq run | met |
| 6 | Shut Company and load; C-pre and C-pre2 equal and plain; LC admitted; C plain, marks equal | met |
| 7 | arms not run written (none); `close` V 68, M 245, so no restore needed; C10: Shut Company by the recorded keys, no company loaded, one tally.exe (pid 23160) owning 9001, `/status` 200, no `bridge_mcp.exe`, no XML after the shut | met |

## Deviations and notes, each recorded in the notes

1. **The fetch's standard error** went to /dev/null in step 0's copy line; its exit was kept in the `&&` chain and the line printed both of its exits as 0. Accepted by the run lead's reply to step 0.
2. **BK1's time** is the backup file's write time (21:30 local, 16:00Z): the Restore list was not opened, so no screen was added to take its time.
3. **The backup path's capitals.** Tally showed the typed path as `C:\<lab root name>\2162-Rc\backup-1` with its own capitals; the folder is the one the sheet names and the file landed in it.
4. **The Backup screen's first state.** Backup Companies opened with the data path on `C:\` (not cloud storage) and the cursor on the company line; Backspace opened the List of Folders as the sheet expects, and the keys went on as written.
5. **The waits for answers.** In each launch's part 5, the second line (has the answer arrived) was typed as a wait: a loop that ran the sheet's `ls` and `tail` every 5 seconds, at most 60 times (the sheet's five minutes), stopping once the call's name and its log line were there. The client's exit in part 7 was waited for the same way. No answer took near the limit.
6. **How the lines were sent.** In LP each call's three lines were separate shell calls; in LG1, LG2 and LC each call's three lines ran in one shell call, in the sheet's order, with the grade only after the wait had seen the answer. No line's text was changed.

Also: Select Company shows no current date, so C10's date check is the Gateway's date at the last load (15-Sep-2026, as at step 1). The screenshot at step 0 listed other windows the screen tool hid; no window of `bridge_mcp.exe` was asked for or seen, as the sheet planned.

## Not covered

As the sheet states: the queue's re-read before a post (`admit_queued_invoice`); a post, its approval box and the read-back of a posted invoice; one code's voucher list alone followed by a launch; a goods ledger; a tax ledger in the grown state; any other screen that grows the answer; how long the grown state lasts in a session that is not shut; another book, release or sitting.

## The record

Tally's raw answers and `run.py` logs (`reads/`, the company number as X at equal length), the generated `rates.xml`, every `ratecmp.py`, `count19.py`, `calls.py` and `grade05.py` output, the self-checks, the C2 and C10 checks and the notes (`evidence/`), and each launch's client log, launch file, requests and answers (`run-*`, `company_number` redacted). Each launch's `01` answer holds the product's own tool descriptions and stays on the lab PC; its grading line is quoted above. Screenshots, BK1 and its listing, the download and its unpacked bundle, both exes, Bridge's data folders and their batch files, the server error files, the whole port and process listings, the `/status` captures and the lab's own `local/` files stay on the lab PC, listed by size and sha256 in `kept-local.txt`. The sheet's own files are on the sheet branch and are not copied again.
