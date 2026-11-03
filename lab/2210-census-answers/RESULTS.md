# Results: #2210 census row, `verify_import` with the census spanned and sent, 3 Nov 2026

**Every EXPECT holds, as the sheet was corrected after the run.** With the book at V 200, every window read in steps 2 to 4 sent one census request spanned `(0, M]`, M being the voucher mark that call read. That covers each build's read, each post's box read and read-back, and both `verify_import` reads. The batch's census answer for 2 April holds its 50 Journals, the payment's for 4 April holds its one, and no 1 April set-up Journal appears. Both posts and both reviews are `posted_verified`. Each `verify_import` sent exactly the first row's order with one census pair added before the first V pair.

One correction to the sheet. As written, it expected a census on every read in a post. After Yes, each post sends its **fixed absence pair**: one whole-window import-verification request sent twice, before Import Data, with no census. Read during the row, this looked like a miss, and I stopped after step 2. The code at 8520f071 sends that pair without ever planning a census (`agent_import_post.rs:947-957`, `runtime.rs:3713-3727`), so the sheet's EXPECT was corrected after the run to leave it out. The pair is recorded below under that name. [verified from the relay; the code reading is the reviewer's, not this row's]

Sheet: `lab/2210-census-sheet` at 1a381f08. Every kit file is equal to its `SHA256.txt`. Build: master 8520f071, run 41537873751, artifact 13185636227. `bridge_mcp.exe` sha256 is bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f, the same before and after (`evidence/exe-before.txt`, `evidence/exe-after.txt`). Env as the sheet's (`run/launch-env.json`), with a new data folder. One `bridge_mcp` process for the row, pid 16444, behind the sheet's relay on 9002 in `pass` mode. The relay recorded 486 requests over two connections, opened twice and closed twice, with no `relay_error` or `tally_closed` line. [verified]

## Book
BRIDGE REVIEW LAB, GUID 63292d7c-d541-42e5-aa9d-2ab5f8b993ff, synthetic, as the first row left it. TallyPrime 7.2 Silver, licensed, loaded alone. [verified]

## Steps
| step | call | answer | seconds |
|---|---|---|---:|
| 0 | MARKS (curl, 9001) | V 52, M 207 | |
| 0 | `setup-vouchers.utf16le.xml` (curl, 9001), sent once | `CREATED 148`, `ALTERED 0`, `ERRORS 0`, `EXCEPTIONS 0`, `LASTVCHID 200` | 5.09 |
| 0 | MARKS | V 200, M 207 | |
| 1 | `list_companies`; `tally_status` | REVIEW LAB only, `verified_tuple`; answered | 0.15; 0.12 |
| 2 | `build_import_xml`, 50 Journals | batch `bridge-b76eddee-4622-47a2-9e69-2370076d29e5` | 5.08 |
| 2 | `post_import`, Yes by mouse | `CREATED 50`, errors 0, `posted_verified` 50, `last_vch_id` 250; `batch_step` `matched`, 200 to 250, step 50 | 27.75 |
| 3 | `build_import_xml`, one Payment | batch `bridge-1da90721-b47f-4ce6-a110-981261ef2fbc` | 6.25 |
| 3 | `post_import`, Yes by mouse | `CREATED 1`, `posted_verified` 1, `last_vch_id` 251 | 23.36 |
| 4 | `verify_import`, batch | `posted_verified` 50, every other count 0 | 4.04 |
| 4 | `verify_import`, payment | `posted_verified` 1, every other count 0 | 3.92 |
| 5 | MARKS (curl, 9001) | V 251, M 207 | |

Each box was read before it was answered (`box/batch-box.txt`, `box/pay-box.txt`). They showed 50 Journals on 2-Apr-2026 with debit and credit of 1,275 each, which is the file's sum, and one Payment on 4-Apr-2026 of 1.00. Each was clicked Yes about 9 s after it was seen. [verified]

## The census, read by read
Each line comes from parsing the decoded request and its answer (`evidence/check.txt`, `check.py`). The census rows are counted as VOUCHER elements carrying a DATE. The one dateless VOUCHER element in each answer is the header's counter.

