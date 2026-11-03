# Results: row 852-C1, a 50-voucher bank batch posted twice behind a capture relay, on a book of about 30,000 vouchers, 3 Nov 2026

**Not reproduced at this scale.** Both posts created 50 vouchers and Bridge kept Tally's reply. Each import exchange took about 6.6 s, well under Bridge's 20 s deadline, and each whole `post_import` call about 49.5 s, under the host's 60 s. Both were `posted_verified` 50 of 50, in the call and again by `verify_import`. No import read over 20 s and no call over 60 s. The relay logged no `reply_lost`, `tally_closed` or `relay_error`, and `bridge_left_s` was null on every request. By the sheet's Decides table this is its first row: #852 stays open with these times. [verified]

Sheet: `lab/852-capture-sheet` at 2c32c3e2, folder `lab/852-capture/`, all 8 files equal to `SHA256.txt`. `check-relay.py relay.py` passed all 20 checks on this PC (`evidence/check-relay.out`). [verified]

Build: CI run 41597505434, a push of master at 838242cb (success), artifact `unsigned-bundle-smoke-windows-latest`, id 13213581672. The zip's sha256 is a865654b1fc3c2b5a9dcd5bc4f8c555605fbcb180f1b8f7d66a196fea91c01e4, equal to the sheet and the API digest. `bridge_mcp.exe` sha256 is eb2c5b088b1b9662f9a5e26832dbf9535133e4a677fb755859ec317f9465408e, equal to the bundle's smoke file, before and after. `pdfium.dll` is equal to the smoke file too. The exe ran over stdio from the unpacked bundle; no installer was run. [verified]

