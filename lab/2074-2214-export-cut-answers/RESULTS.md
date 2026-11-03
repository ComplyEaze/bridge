# Results: 2074-2214 export and cut, the X-only cut by tool alone, 3 Nov 2026

**A cut was seen live three times, and each drain ended in a whole standard read.** In X1, X2 and X3 the mailing read was cut at Bridge's 20 s limit (`party_ledger_master_read_failed`, cause `request_deadline_exceeded`, `failed_request` `ledger_master`). None of the three is VOID. The drain record read owed at once after each cut. Standard reads were refused `tally_endpoint_draining` with `retry_after_s` 10 until Tally had worked through the front and the cut read: three refusals in X1 and X2, one in X3. The first admitted read was whole, with 174 rows equal row for row to the read after it. The voucher mark moved by exactly each front's Journals and the master mark did not move. Part W, the workbook rows, was not run: it waits for the owner. So this row reads only the X line of the Decides table, without W4.

Sheet: `lab/2074-2214-export-cut` at 7a49b5c1, with the X-only cut it records, every file equal to `SHA256.txt` at P0 and again at Close. The cut's gate was settled before the row: row N2 was PASS-present twice, so Tally serves a front first on a warmed connection. Build: master a6c64b8b, merge-queue run 41559265012 (Bridge CI, success), artifact 13196982536 (`unsigned-bundle-smoke-windows-latest`). The zip's sha256 is 9a36fe3b…5c37, equal to the API digest. `bridge_mcp.exe` sha256 0345e74beff3c44ff874fdb45d31306dfb8d7eb694ab0f8db66537ad6857b1bd, equal to the bundle's smoke file, at P0 and at Close. Since d214d6a0, where the sheet's file lines were set, `tally/connection.rs` moved: `Refuse` is now at :2468, `ContinueOnceDrained` at :2470 and `resolved_for_count` at :776. The Deadline→Refuse mapping at :212 is unchanged. The `failed_request` field comes from #2299, merged after the sheet; it is recorded here as an addition, with no EXPECT changed. [verified]

## Book
BRIDGE BATCH LAB (synthetic), GUID a30f23c3-4819-4334-96ab-ffa6fba70976, loaded alone, TallyPrime 7.2 Silver, licensed. The Gateway's current date was 21-Aug-2026 throughout and was not changed. [verified]

## Before the sitting
- **P0.** One tally.exe, owning 9001. `/status` 200. MARKS: one COMPANY row, 224 / 87. The creates were generated and equal `requests-manifest.txt` line for line, apart from Windows line ends. The first step 0's tasklist failed through my helper's quoting (`ERROR: Invalid argument/option`). Step 0 was run again whole, with the command fixed; both are in `evidence/notes.txt`. [verified]
- **S0.** BK-2074-2214 was taken by Tally's Backup screen to the new empty `bk\`: one file, 57,180 B, sha256 99b80149…, at 09:03 local. Marks after: 224 / 87. [verified]
- **B5.** No Bridge process from an earlier row. Four `bridge_mcp` processes ran throughout under the desktop client as its connectors, as in earlier rows. They took no call in this row. Whether they sent anything to Tally was not measured. [unverified]
- **Server.** The first launch used the 2214 row's client, whose exe path is fixed in its code. It started that row's build (sha256 2c41a51e…, 0.6.1) and answered `tools/list` only. The relay logged no connection, so nothing reached Tally. I stopped it, changed the client to take the exe and its hash as arguments, and started the right build with a new empty data folder (`run2/`). The wrong start is kept as `run/`. [verified]
- `list_companies`: exactly BATCH LAB, `verified_tuple`. [verified]
- **R1.** STANDARD twice, 120 s apart: `total` 11 both times, marks 224 / 87. The slower master post was held 26 ms, so the post threshold is the larger of 52 ms and 2,026 ms: **2,026 ms**. Drain record `{"state":"clear"}`. [verified]