| call | read | census `$AlterID` bounds | census date | census rows | V rows |
|---|---|---|---|---:|---:|
| batch build | window | `> 0 AND <= 200` | 2 Apr | 0 | 0 |
| batch post | box read | `> 0 AND <= 200` | 2 Apr | 0 | 0 |
| batch post | read-back | `> 0 AND <= 250` | 2 Apr | 50 | 50 |
| payment build | window | `> 0 AND <= 250` | 4 Apr | 0 | 0 |
| payment post | box read | `> 0 AND <= 250` | 4 Apr | 0 | 0 |
| payment post | read-back | `> 0 AND <= 251` | 4 Apr | 1 | 1 |
| verify batch | window | `> 0 AND <= 251` | 2 Apr | 50 | 50 |
| verify payment | window | `> 0 AND <= 251` | 4 Apr | 1 | 1 |

- One census span per read, never two. Each census request was sent as a pair with equal answers. [verified]
- In each read, M is the mark that the call's High Water pair answered just before (200, 250 or 251). [verified]
- No census or V answer holds a 1 April voucher. [verified]
- Sizes: a census of 50 rows is 124,242 B, a census of one is 5,458 B, and an empty one is 3,038 B. The batch's V answer is 248,988 B and the payment's is 7,960 B. [verified]

**The fixed absence pair.** After each Yes and before Import Data, each post sent the import-verification request for its window, with no census, at an unchanged mark:
- the batch at mark 200: conn 1, requests 149, 151, 155 and 157;
- the payment at mark 250: conn 2, requests 167, 169, 173 and 175.

Each answer is the empty 3,038 B answer before the post, and every answer within each set is equal. [verified]

## Request order per `verify_import`
Codes: S status, E extent, H marks, C census, V import verification.
- First row, as the sheet gives it: `SEESESEHSHSEEVSVSEEVSVSEEHSHSE`.
- This row, payment: `SEESESEHSHSEECSCSEEVSVSEEVSVSEEHSHSE`. This is the first row's order with `CSCSEE` added before the first V pair, as the sheet expected.
- This row, batch: the same, plus a trailing `SE`. The checker counts requests up to one second after each call's end, and the payment's verify began within it. [verified]

## Repeats
- Every request sent twice in a row has equal answers: 0 pairs differ. [verified]
- **NOTE:** across calls, the batch's V request (`9e067c388a65`) answered 90ab784e… in the post's read-back at mark 250 and a8a8403d… in `verify_import` at mark 251. The two differ only in four counters in the answer's header: `LEDGER` 594 to 597, `CURRENCY` 198 to 199, `VOUCHERNUMBERSERIES` and `VOUCHER` 198 to 199. The rows are equal. Row 2162-S1 saw the same counters move with what Tally's session has loaded. [verified]

## Import verification
By parsing VOUCHER elements, the batch's import verification holds 50 rows, all on 2 April, and the payment's holds 1, on 4 April. Their narrations are the files' own (`Census row 1` to `Census row 50`, `Census row payment`), each followed by the `[BRIDGE:…]` marker Bridge adds when it builds. The same text is in `copied/imports/`. [verified]

## Kept
- Tool answers (`run/answers/`), with `company_number` redacted.
- `out/relay.jsonl` and every body in `out/bodies/`.
- The set-up answer, `reads/setup-vouchers.xml`. The set-up request is the sheet's file as it is, sha256 4969041d…, and is not copied again.
- Every MARKS read and request.
- The journal and both import files from the data folder, copied untouched and then masked as `substitutions.txt` says. Nothing else came from the data folder.
- The company GUID, above.

The journal holds no machine path; its only address is the relay's `http://127.0.0.1:9002`. The box captures stay on the lab PC, listed by hash in `kept-local.txt`.

## Not measured
Four other `bridge_mcp` processes ran under the desktop client as its connectors throughout, as in earlier rows. They took no call in this row. Whether they sent anything to Tally on 9001 was not measured. [unverified]

## The book now
BRIDGE REVIEW LAB: V 251, M 207, holding the 148 set-up Journals from this row, the batch and the payment. It is not restored. Loaded, alone, one tally.exe on 9001.

**Confidence:** one book, one sitting, one build, 7.2: PARTIAL.
