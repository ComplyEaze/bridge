# Windows item GS2-L3 v1: what Tally answers ComplyEaze Bridge's own reads for a company that has never held a voucher. (26 Oct 2026.) SYNTHETIC ONLY: THE COMPANY is `BRIDGE FIRST VOUCHER LAB`, created by the owner in step 1 of this item. READ-ONLY after the owner's setup: no voucher is keyed or posted, nothing is imported. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds.

Why: issue 1709 asks ComplyEaze Bridge to post the first voucher into a company that has never held one (the owner's answer of 15 Oct: yes, after one lab sitting on a synthetic company). Bridge refuses today (`empty_book_first_import`) because it reads no voucher mark for such a company. The design that survived review admits the first post only when the company's voucher-mark element is missing in every one of Bridge's existing marks reads and the masters' mark is present, and it records "no mark", never a zero. Before any code, this item captures, on the lab's release, what those reads actually return for an unused company that holds only masters (the normal state of a new client): whether the element is absent, present as 0 or present empty, whether that holds across repeated calls and a reload, and what the book-extent and window reads return. The captures become the fixtures the design is tested on (AGENTS.md P1). The post itself is a later item.

## The build, the relay and the calls
- The `bridge_mcp` program the lab config's line `bridge-lab-gs1` runs (GS1-L1's build of pull request 1867; its sha256 as GS1-L1's go-ahead named; another hash = STOP). Posting and ledger creation OFF. One ComplyEaze Bridge connected, under that server name; one call at a time; each answer kept whole by the assistant with its size and sha256; a call in flight is never cancelled, abandoned or repeated.
- The relay `lab/gs1-relay` at `e5dffc2b`, checked against its SHA256.txt and self-tested as its README says, records every call of steps 3 to 6 into `C:\bridge-lab\gs2-l3\relay\` (a new folder), as in GS1-L1 run-3. A request the relay refuses = the call fails = STOP, folder kept.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there.
- C2 (d) and C3: the operator sends NO request of his own in this item (no MARKS file: the lab's MARKS file names another company in its header, and a changed copy would be a new request shape). Step 0's company check is the Gateway screenshot and Select Company's list; every mark is read by ComplyEaze Bridge's own call, recorded by the relay.
- C5 and C6: no backup and no restore; THE COMPANY is this item's own and is kept, shut, for the later post item.
- OWNER STEPS: the company and its two ledgers are created by the owner on Tally's own screens, one act at a time, with a screenshot before accepting each.

## Steps
0. LAB-COMMON step 0 with NO company loaded (a loaded company = shut nothing, STOP), without its MARKS read (see Differences). Tally's About screen: record the release (7.2 expected; record whatever it is). EXPECT no company named `BRIDGE FIRST VOUCHER LAB` in Select Company's list (one present = STOP).
1. OWNER: Create Company: name `BRIDGE FIRST VOUCHER LAB`, financial year beginning 1-4-2025, books beginning 1-4-2025, no password, every other field as the screen offers; if the Features screen opens, accept it unchanged and record what it shows. Screenshot before accepting. Load it alone. Record the company number from Select Company's list.
2. OWNER, one master at a time, each with a screenshot before accepting: ledger `SYN FV Rent` under Indirect Expenses; ledger `SYN FV Supplier` under Sundry Creditors (bill-by-bill as the screen offers, no GSTIN). No voucher of any kind is opened or keyed.
3. Relay on (as above). Call `list_companies`. EXPECT one company, THE COMPANY; record its `company_guid`.
4. Call `purchase_register` with `company_guid`, `from` 2025-04-01, `to` 2025-04-30. RECORD the answer whole (expected: `state` complete or a refusal; either is a result; a refusal is recorded with its code, not a STOP).
5. Call `purchase_register` again with the same arguments (the omission's persistence within a session).
6. Shut THE COMPANY (Alt+F3 or the screen's Shut Company), load it again alone, screenshot the Gateway, and call `purchase_register` a third time with the same arguments (persistence across a reload). Relay off (Ctrl+C), port back to 9001, server restarted.
7. LAB-COMMON C10. THE COMPANY is kept, shut, as it is: its two ledgers and no voucher.

## What is recorded (nothing is judged at the lab)
- The relay's recording folder whole with `manifest.tsv` and its SHA256 list: in particular every company-marks answer (`Bridge Agent Company High Water`), every book-extent answer and every voucher-window answer of steps 4 to 6.
- The three tool answers and the list_companies answer, whole.
- The company number, the release, the screenshots by name, size and sha256 (never pushed), the notes file.
GS2 reads the recorded answers (parsed, never grepped) for: whether `ALTVCHID` is absent, present as 0 or present empty in THE COMPANY's row of every marks answer; `ALTMSTID` in each; and what the book-extent and window answers hold. The lab commits the evidence and, on GS2's fixture specification, the bodies to become fixtures.
