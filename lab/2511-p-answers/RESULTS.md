# Results: row 2511-P, three plain rupee Receipts on the several-currency lab book, and the ledger catalogue's currency, 2026-11-11

**Outcome: every EXPECT held, and the row ran from step 0 to the close.**
- **P1, P2 and P3:** each imported with CREATED 1 and read back plain.
- **P2:** the new bank entry on `BRIDGE Lab Bank` read plain, not as a composite.
- **P3:** the party's amount landed On Account, and no earlier voucher or bill moved.
- **P4a:** the catalogue request with `CurrencyName` answered the rupee sign on all eight ledgers, equal to the currency Trial Balance.
- **Verification:** each batch read `posted_verified` through `verify_import`.
- **The close:** the restore of `bk-2511p` returned the book to m0; its mvr read is byte-equal to mv0.

There are two deviations, both decided by the run lead before the first write:
- the writes were sent with a UTF-8 header (`run8.py`, below);
- `bridge_txn_id` takes the values `2511-P1` to `2511-P3`.

The lab also made one more MARKS read before p2w and one before p3w (deviation 3).

Each result is PARTIAL: one sitting, one book, one release.

**Who acted.** The lab did everything on the owner's standing word for synthetic books (2026-10-27 and 2026-10-28):
- loading the book;
- the BK;
- the three sends;
- the restore;
- the close.

The go-ahead, the yes for each second build call and the two decisions came from the run lead in chat. No approval window opened. Writes were off, so nothing was posted through Bridge.

## Record

- **Sheet:** `lab/2511-p-sheet` at `39fad6c019d0426ee38401fb0cf1b347e9731d61`. Its 17 kit files are equal to its SHA256.txt, sizes included.
  - The KIT LAST MARKS file, row 683-TN's m2, was taken from `lab/683-tn-answers` at `1b9412f88a2dfe603616a57aeb45b27cf1530cba`. It is 3,404 B, sha256 `3d75dd1a8fb7252b93f55f9a492ddf449e2be3814feaa781685437f2548dafba`, equal to the sheet's line.
- **Build:** the sheet's build line: the Windows bundle of the merge-group run at master `de54a0f26e554f574727a0bc4914b0ad6ca88583`.
  - `bridge_mcp.exe` sha256 `be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce` was hashed again by the client before launch.
  - The server reported `bridge-tally` 0.7.0.
- **Tally:** TallyPrime 7.2 (`tally.exe` file version 1.1.7.2), TallyPrime single-user edition, licensed. No Educational mark showed (C1). The PC date was 2026-11-11.
- **Company:** BRIDGE FOREXGROUP LAB, GUID `1bb48be5-baba-45b2-90d8-7593244915a9`. It is synthetic and listed in master's registry.
  - At step 0 the Gateway showed the period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, and last entry 1-Apr-25, as EXPECT.
  - `list_companies` named this one company, `verified_tuple`.
- **Launch:** one stdio launch by the lab's own client, from step 0 to the close (server pid 13784, exit code 0). The launch file's keys:
  - `BRIDGE_AGENT_DATA_DIR` (new and empty);
  - `BRIDGE_AGENT_ENABLE_IMPORT` = `true`;
  - `BRIDGE_TALLY_HOST` = `127.0.0.1`;
  - `BRIDGE_TALLY_PORT` = `9001`;
  - `BRIDGE_TERMS_ACCEPTED` = `true`.

  `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` were absent. `tools/list` gave 35 tools. No other Bridge called port 9001 for the row. The desktop app's own processes were left running, and the row made no call through them.
- **Requests:** each read was sent by the sheet's `run.py`, after its sha256 was checked against the sheet's SHA256.txt (`send.sh`). Every read was answered with STATUS 1 within a second (`evidence/sends-utc.txt`).

## Checks

