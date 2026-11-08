# A per-book invoice-number index for #2162: design note

Not for merge. Part of #2162. Written 8 Nov 2026. Lines below are at master 9a4cf1f7. Nothing here is built. Every claim is marked **measured** (with its source) or **derived**. Every rule that rests on a derived claim waits for the lab row named under "Lab rows before code" (P1, P6).

## Why

The number rule (adopted 4 Nov) needs every Sales-class number of the financial year, folded (case and the leading zeros of each digit run ignored). The month reads chosen after rows S5 to S7 send twelve Y0 month reads, paired, for **every** invoice. The queue sends them again before the post (`TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md:1486`).

An index built once per company and year, then kept fresh against Tally's own marks, reads the year once instead of once per invoice. The month reads stay the fallback: anything the index cannot prove fresh falls back to them, or is refused.

## 1. The index

- **Content.** A map from a folded number to every Sales-class row holding it: the voucher's number as Tally returned it, its date, its type name, its `ALTERID`, and whether it carries an IRN (`sales_holder_has_irn`, `agent_import_invoice_wire.rs`).
  - Non-Sales rows are read, because Y0 returns every type, and they are dropped.
  - A row whose class does not read refuses the build (`invoice_number_class_unread`).
- **Built by** the twelve Y0 month reads of the financial year: Y0 as built, the read today less its number term, through the one dispatch path (P8).
- **Held** in the agent server's memory, keyed by company GUID and financial year, for the life of the process.
  - It is not written to the private agent folder. The numbers are customer data, and a cache at rest would be one more copy to protect, redact and expire.
  - A restart costs one rebuild (section 4).
- **Invalid, so the index is dropped and rebuilt, or the build is refused:**
  - the process restarts;
  - a marks read returns another GUID, or no GUID (`parse_company_high_water`, `agent_company_checkpoint.rs:5`);
  - the company is not the one loaded, by the existing identity brackets;
  - the invoice's date falls outside the index's financial year; that year has an index of its own;
  - any freshness check in section 2 fails;
  - any of the twelve month reads was refused, timed out, or did not parse. There is no partial index.

## 2. Freshness, the core

Each check reads Tally's marks with the existing request: `render_agent_company_high_water` (`agent_read_profiles.rs:574`), parsed by `parse_company_high_water`. That gives `V` (ALTVCHID) and `M` (ALTMSTID).

