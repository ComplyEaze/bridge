# Row 1029-P: does `voucher_presence` answer present, present with a difference, and absent on a book of about 30,000 vouchers, through a voucher type the book numbers Manual? SYNTHETIC COMPANY ONLY. Not for merge.

Part of #1029, row A's presence half. Written 5 Nov 2026. Lines below are at 187e882b, the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
On BRIDGE CORPUS DENSE, with two vouchers keyed under a new voucher type numbered Manual with duplicates prevented:
1. Is a proposal equal to a keyed voucher answered `present`, on the basis of its manual number, with no difference?
2. Is a proposal with that number but another amount answered `present` with exactly one difference, the amount?
3. Is a proposal with a number the book does not hold answered `absent`?
4. Is a type declared `manual` that the book numbers automatic refused before the window is read?

## Why
- **What #1029 still lacks.** `present`, and `present` with a difference, have never been seen live. The check of a declared-manual type against the book (bridge#1691) has never run on a book of this size.
- **Why keying is needed.** Row 1029-T (4 Nov, `lab/1029-t-answers`) read every one of this book's 24 voucher types as automatic with retain, duplicates not prevented. The tool decides nothing from a number under automatic, so the book as it stands cannot answer 1 to 3. This row adds one type numbered Manual, one party and two vouchers, and takes them out again by a restore.
- **Why keyed and not posted.** A native post refuses a voucher that carries a number (`require_native_numbering`, agent_import_post.rs:3100), and `voucher_presence` cannot identify a natively posted voucher in any case (its description, agent_catalog.rs:808). Keying on Tally's screen is the lab's own act under its standing autonomy.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one on `lab/692-opening-stock` at 991c3ec4, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE CORPUS DENSE`, synthetic, TallyPrime 7.2 Silver, licensed. GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`; its number is the one the Select Company list shows. Books from 1-Apr-2025. Tally's current date there read Tuesday 31-Mar-2026.
- **Its state, from the lab's last note on it** (1029-T, 4 Nov): V 29,900, M 328, all vouchers Sales, numbers 1 to 29,900, 82 a day. The 7 days 1 to 7 Apr 2025 hold 574 of them (`lab/1029-dense-range-read-answers`).
- Not on the PC, another GUID, or Education mode: STOP, the row has no book. Any prompt when it opens: press no key, screenshot, STOP and report its words.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **ComplyEaze Bridge reads.** Steps 4 to 7 are calls of the build below, not raw requests. C3 applies to the raw marks read only. A call with no answer in 10 minutes is a STOP, and it is never sent again.
2. **Size bound** for the raw marks read: 20,000 bytes.
3. **Grant.** BK-P (step 1); the keying of step 2, each master and voucher once; the reads; the restore of BK-P (step 9). Nothing else is written. No raw import is sent.

## Build
BUILD-M, the build already staged for rows 2161-C, 1029-R and 1029-T: the 187e882b build, not master's head.
- It is the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd: artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.
- `agent_presence.rs`, `book_presence.rs` and `agent_catalog.rs` are byte-equal between 187e882b and master's head of 5 Nov 2026, so no line cited in them differs there. `agent.rs` has 8 more lines on master; its lines here are the build's.
- Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing is a STOP. Record the exe's sha256 before and after.

