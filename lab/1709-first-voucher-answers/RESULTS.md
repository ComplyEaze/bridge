# Results: #1709 first-voucher captures (GS2-L3) on #1867 head 796e94f1 and TallyPrime 7.2, 27 Oct 2026. Steps 0 to 3 PASS; steps 4 to 6 incomplete, stopped by the recorder's date fence

The sheet is `lab/gs2-l3/WINDOWS-GS2-L3.md` at 576a08afad2fcf98697c1acab235145fb9fb06db (SHA256.txt: 2 of 2 equal).

## Build, recorder and setting
- The build of pull request 1867 (head 796e94f18e5af783b291ddf3b5e60ec19132df9a, run 40167319272, artifact 12589749758): `bridge_mcp.exe` sha256 bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46 at each launch and after the last call. It reports `bridge-tally` 0.6.0.
- Over MCP stdio, posting and ledger creation off (no `BRIDGE_AGENT_ENABLE_WRITES`, no `_ENABLE_IMPORT`), host 127.0.0.1, port 9011 (the recorder), a new data folder. Two launches (`client-log.txt`): 10:49:29 to the step 3 call; 10:52:34 to 10:54:10 UTC for step 4. The first ended because the step 4 request file was malformed by the operator and the client stopped on reading it, before any call was sent (the recorder shows nothing between its lines 4 and 5). The server's error stream was kept and stayed empty.
- The recorder the recorder at e5dffc2b, files checked against its SHA256.txt (21 of 21), self-test `self-test: ok (10 forwarded, 41 refused, none of the refused reached the target)`. Its recording is `recorder/` with `manifest.tsv` (102 lines).
- TallyPrime 7.2 (Latest), Silver (the About screen; its capture stays on the lab PC).

## Steps
| step | act | result |
|---|---|---|
| 0 | no company loaded; Select Company's list read | no company named BRIDGE FIRST VOUCHER LAB: PASS |
| 1 | BRIDGE FIRST VOUCHER LAB created on Tally's screen: financial year and books from 1-Apr-25, no password, every other field as offered; the Features screen accepted unchanged | company number 100085; loaded alone; "No Vouchers Entered": PASS |
| 2 | ledgers SYN FV Rent (Indirect Expenses) and SYN FV Supplier (Sundry Creditors, bill-by-bill Yes as offered, no GSTIN) created; no voucher screen opened | PASS |
| 3 | `list_companies` (000) | exactly BRIDGE FIRST VOUCHER LAB, GUID f88ff11c-6fa1-4a73-8228-ab1c8c72ad4a, books from 20250401: PASS |
| 4 | `purchase_register` 2025-04-01 to 2025-04-30 (001) | `isError` true, `agent_runtime_read_failed` (`http_status_failure`, endpoint the recorder): the recorder refused one request (below); the row stopped here |
| 5, 6 | the second call; the reload and the third call | not run |

## Finding: the product reads the day before the books begin
During step 4 the recorder forwarded 97 requests (manifest lines 5 to 101), among them the voucher window for 20250401 to 20250430 (line 50) and two company-marks reads `Bridge Agent Company High Water` (lines 56 and 58). Line 102 is a `Bridge Agent Vouchers` request (`0102-request.bin`, 1,756 bytes, sha256 3428714bedea071bfc3b6e42bf93a1b1675c3a1dbc4c8d43c75bc87f6b30b502) naming SVFROMDATE 20250331 and SVTODATE 20250501: the read of the day either side of an empty window. The recorder refused it, `date_out_of_range` with a 403, because its allowed range starts at 20250401 (the recorder script, FIRST_DATE). This company's books begin on 1 April 2025, so the product asked for 31 March 2025, a day before the books. The tool's answer was then the failure above.

The recorder was not changed for this run. A recorder with a wider range and a rerun of steps 4 to 6 belong to a later item.

## What the recording holds for the requester
- Every company-marks answer of the run: `0056-response.bin` and `0058-response.bin` (the two `Bridge Agent Company High Water` reads of step 4).
- Every book-extent answer and the voucher-window answers of step 4 up to line 101, each request and answer byte for byte, listed in `manifest.tsv` with bytes and sha256.
- The answers are recorded, not judged.

## Files
- `answers/000-s3-list_companies.json`, `answers/001-s4-purchase_register.json`: whole, with arguments, send time and round trip; sha256 in `client-log.txt` and `SHA256.txt`.
- `recorder/`: the recorder's folder whole, with `manifest.tsv`.
- `notes.txt`: one line per step with UTC times.
- The company BRIDGE FIRST VOUCHER LAB (100085) is kept on the lab PC as it is: two ledgers, no voucher.
- Kept on the lab PC, by name, bytes and sha256: s0-select-company.jpg 36593 c441b3631ad297345aa5268f643e16533b1c9513c65cd9d9e2856b57a6bf3804; s0-list-around-F.jpg 44471 53e9ebfbc7a9601e4b4e148d078cab4079bd76b89190f9cadc46c2bae2fbf752; s1-company-before-accept.jpg 99841 ae4a85ce99a46003d9f6e0c9225a0a641f94c9aa16d4ab289bbe794b3a13a3fa; s1-features.jpg 144228 2a32c604c10e32f7ee19cdf264d62ad7968493e75005488121f168668e6d74b5; s2-ledger-rent-before-accept.jpg 118314 5b7280f75529459730794aebc3a1ce9615886a723ed55511e7ddb6f50c45606c; s2-ledger-supplier-before-accept.jpg 110916 1e1cc7dcf63ed5b9cb9de0b2db3a715c2ead9315f3dad5d83ffff8d38c542926; the About screen capture (licence details, never pushed) 152051 ef3dca14eedc2fd57f2823c830b9b7fa20bf2f1ec1dc58d1b43ae38f302fbb57.