| step | check | result |
| --- | --- | --- |
| 0 | C2: one `tally.exe`, which owns 9001; GET /status 200; the Gateway as EXPECT; the release, the edition and the PC date recorded; exe hash equal to the build line; IMPORT on and WRITES absent | PASS |
| 1 | m0 against the KIT file: SAME MARKS (ALTVCHID 11, ALTMSTID 214) | PASS |
| 2 | mv0: 59,524 B, byte-equal to row 683-TW's answer; `lm.py --capture` movement 11, capture 11, CAPTURE SAME | PASS |
| 2 | `vcmp.py` twmvy against mv0 (recorded, not an EXPECT): COMMON SAME (common 11, differ 0, only in A 0 (0 not named), only in B 0 (0 not named), named but missing 0) | recorded |
| 2 | c4 (P4a, a new shape): 13,698 B (bound 200,000), STATUS 1, 8 LEDGER rows; the screenshot after it shows the Gateway with no dialog | PASS (P4a below) |
| 2 | m0b: SAME MARKS with m0 | PASS |
| 3 | P1's first build call: `bill_wise_party_unapproved`, one party (BRIDGE INR Customer, row_count 1, credit 100, debit 0); no file | PASS (as EXPECT) |
| 3 | P1's second build call, after the run lead's yes: a file, `batch_id` `bridge-40498869-1272-42fe-86fa-1e446aa75938`, preflight `current_window_readable` | PASS |
| 3 | the file: one VOUCHER with the two entries as written | PASS |
| 3 | BK by C5 to `C:\lab\bk-2511p`: `TDBK1800_<company number>.001`, 37,618 B, sha256 `645d3ef3dba9e7eb7e417b1122ee37ec23b237ded2e7db05ef49867cb6618d3e`, 19:04 local | PASS |
| 3 | mb: SAME MARKS with m0b | PASS |
| 4 | p1w: CREATED 1, every other counter 0, no LINEERROR, LASTVCHID 12; no dialog after it | PASS |
| 4 | m1: MARKS DIFFER from mb (ALTVCHID 12, ALTMSTID 214) | PASS |
| 4 | d1: one voucher | PASS (below) |
| 4 | mv1 against mv0, `--only-b` P1: COMMON SAME (common 11, differ 0, only in A 0 (0 not named), only in B 1 (0 not named), named but missing 0) | PASS |
| 4 | `verify_import` P1: `posted_verified` (posted_verified 1, every other count 0) | PASS |
| 5 | P2's first call: `bill_wise_party_unapproved`, one party (BRIDGE INR Customer, row_count 1, credit 100, debit 0); second call: `batch_id` `bridge-cf17ad15-a81e-45fb-a207-233ba1addd70` | PASS |
| 5 | p2w: CREATED 1, every other counter 0, LASTVCHID 13; m2 DIFFER (ALTVCHID 13); d2 one voucher | PASS |
| 5 | mv2 against mv1, `--only-b` P2: COMMON SAME (common 12, differ 0, only in A 0 (0 not named), only in B 1 (0 not named), named but missing 0) | PASS |
| 5 | `verify_import` P2: `posted_verified` | PASS |
| 6 | P3's first call: `bill_wise_party_unapproved`, one party (BRIDGE USD Customer, row_count 1, credit 100, debit 0); second call: `batch_id` `bridge-ccb399de-9a6b-419e-9262-bfbcb717e6d5` | PASS |
| 6 | p3w: CREATED 1, every other counter 0, LASTVCHID 14; m3 DIFFER (ALTVCHID 14); d3 one voucher | PASS |
| 6 | mv3 against mv2, `--only-b` P3: COMMON SAME (common 13, differ 0, only in A 0 (0 not named), only in B 1 (0 not named), named but missing 0) | PASS |
| 6 | `verify_import` P3: `posted_verified` | PASS |
| 7 | m3r: SAME MARKS with m3 | PASS |
| 8 | the restore of `bk-2511p` by C6; only the two prompts the plan named | PASS |
| 8 | mr: SAME MARKS with m0 (ALTVCHID 11, ALTMSTID 214) | PASS |
| 8 | mvr: byte-equal to mv0; CAPTURE SAME, movement 11, capture 11 | PASS |
| 8 | `vcmp.py` mv0 against mvr (recorded): COMMON SAME (common 11, differ 0, only in A 0, only in B 0) | recorded |
| 9 | Tally's date: Tuesday, 1-Apr-2025, as at step 0 | PASS |
| 10 | C10: the client closed (exit code 0); Shut Company with no prompt; one `tally.exe` owns 9001; GET /status 200; no company loaded | PASS |

## The answers to the sheet's questions

**P4a: the catalogue answers each ledger's currency** (`evidence/c4-ledgers.txt`). Every LEDGER row of `C-cat4.xml` carries `CURRENCYNAME`, read by an XML parser:

