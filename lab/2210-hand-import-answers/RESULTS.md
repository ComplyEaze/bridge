# Results: #2210 hand-import row, `verify_import` of a batch imported from its saved file, with an unmarked copy, 3 Nov 2026

**Every EXPECT holds.** The batch's 50 Journals were built by Bridge and imported from the saved file without Bridge. One unmarked copy of H-005 was entered after them. `verify_import` then read `posted_verified` 50, no `dispatch` key, and one `accounting_fingerprint` duplicate with two voucher ids, so `verification_status` is `verification_incomplete`. Its census was sent and spanned `(0, 302]` on 6 April. The census and the import verification each hold 51 rows: 50 tagged with the batch's `[BRIDGE:…]` tag, and `Hand row 5` with none. The request order is exactly the sheet's 30 exchanges.

One NOTE first: the saved file's first send drew `Unknown Request`, because I sent it with the wrong content type. It created nothing, and the file was then sent once more with its own content type, as agreed.

Sheet: `lab/2210-hand-import-sheet` at 102635bf, all 5 files equal to `SHA256.txt`. Build: master 8520f071, as the census row, `bridge_mcp.exe` sha256 bb8454bc6b52fe800a61ba7faf553427ebae1948a25771a787ed0dad77ba195f, the same before and after. Env: `BRIDGE_AGENT_ENABLE_IMPORT` true, `BRIDGE_TERMS_ACCEPTED` true, host 127.0.0.1, port 9002, a new data folder; `BRIDGE_AGENT_ENABLE_WRITES` not set (`run/launch-env.json`). One `bridge_mcp` for the row, behind the sheet's relay (the census row's, sha256 equal) in `pass` mode. [verified]

## Book
BRIDGE REVIEW LAB, GUID 63292d7c-d541-42e5-aa9d-2ab5f8b993ff, synthetic, as the census row left it. TallyPrime 7.2 Silver, licensed, loaded alone. The Gateway's current date read 4-Apr-2026. [verified]

## Steps
| step | act | result |
|---|---|---|
| 0 | MARKS (curl, 9001) | V 251, M 207 |
| 1 | relay; `list_companies`; `tally_status` | REVIEW LAB only, `verified_tuple`; answered |
| 2 | `build_import_xml`, 50 Journals | batch `bridge-c3160d0e-9fc0-48e5-8e5c-48ba0d502f94`, `voucher_count` 50, no refusal; saved file `imports/<batch_id>.xml`, 31,003 B, sha256 02434e30e7e383cfd04edcf0969cea63db6da6283686fe858ecb89a89fc34425, equal to the answer's `sha256`. No `post_import` |
| 3 | MARKS | V 251 |
| 3 | the saved file, curl on 9001, with `charset=utf-16` | **NOTE:** `<RESPONSE>Unknown Request, cannot be processed</RESPONSE>`, 118 B, no counters |
| 3 | MARKS | V 251, M 207, the answer byte-identical to the one before: nothing was created |
| 3 | MARKS, then the same saved file once more, with `charset=utf-8` | V 251; then `CREATED` 50, `ERRORS` 0, every other counter 0, `LASTVCHID` 301 |
| 3 | MARKS | V 301 |
| 4 | `copy-voucher.utf16le.xml`, curl on 9001, `charset=utf-16` | `CREATED` 1, `ERRORS` 0, every other counter 0, `LASTVCHID` 302 |
| 4 | MARKS | V 302 |
| 5 | `verify_import` of the batch, at once | below |
| 6 | MARKS; relay stopped | V 302, M 207 |
| 7 | the journal and `imports/` copied untouched | sha256 of each copy equal to its source |

[verified]

**The NOTE on step 3.** The saved file is UTF-8 and declares it (`<?xml version="1.0" encoding="UTF-8"?>`, no byte-order mark). I sent it with the census row's curl line, whose header is `text/xml; charset=utf-16`, made for that row's UTF-16 set-up file. Tally answered `Unknown Request`. The marks after it equal those before it byte for byte, so the send created nothing. On the recorded word, the same file was sent once more, after a MARKS read of V 251, with `Content-Type: text/xml; charset=utf-8`. Tally answered in UTF-8 (`Unicode: No`). From then on the sender takes each file's content type from its own byte-order mark or declaration (`raw.py`). Both answers and the send log are kept. [verified]

## Step 5, `verify_import`
- `counts.posted_verified` 50; every other count 0.
- No `dispatch` key. `dispatch_response` is null.
- `pre_import_mark`: `company_high_water`, value 251, master 207.
- `duplicates`: one entry, `kind` `accounting_fingerprint`, `voucher_ids` `guid:…-00000100` and `guid:…-0000012e`.
- `verification_status` `verification_incomplete`.

The Markdown proof holds `**Not verified — this report does not confirm posting.**`, `- Verification status: `verification_incomplete``, and `- Duplicates in this batch: 1`. [verified]

## From the relay's bytes
- Two opens, two closes, no `relay_error` or `tally_closed`. 88 requests. [verified]
- **The build's census:** `$Date` 6 April, `$AlterID > 0 AND <= 251`, answered empty (no 6 April row then). [verified]
- **The verify's census:** one request, sent as a pair with equal answers: `$Date` 6 April, `$AlterID > 0 AND <= 302`. Its answer holds 51 rows, all on 6 April. [verified]
- **The verify's import verification:** 258,392 B, sent twice as a pair and replayed as a second pair, all four answers equal. 51 rows, all on 6 April, none on 1, 2 or 4 April. 50 narrations end in the batch's tag (`Hand row N [BRIDGE:…]`), and one, `Hand row 5`, has none. [verified]
- **Request order of the verify:** `SEESESEHSHSEECSCSEEVSVSEEVSVSE`, 30 exchanges, as expected. Its marks read V 302. [verified]
- **Repeats:** every request sent twice in a row within a read had equal answers. My checker lists one pair as differing: the company-extent request that ended the build (conn 1, request 58) and the one that opened the verify (conn 2, request 2). These are different calls, 1 min 51 s apart, with both imports between them, so their answers differ, as the book did. [verified]
- **NOTE, REMOTEID.** Tally kept none of the saved file's REMOTEIDs. All 51 rows carry a `REMOTEID` equal to their own `GUID`; the saved file's REMOTEIDs survive only inside the narration tags. Earlier rows saw the same: Tally discards seed REMOTEIDs on vouchers. [verified]

## Kept
- Tool answers, with `company_number` redacted.
- `out/relay.jsonl` and `out/bodies/`.
- Every curl request and answer, both import answers included, and every MARKS.
- The journal and the whole `imports/` folder (saved file, baseline, JSON and Markdown proofs).
- The company GUID, above.

Narrations are the files' own. The journal holds no machine path; its only address is the relay's `http://127.0.0.1:9002`. No screens were taken in this row.

## The book now
BRIDGE REVIEW LAB at V 302, M 207, holding the 50 Journals and the copy on 6 April beside the census row's vouchers. Not restored, as the sheet says. Loaded alone; the relay and Bridge are stopped.

**Confidence:** one book, one sitting, one build, 7.2: PARTIAL.
