# Windows item 1029-AB v0.1 DRAFT: a 7-day voucher range on a book of real volume, and the after-post read-back on a busy book (25 Oct 2026; for issue #1029, rows A and B of the release-candidate sheet). SYNTHETIC ONLY: THE COMPANY = BRIDGE CORPUS DENSE (one financial year, 1 Apr 2025 to 31 Mar 2026, voucher mark about 29,900, about 82 vouchers a day; the book the 22 to 25 September runs used). Its company number and GUID are recorded at step 0 and every later step is checked against that GUID. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. Part A only reads. Part B posts 15 Journals in five approved batches and is undone by restoring a backup. Needs the run coordinator's go-ahead, and the owner on the Tally screen for every approval.

Why: #1029 item 2 (a range of several days read in one request, with the rule that labels a range `complete` only when every voucher read was checked against a separate count) was seen only on a book of 8 vouchers. Item 3 (how often the read-back after an approved post refuses on a busy book) has no measurement. Both are measurements only; nothing is built or changed by this item.

## The build and its settings
- TallyPrime Release 7.2 or the lab's current release, licensed (Educational mode anywhere = STOP), port 9001. Part A uses the `bridge_mcp` program of master at the head the go-ahead names, from that head's own CI run; the go-ahead names its sha256, and a program with another hash is not started. Posting and ledger creation OFF for part A. Server name `bridge-lab-1029-r`.
- Part B uses the same program with posting ON under a second server name `bridge-lab-1029-w`; part A's server is shut first. Exactly one ComplyEaze Bridge is connected to the assistant at a time; every tool call goes to it, one at a time, with exactly the arguments the step gives; a call in flight is never cancelled, abandoned or repeated.
- After every call the assistant keeps the answer whole as a file (named by step) with its size and sha256. Part B's own egress receipt (`egress_log`) is read at step 9.

## Differences from LAB-COMMON
- C1: THE COMPANY already exists; it is found by name in Select Company. Not found, or two companies of that name: STOP.
- C3: size bounds: MARKS 20,000 bytes; the 7-day VCH read 8,000,000 bytes (about 575 vouchers at about 3.3 KB each, plus the ledger entries); a tool answer above 1,000,000 bytes = STOP unless the step says otherwise.
- C5: one backup, B0, before the first post. C6: one restore, of B0, at step 10.
- OWNER STEPS: step 7 (the five approvals). The operator never keys.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
- MARKS: `dense-marks.utf16le.xml`, 982 bytes, sha256 040ac47754c40a4e2e736f8be2b3f37cac924213ea66c8c4f0074222f9a00280.
- VCH7, the 7-day window 20250901 to 20250907: `dense-vch-7day.utf16le.xml`, 1,754 bytes, sha256 f2ec9db100a74e4d1c86b0ffd45d1a4c2e4184b02a705d385672f6c6d782a80a. Both are the pinned `Bridge Agent Company High Water` and `Bridge Identity Probe Window` requests of the 13 October set with only the company, the collection and filter names, and the dates changed (checked: the same templates reproduce those requests' recorded hashes).

## Steps (LAB-COMMON start and step 0 first)
- 0. Step 0 (C2). MARKS: EXPECT exactly one COMPANY row; RECORD GUID, ALTVCHID and ALTMSTID. EXPECT ALTVCHID between 25,000 and 35,000, else STOP ("not the dense book").
- 1. BACKUP B0 (C5) to the NEW folder `C:\bridge-lab\1029-AB\backups\B0\`; record both marks after it.
- 2. VCH7 raw (C3, once). Decode and count (C4). EXPECT at least 400 vouchers in the window and every GUID beginning with THE COMPANY's GUID. RECORD the count per day. This is the separate count the tool's answer is compared with. A count under 400 = STOP ("the window is not dense").
- 3. Part A, tool call 1: `voucher_read_cost` for 2025-09-01 to 2025-09-07. RECORD the answer whole (it states the preview's floor of requests).
- 4. Part A, tool call 2: `vouchers` for 2025-09-01 to 2025-09-07, ONE call, no paging arguments. EXPECT: the window labelled complete; the item count equal to step 2's count; no `voucher_window_part_not_admitted`; the number of Tally requests the answer states within the floor step 3 gave. A refusal, a partial label or a count that differs is a RESULT: record it and go to step 8 (no step 5 or 6).
- 5. Part A, tool call 3: `voucher_presence` with two proposals on that window: P1 = one voucher copied exactly from step 2's table (type, number, date, party, ledger entries and amounts); P2 = the same with one amount raised by 1.00 and a voucher number not in the table. EXPECT `present` for P1 and `absent` for P2. RECORD both answers whole.
- 6. MARKS. EXPECT both marks equal to step 1's after-backup marks (reads change nothing), else RECORD as a result and STOP before part B.
- 7. Part B (server `bridge-lab-1029-w` only). Five batches of three Journals each, built with `build_import_xml` and posted with `post_import`, each batch approved by the owner on the native window, dates inside THE COMPANY's year, narration `1029-B<batch>-<k>`, amounts 10.00 to 70.00 between two ledgers the book already has. For each batch: MARKS before; the post; the tool's outcome label; MARKS after. COUNT the batches whose read-back after the post is refused (the tool's outcome is not `posted_verified` and it names `verify_import`). EXPECT none. A refused batch is a RESULT: run `verify_import` once for it, record, and continue with the next batch. An import counter failure, or a dialog in Tally = STOP (C7).
- 8. VCH7 raw once more. Record the count. (After part B it should differ from step 2 only by Journals dated inside the window, if any: say which.)
- 9. `egress_log`: RECORD the answer whole (every send of part B).
- 10. RESTORE B0 (C6), then MARKS and VCH7 raw: EXPECT the marks equal step 1's after-backup marks, and the VCH7 table equal to step 2's, compared by program; else STOP "the book is not as found".
- 11. Close (C10).
- THE RESULT is one page: step 2's count per day; steps 3 and 4's answers (floor, requests used, label, count, any refusal); step 5's two answers; for each of the five batches the marks before and after, the outcome label and whether the read-back was refused; the number refused out of five.

## Stop rules (beyond LAB-COMMON)
The wrong book at step 0; step 2 under 400; a tool call that does not answer within 10 minutes (never repeated; leave it running, report); any Tally dialog; a write that is not one of the five batches.
WHAT THE BOOK HOLDS IF THE ITEM STOPS: before step 7, as found (B0 exists); during or after step 7, as found plus the batches posted so far (listed by narration); after step 10, as found. The way back is B0, on the run coordinator's word.

## What it decides
- Item 2: whether a 7-day window on a book of real volume reads `complete` in one request with the count equal to a separate raw count, and what it costs. Item 3: how often the post read-back refuses on this book across five batches (n = 5; a rate from five is only an indication).
Not covered: a different release or licence tier; a book of a different shape; batches larger than three.
End state: BRIDGE CORPUS DENSE as found; one new backup B0 on the lab PC; Tally on 9001 with no company loaded.

Evidence branch: `lab/1029-ab-answers`. Report after step 4 and at the end, without waiting in between, or at a STOP.