| NAME | PARENT | ISBILLWISEON | CURRENCYNAME | equal to tb3 |
| --- | --- | --- | --- | --- |
| BRIDGE Export Sales | Sales Accounts | No | `₹` | yes |
| BRIDGE Import Purchases | Purchase Accounts | No | `₹` | yes |
| BRIDGE INR Customer | Sundry Debtors | Yes | `₹` | yes |
| BRIDGE Lab Bank | Bank Accounts | No | `₹` | yes |
| BRIDGE USD Customer | Sundry Debtors | Yes | `₹` | yes |
| BRIDGE USD Supplier | Sundry Creditors | Yes | `₹` | yes |
| Cash | Cash-in-Hand | No | `₹` | yes |
| Profit & Loss A/c | Primary | No | `₹` | yes |

- The "equal to tb3" column compares with row 683-TW's `tb3` answer, read with the same parser (`evidence/tb3-ledgers.txt`; that file is on `lab/683-tn-answers`).
- Each row's children: `BRIDGECOMPANYGUID`, `BRIDGECOMPANYNAME`, `CURRENCYNAME`, `GUID`, `ISBILLWISEON`, `LANGUAGENAME.LIST` and `PARENT`.
- One XML-forbidden character reference was dropped before the parse, as C4 says. Row 683-TW's tb3 holds one too.
- By the sheet's Decides, the catalogue can carry the ledger's currency on 7.2: **PARTIAL**.

**P1: a plain rupee Receipt into Cash** (`evidence/d1-voucher.txt`).
- The voucher: 20250402, Receipt No. 4, MASTERID 12, ALTERID 12, GUID `<book GUID>-0000000c`.
- `Cash` -100.00, ISDEEMEDPOSITIVE Yes.
- `BRIDGE INR Customer` 100.00, ISDEEMEDPOSITIVE No, with one bill allocation: NAME empty, BILLTYPE On Account, AMOUNT 100.00.
- Both entries read plain. `vcmp.py` exit 0 named the one new row, and `verify_import` read `posted_verified`.
- By the Decides, the per-ledger rule's admitted case imports and reads back plain through master's own build: **PARTIAL**.

