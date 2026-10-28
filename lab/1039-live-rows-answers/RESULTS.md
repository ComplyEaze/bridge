# Results: #1039 G1 row A, 28 Oct 2026. An edit of a posted voucher's narration on Tally's screen kept its MasterID and GUID; its AlterID and the voucher mark each moved by one; verify_import then read the batch as bound and the voucher as posted_divergent with diffs exactly ["narration"]

The sheet is `lab/1039-live-rows` at 85cb14b74bfa10c96bf885a1febc6addc03762e0 (`SHA256.txt` 3 of 3 equal). The company is BRIDGE CONCUR LAB, new and synthetic: number 100091, GUID 88239cf8-ef69-4cea-8dff-4d1fd91453ee, books from 1 April 2025. TallyPrime 7.2, Silver, Licensed.

The build is the 0.6.1 cut (the README's first choice): master 67f647e1a292e8d7815053b5c08289cba6402795, run 40456677783, artifact 12712717489. Its `bridge_mcp.exe` sha256 6f520338b7f9a25180977c40a5f1d9b03dfa5774df5b5d9a6e65b9c2a050728d was the same at the first call and at the end. One stdio launch served the row (bridge-tally 0.6.0), with posting ON. The sheet's OWNER steps were done by the lab on the owner's word of 28 Oct: the company and masters, the seed, the approval window, the screen edit and the posting setting.

## Row A
| step | what | M | V |
|---|---|---|---|
| A1 | `list_companies`: exactly BRIDGE CONCUR LAB; marks from `ledger_masters` (reused false) | 210 | 1 (the seed) |
| A2 | one Receipt G1-A posted and verified (below) | 210 | 2 |
| A3 | narration altered on Tally's screen to "G1-A edited", saved with no prompt | 210 | 3 |

**A2.**
- **Build:** `build_import_xml` built batch `bridge-d3409de1-30ef-4352-935e-2b75139fce60`, one voucher: Dr BRIDGE Lab Bank 1.00, Cr BRIDGE Conc Party 1 1.00, 10 Apr 2025.
- **First `post_import`:** it returned `pending` after 40.2 s, with `retry_after_s` 15 and nothing sent. The approval window had opened behind Tally (a child of the row's server, the CUT exe). It was read whole and answered Yes by mouse; the person's mouse click is not measured.
- **Second `post_import` (same batch):**
  - `posted_verified`, resent false, created 1, `last_vch_id` 2;
  - `target_voucher_step`: before 1, after 2, step 1, `matches_created` true;
  - `post_span_binding` bound.
- **`verify_import`:** `posted_verified`, bound.

| read | voucher | MasterID | AlterID | GUID | narration |
|---|---|---|---|---|---|
| after A2 (`vouchers` 10 Apr 2025) | Receipt No. 2 | 2 | 2 | 88239cf8-ef69-4cea-8dff-4d1fd91453ee-00000002 | G1-A |
| after A3 | Receipt No. 2 | 2 | **3** | 88239cf8-ef69-4cea-8dff-4d1fd91453ee-00000002 | G1-A edited |

**A4, `verify_import` of the same batch:**
- `verification_status` `verification_incomplete`, error `import_reconciliation_required`;
- `post_span_binding.state` `bound`;
- the voucher reads **`posted_divergent` with `diffs` exactly ["narration"]**, MasterID 2 and GUID unchanged. This is the sheet's first branch.
- The answer gives no `next_step`. Its lead says to check the voucher in Tally, and that the saved batch will not be sent again.

## The three deltas and the GUID relation
- **MasterID:** unchanged by the edit (2).
- **GUID:** unchanged by the edit. It equals the company GUID followed by the MasterID as 8 hex digits (2 -> `-00000002`).
- **AlterID:** 2 -> 3 (+1).
- **Voucher mark:** V1 2 -> V2 3 (+1, the prediction). The master mark was 210 throughout.

**PASS (row A).** Both tables are recorded whole, the three deltas and the GUID relation are stated, and no MasterID or GUID changed.

## Rows B and C
Not run: the edition is Silver (not multi-user) and there is no second client.

## The book and what follows
- **BK1:** taken before the first keying: `TDBK1800_100091.001`, 24,166 B, sha256 80c3515b1080a63c07676219c3443db37efbac1769af2347e00e234ad3cccd45. It stays on the lab PC.
- **After row A:** the book holds the seed and G1-A as edited.
- **Deferred:** the AFTER restore of BK1 and C10 wait, as the order's row note says, until row 21 (`lab/1039-refusal-row`) and the #1039 dialog row have used this company.
- **Current date:** Tally's current date was changed to 10-Apr-2025 to reach the voucher in Day Book. It is set back to 1-Apr-2025 before the close.

## Files
- `answers/`: every tool answer whole (000 to 015).
- `client-log.txt`.
- `notes.txt` (UTC, one line per step).
- `SCREENSHOTS.txt`: the captures and BK1 by sha256; they stay on the lab PC.