Over stdio, on 9001: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR` for the whole row, `BRIDGE_TERMS_ACCEPTED=true`. `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` are unset: no call of this row can write. One `bridge_mcp` process for steps 3 to 7.

**The client** is the lab's stdio inbox client of 852-C1, as 1029-T used it: no cancel and no call limit. Record it.

## Kit
| file | what |
|---|---|
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID, UTF-16LE with a BOM. A byte copy of `lab/1029-t/marks.utf16le.xml`, already this book's request. Send it as C3 says. |
| `calls/c0-control.json` | step 5's call: `voucher_presence`, `numbering` Sales `manual`, one proposal copied from the book's own Sales voucher 1 (1-Apr-2025, `DN Party 002`, 541.01). |
| `calls/c1-presence.json` | step 6's call: `voucher_presence`, `numbering` `Lab Manual Sales` `manual`, proposals P1, P2 and P3 (below). |
| `calls/c2-vouchers.json` | step 7's call: `vouchers` over 1 to 7 Apr 2025 with `voucher_type` `Lab Manual Sales`. |
| `LAB-COMMON.md` | v3, above. |

Each call file is `{"tool": ..., "arguments": ...}`: send `arguments` unchanged as that tool's arguments. `SHA256.txt` lists every other file here with its sha256, in `sha256sum -c` form. Check it first, and check each call file's sha256 again just before it is sent.

## The keyed vouchers and the proposals
| | date | type | number | party | amount |
|---|---|---|---|---|---|
| K1, keyed | 3-Apr-2025 | `Lab Manual Sales` | `LPM-1` | `Lab Presence Party` | 2,500.00 |
| K2, keyed | 5-Apr-2025 | `Lab Manual Sales` | `LPM-2` | `Lab Presence Party` | 3,700.00 |
| P1, proposed | 2025-04-03 | `Lab Manual Sales` | `LPM-1` | `Lab Presence Party` | 2500.00 |
| P2, proposed | 2025-04-05 | `Lab Manual Sales` | `LPM-2` | `Lab Presence Party` | 3750.00 |
| P3, proposed | 2025-04-06 | `Lab Manual Sales` | `LPM-3` | `Lab Presence Party` | 99999.00 |

Each proposal's entries are the party at minus the amount and `DN Sales` at the amount, the signs `vouchers` reads on this book's Sales vouchers. The tool compares the sum of the positive entry amounts, so the sign convention does not decide the comparison (`magnitude_of`, book_presence.rs:2691).

P3 has no voucher of its party on its date, and no voucher of its party at its amount. The book's Sales amounts on 1 and 2 Apr 2025 run from 541.01 to 6,445.45. A book voucher on 6 Apr 2025 at 99,999.00 would make P3 `possibly_present` with a candidate: recorded, not a STOP.

## Steps
One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. **Load and check.** Check `SHA256.txt`. LAB-COMMON step 0 (C2) with THE COMPANY loaded alone. **m0**: the marks read, EXPECT one COMPANY row, this GUID, **V 29,900 and M 328**. Other marks: recorded and reported before step 1, not a STOP. Record Tally's release, Tally's date and the PC's date.

1. **BK-P** (C5), into the new empty folder `C:\bridge-lab\1029p\backups\bkp\`. Record its name, bytes and sha256 and the time the Restore list will show. Then the marks read **m0b**; a move across the backup is recorded, not a STOP.

2. **Keying on Tally's screen**, by the operator, one act at a time, with a screenshot before each accept (Ctrl+A). Record each screen's path and each field's exact name and value as shown. A screen that differs from this text (a field missing, a value not offered, an extra screen such as bill-wise or tax details): STOP, screenshot and report. Nothing is improvised. C7 holds on every master and voucher-type screen.

   a. **Voucher type** `Lab Manual Sales`, by Create > Voucher Type:
      - Select type of voucher: `Sales`;
      - Method of voucher numbering: `Manual`;
      - Prevent duplicates: `Yes`. The screen's own default after Manual is No, and the tool refuses a Manual type that does not prevent duplicates (`book_numbering_json`, agent_presence.rs:399-419), so this field is set and its screenshot kept;
      - every other field as offered, and recorded.

   b. **Ledger** `Lab Presence Party`, by Create > Ledger: under `Sundry Debtors`; Maintain balances bill-by-bill `No`; no opening balance; every other field as offered, and recorded.

   c. **K1**, then **K2**, each by Vouchers, as a voucher of type `Lab Manual Sales`:
      - choose the type from the list Tally offers for Sales (F8) or by Other Vouchers (F10), and record which;
      - the mode `Accounting Invoice`; if the screen opens in another mode, change it with Ctrl+H, and if `Accounting Invoice` is not offered, STOP;
      - the date by F2, as in the table; the number in `Voucher No.`, as in the table;
      - `Party A/c name` `Lab Presence Party`; one line, ledger `DN Sales`, amount as in the table;
      - no narration;
      - accept, and record what the screen shows after the save.

   Then the marks read **m1**: EXPECT **V 29,902**, two above m0b. Record M; it is expected above m0b's. V otherwise: STOP.

3. **Start `bridge_mcp`.**
   - `list_companies`: EXPECT this GUID, `books_from` 20250401.
   - `tally_status`: record release, licence and mode. Education mode: STOP.
   - `masters` with `{"company_guid": <this GUID>, "kind": "voucher_types"}`, every page. Record the `Lab Manual Sales` row and the `Sales` row whole: `name`, `parent` and `series_numbering`. EXPECT `Lab Manual Sales`: `observation` observed, `method` manual, `duplicates` prevented; `Sales`: automatic, `auto_retain`, `not_prevented`, as 1029-T read it. `Lab Manual Sales` otherwise: STOP, report the row; steps 5 to 7 would answer nothing.

4. **m2**, the marks read. EXPECT byte-equal to m1.

5. **C0, the control.** `voucher_presence` with `calls/c0-control.json`. EXPECT refused with `presence_numbering_method_differs_from_book`, its candidates naming `Sales` with `declared` manual and `book` observed automatic. Keep the answer whole and its round trip.
   - Not refused: keep the answer whole, report it before step 6. It is a finding: the check let a type the book numbers automatic through.

6. **C1, the question.** `voucher_presence` with `calls/c1-presence.json`. Keep the answer whole, with its round trip and every page (`offset` from `next_offset`, until none is given). EXPECT:
   - `numbering_read` with `read_before_window` true and the `Lab Manual Sales` row `agrees` true;
   - the window `complete`;
   - **P1**: `present`, basis the manual voucher number, `differences` empty;
   - **P2**: `present`, basis the manual voucher number, `differences` exactly one, the amount, proposed 3750.00 and observed 3700.00;
   - **P3**: `absent`.

   Any other answer is recorded whole, with each proposal's `presence`, basis or reason, candidates and differences. It is not a STOP; it goes to the run coordinator before step 9.

7. **C2.** `vouchers` with `calls/c2-vouchers.json`. Keep the answer whole. EXPECT `state` complete and two rows, K1 and K2. Record each row's date, type, number, party, `amounts` with `is_deemed_positive`, GUID, MasterID and AlterID, `cancelled` and `optional`.

8. **m3**, the marks read. EXPECT byte-equal to m1. Then `egress_log` once, the whole answer. Stop `bridge_mcp`. Record the exe's sha256.

9. **Restore BK-P** (C6). Then the marks read **m4**. EXPECT the COMPANY row's GUID, V and M equal to m0b's. Anything else = STOP.

10. LAB-COMMON C10.

STOP RULES:
- C7 and C8;
- any dialog on Tally's screen, before or after a call;
- any call with no answer in 10 minutes;
- a marks answer with other than one COMPANY row, or a GUID that is not this book's;
- V not as step 2 expects; any mark that moves in steps 3 to 8;
- step 3's `Lab Manual Sales` row other than manual and prevented;
- Education mode.

After a STOP, nothing more is sent or keyed. The book then holds whatever of step 2 was done; the way back is BK-P, restored only on the run coordinator's word (C8).

## What the result decides
| what C1 shows | reading on #1029 |
|---|---|
| P1 `present` with no difference, P2 `present` with only the amount, P3 `absent`, the window complete, the type agreeing | **Row A's presence half is measured live on this book:** each of the three answers seen once, through the Manual admission, on about 30,000 vouchers. With row A's first half (the dense read of 26 Oct) and row B (1029-R), this is what row B's sheet names as closing #1029; the closing is recorded on the issue with this row's confidence. |
| P1 or P2 `possibly_present` | **A finding.** The reason names the gap: a number that did not decide on a type the book numbers Manual. Reported whole; nothing follows until the run coordinator reads it. |
| P2 `present` with a difference other than the amount, or none | **A finding** on `differences` (book_presence.rs:2449). |
| P3 `possibly_present`, reason `window_not_proven_complete` | **A finding:** the window that read `complete` on 26 Oct did not qualify with two more vouchers in it. |
| P3 `possibly_present` with a candidate | The candidate's rule is recorded. If a book voucher resembles P3 by date and amount, `absent` was withheld correctly, and a new proposal is a new sheet. |
| C1 refused | Its code is recorded. `presence_numbering_method_differs_from_book` here, after step 3 read the type as manual and prevented, is a finding on the admission. |
| C0 not refused | **A finding** on the admission (step 5). |

## Code admits the trigger (187e882b, the build's head)
- **The tool:** `voucher_presence` is dispatched at agent.rs:3433. It is a read, and with writes unset nothing here can send an import.
- **What is refused before any read:** a proposal dated outside the window (agent_presence.rs:98) and a proposal whose type has no declared method (:104). P1 to P3 and C0's proposal are inside 1 to 7 Apr 2025 and each type is declared.
- **The Manual admission, before the window:** `read_presence_numbering` (called at agent_presence.rs:124, defined at :285) reads the master mark and the book's voucher-type numbering. It refuses `presence_numbering_method_differs_from_book` (:347) when a type declared `manual` is not, in the book, Manual with exactly one series that prevents duplicates (`book_numbering_json`, :399; prevented required at :419). That is step 5's trigger, with Sales, and step 6's pass, with `Lab Manual Sales`.
- **The window:** `read_entry_window_rows` (:129), then every party and entry ledger the rows carry must be in the catalogue read first (:162-173, else `ledger_catalogue_incomplete`). `Lab Presence Party` is keyed in step 2, before the catalogue read. Completeness comes from `window_read` (:201). The catalogue is read again and compared (:216-230, else `ledger_snapshot_drifted`); nothing is keyed between the two.
- **A number decides only within a type the window observed:** `type_observed` (book_presence.rs:1851) and `number_decides` (:1900), with `find_number_matches` (:2005) matching by type and number. K1 and K2 are in the window, so `Lab Manual Sales` is observed. The book's Sales numbers are 1 to 29,900, so no `LPM-` number is shared.
- **`present`:** `identity_selections` (:2205) selects the one voucher whose number matches, when the number is unique in the proposals. `selected_status` (:2237) then `settled` (:2422) give `present` with `differences` (:2449), the amount difference at :2471. A cancelled or optional voucher is never `present` (:2428): K1 and K2 are neither.
- **`absent`:** with no number match and no resemblance (`resemblances`, :2366: same date and party and amount, party and amount, date and amount, or date and party), `unresembled_status` (:2299) gives `absent` only when a manual number was supplied, the party could be compared completely and the window read complete.

## Who sends each request (187e882b)
| step | request | sent by | the count |
|---|---|---|---|
| 0, 1, 2, 4, 8, 9 | marks, raw | the operator's curl on 9001, not ComplyEaze Bridge | one COMPANY row. V is its voucher mark (ALTVCHID) and M its master mark (ALTMSTID); byte-equal answers mean nothing in the book changed between them. |
| 3 | the company list | `list_companies`, agent.rs:3380 | one company with this GUID |
| 3 | the voucher-type export | `masters`, agent.rs:3394 | one row per voucher type the book holds |
| 5, 6 | catalogue, numbering, window, catalogue again | `voucher_presence`, agent.rs:3433 | step 5 stops after the numbering read; step 6 counts the window's vouchers and answers per proposal |
| 7 | the window | `vouchers`, agent.rs:3436 | the window's rows of type `Lab Manual Sales` |

## Not measured by this row
- a Gold book (the owner's ruling of 5 Nov 2026 that Gold is Silver for everything);
- a narration-marker identity (a ComplyEaze Bridge file imported by hand);
- a type numbered Manual without duplicate prevention, or with more than one series;
- a party whose comparison is cut short, and a proposal with no party;
- more than one call, or a window wider than 7 days;
- TallyPrime 7.1.

## Keep
On `lab/1029-p-presence-answers`, with `SHA256.txt`:
- every tool answer whole, with its size, sha256 and round trip;
- m0, m0b, m1, m2, m3 and m4;
- BK-P's name, bytes and sha256, and `restore-plan.txt`;
- the keying record of step 2, screen by screen;
- a RESULTS.md with C1's three answers first, then C0's;
- a notes file with UTC times, one line per step.

Answers that carry the company number stay on the lab PC, listed by size and sha256. Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.