**P2: the same Receipt into `BRIDGE Lab Bank`** (`evidence/d2-voucher.txt`).
- The voucher: 20250403, Receipt No. 5, MASTERID 13, GUID `<book GUID>-0000000d`.
- `BRIDGE Lab Bank` -100.00, ISDEEMEDPOSITIVE Yes. This is a plain figure, not a composite.
- `BRIDGE INR Customer` 100.00 On Account.
- The bank ledger already holds dollar composites, but they did not change the new rupee line.
- The other twelve vouchers read SAME by `vcmp.py`.
- **PARTIAL** (the Decides' first case).

**P3: the same Receipt from `BRIDGE USD Customer`** (`evidence/d3-voucher.txt`).
- The voucher: 20250404, Receipt No. 6, MASTERID 14, GUID `<book GUID>-0000000e`.
- `Cash` -100.00, ISDEEMEDPOSITIVE Yes.
- `BRIDGE USD Customer` 100.00, ISDEEMEDPOSITIVE No: plain. Its one bill allocation is NAME empty, BILLTYPE On Account, AMOUNT 100.00.
- The amount landed On Account, not against any of the party's dollar bills.
- The thirteen earlier vouchers read SAME by `vcmp.py`, the dollar bills' vouchers among them. So no bill moved that no voucher touched.
- By the Decides, this decides nothing until the run lead reads it.

**Other things each day read showed (recorded, not EXPECTs).**
- `PARTYLEDGERNAME` read the money ledger on all three vouchers: `Cash`, `BRIDGE Lab Bank`, `Cash`. Each file sent the party (`BRIDGE INR Customer` or `BRIDGE USD Customer`) there.
- The money entry carried one empty `BILLALLOCATIONS.LIST`.
- Each narration reads as built: the sheet's text, then Bridge's own tag (`2511-P1 [BRIDGE:<REMOTEID>]`).
- The REMOTEID in each file was Bridge's own (P1 `ff97f108-294e-8de4-a48a-5b3d9385da47`). Each read voucher's REMOTEID attribute is its GUID, as row 2507-T saw.

## The three files and the sends

| row | `batch_id` | file bytes | file sha256 | REMOTEID | send | answer |
| --- | --- | --- | --- | --- | --- | --- |
| P1 | `bridge-40498869-1272-42fe-86fa-1e446aa75938` | 1,044 | `4dc6b5bbcbc574c4efe25b42da7c35c041fc7a8d2d8ca5577c15c8141467d03b` | `ff97f108-294e-8de4-a48a-5b3d9385da47` | p1w 13:35:49Z | 270 B, CREATED 1, LASTVCHID 12, LASTMID 0 |
| P2 | `bridge-cf17ad15-a81e-45fb-a207-233ba1addd70` | 1,055 | `9a949dd64857bca6db8a01dd1f11fff6a4e2b11386fb463d3ba514a6639a298e` | `f4e4ec20-08bb-89c6-94b2-d55b31202d03` | p2w 13:37:46Z | 270 B, CREATED 1, LASTVCHID 13, LASTMID 0 |
| P3 | `bridge-ccb399de-9a6b-419e-9262-bfbcb717e6d5` | 1,044 | `07d1e3bd48f16578451fc0a47416c725c887665020bb969733dd41ed511e87b3` | `a3efe9ed-70bb-80de-bbaf-9739911d0b4c` | p3w 13:39:06Z | 270 B, CREATED 1, LASTVCHID 14, LASTMID 0 |

- **Each file's element names** (`evidence/p*-file.txt`): ALLLEDGERENTRIES.LIST, AMOUNT, BODY, DATE, EFFECTIVEDATE, ENVELOPE, HEADER, IMPORTDATA, ISDEEMEDPOSITIVE, LEDGERNAME, NARRATION, PARTYLEDGERNAME, REPORTNAME, REQUESTDATA, REQUESTDESC, STATICVARIABLES, SVCURRENTCOMPANY, TALLYMESSAGE, TALLYREQUEST, VOUCHER, VOUCHERTYPENAME.
- **Each file's shape:**
  - one VOUCHER with `ACTION` Create and `VCHTYPE` Receipt, and no VOUCHERNUMBER;
  - two ledger entries: the money ledger with ISDEEMEDPOSITIVE Yes and -100.00, and the party with No and 100.00.
- **Each answer's counters:** ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS were all 0, with no LINEERROR (`evidence/p*w-answer.txt`). Each was written in UTF-8.
- **Each build's second call:**
  - `on_account_approved` named the row's party;
  - `verification_preflight` read `current_window_readable` for that one day, with `source_rows` 0;
  - `observed_profile` read TallyPrime 7.2, Licensed.
- **What the sends did not go through.** They did not go through Bridge's encoder or post path. Each file was sent byte for byte as Bridge saved it, under a `charset=utf-8` header (deviation 1).

**The build arguments** (`args/`) were written to the notes before each call:
- **P1:** Receipt, 2025-04-02, `BRIDGE INR Customer` Cr 100.00, `Cash` Dr 100.00, narration `2511-P1`, `bridge_txn_id` `2511-P1`.
- **P2:** as P1, with 2025-04-03 and `BRIDGE Lab Bank` in place of `Cash`.
- **P3:** as P1, with 2025-04-04 and `BRIDGE USD Customer` in place of `BRIDGE INR Customer`.

**The approvals.** Each second call carried exactly the party_digest its own first call returned:
- P1 `e54f498b7607e015e40f4d58af3be7bb4b8321a85c88693a9bba3713a2720273`;
- P2 `249cd7e682b1dce5c1e5edbbb727181623b65a401313df8c92b608147c045612`;
- P3 `f3fd837d25452fd084f1c8f7ee329124f4f662db354853012b1fcccd5f78e3e4`.

P1's second call went after the run lead's yes. P2's and P3's went under the same yes, given for a first call that named the row's party alone with row_count 1, credit 100 and debit 0, and no other code. Both first calls did.

## The marks

| read | ALTVCHID | ALTMSTID | against | result |
| --- | --- | --- | --- | --- |
| m0 | 11 | 214 | KIT LAST MARKS | SAME |
| m0b | 11 | 214 | m0 | SAME |
| mb | 11 | 214 | m0b | SAME |
| m1 | 12 | 214 | mb | DIFFER (EXPECT) |
| m2pre | 12 | 214 | m1 | SAME |
| m2 | 13 | 214 | m2pre | DIFFER |
| m3pre | 13 | 214 | m2 | SAME |
| m3 | 14 | 214 | m3pre | DIFFER |
| m3r | 14 | 214 | m3 | SAME |
| mr | 11 | 214 | m0 | SAME (EXPECT) |

## The backup and the restore

**BK.** C5 by Tally's own Backup screen with the book loaded:
- `C:\lab\bk-2511p` was absent, then made new and empty (`evidence/bk-before-listing.txt`).
- Backup Data Path was changed to Specify Path, `C:\lab\bk-2511p`, with this one company. The screen was captured before the accept.
- `Accept ?` was answered y. No other message appeared.
- The folder then held one file (`evidence/bk.txt`).

**Restore.** C6, after `prep-c6.py`:
- The BK was hashed again, equal to its record, and its folder held that file alone.
- `restore-plan.txt` was written. It stays on the lab PC, as it names the company number.
- The company's data folder held 24 entries before.
- Company > Shut > the one row, with no prompt.
- Data > Backup & Restore > Restore:
  - Restore Data Path: Tally's data folder, as offered.
  - Source: Specify Path, `C:\lab\bk-2511p`.
  - The list showed the one row BRIDGE FOREXGROUP LAB with its number and 11-Nov-26 (7:04 pm), the BK's time. It was selected alone and the screen captured.
- `Accept ?` = y.
- Then: "Company data folder (`<company number>`) already exists. Selecting Skip will cancel the Restore process for this company. Selecting Replace will overwrite the existing folder." The number was THE COMPANY's. Answered r (Replace).
- No other prompt appeared. The data folder held 17 entries after, as the 683 rows saw after a restore.
- The book was loaded alone with no prompt. The Gateway then read as at step 0.

## Deviations

1. **The writes' header, on the run lead's decision before p1w.**
   - `run8.py` is the kit's `run.py` with one line changed: `Content-Type: text/xml; charset=utf-8` in place of `charset=utf-16` (`evidence/run8-diff.txt`). `run8.py`'s sha256 is `998ac2b5173ec8545f906302ce32b079503cf31589e6c247259246a2540ecbd7`.
   - It sent p1w, p2w and p3w only. Every read went by `run.py`.
   - The reason: the files Bridge saves are UTF-8 with no BOM and declare `encoding="UTF-8"`. Row 2210-H sent such a file under `charset=utf-16` and got `Unknown Request`. The same file under `charset=utf-8` gave CREATED.
   - `send-w.sh` checked each file's sha256 against its build result just before the send, for both the row-folder copy and Bridge's saved file.
2. **`bridge_txn_id`:** the sheet names none, so the values are `2511-P1`, `2511-P2` and `2511-P3`, on the run lead's decision.
3. **Two more MARKS reads, m2pre and m3pre.**
   - C3 sends a write only when the MARKS read just before it holds the company's row alone.
   - After P1 the reads before P2's send were d1, mv1, `verify_import` and P2's two build calls. So one MARKS read was sent right before p2w, and one before p3w.
   - Each read was SAME with the marks before it.
4. **One `list_companies` call** came before P1's first build, to read the `company_guid`.
5. **`evidence/bk.txt`'s first line was rewritten** by `fix-bk-line.py`, as the shell had read one backslash as an escape. The file's other line, with the backup's name, size and hash, is as written.

## Not established

- Another release, another book, or a ledger kept in `$` (rows P4b and P5).
- A foreign amount in an import.
- `post_import`.
- What a UTF-16 body declaring `encoding="UTF-8"` does. That is what Bridge's own post path sends, and this row's sends did not use it.

## Files

- **answers/:** each Tally answer whole, by its label.
  - The three import answers are `a-p1w-*`, `a-p2w-*` and `a-p3w-*`.
  - The sheet already carries `a-twmvy` and the request files.
  - The three import files, as sent, are `bridge-*.xml` in requests/.
- **answers-mcp/:** each tool answer whole, with the calls (inbox/), the client's log and the server pid.
  - `company_number` values are shown as `<redacted: company number>`.
  - The `tools/list` answer and the three second build answers stay on the lab PC, as each holds an edition word (kept-local.txt). Their fields are in the tables above and in notes.txt.
- **data/:** the server's data folder: the imports, their baselines and proofs, the import journal and the egress log. Any `company_number` value in them is redacted the same way.
- **logs/:** `run.py`'s and `run8.py`'s send logs.
- **evidence/:** the following, plus the parser printouts and the `lm.py` and `vcmp.py` outputs whole:
  - notes.txt (UTC, one line per step);
  - sends-utc.txt;
  - the port listing, cut to its port-9001 LISTENING lines.
- **scripts/:** the row's own helpers.
- **args/:** the call arguments.
- **kept-local.txt**, **SCREENSHOTS.txt**, **substitutions.txt** and **SHA256.txt**.
