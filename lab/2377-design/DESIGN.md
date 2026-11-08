# Design note: one party's open bills through Tally's Ledger Outstandings report (#2377)

Not for merge. Part of #2377. Lines are cited at master 76705cfb. Written 8 Nov 2026. Nothing here is built, and nothing here has been sent to Tally beyond the two lab rows it cites. Confidence follows `docs/tally/TALLY_PROTOCOL_REFERENCE.md`: each fact is marked measured (with its row), inferred, or unverified.

## Why

On a large book at a year-end date, `outstandings` is refused at 20 s with `native_bills_report_deadline_exceeded` (`agent.rs:2558-2568`). Every call reads the whole Bills Receivable and Bills Payable reports, each as a pair of posts (`tally/runtime.rs:4144-4147`, `:4199-4202`). A party detail runs that same read first. The Claude desktop app on Windows gives one tool call about 60 s (#2119 row H), so a longer request limit cannot help (#2158, closed). A read of one party's bills is small: 248 bills answered in under 2 s on the dense book (#2119, 4 Nov).

## 1. The request shape

**The shape.** It is the Ledger Outstandings export first sent on 27 Oct, in `lab/1356-party-term-and-ledger-outstandings-run-sheet`, `requests/r04_ledger_outstandings_p1_to_20260331.request.utf16le.xml` (answers at `b761d18e`):

```xml
<ENVELOPE><HEADER><VERSION>1</VERSION><TALLYREQUEST>Export</TALLYREQUEST><TYPE>Data</TYPE>
<ID>Ledger Outstandings</ID></HEADER><BODY><DESC><STATICVARIABLES>
<SVCURRENTCOMPANY>{company}</SVCURRENTCOMPANY><LedgerName>{party}</LedgerName>
<SVTODATE TYPE="Date">{to}</SVTODATE><SVEXPORTFORMAT>$$SysName:XML</SVEXPORTFORMAT>
<EXPLODEFLAG>Yes</EXPLODEFLAG></STATICVARIABLES></DESC></BODY></ENVELOPE>
```

**What a builder fixes in code.**
- The report ID, the static variable names, `SVEXPORTFORMAT` and `EXPLODEFLAG`.
- No TDL and no `$$` function built from a value.
- Three values only: the company, the party and the to-date.
  - The company and the party are XML text, escaped once, as `render_native_bills_request` escapes the company (`native_outstandings/request.rs:131-140`).
  - The to-date is a `TallyDate`.
- **The party value is the book's own ledger `NAME`**, read in the same call (section 2), never the user's spelling.

**What is measured.**
- **Small book, 27 Oct, one party** (`b761d18e`, RESULTS.md "Capture 2"; partial):
  - two sends to 31 Mar 2026 were byte-equal, and one to 31 Jul 2025 answered as well;
  - the answer is an `ENVELOPE` of `BILLFIXED` groups (`BILLDATE`, `BILLREF`), each followed by `BILLOP`, `BILLCL`, `BILLDUE`, `BILLOVERDUE` and its voucher lines;
  - a debit bill is negative and a credit bill positive;
  - an On Account row came as a `BILLFIXED` whose `BILLREF` is `On Account`;
  - every bill equals Tally's Ledger Outstandings screen.
- **Dense book, 4 Nov, one party** (`lab/2119-l-answers` at `c6899415`, RESULTS.md; partial):
  - 248 bills, sent twice, 1.667 s and 1.586 s, 205,336 bytes each, byte-equal;
  - every bill equals the party's rows in the Bills Receivable report of the same sitting, by reference and date;
  - the total and ten printed rows equal the screen;
  - that party had no credit bill and no On Account row.
- **Not measured:** how Tally treats a `LedgerName` that names no ledger; how it treats a value holding characters such as `&`, `<`, `"`, `$`, `#`, `:`, a doubled or trailing space, or non-ASCII text; a party with no open bills; a payable party; bill-wise off; a foreign-currency ledger.

**Under P7, until section 4's rows pass:**
- The builder starts as a candidate.
- It admits a party name only from a closed alphabet: letters, digits, single inner spaces, and `. , - ( ) /`.
- It refuses any other name before anything is sent, as `party_name_not_admitted`.
- The alphabet widens only by a lab row per character class.

## 2. What a party-detail call sends, and what it refuses

**The surface.** `outstandings` with `party` and a new `detail` value, `open_bills`.
- Today `party` without `detail` is refused (`party_requires_detail`, `agent_bill_trail.rs:736`), and that stays.
- `bill_trail` and `unadjusted` keep their present path: a whole-book read, then a voucher read from the start of the books. They are out of this slice, because their voucher read is a cost of its own on a large book.

**The sequence for `open_bills`, all through the one dispatch path** (P8):

1. The read boundary and the company identity bracket, as `outstandings` opens today (`tally/runtime.rs:4107-4111`).
2. **The party resolved from the book.**
   - The ledger catalogue is read, and the user's spelling is matched under the existing party rule: exact, or differing only in ASCII case and spaces (`agent_catalog.rs:545`).
   - None matches: `party_not_in_book`.
   - More than one matches: `party_ambiguous`, listing the book's names.
   - A ledger with bill-wise off: `party_not_billwise`.
   - A foreign-currency ledger: refused as `outstandings` refuses one today.
   - The outcome is a `ResolvedParty` holding the ledger's `NAME` and GUID. It is the only input the request builder takes.
3. **The ledger snapshot for that one ledger.** It uses `render_native_ledger_snapshot_request_for_ledgers` with its GUID filter (`native_outstandings/request.rs:186-194`, `:222-225`), over the same period as the outstandings read. That gives its closing balance at the to-date.
4. **The Ledger Outstandings request, as a pair.** It goes through the paired read the Bills reports use (`fetch_native_report_paired`). Two answers that differ are `party_bills_drifted`, as `native_bills_report_drifted` is today.
5. **The identity bracket closes, and the company's marks are compared.** A rename or an alteration between the reads refuses the call, as the party detail does today (`agent_outstandings.rs:43-56`).
6. **The tie** (section 3). Only a tied read is answered.

**What it never does.**
- It never reads the whole Bills reports.
- It never computes a whole-book total.
- It never answers a refused case as complete.

## 3. "Nothing found" against "the read failed"

An `ENVELOPE` with no `BILLFIXED` is ambiguous until proved otherwise. It may mean any of these:
- the party has no open bills;
- Tally matched no ledger by that name (not measured, section 4);
- Tally answered for another company.

So an empty answer is never taken as zero bills on its own.

**Failures of the read are typed before any bill is counted:**
- a transport error: a timeout, `ResponseTooLarge`, `ResponseTruncated`, or a refused connection;
- an HTTP status other than 200;
- an answer that does not parse as the envelope above;
- a Tally error line in the answer.

Each is a refusal with its own cause and the request that failed, as `party_ledger_master_read_failed` names its request today. None of them becomes "no bills".

**Answers are tied before they are told.** The build has three types:
- `PartyBillsRead::Tied { bills, on_account }` is constructible only when Σ `BILLCL` over the answer's rows, On Account included, equals the ledger snapshot's closing balance at the same to-date, with the sign the rows measure;
- `PartyBillsRead::NoOpenBills` is constructible only from an answer with no `BILLFIXED` and a snapshot whose closing balance is zero;
- `PartyBillsRefusal::Untied { bills_total, ledger_closing }` covers every other case. That includes an empty answer for a party whose ledger closes non-zero, which is how an unmatched name would show.

The handler takes only a `PartyBillsRead`. A refusal cannot reach the figures (P2, P3).

**What the tie rests on is unverified.**
- The equality is inferred from the Bills reports' own residual rule (`native_outstandings/request.rs:158-165`).
- It has not been measured for this report.
- This report takes only a to-date, so the snapshot's period start (the books' first day) is an assumption to check.
- An On Account row's `BILLREF` is the text `On Account`. A real bill with that reference would be read as On Account; a row in section 4 checks how Tally tells them apart.

