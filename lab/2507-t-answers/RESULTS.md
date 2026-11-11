# Results: row 2507-T, a Stock Journal transfer sent by XML, four shapes, 2026-11-11

**Outcome: every step run, T0 to T3 and the end; no stop after the first write.** The four questions have answers:
- **T0, the baseline:** created, one `in` and one `out` line as sent; Bridge's read equals Tally's screen.
- **T1, the id sent:** created. The row's GUID is Tally's own (the company GUID and the MASTERID), not the `REMOTEID` sent. Sent again (T1b), the same voucher was **altered** in place: `ALTERED 1`, the same GUID and MASTERID, a higher ALTERID, still one row.
- **T2, one list name for both sides:** **refused**, `EXCEPTIONS 1`, no `LINEERROR`, nothing created; the marks did not move. A finding, as the sheet allows.
- **T3, no outward value:** created, and **Tally did not value the transfer**: both sides read with rate and amount empty, and the screen shows both sides blank with no total value.

**Fails:** none. No Bridge call errored; every figure Bridge read equals Tally's screen; every line's direction agrees with the screen (`out` = Source (Consumption), `in` = Destination (Production)); every restore brought back step 0's marks byte for byte.

**One deviation from the sheet's EXPECTs (deviation 1):** an empty day reads `partial`, not `complete`, on this build. The run lead decided before the first write how such a day passes (below). Every empty-day check passed.

**Who acted.** All by the lab, on the owner's standing words for synthetic books (2026-10-27 and 2026-10-28): B0, each restore, and the screen views. The four imports were lab curl requests under the owner's decision of 2026-10-26 for shape discovery. The go-ahead came from the run lead.

## Record

- **Build:** the lab's master build, the same exe as row 1761-S4: the Windows bundle `unsigned-bundle-smoke-windows-latest` of the Bridge CI `merge_group` run of #2500 at `06268abe63d659a605ca2592ee4d0a2d65eba0ea`, then origin/master's head (evidence/build-hashes.txt).
  - `bridge_mcp.exe` sha256 `507da806d9d49a72fdb144be3263e6b1c303bfc964b103616b0ea92ee7b02bf6` before the launch and again at the end.
  - The server reported `bridge-tally` 0.7.0.
