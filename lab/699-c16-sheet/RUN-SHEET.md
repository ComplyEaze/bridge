# 699-C16: nineteen vouchers added to BRIDGE SFT LAB for clause 16(a)/(d), and the book's audit read recorded byte for byte. WRITE sitting, then READ-ONLY.

Not for merge. Part of #699 (clause 16(a)/(d), step 1). Written 4 Nov 2026 at master `1400175e`; C16 to C18 and `Tax Held C16`, then C19,
added the same day, before any of it was run. Row C16-R, at the end, was added after the row ran. `SHA256.txt` lists every file here with its bytes; check it before step A0. Confidence of everything here until the row runs: unverified.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/699-sft-candidates-sheet`, sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where this sheet and LAB-COMMON differ, the stricter rule holds.

THE COMPANY is `BRIDGE SFT LAB`: synthetic, TallyPrime 7.2 Silver, on the lab PC, with the number and the GUID that
`lab/699-sft-lab-answers` RESULTS.md records. "This GUID" below means that one. `requests/marks-sft.utf16le.xml` filters on it.
Its last marks on record are V (ALTVCHID) 34 and M (ALTMSTID) 219, read twice on 4 Nov 2026 in row 699-R1 (`lab/699-sft-read-answers`,
s1 and s9, byte-equal). It holds 34 vouchers and 13 ledgers. Not on the PC, another GUID, or Educational mode: STOP, the item has no
book.

Not a client book, and nothing in it is seeded from client data. Every amount below is synthetic.

## Why
Clause 16(a)/(d) of Form 3CD asks for amounts not credited to the profit and loss account. A list of candidates for it will be built in
the tax-audit crate only after it is checked against a book that Tally itself produced, read by Bridge's own read path (AGENTS.md P1).
This item adds the book's new shapes and records that read. Each voucher below changes one thing against another voucher of the book:
- what is credited (a capital, reserve, liability, tax, suspense, income, loan or user-defined head);
- what is debited on the same voucher (cash, bank, an expense, an asset, a party, an overdraft, the profit and loss account), and
  a voucher that debits two of these;
- a tax ledger credited on a sale and on a payment, beside one credited alone, and one debited beside an expense on a bill;
- one head credited twice on one voucher.

The book's 34 vouchers stay as they are and are part of the book read.

## Phase A: the new masters and vouchers (WRITE, keyed by hand)
Every write in this item is keyed by hand on Tally's screens. Nothing is imported. Nothing is sent to port 9001 in Phase A except
C2's own reads and the marks reads named below. Each marks read is `requests/marks-sft.utf16le.xml` sent raw to 9001 by curl as
C3 says, never through the relay; write down V and M and keep the answer with its sha256.

A0. Load THE COMPANY alone (C1). Run C2 step 0 (its MARKS read is the marks read above). Record Tally's release, Tally's current
    date and the PC's date. **a0 marks**: EXPECT one COMPANY row with this GUID, V 34 and M 219. Another V or M: STOP, the book is
    not as left.

A1. Back up THE COMPANY by C5 into the new empty local folder `BK-C16`. Record the file's name, bytes and sha256 and the time the
    Restore list will show. Then **a1 marks** (C5's own MARKS read). A restore of `BK-C16` is the way back for everything below,
    made only on the run coordinator's word (C6).

A2. Make no change to any other company, and no change to any of THE COMPANY's existing masters or vouchers.

A3. Create one group by hand: `Grants C16`, under Primary. Nature of Group = Income. "Does it affect Gross Profits?" = No. Leave
    every other field at the value Tally shows; screenshot the screen before accepting.

A4. Create these ledgers by hand. "Maintain balances bill-by-bill" is No wherever Tally shows it. Leave every other field at the value
    Tally shows, except as the table says.

    | Ledger | Group | Note |
    |---|---|---|
    | `Interest C16` | Indirect Incomes | |
    | `Reserve C16` | Reserves & Surplus | |
    | `Refund C16` | Duties & Taxes | Type of Duty/Tax = Others, if Tally asks |
    | `Suspense C16` | Suspense A/c | |
    | `Creditor C16` | Sundry Creditors | |
    | `Expense C16` | Indirect Expenses | |
    | `Provision C16` | Provisions | |
    | `OD C16` | Bank OD A/c | no bank details |
    | `Grant C16` | Grants C16 | |
    | `Tax Held C16` | Duties & Taxes | Type of Duty/Tax = Others, if Tally asks |

    Then **a4 marks**. Record V and M; EXPECT V 34. M is recorded, not predicted.

A5. Key these vouchers by hand, in this order, each exactly as written: one line per ledger named, the amount and side named,
    nothing else, no bill references. Narration as written. "Receipt", "Payment", "Journal" and "Sales" are Tally's voucher types of
    those names; the voucher number is the one Tally offers. Every date lies before Tally's current date.

    | # | Type | Date | Debit | Credit | Narration |
    |---|---|---|---|---|---|
    | C1 | Receipt | 30-Jun-2025 | `Bank SFT` Rs 5,000 | `Capital SFT` Rs 5,000 | Bank interest credited |
    | C2 | Receipt | 30-Sep-2025 | `Bank SFT` Rs 6,000 | `Interest C16` Rs 6,000 | Bank interest credited |
    | C3 | Receipt | 15-Oct-2025 | `Bank SFT` Rs 7,000 | `Reserve C16` Rs 7,000 | Claim received |
    | C4 | Receipt | 20-Oct-2025 | `Bank SFT` Rs 8,000 | `Refund C16` Rs 8,000 | Duty refund received |
    | C5 | Receipt | 25-Oct-2025 | `Cash` Rs 9,000 | `Suspense C16` Rs 9,000 | Receipt not identified |
    | C6 | Journal | 31-Oct-2025 | `Expense C16` Rs 10,000 | `Creditor C16` Rs 10,000 | Bill for services |
    | C7 | Journal | 30-Nov-2025 | `Expense C16` Rs 11,000 | `Provision C16` Rs 11,000 | Provision for expenses |
    | C8 | Journal | 15-Dec-2025 | `Creditor C16` Rs 4,000 | `Reserve C16` Rs 4,000 | Balance written back |
    | C9 | Payment | 20-Dec-2025 | `Creditor C16` Rs 3,000 | `OD C16` Rs 3,000 | Paid from overdraft |
    | C10 | Journal | 31-Dec-2025 | `Profit & Loss A/c` Rs 12,000 | `Capital SFT` Rs 12,000 | Transfer |
    | C11 | Receipt | 10-Jan-2026 | `Bank SFT` Rs 13,000 | `Interest C16` Rs 6,500 and `Capital SFT` Rs 6,500 (two lines) | Bank interest credited |
    | C12 | Journal | 20-Jan-2026 | `Asset SFT` Rs 14,000 | `Creditor C16` Rs 14,000 | Asset bought on credit |
    | C13 | Receipt | 25-Jan-2026 | `Bank SFT` Rs 15,000 | `Grant C16` Rs 15,000 | Grant received |
    | C14 | Journal | 31-Jan-2026 | `Party Four` Rs 16,000 | `Suspense C16` Rs 16,000 | Claim on customer |
    | C15 | Receipt | 5-Feb-2026 | `Bank SFT` Rs 3,000 | `Capital SFT` Rs 1,000 and `Capital SFT` Rs 2,000 (two lines, one ledger) | Two credits |
    | C16 | Receipt | 10-Feb-2026 | `Bank SFT` Rs 19,900 and `Expense C16` Rs 100 (two lines) | `Advance SFT` Rs 20,000 | Advance received less bank charges |
    | C17 | Payment | 15-Feb-2026 | `Creditor C16` Rs 5,000 | `Bank SFT` Rs 4,500 and `Tax Held C16` Rs 500 (two lines) | Paid less tax withheld |
    | C18 | Sales | 20-Feb-2026 | `Party One` Rs 11,800 | `Sales SFT` Rs 10,000 and `Tax Held C16` Rs 1,800 (two lines) | Sale with tax |
    | C19 | Journal | 25-Feb-2026 | `Expense C16` Rs 10,000 and `Tax Held C16` Rs 1,800 (two lines) | `Creditor C16` Rs 11,800 | Bill for services with tax |

    - C18 is keyed in accounting invoice mode, as S13 was: `Party One` as the party ledger, then `Sales SFT` and `Tax Held C16` as its
      two lines.
    - C16, C17, C18 and C19: if Tally will not take a voucher's two lines as written, accept nothing: screenshot, record what it showed,
      leave the voucher with Ctrl+Q after a screenshot showing the screen's title, record "Cnn not keyed", and go on.
    - C10: if Tally's ledger list on the Journal does not offer `Profit & Loss A/c`, key nothing for C10: Ctrl+Q after a screenshot
      showing the screen's title, record "C10 not keyed: not offered", and go on with C11.
    - C15: if Tally will not take `Capital SFT` on a second line, or joins the two lines into one, accept nothing: screenshot,
      record what it showed, leave the voucher with Ctrl+Q after a screenshot showing the screen's title, and go on.
    - Any other voucher Tally will not accept as written: STOP.

    Then take screenshots (C11: they stay on the lab PC): the Day Book for 1-Apr-2025 to 31-Mar-2026, all vouchers, amounts visible;
    and the Group Summary of `Grants C16`.

A6. Record, as a table, each voucher keyed in A5: its number, type, date, every line's ledger, amount and side, and its narration, all
    as keyed. Mark C10, C15, C16, C17, C18 and C19 as keyed or not keyed. Then **a6 marks**. EXPECT V = 34 plus the number of vouchers A6
    records as keyed (53 if all nineteen were). Another V goes to the run coordinator before Phase B (C11). M is recorded.

STOP, in Phase A:
- on any dialog other than the voucher and master screens' own accept and the prompts C5 quotes (C7);
- on any feature prompt (GST, inventory, cost centres, TDS);
- if Tally shows a negative cash or bank warning: report it, and go on only on the run coordinator's word.

## Phase B: the read (READ-ONLY, through the recording relay)
Phase B is row 699-R1's steps 2 to 10 again, on this book, with the same build, the same relay and the same arguments
(`lab/699-sft-read` ROW.md at 66cc762c, "Build", "Kit" and "Steps"). Only these differ:
- the relay folders are `c1`, `c2` (and `c2b` if its repeat is made) and `c3`, each new and empty;
- the relay's `--ledger-name` is `"Unused 699 C16"`;
- the marks that open and close Phase B are the **a6 marks**: R1's step 1 is replaced by A6's read, and R1's step 9 (**b9 marks**)
  EXPECTs an answer byte-identical to a6's (compare their sha256), V and M equal to a6's;
- R1's step 6 EXPECTs, in place of 34: the number of vouchers in all that A6's table and the 34 earlier ones add up to, each with this
  GUID's prefix, with the types and dates A6 and `lab/699-sft-lab-answers` A5-VOUCHERS.md record.

The build is the one R1 ran: the Windows bundle of the push of master at `d77d0682`, zip sha256
`bd7e796174a185b1523258c3052acad67c29a7d8b2e68d99d5b12d6ce60cc2bf`, and `bridge_mcp.exe` sha256
`c41fe5df31b3dc6a81c1eb90848837012579c041b6034531721c80c2cabb531c` as R1 recorded it. Another digest is a stop. Its audit read sends,
byte for byte, the requests master sends at `1400175e`: the app's replay of R1's capture checks each one on master.

The tool call is `audit_test_run` with `test` `cash_44ab`, the one test the build admits; the parts it reads are the same for every test.

## Kit
| file | what |
|---|---|
| `LAB-COMMON.md` | the lab's common rules, v3, byte copy |
| `lost_answer_relay.py` | the recording relay R1 used, byte copy of `lab/699-sft-read` (sha256 `060c3b6c…` there and here). It forwards only GET /status and POST reads to 127.0.0.1:9001, answers anything else 403 without forwarding it, and records every exchange byte for byte |
| `templates/suspense_create_request.utf16le.xml` | the template the relay loads at start, byte copy. No call in this item sends a ledger create |
| `requests/marks-sft.utf16le.xml` | the marks read of THE COMPANY, byte copy (1,636 B). Send it with `Content-Type: text/xml; charset=utf-16` |
| `.gitattributes` | `* -text`, so no file here is changed on checkout |

Run `python -I lost_answer_relay.py --self-test` before Phase B: EXPECT `self-test: ok`.

## What to report
| step | result |
|---|---|
| a0: V, M, sha256; Tally's release; the dates | |
| a1: the backup's name, bytes, sha256, time; a1 marks | |
| a3, a4: each master as created; a4 marks | |
| A6: the table; C10, C15, C16, C17, C18 and C19 keyed or not; a6 marks | |
| Phase B: everything R1's "What to report" lists, for c1, c2 and c3, and b9 against a6 | |

Read every number from its file, not from the screen. Put any STOP, any refusal and any EXPECT not met in the first line.

## Decides
| what the item shows | reading on #699 |
|---|---|
| Phase A as written, and Phase B as R1's "Decides" first row, on this book | `c2`'s exchanges become the fixtures of clause 16(a)/(d)'s tests, measured against the synthetic lab book only |
| C10, C15, C16, C17, C18 or C19 not keyed | the book lacks that shape; the fixtures stand without it, and the shape is named as not measured |
| a refusal in Phase B, a `refused` line, a count not as A6 records, or b9 not equal to a6 | as R1's "Decides" says for each: no fixture from this item |

## Stop rules
- Any STOP of Phase A above.
- a0 not V 34, M 219.
- Every stop of R1's "Stop rules", in Phase B.
- Educational mode.

## Evidence and after
Push branch `lab/699-c16-answers` with `SHA256.txt`, `RESULTS.md` (the table above), the A6 table, every marks answer, and the relay
folders whole, as R1's "Keep" says. A captured file that holds the company number, or a digit run of 10 or more outside VOUCHERKEY,
stays on the lab PC, listed by size and sha256, as R1's results did. Screens stay on the lab PC, listed by name and sha256. Send the
SHA to the run coordinator before pushing.

Restore: none. Keep THE COMPANY as it stands after Phase B. `BK-C16` holds it as R1 read it, with 34 vouchers.

## Row C16-R: one voucher keyed again, so its window can be pushed (WRITE, then READ-ONLY)
Added 4 Nov 2026, after the row above ran (`lab/699-c16-answers` at 491b4ce4). Run it after row 2057-C, on the run coordinator's
word. Everything above still applies, except as said here.

**Why.** Phase B's answer to the 1-Feb-2026 to 31-Mar-2026 voucher window (`relay/c2/0033`, and its copies in c3) stays on the lab
PC: one NAME element in it has a GUID-shaped value with a run of 10 digits. That window holds C15 to C19, which the fixtures need.
- Tally made that value when the voucher was keyed. Keying the voucher again gives it a new one.
- Keying again changes the book's marks and census. The whole audit read is therefore made again (R5), so that every fixture comes
  from one read.

R0. Load THE COMPANY alone (C1), then **r0 marks**: EXPECT one COMPANY row with this GUID, byte-identical to the a6 and b9 answers
    (sha256 dda4ec59f9d5139eb1403d37b989e60c973da1b16556f443ba2e6a0e00fc5455; V 53, M 230). Anything else: STOP.

R1. **Name the voucher** from the kept file `relay/c2/0033-response.bin`, by structure only:
    - find the VOUCHER element that holds the NAME element with the GUID-shaped value, by parsing the file, not by a text search;
    - record that voucher's VOUCHERTYPENAME, VOUCHERNUMBER and DATE, and the name of the list element directly above that NAME;
    - record nothing else from it, and never the value.

    EXPECT exactly one such voucher. None, or more than one: STOP and report the count.

R2. **Back up** THE COMPANY by C5 into the new empty local folder `BK-C16R`. Record the file's name, bytes and sha256, and the time the
    Restore list will show. Then the marks: EXPECT them byte-identical to r0's.

R3. **Delete that voucher alone.**
    - Open it from the Day Book for its date (Display More Reports, Day Book, F2 to that date), Enter on its row, then Alt+D.
    - Tally's delete confirmation for that voucher is the one prompt this step names. Copy its text exactly, screenshot it, answer
      Yes.
    - Then screenshot the Day Book for that date, showing the voucher gone and every other voucher of that date still there.
    - Then **r3 marks**. Record V and M; they are recorded, not predicted.

R4. **Key it again by hand**, on its own date: the same type, ledgers, amounts, sides and narration as it had.
    - For C1 to C19, these are in the A5 table above and `lab/699-c16-answers` `evidence/a6-table.tsv`.
    - For an earlier voucher, they are in `lab/699-sft-lab-answers` A5-VOUCHERS.md.
    - Bank Allocations are accepted as offered, as in A5. The voucher number is the one Tally offers; record it.
    - Tally's prefills are met as in the earlier sitting (Enter on the To/By selector, then the ledger name and amount typed over the
      prefill).
    - If Tally will not take the voucher as it was, accept nothing: screenshot, then Ctrl+Q after a screenshot showing the screen's
      title. If Tally then asks whether to quit the voucher, that prompt is named here: copy its text, screenshot it, and answer so
      that nothing is saved (Yes to quit). Record "not keyed again", then restore `BK-C16R` (C6, on the run coordinator's word), take
      the marks (EXPECT them byte-identical to R2's), and the row's answer is "not cleared".

    Then **r4 marks**. Record V and M. EXPECT the answer to differ from r3's.

R5. **The read again.** Phase B of this sheet once more, with these differences:
    - the relay folders are `r5-list`, `r5` and `r5v` in place of `c1`, `c2` and `c3`, each new and empty;
    - the relay's `--ledger-name` is `"Unused 699 C16R"`;
    - the marks that open and close the read are the r4 marks, and the closing read EXPECTs an answer byte-identical to r4's;
    - the vouchers count EXPECTs 53, with the types and dates on record, the voucher of R1 under the number R4 records.

    The build and the tool call are Phase B's.

R6. **The check.** In `r5`, take the POST whose request is byte-identical to `relay/c2/0033-request.bin` (sha256
    26933e853602a146e3fab3bb449b0a45ea8bb14f27782265bc1f3dfa80abe60a); none, or one whose request differs: STOP.
    - Decode its answer and count every run of 10 or more digits by the keep rule that sorted `lab/699-c16-answers`. The same two
      places are exempt: inside a whole 64-hex or 40-hex hash value, and the whole value of a VOUCHERKEY element. Use the same
      script.
    - Record the count, and for each run the name of the element holding it, never its value.
    - Then sort every other file of `r5` and `r5v` by the keep rule, as before.
    - **EXPECT: no run in that answer.** If one remains: restore `BK-C16R` (C6, on the run coordinator's word), take the marks
      (EXPECT them byte-identical to R2's), and the row's answer is "not cleared".

**What to report** (first line: any STOP, any EXPECT not met, and "cleared" or "not cleared"):

| step | result |
|---|---|
| r0 marks | |
| R1: the voucher's type, number and date; the list element above the NAME | |
| R2: the backup's name, bytes, sha256, time; its marks | |
| R3: the prompt's text; the Day Book screenshot's name; r3 marks | |
| R4: the number Tally offered; every line as keyed again; r4 marks | |
| R5: everything Phase B reports, for `r5` and `r5v`, and the closing marks against r4 | |
| R6: the request's sha256; the digit-run count and the elements holding any; how each file of `r5` and `r5v` was sorted | |

**Evidence.** Forward commit on `lab/699-c16-answers`: the folders `r5` and `r5v` and the marks answers, under the keep rule as
before. Add the new files to `SHA256.txt`. Send the SHA to the run coordinator before pushing. If cleared, THE COMPANY stays as it
stands after R5, and `BK-C16R` holds it as Phase B read it.