Env, as the sheet: `BRIDGE_AGENT_ENABLE_WRITES=true`, `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, one new data folder for the whole row, `BRIDGE_AGENT_ENABLE_BATCH_POST` absent, redaction unset (`runA/launch-env.json`). Port 9001 for step 3 only (`run3/`), 9002 behind the relay for steps 4 to 8 (`runA/`, one process for both batches). **The client** is the lab's stdio client: it sends no cancel and has no call limit (it waits with no timeout). Four `bridge_mcp` processes ran idle under the desktop client, as in earlier rows. They took no call. Whether they sent anything to Tally was not measured. [verified; the processes unverified]

## Book
BRIDGE LARGE LAB, GUID 2ba6df28-d1e3-4e81-b6d2-eb50f0a693ab, TallyPrime 7.2 Silver, licensed, loaded alone. It opened with no prompt. Tally's current date read Wednesday, 31-Mar-2027; the PC's date was 3-Nov-2026 (14:34 IST). [verified]

| step | what | result |
|---|---|---|
| 0 | marks raw on 9001; `GET /status` | one COMPANY row, this GUID, **V 29,930**, M 210; status 200 |
| 1 | BK, Tally's Backup screen, to a new empty local folder | `TDBK1800_<company number>.001`, 9,047,800 B, sha256 21fa4777b948be272c6738f53a0c793a6f068cca865c062e8c61c18fd5e5d4bb, 14:35 local; kept on the lab PC |
| 2 | `seed-bank-ledger.xml` to 9001, `charset=utf-8` (from its own declaration) | CREATED 1, every other counter 0; marks V 29,930, M 211 |
| 3 | `masters` `kind=voucher_types` on 9001 | Payment, Receipt and Contra each `series_numbering.method` automatic, `sub_method` auto_retain: they stay, nothing altered in Tally; marks 29,930 / 211 |
| 4 | relay `a`; `list_companies` on 9002 | this GUID, `verified_tuple` |
| 5 | `build_import_xml` with `c50a.json` | batch `bridge-ae5034d3-c28d-427a-9251-c9b6718d0dac`, `voucher_count` 50, 18.06 s, no refusal |
| 6–7 | `post_import`; `verify_import`; `egress_log`; marks | below; marks **V 29,980** (+50), M 211 |
| 8 | relay `b`; `list_companies`; build `c50b.json`; post; verify; egress; marks | batch `bridge-08d8a991-7c7f-4dc6-82b6-cca1867729f5`, 50, 20.82 s; marks **V 30,030** (+50), M 211 |
| 9 | marks; Shut Company (Alt+K, Shut, Enter) | V 30,030; shut. The 100 vouchers and the ledger stay; BK is the way back |

No one keyed in Tally from step 5 to step 8. Each approval was answered Yes by mouse from the box's own button, about 1.4 s after the box showed. A helper first read the box's text and wrote Yes only because it named this company, GUID, batch and 50 vouchers (`boxcheck.py`, `box/`). [verified]

## The two posts
| | a | b |
|---|---|---|
| requests in the call before the import: count, sent bytes, answered bytes, summed `answer_s` | 144; 188,108 B; 29,199,413 B; 12.281 s | 144; 188,108 B; 29,200,521 B; 13.081 s |
| the import line: `sent`, `answered`, `first_byte_s`, `answer_s`, `bridge_left_s` | 66,616 B; 548 B; **6.593 s**; **6.593 s**; null | 66,616 B; 548 B; **6.547 s**; **6.547 s**; null |
| Tally's counters, from `import-1.answer` | CREATED 50, ALTERED 0, DELETED 0, ERRORS 0, EXCEPTIONS 0, LASTVCHID 29980, no LINEERROR | CREATED 50, the same, LASTVCHID 30030 |
| any `reply_lost`, `tally_closed`, `relay_error` | none (events: listening, open, sent) | none |
| requests in the call after the import: count, sent bytes, answered bytes, summed `answer_s` | 73; 96,756 B; 13,039,675 B; 6.139 s | 73; 96,756 B; 13,040,595 B; 6.264 s |
| the call: start, end (UTC), seconds; box seen; Yes clicked | 09:08:10.502, 09:08:59.867, **49.365 s**; 09:08:27.357; 09:08:28.812 | 09:10:45.785, 09:11:35.476, **49.691 s**; 09:11:05.340; 09:11:06.588 |
| the import sent (relay `sent` event, UTC) | 09:08:38.948 | 09:11:14.211 |
| result | `dispatch.state` `posted_verified`, `counters.created` 50, `response_state` `response_clean`, `resent` false; no `error` | the same |
| `egress_log`'s `response_prepared` line for the post | present, `rows_prepared` 50 | present, `rows_prepared` 50 |
| `verify_import` | `posted_verified` 50, every other count 0; 16.06 s | `posted_verified` 50, every other count 0; 15.94 s |
| Tally's screen before and after | the Gateway, no dialog, before and after (after: Tally unfocused, bands grey, as the click left the focus on the box's owner) | the same |

Every figure above is read from `relay/{a,b}/relay.jsonl`, the import files, the tool answers and the client's log, by `report.py` (`evidence/report.txt`). The call window is the client's send time plus its round trip. [verified]

**Where the 49.5 s goes (a):** about 17 s of reads before the box showed (09:08:10.5 to 09:08:27.4). About 10 s from Yes to the import being sent (09:08:28.8 to 09:08:38.9), the post's own re-reads. 6.6 s for the import. About 14 s of read-back after it (09:08:45.5 to 09:08:59.9). The reads before the import summed 12.3 s of Tally time over 29.2 MB. b is the same shape. [verified for the stamps; the split is read from them]

## Decides
Both `created` 50, the reply kept, `posted_verified` 50 of 50, each import under 20 s, each call under 60 s: **not reproduced at this scale; #852 stays open with these times.** The import exchange used about a third of Bridge's 20 s deadline. Each call left about 10 s of the host's 60 s, and about 17 s of each call was the approval wait. [verified]

## Limits, as the sheet's
Silver, one user, four ledgers. The failing book was a client's Gold book whose reads were about 100 requests and 15.9 MB. Here each call sent 217 requests (144 before the import, 73 after) and read about 42 MB. Two posts only. The relay reads each answer whole before forwarding it, so large reads reach Bridge a little late.

## Kept
- `relay.jsonl` of both folders, both `import-1.request` (66,733 B with headers) and `import-1.answer`.
- Every tool answer whole (`company_number` redacted, in the JSON and in its text).
- The client logs and call times, the marks reads, the seed answer.
- The box logs, the relay check, the report.

On the lab PC only, listed in `kept-local.txt`: the backup, the screens and the box captures.

## The book now
BRIDGE LARGE LAB at V 30,030, M 211, holding batch a and b (100 vouchers, 11 to 30 Oct 2026) and `Lab Vol Bank`. Not restored, as the sheet says. Shut, no company loaded, one tally.exe on 9001.

**Confidence:** one book, two posts, one sitting, 7.2 Silver: PARTIAL.
