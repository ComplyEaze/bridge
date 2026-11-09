# Row 2440-CP: a Sales invoice built and posted by the build of #2440, without and with its e-invoice details, and read back

Not for merge. Part of #2162 (#2440). Written 9 Nov 2026 at master ae38a75c, where this branch starts. One row, on the synthetic book BRIDGE EINVOICE LAB. `LAB-COMMON.md` in this folder (v3, byte-equal to row 2162-IRN10's at 836dd394) applies in full; where this sheet and it differ, the stricter holds. Record the TallyPrime version, the licence mode and the date. Confidence of everything here until the row runs: unverified.

**WHY.** #2440 lets an invoice carry the e-invoice details the portal already registered (`invoice.e_invoice`: `irn`, `ack_no`, `ack_date`). The build writes them into the import as `IRN`, `IRNACKNO` and `IRNACKDATE`, shows them in the approval window, and after the post reads them back by the voucher's GUID. Row 2162-IRN9 imported those three elements by a raw request and Tally stored them as sent. No row has yet sent an invoice through the build's own path: the build's reads, its refusals, the approval, the post and the read-back, together. This row is that live evidence (P1). It checks:
- that the build admits an invoice on a book set up as it requires, and posts it;
- that the same invoice with the details is posted, and its read-back finds the three values as sent;
- what Tally stored, read raw, for each.

**WRITES.**
- One ledger is seeded by a raw import (`I-seed.xml`), under the owner's seed rule. It is never evidence of its own shape.
- Two duty ledgers and one Sales voucher type are keyed BY HAND on Tally's screen.
- Two invoices are posted by the build's own `post_import`, one at a time, each after [OWNER]'s approval.
- The book is restored to BK1 at the end.

**[OWNER] steps** are marked [OWNER]: the owner at the lab PC. Everything else is the operator's. The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## The book as last read
Row 2162-IRN10's read of BRIDGE EINVOICE LAB, as recorded there, after its restore:
- Maharashtra; books from 1-Apr-25; period 1-Apr-25 to 31-Mar-26; Tally's current date 3-Apr-2025; e-invoicing on.
- Vouchers Sales 1 (with an IRN), Sales 2 and Payment 1, all dated 2025-04-03.
- The Sales type: Automatic numbering, prevent-duplicates No. The build refuses that type, so this row keys its own.
- `BRIDGE IRN Customer` in Karnataka. An inter-state invoice is outside what the build posts, so this row seeds a customer in Maharashtra.
- `BRIDGE IRN Sales`: taxable, 9, 9 and 18, HSN 998313.
- No CGST or SGST ledger. `BRIDGE IRN IGST` has no duty head and rate 0, and is not used here.
- Not read on this book: the company's HSN settings. Step 1 reads them before any write.

## What the build requires, and where the row meets it
| the build requires | met by |
| --- | --- |
| a Sales-class type with one series, Manual, prevent-duplicates Yes | `BRIDGE CP Sales`, keyed in step 5 as row 2162-KD keyed one |
| a party under Sundry Debtors, not bill-wise, Regular, its GSTIN in force on the date with a valid check character, its state that of the supply | `BRIDGE CP Customer MH`, seeded in step 3: from 20250401, Regular, Maharashtra, `27ZZZZZ0000Z3ZO` (its check character computed when the sheet was written) |
| a CGST and an SGST ledger under Duties & Taxes, each at half the rate, rounding Not Applicable with limit 0 | `BRIDGE CP CGST` and `BRIDGE CP SGST`, keyed in step 4 |
| a sales ledger at IGST 5 or 18 | `BRIDGE IRN Sales` (18), checked in step 1 |
| the company's HSN taken from the ledger for all sections, with classifications off | checked in step 1, never changed by this row |
| a supply within the state; tax equal to the rate times the taxable value | both invoices: Maharashtra, 100.00 taxable, 9.00 CGST, 9.00 SGST |

## Build (holds #2440's head)
The build is the Windows bundle `unsigned-bundle-smoke-windows-latest` of a Bridge CI run started by hand on `fix/2162-e-invoice-details` (a draft pull request builds no bundle). The lab downloads it into a new empty folder and checks three hashes:
- the zip's sha256 against the artifact's digest;
- the `.mcpb` against `archive_sha256` in its `mcpb-smoke.json`;
- `bridge_mcp.exe` against `binary_sha256` in the same file.

