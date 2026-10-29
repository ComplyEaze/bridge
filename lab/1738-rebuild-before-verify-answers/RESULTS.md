# Results: #1738, the hand-imported batch never verified, then rebuilt and posted, 29 Oct 2026, master 218eaba1, BRIDGE TRANSFER LAB (synthetic). **Without a `verify_import`, the rebuild X2 is admitted. Its `post_import` is then refused `import_preexisting_identity` before any approval box: the post's own book read finds both rows already in the book. Nothing was sent; the book held two Contras, not four.** The unsent X2 afterwards verifies as `verification_incomplete` (`not_attributable` 2), not as posted. The book was restored from S0

The sheet is `lab/1738-rebuild-before-verify` at c96d5853e977203a6a9e1811930cb3d5309a32d1. `SHA256.txt` matched 3 of 3.

**Who acted.** The lab did both writes (the ledger and the hand import), the S0 backup and the restore, on the owner's standing word of 27 Oct. Posting was on for launch 2 only, on the owner's word of 28 Oct.

## Build and setup
- **Build:**
  - master 218eaba1b5df10dfc8953f5ae3785e4a608f041e (Bridge CI run 40818364258);
  - the whole bundle: `bridge_mcp.exe` 41b86dac…, `pdfium.dll` 79d4676b….

  Both hashes were equal at every check.
- **Launches.** Two, sharing one new data folder: launch 1 posting off, launch 2 posting on. The launch file went back to launch 1's hash after.
- **Company:** BRIDGE TRANSFER LAB, 100051, d6e4c24f-8c7a-4797-93ec-e44bdff60bc9, loaded alone.
- **S0:** `TDBK1800_100051.001`, 28,318 B, 12:34 IST, sha256 eafcfc45f5aa3526ee304a369942a0c8a8d7779ace7705ff669b2db829a4d904.
- **`verify_import` before step 7:** none, in either launch, on any batch.

## Steps
| step | read | marks (M, V) |
|---|---|---|
| 0 | the four ledgers; Contra `auto_retain`; August empty; 32 tools | 210, 7; after S0 210, 7 |
| 0b | `TLab Suspense` under Suspense A/c, no opening balance | 211, 7 |
| 1 | p1: E1 and E2. x1: batch `bridge-6de4b7ce-1225-460e-804f-47018f689ead`, 2 Contras; file sha256 ce84f5dc… | |
| 2 | hand import: "Import completed successfully", no counters; August: exactly two Contras (no. 6 on 3 Aug, no. 7 on 4 Aug) | before 211, 7; after 211, 9 |
| 3 | `local_data_report`: journal `batches` 1, **`sent_or_found_posted` 0**, `no_dispatch_never_verified` 1, no transfers section. Journal file: one line, X1 **`built`**, no verification or dispatch line | |
| 4 | August: the same two; 32 tools, no `post_import` | |
| 5 | 34 tools (`post_import`, `acknowledge_post_review`) | 211, 9 |
| 6 | p3: the same ids. **x2 ADMITTED:** batch `bridge-263d1b59-eea9-4214-b132-62b4c3453bf8`, file sha256 413736c9…; `next_step` says call `post_import`; its first warning says not to also import the file; nothing names the vouchers already in the book | |
| 7 | **`post_import` X2 refused `import_preexisting_identity` in 2.76 s, before any box.** `preexisting_txn_ids` E1 and E2; `attempt_recorded` false; `dispatch_response` null. No box on the screen and no dialog child. No second call | V3 9, V4 9 |
| 7 reads | August: **two** Contras, the same (not four). `egress_log`: the post's trail is 24 reads, no import. Journal: X2 `built`, plus one X2 `verification_incomplete` line stamped inside the post (its own pre-box read) | |
| 7 extra | `verify_import` X2: **`verification_incomplete`**, `not_attributable` 2, `posted_verified` 0, delta 0. `verify_import` X1: `posted_verified` 2 of 2 (`narration_tag`). `local_data_report`: `sent_or_found_posted` 1, `no_dispatch_never_verified` 1 | |
| 8 | restore from S0 (Accept Yes, Replace for 100051): the four ledgers, **`TLab Suspense` absent**; August none; Contra as step 0 | 210, 7 = after S0 |
| C10 | no company loaded; one tally.exe on 9001; `/status` 200 | |

## The finding
1. **The rebuild is admitted when the hand import was never verified.**
   - Here (no `verify_import`), the journal held X1 only as `built`, and X2 was built with a file.
   - On 29 Oct (`lab/1738-hand-import-rebuild-answers`), after a `verify_import` that recorded X1 `posted_verified`, the same build was refused `import_txn_already_posted`.
   - So the build's refusal depends on a prior verification, as the code says.
2. **The post checks the book itself.** X2's `post_import` read the book and refused `import_preexisting_identity` before any approval box. It named both rows, sent nothing, and recorded no attempt. **On this build the approval box cannot be reached for a post after a file import, so its text was not read.**
3. **No duplicate was sent.** The book held two Contras, the voucher mark did not move, and `egress_log` holds no import.
4. **The unsent X2 is not recorded as posted.** `verify_import` X2 reads `verification_incomplete`, with both rows `not_attributable`. X1 (the batch whose file was imported) reads `posted_verified`.
5. **X2's build answer tells the person to post** and warns against importing its file after a post. It does not mention the two vouchers already in the book. The post then refuses them.

**PASS (the sheet ran):**
- every step done; every call answered and kept;
- nothing sent by the tool; no `verify_import` before step 7;
- the hand import made exactly two vouchers (by the reads);
- the pre-box refusal was kept whole;
- the restore returned the marks, an empty August and the four ledgers.

**Not measured:**
- the approval box's text (not reachable), and a Yes on it;
- a partial or edited hand import; a rejected send;
- `transfer_double_booked`; statement B and the pairing;
- more than two rows; TallyPrime 7.1; Gold;
- Tally's import counters (the result box shows none).

**Confidence:** one sitting, PARTIAL until repeated.

## Files
- `answers/`: every call, whole.
- `client-log.txt`.
- `evidence/`:
  - X1's and X2's files;
  - the journal file at step 3 and at the end;
  - the step 0 and C10 checks;
  - the schemas;
  - the post's start and end stamps;
  - the step 8 folder listings.
- `restore-plan.txt`.
- `scripts/q.py` and `scripts/step.py`.
- `substitutions.txt`, `notes.txt` (UTC), and `SCREENSHOTS.txt` (the captures, kept on the lab PC).