## Parts A, M and F
| step | what | result |
|---|---|---|
| A1 | `create-000` by the 2214 runner | `CREATED` 1, marks 225 / 87, capture equal to the Gateway's |
| A2 | MAILING | `total` 12; `Lab Mail Load 000`: five entries, 1 to 5 April 2026, oldest first, `5 Lab Street, Lab Area`, PIN 560001, Karnataka |
| M1 | `create-001` to `create-160` | each `CREATED` 1, every other counter 0; marks 385 / 87 = 225 + 160; every capture equal to the Gateway's |
| M2 | STANDARD | `total` 172, 385 / 87 |
| M2 | MAILING | answered whole in three pages, `total` 172; both master posts 30,122,530 B, held 10,648 and 4,010 ms; drain record clear |
| F0 | `front-ledgers.xml` | `CREATED` 2; master mark 385 to 387 |
| F0 | `front-probe.xml`, 500 Journals | `CREATED` 500, every other counter 0, no `LINEERROR`; 16.922 s; voucher mark 87 to 587, master unmoved; capture: Gateway, no dialog |
| F1 | n50, n17 | t = 16.922 s: n50 = 1,477, n17 = 502 |
| F2 | fronts | `front-x1` 500–1976, `front-x2` 1977–3453, `front-x3` 3454–3955 (REMOTEID index ranges); no `front-w4` |

[verified]

- **NOTE, M2's count.** The sheet expects "32,005 entries in all". The read holds **32,012**. The 161 `Lab Mail Load` parties hold exactly 32,005 (160 × 200 + 5), each party oldest first. Seven of the book's own 11 ledgers hold one entry each: Lab Capital, Lab Charges, Lab Seed Income, Party A, Party B, Party C, Synthetic Bank Ledger. The master posts are byte-equal in size to row 2214's L6, so L6's 32,005 likely counted the created parties only. [verified for the count; the reading of L6 is unverified]
- **NOTE, C3.** After `front-ledgers` the MARKS read went 0.4 s before the window capture that C3 asks for after a write. The capture taken after it shows the Gateway, no dialog. [verified]

## Part X
Each go: drain record clear, MARKS, STANDARD. Then arm the relay, MAILING, copy the drain record (D1), STANDARD until admitted (D2, D3), STANDARD again (Whole), MARKS, drain record. `x.py` ran each one; `evidence/x-log.txt` holds every line.

| | X1 | X2 | X3 |
|---|---|---|---|
| front, Journals | 1,477 | 1,477 | 502 |
| MAILING: wall, refusal | 22.00 s, `party_ledger_master_read_failed` | 21.95 s, the same | 21.99 s, the same |
| its master post | `request_deadline_exceeded`, held 20,011 ms | 20,006 ms | 20,020 ms |
| `failed_request` | `ledger_master` | `ledger_master` | `ledger_master` |
| relay, in order | `front_written`, `forwarded` (front 1), `bridge_cut` | the same, front 2 | the same, front 3 |
| `front_written` to `bridge_cut` | 19.985 s | 19.954 s | 20.016 s |
| `first_byte` of the cut post before `bridge_cut` | none | none | none |
| `front_answered` against `bridge_cut` | 32.73 s after | 31.81 s after | **1.63 s before** |
| D1 drain record | `owed`, `answered` 0 | the same | the same |
| D2 refusals before admission | 3, each `tally_endpoint_draining`, `retry_after_s` 10, no post sent | 3, the same | 1, the same |
| D2 refusals' status sends | 1 each: two cut at 5 s (`request_deadline_exceeded`), then one answered after 4,841 ms | two cut at 5 s, then one answered after 4,061 ms | one answered after 1,393 ms |
| `answered` in the record | 1 after the third refusal; cleared within the admitting call | the same | 1 after the first refusal; cleared within the admitting call |
| D3: gap from D1 to the admitted call's end | 50.83 s | 50.22 s | 16.77 s |
| D3: master posts held | 68, 72 ms | 81, 64 ms | 69, 63 ms |
| Whole: `total`, rows equal | 174; 174 of 174 | 174; 174 of 174 | 174; 174 of 174 |
| MARKS after | 387 / 2064 (+1,477) | 387 / 3541 (+1,477) | 387 / 4043 (+502) |
| drain record after | clear | clear | clear |
| `front-<k>.answer` | `CREATED` 1477, every other counter 0 | `CREATED` 1477, the same | `CREATED` 502, the same |

