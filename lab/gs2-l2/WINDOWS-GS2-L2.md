# Windows item GS2-L2 v1: how much of a real book's GST the per-ledger read sets aside, by reason. (26 Oct 2026.) READ-ONLY. THE BOOK is one client book from the lab's approved list of client books (owner's approval of 5 Oct 2026; never REGISTER LAB), chosen by the run coordinator for a month with many purchases and expense journals; its name, number and GUID go only in the private notes, never in a commit, a branch, a file name or any public text. LAB-COMMON v3 applies in full; where an item and LAB-COMMON differ, the stricter holds. No request writes; nothing is keyed.

Why: `gst_by_counter_ledger` (pull request 1867) attributes a voucher's GST to its one expense or purchase ledger and sets every other voucher aside under a named reason. On a synthetic book every shape it meets was keyed to test it. Whether it is useful for a CA's GSTR-9C Table 14 working depends on one number no synthetic book can give: on a real book, what share of the debit-side GST lands on a counter ledger, and what share is set aside, under which reasons (for example multi-line bills under `several_counter_ledgers`, item invoices, untyped duty ledgers). If most of it is set aside, the tool is not worth merging as it stands, and the reasons say what to change. This item measures that share, and nothing else.

## The build and its settings
- The `bridge_mcp` program of pull request 1867 at the head the run coordinator's go-ahead names, from that head's own CI run (the go-ahead names the program file's sha256; a file with another hash is not started). Posting and ledger creation OFF. `BRIDGE_AGENT_REDACTION=mask_parties`, so every ledger name in an answer is masked. Port 9001, no relay (no client byte is recorded or kept as a fixture).
- Exactly ONE ComplyEaze Bridge of this build is connected to the assistant, under the server name `bridge-lab-gs2`; every tool call goes to it with exactly the arguments the step gives, one call at a time; the assistant keeps each answer whole as a file in the evidence folder with its size and sha256. A call in flight is never cancelled, abandoned or repeated.

## Differences from LAB-COMMON
- C1: THE BOOK is a client book (owner's approval of 5 Oct 2026, read-only here). It is loaded alone.
- C3: the operator's only request is `requests/marks.utf16le.xml` (GS1-L1's MARKS file, byte for byte), size bound 8,192 bytes. A tool answer above 1,000,000 bytes = STOP.
- C5 and C6: no backup and no restore (nothing is written).
- EVIDENCE: the tool answers and MARKS answers stay on the lab PC, in the private notes' folder, and are deleted when the run's review ends; nothing from THE BOOK is committed except the aggregate table of "What is committed" below.

## Requests (in requests/; bytes and sha256 in SHA256.txt)
| file | what | the change from its source |
|---|---|---|
| marks.utf16le.xml | MARKS of the loaded company | none: GS1-L1's file, byte for byte |
Before step 0: `python -I check-request-ids.py requests`, EXPECT `census: ok 1, skipped 0, FAIL 0`.

## Steps
0. Load THE BOOK alone (C1), then LAB-COMMON step 0. MARKS: record both marks and THE BOOK's GUID in the private notes.
1. `list_companies`: EXPECT THE BOOK as the one company; record its `company_guid`.
2. Size, one day: `gst_by_counter_ledger` with `company_guid`, `from` = `to` = the first day of the month the run coordinator chose. RECORD `state`, `vouchers_observed` and any refusal code. A refusal for size (for example a book or window too large) = record its code and STOP: the finding is that the tool cannot read this book at all. Any other refusal = record it and STOP.
3. The first week of that month (days 1 to 7), then the whole month, each one call with `from` and `to` as said. After each, MARKS (EXPECT both marks unchanged: the tool writes nothing; a change means someone else wrote: record it, the totals of that call are void, STOP). A size refusal on the month = record it and use the week as the result.
4. LAB-COMMON C10.

## What is committed (the only output that leaves the lab PC)
One table, made by the operator from the last complete answer (the month, or the week if the month was refused), with no ledger name, no voucher, no date and no amount:
- `vouchers_observed`, `vouchers_without_gst_entry`, `vouchers_cancelled_optional_or_post_dated` (counts);
- the number of counter ledgers, and their total `vouchers`;
- per `set_aside` reason: its `vouchers` count;
- the SHARE of debit-side GST, all heads added, that is attributed to counter ledgers and that is set aside, each in whole percent, and per set-aside reason its share in whole percent; the same for credit-side GST;
- the window's length in days and whether it was the month or the week.
Rounded shares only: no rupee figure from THE BOOK appears in the table.

## Pass / finding
There is no pass or fail on THE BOOK's data: the table is the result. A refusal or a moved mark is recorded as its own finding.
