# Results: #690 L0b Part A, ten ledgers created in one import request, 29 Oct 2026 (UTC). **The one request of ten new party ledgers counted `CREATED 10` and nothing else; the master mark stepped by exactly 10 (208 to 218) and the ten new ledgers took ALTERIDs 209 to 218 in request order (`L0b Batch 01` 209 ... `L0b Batch 10` 218). Every ledger and group present before kept its GUID and ALTERID. Tally answered in 0.477 s. BK0 restored; the book reads back byte-identical to step A0. Part B was not run (no build with a batch limit of ten exists).**

| question (the sheet's table) | answer | from |
|---|---|---|
| Does one request of ten new ledgers count `CREATED 10` and nothing else? | **yes**: CREATED 10; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS 0; LASTMID 0, LASTVCHID 0; no LINEERROR | A1 |
| Does `ALTMSTID` step by exactly ten (M1 = M0 + 10)? | **yes**: M0 208, M1 218 | A0, A2 |
| Do the ten new ledgers take M0+1 to M0+10, in request order? | **yes**: 01 209, 02 210, 03 211, 04 212, 05 213, 06 214, 07 215, 08 216, 09 217, 10 218 (GUID suffixes ...cf to ...d8, one apart, in the same order) | A2 |
| Is every ledger and group of A0 unchanged? | **yes**: the 3 ledgers and 28 groups of A0 keep their GUIDs and ALTERIDs; no group was added | A0, A2 |
| How long did Tally take to answer the request, against L0's three? | **0.477 s** for ten (curl `time_total`; sent 18:32:16.377Z, answered 18:32:16.903Z); L0's request A of three took 0.817 s and request B 0.309 s, so on this book the time does not grow with the count at this size | A1 |

By the sheet: "The batch limit rises to ten only if A1 shows `CREATED 10` and A2 a step of ten with the span M0+1 to M0+10": both hold, on one sitting.

## Steps and checks
- **Sheet:** `lab/690-batch-ledger-create-l0b` at 754d7c5ebcafde67a8213efff16e0bca622cd42a; `SHA256.txt` 5 of 5 equal before the run, and each request file's sha256 checked against it again before its send.
- **Step 0** (no company loaded): one tally.exe owns 9001; `/status` 200.
- **BK0** (before A0): Tally's Backup with the company only selected, to a new empty folder: `TDBK1800_100092.001`, 23,613 B, sha256 78ad3f0f6edd67cfd844c2ec2744d212d06a1e31044fc5389cc8979ae7227af9 (`evidence/bk0.txt`).
- **The company:** BRIDGE LEDGERBOX CI LAB (100092), GUID 5848d0ca-ed85-4c97-ab20-e045893a9803, loaded alone by its name filter; licensed TallyPrime 7.2 Silver (title band); Current Period 1-Apr-25 to 31-Mar-26; Tally's date Tuesday 1-Apr-2025; no vouchers.
- **Requests:** each sent once, `Content-Type: text/xml; charset=utf-16`, the answer written with `--output`, and parsed with an XML parser (`evidence/log.txt` has each request's times and size and each answer's size and sha256; `evidence/<seq>-<label>-<name>.json` the parsed rows).
- **A0:** marks 1 company row, M0 208; ledgers 3 (Cash 32, Profit & Loss A/c 31, LBox Suspense 208); groups 28. No ledger named `L0b Batch` under any case or spacing.
- **A1:** the Gateway with no dialog before the send (checked on screen; Tally's side panel "Try Now" is not a dialog).
- **A2:** marks M1 218; ledgers 13; groups 28 (`evidence/a0-a2.txt`).
- **A3:** nothing sent; the answers and the log are in `answers/` and `evidence/`.
- **A4:** THE COMPANY shut from the Company menu; Tally's Restore (Data, Backup & Restore, Restore) from BK0's folder, which listed the company with the backup's time 30-Oct-26 0:00 am local (18:30 UTC); the company selected only; End of List; "Accept ?" Yes and "Company data folder (100092) already exists ... Replace" by mouse; no other prompt. Then reloaded alone by its name filter.
- **A5:** marks 208, byte-identical to A0's (sha256 83dda085...); ledgers byte-identical to A0's (sha256 a27a54e8..., 3 ledgers, the same GUIDs and ALTERIDs; no `L0b Batch` ledger) (`evidence/a0-a5.txt`). PASS.
- **C10:** the company shut from the Company menu; Select Company with none loaded; one tally.exe owns 9001; `/status` 200; no request after the shut.

## Deviations
1. **A4 by the lab**, not the owner: the restore was made by the lab on the owner's standing word for synthetic books.
2. **The Gateway check between reads:** the Gateway was seen with no dialog after each load (before A0 and A5) and before A1; the reads of A0, of A2 and of A5 were each sent as a run with no screen look between them. Every read answered 200 with bytes equal to its length, and A2 followed A1 with no screen action between.
3. **No window captures saved to file** for this row; the screen checks were made by eye on the screen tool.
4. **Aliases** at A0 were not read separately: the sheet's ledger catalogue returned three ledgers, none an `L0b Batch` name.

**Confidence:** one sitting, one synthetic book, TallyPrime 7.2 Silver: PARTIAL until repeated.

**Who acted.** The lab, on the owner's standing word for synthetic books.