[verified]

- **Not VOID.** In each go the mailing read was cut, not answered, and no `first_byte` came for its post before `bridge_cut`. In X3 the front ended 1.63 s before the cut, so Tally had the mailing read under way at the cut, as the sheet meant. [verified]
- **D3's admitted read** kept its master posts far under the 2,026 ms threshold. [verified]
- **Timing after the cut.** In every go, `front_answered` came before the first quick status answer: 14.3, 14.6 and 14.6 s before it. The status send that answered slowly came about 4 s after `front_answered` in X1 and X2, which is about the time M2's second mailing post took. That suggests Tally ran the cut mailing read after the front, with nobody listening. [hypothesis]
- **Order of refusals.** In X1 and X2 each of the first two refusals carried a status send cut at Bridge's 5 s status limit while Tally was still busy. The third carried one that answered, and the call after it was admitted. [verified]

## Close
1. Reads only. The server and the relay were stopped. `relay.jsonl` parsed: 12 opens, 12 closes, no `relay_error`, no `tally_closed`. Its 7 `bridge_cut` lines are the three mailing cuts and four status sends Bridge closed at 5 s. [verified]
2. The inputs, the 201 creates and the exe were re-hashed: all equal. [verified]
3. **Restore of BK-2074-2214 by C6**, on the recorded word:
   - The plan is in `evidence/restore-plan.txt`.
   - Before: the backup's hash checked, its folder holding that one file, BATCH LAB shut.
   - The Restore list showed BRIDGE BATCH LAB with its company number, 3-Nov-26 9:03 am. Selected alone and captured before accepting.
   - Prompts: "Accept ?" answered Yes. "Company data folder (<company number>) already exists…", with BATCH LAB's number, answered Replace. No other prompt came.
   - The company's data folder was listed before and after; the listings stay on the lab PC.
   - [verified]
4. **MARKS after the restore: 224 / 87, byte-identical to S0's and P0's answer** (sha256 566be20d…). `ledger_masters` on 9001 with a new server: basic `total` 11. The same read with `fields=compliance` gave 11 rows, each equal to R1's. [verified]
5. C10: BATCH LAB shut (Alt+K, Shut, Enter). One tally.exe (1200) owning 9001, none on 9002, `/status` 200. Select Company showing, no company loaded. No XML request after the shut. [verified]

## Decides
X1, X2 and X3 are not VOID. The standard read was admitted whole each time, and the marks moved as stated. **A cut has been seen live by tool.** The row's Decides line also asks for "W4 as EXPECT", and W4 was not run in this cut. So any decision on moving `Deadline` to `ContinueOnceDrained` waits for W4. This sheet moves nothing.

## Kept
- Tool answers for all three server runs, with `company_number` redacted.
- `relay/relay.jsonl`, `relay/bodies/`, and the three front answers.
- The raw reads (MARKS, front imports) and the creates' answers.
- The drain record copies, the logs and the scripts.
- On the lab PC only, listed in `kept-local.txt` with size and sha256:
  - the 30,122,530 B mailing answer body and the three M2 MAILING tool answers, each over 2,000,000 B;
  - the generated creates and fronts;
  - the window captures, process and port listings, folder listings, and the backup.

## The book now
BRIDGE BATCH LAB restored to 224 / 87 with 11 ledgers, shut. No company loaded; one tally.exe on 9001.

**Confidence:** one book, one sitting, one build, 7.2, three goes: PARTIAL.