**Why V says enough.** Measured on gateway writes: creating, altering, cancelling or re-dating a voucher each moves `V` by 1, and deleting one moves it by 2 (§11c.5's table, the "`ALTVCHID` on each change" row). `V` is a monotonic high-water mark (§10, VERIFIED). Screen edits were measured once (§11c.5, 2026-09-24, PARTIAL).

**Why M is needed too.** A voucher's class is `$$IsSales` of its voucher type. Altering a type's parent could change the class of every voucher of that type without moving `V`. That is derived, and lab row L2 measures it. So the index requires `M` unchanged as well.

**Types (P2, P3).** The trap is made unrepresentable rather than left to a rule:

| type | constructed only by | what it proves |
|---|---|---|
| `IndexMarks { guid, v, m }` | a parsed marks answer whose GUID is the verified company's | the book's state at one moment |
| `NumberIndex` | `NumberIndex::build(before: IndexMarks, months: [MonthAnswer; 12], after: IndexMarks)` | `before == after`, so no change during the build; the twelve months tile the year |
| `FreshIndex<'a>` | `NumberIndex::fresh(&'a self, now: IndexMarks) -> Result<FreshIndex<'a>, IndexStale>`, only when `now == built` (GUID, V and M) | nothing that moves V or M has happened since the build |
| `IndexStale` | a typed refusal; never a user-facing answer on its own | the index is dropped and rebuilt once. A second failure refuses the build as `invoice_number_index_stale` |

The admission's number check takes only a `FreshIndex`, never a `NumberIndex`. So a check against a stale index does not compile.

**After this build's own posts.**
- `NumberIndex::after_own_post(self, post: VerifiedPost, now: IndexMarks) -> Result<NumberIndex, IndexStale>` may update the index locally, adding the posted invoice's folded number, date, type and AlterID, only when all of these hold:
  - `post` is `posted_verified`: its read-back matched as built;
  - `now.guid == built.guid` and `now.m == built.m`;
  - `now.v == built.v + post.step`, where `step` is the voucher step the post path already reports (`target_voucher_step`).
- **What proves nothing else changed.** Every other voucher change moves `V` by at least 1 (§11c.5). Exact equality after adding this post's own step therefore leaves no room for another change.
- The post's own step is measured through Bridge's post path for 50 and 200 vouchers (§11c.5, 2026-09-26, VERIFIED). It is not yet measured for one invoice with a Sales item line, so that waits on lab row L1.
- Any other `V` or `M` means a rebuild.

**What the index can and cannot get wrong.**
- **Deletion.** A deleted voucher has no AlterID to find (§10, "Known gap — deletions"). Its number stays in the index until a rebuild. That refuses a reuse of the number, which is the safe side, never an admission.
- **Renumbering.** A renumbered voucher moves `V`, which forces a rebuild.
- **Auto Renumber series.** Numbers that change behind the index are already refused at the build (`import_voucher_numbering_not_retain`, `agent_import_numbering.rs`).
- **A screen action that moved neither mark** (§11c.5, the 25 Oct GST status prompt) is recorded as changing GST status, not numbers. It is the reason lab row L1 checks every number- or class-changing screen action.

**No answer is never "unchanged" (P5).** A marks read that fails, times out, returns no `COMPANY` row or omits `ALTVCHID` is a typed failure. It never counts as a match.

## 3. The pair and the queue re-read

**The pair.**
- `post_admitted_read` (`agent.rs:2880`) sends every admitted read twice and compares the two answers. That catches a change during the read and an answer that differs on resend.
- Bracketing one send with marks before and after catches every change that moves V or M during it, at about 0.01 s per marks read (measured, S6 m1 and m2).
- It does not catch an answer Tally renders differently on resend with no change. The evidence against that, measured: S6's two year sends and S7's four month sends were each byte-equal.
- **So yes, for the index build only, if the maintainers rule it.** The pair is a project-wide invariant, not this design's to drop. Until a ruling, the build keeps the pair, and section 4 counts both ways.

**The queue's second full check** (`VOUCHER_WRITES.md:1486`).
- The queue sends the admission's reads again, the number and its control included.
- With the index, the number part becomes one marks read: `fresh(now)` on the index the admission used. If it holds, the number is still absent, by section 2's argument.
- The gap the bullet records is unchanged: "a number taken after the queue's number read" is caught only by the voucher type's own duplicate guard. The marks read narrows that window; it does not close it.
- The other re-reads in the queue (compliance listing, rates, types, registration) are masters, already covered by `post_masters_moved`. This design does not change them.

## 4. Cost

| step | dense book (29,900 Sales a year) | a typical book (derived: about 200 vouchers a month) |
|---|---|---|
| marks read | about 0.01 s (measured, S6 m1 and m2) | the same (derived) |
| index build: 12 months × 2 sends | 36 to 48 s at S7's 1.5 to 2.0 s a month; 200 s at S6's 8.38 s (both derived from measured single months; the spread between S6 and S7 is unexplained) | under 17 s (derived from 0.18 to 0.69 s for an 82-row day, S6 s1 and s2) |
| the same with the marks bracket in place of the pair | about half of the line above (derived) | about half (derived) |
| first invoice of the session | the build, plus the admission's other reads | the same |
| a later invoice, V and M unchanged | 2 marks reads, about 0.02 s, plus the admission's other reads (derived) | the same |
| a later invoice after this session's own verified post | 1 marks read and a local update (derived) | the same |
| a later invoice after anyone else's change | a rebuild: the build's figure again | the same |
| the queue's number re-read | 1 marks read in place of the month reads (derived) | the same |
| **today's read, for comparison** | N over the year, 24.81 s: past the 20 s limit, so refused (measured, S6 s5) | not measured |

Part C of S7 is running. Its twelve-month sums replace this table's derived build figures when it reports.

## 5. Lab rows before code (synthetic books only: `BRIDGE PILOT LAB`, `BRIDGE CORPUS DENSE` or a new synthetic company)

- **L1, V step per action.**
  - On one Sales voucher, record V and M before and after each of these, one at a time: a gateway create of one invoice through the build's own request; and on the screen, an alteration that keeps the number, a change of number, a cancel, a delete, and a re-date.
  - EXPECT: each moves V by at least 1, and the create by exactly the step the post path reports.
  - Decides whether `after_own_post`'s equality proves what section 2 says.
- **L2, a class change.**
  - Alter a Sales-class voucher type's parent. Record V and M, and read Y0 for a day holding one of its vouchers, before and after.
  - Decides whether M must be unchanged (expected), or whether V alone would do.
- **L3, the AlterID delta.**
  - Send Y0 with ` AND $AlterID > K` added to its filter, for K below the mark, on the mixed pilot book and on the dense book. Compare its rows element for element with Y0's rows whose ALTERID exceeds K, and time it against Y0's month.
  - `$AlterID > N` filters at the server on its own (§10.1, VERIFIED). Combined with Y0's date formula it is a new shape, so it is a candidate builder (P7).
  - If it serves, it could replace a rebuild after a small change. This design does not depend on it.
- **L4, persistence across reloads.**
  - Shut and reload the company, and restart Tally, with marks read before and after.
  - S6's m1 equalled the previous row's m1 across sittings (measured), but a reload inside one sitting is not measured.
  - Decides whether the index may survive a company reload inside one process. The design assumes not: a reload is treated as invalid until L4 says otherwise.
- **L5, the pair.**
  - Only if the maintainers want the marks bracket to replace the pair: send one dense month twelve times, marks bracketed, and confirm the answers are byte-equal.
  - S6 and S7 each give two to four such sends already.

## Not decided here

- The refusal codes' final names, and the remedy text.
- Whether the index also serves the post-check (`folded_number`, `agent_import_verification.rs`). It reads the same rows.
- The pair ruling (section 3).

## Deviation, 8 Nov 2026: the row's AlterID

Section 1 lists the row's `ALTERID` in the index. Y0's fetch list carries none, so that field needs the one-field change of row L3a (`lab/2162-ix/ROW.md` on `lab/2162-index-sheet`, where this note was first written). Freshness rests on the company marks, not on row AlterIDs, so the rest stands.

## Deviation, 8 Nov 2026: a catch-up read is costly on a dense book

**Measured** (the index sheet, Parts 2 and 3, 8 Nov 2026):
- L3a: Y0 with `ALTERID` added to its fetch list answers, 41 rows on the pilot book.
- L3b on the pilot book, K 58: 5 rows, equal to L3a's rows above K, in 0.075 s.
- L3b on the dense book over its year, K 29800: 100 rows, each found in S6's year read, in 15.08 s and 292,626 bytes. The full-year read took 22.66 s in the same sitting. So the delta term saves about a third, not the read: Tally still walks the year.
- L4: V and M byte-equal across Shut Company and reload, and across Quit and restart.
- L5: twelve April reads equal, with the marks stable between them.

**What changes.**
- Section 4's 0.02 s figures stand. They are the marks reads, and the local update after this session's own verified post (section 2). Neither sends a delta read.
- A catch-up after anyone else's change is not cheap on a dense book: about 15 s, against about 22 s for a rebuild. Section 5's L3 note ("it could replace a rebuild after a small change") is therefore worth little there. The design keeps the rebuild as the path after a foreign change, and the delta read stays optional.
- **Unverified:** the cost scales with the date window the read walks, not with the number of rows changed. One dense measure cannot show that.

**Would the changed vouchers' month make it cheap?** Not as a catch-up. The marks say that something changed, not where. The month of a foreign change is not known until a read finds it. Twelve month-window delta reads walk the same year, so they are expected to cost about the same as one year read (derived). A month window helps only when the month is known in advance, and an admission's number check needs the whole year, so no such case is known in this design. A later lab row could measure the delta term on one dense month against that month's plain Y0 read. That would show whether the cost follows the window (unverified). Nothing here changes code, and nothing waits on that row.

## Deviation, 8 Nov 2026: screen actions and the marks (the index sheet, Part 1)

**Measured** (`lab/2162-index-answers` at 2cf5df51, RESULTS.md, on BRIDGE BILLTRAIL LAB, a synthetic book, in a voucher type `IX SALES` created under Sales with manual numbering):
- k1 to k6 are a create, a narration, a number change, a re-date, a cancel and a delete, each made on Tally's screen. Each moved V (`ALTVCHID`) by exactly 1 and M (`ALTMSTID`) by 0. k7, a second create, did the same.
- Creating the voucher type `IX SALES` moved M by 2 and V by 0.
- k8: in Voucher Type Alteration, Return and Shift+Tab both pass over the type's parent field without a message. So the parent could not be changed, and the attempt moved neither mark.
- A delete on the screen (k6) moved V by 1. A delete through the Gateway moved it by 2 (§11c.5).
- The restore of the sitting's backup returned both marks to `bm0`, and the day's read found no `IX-` row.

**What changes.**
- Section 2's equality check is sound for these six screen actions, in a child type of Sales: each moves V, so an index whose stored V equals the book's has seen none of them since. The check needs V to move, not to move by a set amount, so the screen delete's +1 against the Gateway's +2 changes nothing.
- A master change (a new voucher type) moves M and not V. So a freshness check that compares only V misses a type change. Section 2 compares both marks, and that stands.
- **Unverified:** whether changing a voucher type's parent, and so its class, moves a mark. This release gave no way to make that change on the screen. Until a later row measures it, nothing in this design rests on it, and section 5's row for it stays open.