It sends those three hashes to the run coordinator, who fills the line below by a new commit on this branch, after checking that the run's commit is #2440's head. While any field reads `<to fill>`, this row does not run: STOP. Before the first call, hash the exe that will run and compare it with the line; any other hash = STOP. Record the version the tool reports.

RC COMMIT: <to fill>  CI RUN: <to fill>  ARTIFACT: <to fill>  ZIP DIGEST: <to fill>  EXE SHA256: <to fill>

Run it as the lab runs a staged exe over stdio, ONE launch from step 6 to the end of step 10, never relaunched in between. Environment:
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- ONE new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\2440-cp\data\`;
- `BRIDGE_TERMS_ACCEPTED=true`, the person responsible for the lab accepting the Terms of Use;
- [OWNER] posting ON: `BRIDGE_AGENT_ENABLE_WRITES=true`, set before the launch (step 6 records only that it is present);
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset, and `BRIDGE_AGENT_ENABLE_BATCH_POST` absent (it is retired).

No other ComplyEaze Bridge may make a call against port 9001 during the row, and no raw request is sent while the exe runs. Record the process list at step 6.

**THE APPROVAL WINDOW** opens from the exe's own process and may open behind other windows (look for it on the taskbar). While it is open nobody presses a key on the PC and no tool is called, except the step's own repeat `post_import`. [OWNER] answers it by mouse, within its time. Record the time it was seen open and the time it was answered, and its text word for word.

**Marks.** MARKS means `ledger_masters`, first page, no `snapshot_id`: M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`, and `snapshot.reused` must read false. Record each answer whole.

**Calls.** Only the build's own tools, one at a time. An answer's `next_step` or `next_steps` is recorded, never acted on: this row's steps decide every call.

## Files
`gen.py` writes every request but `M-marks.xml` from three templates beside it, each change asserted. `SHA256.txt` holds every file's hash.
- **`M-marks.xml`:** byte-equal to row 2162-IRN10's: the COMPANY rows of the loaded book, with GUID, ALTVCHID and ALTMSTID.
- **`O-company.xml`:** the committed request `pilot-lab-company-object.request.utf16le.xml` (every field of one company, by name) for BRIDGE EINVOICE LAB. Its pilot answer, committed beside it, holds `ISGSTCLASSIFON` and one `CMPGSTDETAILS.LIST` with `HSNSOURCEPRIORITY` and `HSNAPPLICABLESECTION`.
- **`I-seed.xml`:** row 2162-S9a's `L-seed.xml` (one `LEDGER` create under Sundry Debtors, not bill-wise, with one `LEDGSTREGDETAILS.LIST`), with the company, the name, the date, the place of supply and the GSTIN changed to this row's customer. It sets no party type.
- **`K-customer.xml`, `K-cgst.xml`, `K-sgst.xml`, `K-sales.xml`:** row 2162-IRN10's ledger read by name (every native method, no fetch list), for each ledger this row uses.
- **`F-0101.xml`, `F-0102.xml`:** row 2162-IRN10's voucher read by number (every native method), for each invoice this row posts.
- **`run.py`:** byte-equal to row 2162-IRN10's. It sends each file once, writes each answer with `curl -o`, and stops at a curl failure, an empty answer, an answer above its bound, or Unknown Request.
- **`count8.py`:** row 2162-IRN10's `count.py`, with the tax family of its K kind widened to the rounding fields and a kind O added for the company object (each change marked CP). When the sheet was written:
  - on the committed pilot answer, as an O answer, it printed STATUS '1', `ISGSTCLASSIFON` 'No', one `CMPGSTDETAILS.LIST` with `HSNSOURCEPRIORITY` 'Ledger', `HSNAPPLICABLESECTION` 'All Sections' and `APPLICABLEFROM` '20260401', and one `HSNDETAILS.LIST`;
  - on a test answer with no company (not kept), it printed STOP and exited 1.

**Before each raw write** (the seed in step 3): `run.py URL s3m M-marks.xml`, then `python3 -I count8.py --before-write a-s1-M-marks.xml a-s3m-M-marks.xml`. Before each keyed write (steps 4 and 5), the same with that step's label. Only if it passes, and no key was pressed in Tally since, make the write.