- **Sheet:** `lab/2507-t-sheet` at `db2d6651d2adbede912965a4d1e79dc6f2379cd3`, 7 of 7 files equal to its SHA256.txt. The requests in requests/ are byte copies of the sheet's.
- **Tally:** TallyPrime single-user edition, `tally.exe` file version 1.1.7.2, licensed, no Educational mark.
- **Company:** BRIDGE SHAPE LAB, synthetic, loaded alone, GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346` (row 2016's; `verified_tuple`).
  - Gateway period 1-Apr-26 to 31-Mar-27, current date 20-May-2026, date of last entry 20-May-26, "(e) Data exceptions exist" (as found).
- **B0:** by Tally's Backup (C5) at step 0: `TDBK1800_<company number>.001`, 115721 B, sha256 `fbe17805840636daf30aa7af199da1ac3526f904ae48cae2b565310fcde79410`, written 2026-11-11 15:51:29 local. It was re-hashed equal before each of the four restores. It stays on the lab PC.
- **Launch:** one stdio launch from step 0 to the end (server pid 13184, exit code 0). The launch file's keys:
  - `BRIDGE_AGENT_DATA_DIR` (new and empty);
  - `BRIDGE_TALLY_HOST`;
  - `BRIDGE_TALLY_PORT`;
  - `BRIDGE_TERMS_ACCEPTED`.

  No posting setting: Bridge only read in this row. The desktop app's own processes were left running, and the row made no call through them.
- **Sends:** each request by `curl.exe --max-time 60` (c3-send-mt.ps1, the lab's c3-send.ps1 with that one flag added), its sha256 checked against the sheet first. Each answer and its headers are kept whole in raw/. Every import was answered within the limit.

## Checks

| step | check | result |
| --- | --- | --- |
| 0 | C2 (Tally answering on 9001); `list_companies`: BRIDGE SHAPE LAB alone, row 2016's GUID | PASS |
| 0 | marks (raw/m0.xml): ALTVCHID 126, ALTMSTID 300 | recorded |
| 0 | `vouchers` 2026-05-20, the control: `complete`, total 13 | PASS |
| 0 | `vouchers` 2026-05-21: total 0, `partial`, `empty_window_wider_read_nonempty`; 2026-05-22 to 2026-05-31: total 0, `partial`, `empty_uncorroborated` | the empty-day check PASS (deviation 1) |
| 0 | check 4 of deviation 1: `vouchers` 2026-05-01 to 2026-05-31, `complete`, total 13, every row dated 2026-05-20 | PASS |
| 0 | the item's GUID from `masters` `stock_items`: Hydrochloric Acid 33pc, `3a6bd6e1-b835-4bff-89dd-8a6af138c346-0000010d`; `stock_by_batch` as of 2026-05-31: H-2504-A at Main Location holds at least 2 Kgs | PASS |
| 1 | T0: `CREATED 1`, every other counter 0, no `LINEERROR`; marks ALTVCHID 127, ALTMSTID 300 | PASS |
| 1 | T0 read (008): one Stock Journal, `in` Factory Floor H-2504-A 2.000 Kgs, rate and amount empty; `out` Main Location H-2504-A 2.000 Kgs, 30.00/Kgs, 60.00 | PASS |
| 1 | T0 on screen: Source (Consumption) Main Location 2.000 Kgs, 30.00/Kgs (in italics), 60.00; Destination (Production) Factory Floor 2.000 Kgs, rate and amount blank | equal to the read |
| 1 | restore B0; marks byte-equal to step 0's; the empty-day check | PASS |
| 2 | T1: `CREATED 1`; marks ALTVCHID 127, ALTMSTID 300; read (011): GUID `...-00000063`, ALTERID 127, MASTERID 99, lines as T0 | recorded (no EXPECT on the GUID) |
| 2 | T1b, the same request again with no restore: `CREATED 0`, `ALTERED 1`, every other counter 0; marks ALTVCHID 128, ALTMSTID 300; read (012): one row, the same GUID and MASTERID, ALTERID 128 | **the first altered** |
| 2 | restore B0; marks byte-equal; the empty-day check | PASS |
| 3 | T2: `EXCEPTIONS 1`, every other counter 0 (`LASTVCHID` 0), no `LINEERROR`; marks ALTVCHID 126, ALTMSTID 300; 2026-05-21 still empty | **refused, nothing moved (a finding)** |
| 3 | restore B0; marks byte-equal; the empty-day check | PASS |
| 4 | T3: `CREATED 1`, every other counter 0; marks ALTVCHID 127, ALTMSTID 300 | PASS |
| 4 | T3 read (019): `in` Factory Floor 2.000 Kgs and `out` Main Location 2.000 Kgs, both with rate and amount `empty` | recorded |
| 4 | T3 on screen: Source Main Location 2.000 Kgs and Destination Factory Floor 2.000 Kgs, rate and amount blank on both sides, no value in the totals | equal to the read |
| 4 | restore B0; marks byte-equal; the empty-day check | PASS |
| 5 | `list_companies`: BRIDGE SHAPE LAB alone; marks byte-equal to step 0's; the 2026-05-21 empty-day check; `bridge_mcp.exe` hash equal | PASS |

Each import's capture shows the Gateway with no dialog. Each restore answered exactly the two prompts its plan named ("Accept ?" with y, the already-exists prompt with r for Replace), and each load showed the Gateway as at step 0 with no prompt.

## What the rows answer

**T0.** The recorded shape imports as a Stock Journal in Tally's Transfer of Materials view. `INVENTORYENTRIESOUT.LIST` is the Source (Consumption) side and Bridge reads it as `out`. `INVENTORYENTRIESIN.LIST` is the Destination (Production) side, read as `in`. The in side's rate and amount read `empty`, as row 2016 read them. The screen shows them blank, while the source side's rate shows in italics. The batch is in Bridge's read (`batches[0].batch`) on both sides. The screen does not show a batch column.

**T1.** The voucher's GUID is `<company GUID>-00000063`, which is Tally's own form: the company GUID and the MASTERID 99 in hex. The `REMOTEID` sent (`lab-2507-t1-0001`) is not the row's GUID. Bridge's stock-line read does not fetch `REMOTEID`, as the sheet notes, so the read-back cannot find the voucher by the id sent. It finds it by date and by the GUID Tally gives back.

**T1b.** The same request sent again altered the first voucher: `ALTERED 1`, still one row, the same GUID, MASTERID and number, and ALTERID 127 to 128. Field by field, the two reads differ only in `alter_id`, `voucher_alter_id` and their read times. So a resend of this shape with a `REMOTEID` does not create a second voucher.

**T2.** With both lines under the one list name `ALLINVENTORYENTRIES.LIST` (children unchanged, `ISDEEMEDPOSITIVE` Yes on the in side and No on the out side), Tally refused the voucher: `EXCEPTIONS 1`, no `LINEERROR`, and `LASTVCHID` 0. No row was created and the marks did not move. So a builder for this view has to write the two named lists.

**T3.** With no `RATE` and no `AMOUNT` on the out side, Tally created the voucher with no value on either side. Bridge read both sides' rate and amount `empty`, and the screen shows both blank with no value in the totals. Tally did not value the transfer at the source godown's cost on import. If a builder wants the transfer valued, it has to send the outward value itself.

## Deviation 1: an empty day reads `partial` on this build

The sheet's EXPECT for an empty day is "a complete answer with no refusal and a total of 0 rows". Build 0.7.0 cannot give that next to a day that has vouchers. The `vouchers` tool's own description (answers-mcp/answers/000-tools-list.json is kept on the lab PC; the text is quoted from it):

> `state` is `complete` only when the window's rows were checked voucher for voucher against a count ComplyEaze Bridge made of the window first, or the window was empty, a read widened by a day on each side was also empty, and the company has no vouchers (`reason` `company_has_no_vouchers`); otherwise it is `partial`, with `reason` `nonempty_window_unqualified` when rows were not counted, `empty_uncorroborated` when an empty window's widened read is also empty but the company has vouchers, and `empty_window_wider_read_nonempty` when the widened read found rows on the neighbouring days (the window's own read found nothing and those rows cannot count it, so an empty window beside a busy day cannot reach `complete` today).

The step 0 reads stopped the row before any write, and the run lead decided (2026-11-11) that an empty-day check passes only when all of these hold:
1. `isError` false, no refusal, total 0, every part served, and `state` `partial` with `reason` `empty_window_wider_read_nonempty` or `empty_uncorroborated` (any other reason is a stop);
2. the 2026-05-20 control read is `complete` with total 13;
3. `marks.xml` equals step 0's (ALTVCHID 126, ALTMSTID 300), again after each restore;
4. once at step 0, a `vouchers` read of 2026-05-01 to 2026-05-31 with no row dated after 2026-05-20.

check-empty.py applied conditions 1 to 3 to each check's answers. All six checks passed: step 0, after each of the four restores, and step 5. Condition 4 passed at step 0 (005).

## Other notes

- **The marks answer carries more than the three marks.** `marks.xml` fetches `GUID`, `ALTVCHID` and `ALTMSTID`, but Tally's answer also holds other company fields. GODOWN, STOCKITEM, CURRENCY, TAXUNIT, VOUCHERNUMBERSERIES and VOUCHER read 0 at step 0. They read above 0 after T0, T1 and T1b. After the refused T2 only TAXUNIT and VOUCHERNUMBERSERIES read 1. Each read 0 again after its restore and load, where the whole answer was byte-equal to step 0's. They are outside the request's fetch, so the row read "nothing moved" at T2 on the three marks; the two fields are recorded here for the run lead.
- **Side files at a restore.** Each restore took the company folder from 24 entries to 17 (seven Tally side files gone), and the load brought six back (`Index.1800` not), as on BRIDGE PILOT LAB in an earlier row. The listings name the company number, so they stay on the lab PC.
- **Each restore** followed a written plan (restore-plan-t0.txt to restore-plan-t3.txt, kept local as they name the company number), made by prep-c6.py, which also re-hashed B0.
- **The screen views** (T0 and T3): Day Book, F2 to 21-5-2026, Enter on the one row, which opens Inventory Voucher Alteration (Secondary). Nothing was typed. Each was left by Ctrl+Q once a capture showed its title (no prompt), and the Day Book by Escape. The marks read after each view were byte-equal to the ones before it.

## Close

- The client closed on its inbox file at 2026-11-11 10:44 UTC: session ended, server pid 13184, exit code 0.
- C10: Company > Shut > BRIDGE SHAPE LAB, with no prompt. Select Company was then shown with no company loaded (shots/s5-shut.png, on the lab PC).
- BRIDGE SHAPE LAB is left as B0 restored it, with step 0's marks. B0 stays on the lab PC by name, size and sha256.

## Deviations

1. The empty-day check, as above.
2. New lab helpers, all read-only on the book: marks.py (the three marks and a byte comparison), check-empty.py (deviation 1's conditions 1 to 3), prep-c6.py (C6's prep), list-df.py (the company folder listing, kept local) and send.ps1 (one sheet request by c3-send-mt.ps1).
3. The book's GUID is row 2016's, which is on its answers branch and not on master. The stage checks it against the step 0 `list_companies` answer and the excluded GUID instead.
4. T1b's answer went to its own file (raw/t1b-answer.xml), since the send helper refuses to overwrite an answer.

## Not established

- Whether Tally matched T1b to T1 by the `REMOTEID` or by other fields: the sheet sends no resend of T0 without one.
- Which line T2's exception was for: Tally's answer names none.
- Rows T6 to T14 and TB, as the sheet says.

## Files

- **requests/:** the five sheet requests as sent, byte copies.
- **raw/:** each curl answer and every `marks.xml` answer whole, each with its headers file.
- **answers-mcp/:** each tool answer whole, with the calls (inbox/), the client's log and pid. `company_number` values are shown as `<redacted: company number>`. Files that stay on the lab PC are in kept-local.txt.
- **data/:** the server's data folder, whole.
- **evidence/:** notes.txt (UTC, one line per step), prep.txt, send-log.txt, the tools log, build-hashes.txt, and the port listing cut to its port-9001 LISTENING lines.
- **scripts/:** the row's helpers.
- **args/:** the call arguments.
- **kept-local.txt:** files kept on the lab PC by size and sha256.
- **SCREENSHOTS.txt:** the captures by name and hash.
- **substitutions.txt:** each pushed copy that differs from its original.
- **SHA256.txt:** every pushed file.