## 4. Lab rows needed before any code

One variable a row (P6), on synthetic books, each with a backup and a restore. Reads only, by fixed request files, with answers kept whole. Every row also records the company's marks before and after.

| row | book | what it measures |
|---|---|---|
| L1 | a second book: BRIDGE BILLTRAIL LAB (small) | a party with debit and credit bills and an On Account row: Σ `BILLCL` against the ledger's closing balance from the ledger snapshot, at two to-dates |
| L2 | the same | a party ledger with bill-wise on and **no open bills** (all settled), and its closing balance |
| L3 | the same | a `LedgerName` that names **no ledger**: the answer's bytes, an error line or an empty envelope, and `RESPSTATUS` |
| L4 | the same | a `LedgerName` that differs from the ledger's name only in case, and only in a doubled space: same bills, or none |
| L5 | the same, with lab-made parties | names with **special characters**, one class a send: `&`, `<` and `>` (escaped), `"` and `'`, `$` and `$$`, `#` and `:`, a non-ASCII name, a trailing space |
| L6 | the same | a payable party, and a debtor whose advance exceeds its bills |
| L7 | the same | a real bill whose reference is the text `On Account` |
| L8 | BRIDGE CORPUS DENSE | the dense book's three largest parties: whole time and bytes for each, under 20 s each |

**The stop rule.** A row whose answer is not whole, or whose marks move, is void and is repeated. The rows' answers become the build's fixtures (P1).

**What would change this design.**
- **L3 is the deciding row.** If Tally answers an unknown name with an error line, that error becomes a typed refusal of its own. If it answers with an empty envelope, the tie of section 3 is the only guard.
- **L1 failing** (the tie does not hold) would mean the build cannot release figures. It would then refuse, not guess.
- **L5** sets the closed alphabet. A class that Tally reads as anything other than text stays refused.

## The three questions

1. **What existing component could do this:** the paired read and the dispatch path, `xml_escape`, the identity bracket and marks check, the ledger catalogue and its party rule, the GUID-filtered ledger snapshot, and the native outstandings date parser.
2. **What is deleted:** for a party's open bills, the whole-book Bills read that `bill_trail` and `unadjusted` wait on is not sent. Nothing in code is deleted in this slice.
3. **What breaks if it is not built:** on a large book at a year-end date, no party's outstanding can be read through Bridge. Master's 20 s refusal stays the known limit.