## Before the sitting
1. [OWNER] Tally is LICENSED on the lab PC. Educational mode anywhere on screen = STOP (C1).
2. [OWNER] The computer-use grant covers the path the exe of the build line will run from. Without it this row does not run.
3. Check every file against `SHA256.txt` (Get-FileHash).
4. [OWNER] Go-ahead for this row.

## Part A: the book, by raw requests (no ComplyEaze Bridge running)

### Step 1: load and read, before any write
Load BRIDGE EINVOICE LAB alone, as C2 says (its GET /status checks stay). Record the period and Tally's current date. Then `s1`: `M-marks.xml O-company.xml K-sales.xml K-customer.xml K-cgst.xml K-sgst.xml`, and `count8.py` on each answer. EXPECT:
- M: one COMPANY row, with the book's GUID. Record ALTVCHID and ALTMSTID.
- O: the book's GUID; `ISGSTCLASSIFON` No; a `CMPGSTDETAILS.LIST` whose `APPLICABLEFROM` is on or before 20250403, with `HSNSOURCEPRIORITY` Ledger and `HSNAPPLICABLESECTION` All Sections.
- K-sales: one LEDGER row, `BRIDGE IRN Sales`, with an IGST rate of 18 and HSN 998313.
- K-customer, K-cgst, K-sgst: 0 LEDGER rows each.
- Tally's current date 3-Apr-2025, the period 1-Apr-25 to 31-Mar-26.

Anything else: record it and STOP before any write. The build cannot admit an invoice on this book without a change to its company settings, and this row makes none. That is a result for the run coordinator, not a failure of the row.

### Step 2: the backup
Backup BK1 (C5) into a NEW empty folder `C:\bridge-lab\2440-cp\backups\BK1\`: record the name, size and sha256. Then `s1b`: `M-marks.xml`, EXPECT `--same-row` equal to `s1`.

### Step 3: the customer, seeded
`s3m` and the check, then `s3`: `I-seed.xml`. Screenshot the window. EXPECT CREATED 1, every other counter 0, no LINEERROR, and no dialog. Then `s3r`: `K-customer.xml M-marks.xml`. EXPECT:
- one LEDGER row, `BRIDGE CP Customer MH`, PARENT Sundry Debtors, `ISBILLWISEON` No, one GSTIN;
- one `LEDGSTREGDETAILS.LIST`: `APPLICABLEFROM` 20250401, `GSTREGISTRATIONTYPE` Regular, `PLACEOFSUPPLY` Maharashtra;
- in M, ALTVCHID as in `s1` and ALTMSTID above it.

Any other answer: record it and go to step 11.

### Step 4: the two duty ledgers, keyed on screen
`s4m` and the check. From the Gateway: Create, then Ledger. Key `BRIDGE CP CGST`:
- Under: Duties & Taxes;
- Type of duty/tax: GST; Tax type: Central Tax;
- Percentage of calculation (or the field Tally offers for the rate): 9;
- Rounding method: Not Applicable.

Then `BRIDGE CP SGST` the same, with Tax type State Tax. Leave every other field as Tally fills it. Screenshot every part of each screen before its save, and write down every field it shows with its value, as text. If Tally does not offer a field above, or offers no rate field, stop before saving and ask. After the last save, leave the creation screen as C7 says and screenshot the Gateway.

Then `s4r`: `K-cgst.xml K-sgst.xml M-marks.xml`. EXPECT one LEDGER row each, PARENT Duties & Taxes, `TAXTYPE` GST, `GSTDUTYHEAD` Central Tax and State Tax, `RATEOFTAXCALCULATION` 9, `ROUNDINGMETHOD` Not Applicable and a `ROUNDINGLIMIT` of 0 or absent; in M, ALTVCHID as in `s1`. Record every leaf the K kind prints. A field that reads otherwise is recorded, not a STOP: step 7 shows whether the build admits it.

### Step 5: the Sales type, keyed on screen
`s5m` and the check. From the Gateway: Create, then Voucher Type, as row 2162-KD did:
- Name `BRIDGE CP Sales`; Select type of voucher `Sales`;
- Method of Voucher Numbering `Manual`;
- Prevent creating duplicate Voucher Nos. `Yes` (on that row, this field appeared under the method once Manual was chosen, showing No).

Leave every other field as Tally fills it. Screenshot and write down every field before saving. On row 2162-KD, Ctrl+A saved with no prompt and opened a new blank Voucher Type Creation screen; two Ctrl+Q presses, each after a capture of the screen's title, went back to the Gateway. Record the keys used here. Then `s5r`: `M-marks.xml`, EXPECT ALTVCHID as in `s1` and ALTMSTID above `s4r`'s.

The book: the masters of steps 3 to 5 added; no voucher.

## Part B: the build (ONE launch)

### Step 6: step 0
1. LAB-COMMON C2 step 0 with the book loaded alone, without its MARKS request. Record the process list and that `BRIDGE_AGENT_ENABLE_WRITES` is present. Hash the exe against the build line.
2. `tools/list`: record the answer whole. EXPECT `build_import_xml` whose input schema holds `e_invoice` under `invoice`, requiring `irn`, `ack_no` and `ack_date`; `post_import`; `verify_import`; and `vouchers` with `include_e_invoice`. Any missing = STOP (not this build, or posting is off).
3. `list_companies`: EXPECT exactly BRIDGE EINVOICE LAB. Record its GUID; every later call passes it as `company_guid`.
4. MARKS: M6 and V6. EXPECT M6 and V6 equal to `s5r`'s ALTMSTID and ALTVCHID.
5. `masters` with `kind` `voucher_types`: record the `BRIDGE CP Sales` row whole. EXPECT one series, numbering manual, prevent-duplicates on. Otherwise record it and go to step 11.

### Step 7: invoice A, built (not sent)
`build_import_xml` with:
```json
{"company_guid": "<the GUID of step 6>", "vouchers": [{
  "bridge_txn_id": "cp-a", "date": "2025-04-03", "voucher_type": "Sales", "voucher_number": "CP/25-26/0101",
  "invoice": {"voucher_type_name": "BRIDGE CP Sales", "place_of_supply": "Maharashtra"},
  "entries": [
    {"ledger": "BRIDGE CP Customer MH", "amount": "118.00", "side": "Dr"},
    {"ledger": "BRIDGE IRN Sales", "amount": "100.00", "side": "Cr"},
    {"ledger": "BRIDGE CP CGST", "amount": "9.00", "side": "Cr"},
    {"ledger": "BRIDGE CP SGST", "amount": "9.00", "side": "Cr"}]}]}
```
EXPECT `voucher_count` 1 and a `batch_id`: record the answer whole and that `batch_id` as A. A refusal is a FINDING: record it whole. It names the requirement this book does not meet, which is this row's result for the composed path. Then go to step 10.

### Step 8: invoice A, posted
1. MARKS: V8a, immediately before the post. EXPECT V8a = V6.
2. Tell [OWNER] a window will open. `post_import` with `company_guid` and `batch_id` A. [OWNER] reads the window and answers **Yes** by mouse within its time. EXPECT the window to show no line beginning `E-invoice`.
   - If `approval.state` reads `pending`: wait `approval.retry_after_s` seconds and call `post_import` again with A.
   - If it reads `approved` without a send: call again at once. This is the tool's own flow, not a retry.

   Record every answer whole.
3. If an answer reads `import_approval_timed_out`: record it whole, read MARKS once, and go to step 10. A is not called again without the run coordinator's word.
4. EXPECT in the answer that sent the post `dispatch.state` `posted_verified` and `dispatch.counters.created` 1. Then, before any other call, screenshot the whole Tally window: a dialog or a half-entered screen = C7, STOP.
5. `verify_import` of A: EXPECT `verification_status` `posted_verified`. Record the answer whole.

Any other final state: record it whole and go to step 10. While invoice A is sent and not verified, the build refuses any further invoice of this company (`invoice_company_stopped`), so step 9 would test nothing.

### Step 9: invoice B, with its e-invoice details, built and posted
1. `build_import_xml` as step 7, with `bridge_txn_id` `cp-b`, `voucher_number` `CP/25-26/0102`, and in `invoice`:
   ```json
   "e_invoice": {"irn": "d4e5f6a7b8c9d0e1d4e5f6a7b8c9d0e1d4e5f6a7b8c9d0e1d4e5f6a7b8c9d0e1", "ack_no": "445577", "ack_date": "2025-04-03"}
   ```
   The IRN is synthetic, 64 lower-case hexadecimal characters, and differs from row 2162-IRN9's. EXPECT `voucher_count` 1 and a `batch_id`, B. A refusal is a FINDING: record it whole and go to step 10.
2. As step 8, with B: MARKS V9a (EXPECT V8a + 1: one voucher created; another value is recorded, not a STOP), then `post_import`. EXPECT the window to show, after the bill allocation, the two lines `E-invoice IRN: ` followed by the IRN above, and `E-invoice ack: 445577 of 2025-04-03`. Record the window's text word for word.
3. EXPECT `dispatch.state` `posted_verified` and `dispatch.counters.created` 1. An answer reading `posted_divergent` with `invoice_fields` naming any of `IRN`, `IRNACKNO`, `IRNACKDATE` or an `e_invoice_` code is a FINDING: Tally did not store, or the read did not find, a value as sent. Record it whole and go on.
4. Screenshot the whole Tally window (C7), then `verify_import` of B, recorded whole.

### Step 10: the read-back, then the close of the launch
1. MARKS: record M and V.
2. `vouchers` from 2025-04-03 to 2025-04-03 with `include_e_invoice` true: record the answer whole. EXPECT the three vouchers of the book as last read and, if steps 8 and 9 posted, `CP/25-26/0101` and `CP/25-26/0102` on `BRIDGE CP Sales`, each with the four entries sent:
   - 0101: `e_invoice` `irn`, `ack_no` and `ack_date` each absent;
   - 0102: each present, with the values sent (`ack_date` as Tally stores it).
3. Close the exe. That ends the launch.

## Part C: raw reads, then the restore (no ComplyEaze Bridge running)

### Step 11: what Tally stored
`s11`: `F-0101.xml F-0102.xml M-marks.xml`, then `count8.py` on each, and `python3 -I count8.py --diff a-s11-F-0101.xml a-s11-F-0102.xml` when each holds one VOUCHER row. Record every e-invoice element and GST header field it prints. EXPECT `IRN`, `IRNACKNO` and `IRNACKDATE` on 0102 as sent (`IRNACKDATE` 20250403), and on 0101 absent or empty. Record the difference list whole: a difference beyond the number, the identity fields and the three elements is recorded, not judged.

### Step 12: the restore
1. [OWNER] Restore BK1 (C6), on the run coordinator's explicit RESTORE. There is no fallback backup: a restore that does not complete is a STOP, reported as C6 says.
2. `s12`: `M-marks.xml K-customer.xml F-0101.xml`. EXPECT `--same-row` against `s1` equal, and 0 LEDGER rows and 0 VOUCHER rows. Else STOP, and report what the book holds.

The book: as at BK1.

## STOP RULES
C7 and C8; any dialog before a call or a request; an approval window other than steps 8 and 9's; a mark that moves outside the step that changes it; any answer outside the stated shape. On a STOP during Part B, close the exe first, then go to step 12 on the run coordinator's word.

## Not covered by this row
- An inter-state invoice (IGST) and an unregistered customer: the build refuses both today.
- A refusal of malformed e-invoice details (the build's unit tests carry those), and the same details sent on two invoices.
- An e-invoice generated by Tally itself, a cancelled IRN, a QR code, and any portal call.
- A book with another HSN setting, a duty ledger with another rounding method, a type with more than one series.
- Any TallyPrime release other than the one recorded, and any other book.

## Close and evidence
- **C10:** Shut Company; leave no company loaded. The book stays on the lab PC: it is synthetic.
- **C11:** evidence to a new branch `lab/2440-composed-answers`, with `SHA256.txt`, after the pre-push scan and the run coordinator's yes. It holds:
  - every answer whole with its size and sha256 (an answer holding a company number or a long digit run is kept on the lab PC and listed by hash);
  - each step's `count8.py` output and the step 11 difference list;
  - the step 4 and 5 screens as text, and the screenshots by name, size and sha256;
  - the two approval windows' text and their open and answer times;
  - the data folder's file listing after step 10 (names, sizes, sha256);
  - BK1's name, size and sha256, and the build's hashes;
  - a notes file with UTC times and one line per step.

  A RESULTS.md gives each check as PASS or FAIL, with the build, book, date and record: the book's settings (step 1), the masters (steps 3 to 5), the build of A (step 7), the post of A (step 8), the post of B (step 9), the read-back (steps 10 and 11) and the restore (step 12).
