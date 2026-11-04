# Row 1029-T: how does BRIDGE CORPUS DENSE number its voucher types? One `masters` read of its voucher types, read only

Not for merge. Part of #1029. Written 4 Nov 2026. Lines below are at 187e882b, the build's head, and this branch starts there. Confidence of everything here until the row runs: unverified.

## Question
Is the Sales type of BRIDGE CORPUS DENSE numbered Manual, with duplicate numbers prevented? And how is each other type numbered?

## Why
- **What row A's second half needs.** It is a `voucher_presence` call on a dense window with one booked and one unbooked proposal, expecting `present` and `absent`.
- **What the tool requires for that.** `absent` comes only for a type declared `manual` that carries its number, with a party. A type declared `manual` is refused unless the book numbers it Manual with exactly one series that prevents duplicates (the tool's description, agent_catalog.rs:808).
- **Why this book.** DENSE is the only dense lab book whose vouchers carry both a party and a number. Its Sales vouchers show `DN Party nnn` and run 1 to 574 over 1–7 Apr 2025 (`lab/1029-dense-range-read-answers`). But no lab file has read its voucher types.
- **What this row decides.** Whether that half can run read-only on this book. Nothing else is asked.

## Book
`BRIDGE CORPUS DENSE`, synthetic, TallyPrime 7.2 Silver, licensed, on the lab PC, loaded alone. GUID begins `d45bc1b0`; the whole GUID is in `lab/1029-dense-range-read-answers`.
- **Its state, from the lab's last note on it:** V 29,900, M 328. Row 725-D1 (`lab/725-dense-200`), ahead of this row in the queue, restores it to its own step-1 state when it ends.
- **The data folder** holds 20 files while the company is shut and 23 while it is loaded (725-D1's count).

Not a client book, and nothing seeded from client data. Not on the PC, another GUID, or Education mode: stop, the row has no book. Any prompt when it opens: press no key, screenshot, stop, and report its words.

**Read only.** No write, no backup, no restore, and no key pressed in Tally beyond loading and shutting the company.

## Build
BUILD-M, the build already staged for rows 2161-C and 1029-R: the 187e882b build, not master's head.
- It is the `unsigned-bundle-smoke-windows-latest` artifact of CI run 41772223948, the merge-queue run of #2329 at 187e882b5176a3b577d57f257a6ec4319fc2aecd: artifact id 13295846440, 59,336,955 bytes, GitHub digest `sha256:1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a`, expiring 11 Nov 2026.
- Master has since moved to 91059155 (#2331). That commit changes only the gate's data files, so no line cited here differs.
- Check the zip's sha256 against the digest, and `bridge_mcp.exe` against `mcpb-smoke.json`, before the first call. Either differing is a stop.

Over stdio, on 9001: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, one new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true`. `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` are unset. Record the client.

## Kit
| file | what |
|---|---|
| `marks.utf16le.xml` | the company's GUID, ALTVCHID and ALTMSTID, UTF-16LE with a BOM. A byte copy of `lab/725-dense-200/marks.utf16le.xml`, which is already this book's request. Send it with `Content-Type: text/xml; charset=utf-16`. |

`SHA256.txt` lists every other file here with its sha256, in `sha256sum -c` form. Check it first.

## Steps
One at a time. Every answer goes to a file. A file with no answer is a failed step, not an empty one.

0. **Load and check.**
   - Check `SHA256.txt`. Count the data folder's files while the company is shut: EXPECT 20.
   - Load BRIDGE CORPUS DENSE alone. Screenshot. `GET /status` on 9001. Count the folder again: EXPECT 23.
   - Record Tally's release, Tally's date and the PC's date. Another count is recorded and reported, not a stop: nothing here writes.
1. **m1.** Marks raw on 9001. EXPECT one COMPANY row with this book's GUID, V 29,900 and M 328. Other marks are recorded and reported, not a stop. Keep the answer as `m1-marks.xml`.
2. **Start `bridge_mcp`.** `list_companies`: EXPECT this GUID, `books_from` 20250401. `tally_status`: record release, licence and mode. Education mode: stop.
3. **The read.** `masters` with `{"company_guid": <GUID>, "kind": "voucher_types"}`. Keep the whole answer.
   - If it says there are more rows (a `next_offset`), call again with that offset, until no `next_offset` is given. Keep each page.
   - A refusal: keep it whole and go to step 4. A refusal answers nothing here.
4. **m2.** Marks raw on 9001. EXPECT the answer byte-equal to m1's. Keep it as `m2-marks.xml`.
5. **Close.** Stop `bridge_mcp`. Shut the company by Tally's own Shut Company and write down the path and keys. Count the folder: EXPECT 20. Screenshot: no company loaded. One tally.exe owns 9001.

## Record
- For **every** voucher-type row:
  - `name`, `parent`, and the whole `series_numbering` object: `observation`, plus `method`, `sub_method` and `duplicates`, or `count`, or `code`;
  - how many rows there are.
- For the **Sales** row, on its own line: `series_numbering.observation`, `method` and `duplicates`.
  - If another row has Sales as its parent, its line too.
  - If no row is named Sales, say so, and list every row whose parent is Sales.
- `first_pair_differed`, if the answer carries one.
- m1 and m2: bytes, sha256, V and M, and whether they are byte-equal.
- The folder counts at each point, the release, the dates and the client.

## Decides
| what the Sales row shows | what follows for row A's presence half |
|---|---|
| `observation` observed, `method` manual, `duplicates` prevented | **DENSE qualifies as it stands.** The half can run read-only on 1–7 Apr 2025, with `numbering` Sales `manual`: one proposal from a booked Sales voucher (its number, party and amounts) expecting `present`, and one with an unused number expecting `absent`. That is a separate sheet. |
| `method` manual, `duplicates` not_prevented | **Not as it stands:** the tool refuses a type declared manual without prevention. |
| `method` automatic, of either sub-method, or automatic_manual_override | **Not as it stands:** the tool decides nothing from a number under automatic. |
| `observation` series_count above 1, or field_invalid | **Not as it stands:** the tool needs exactly one series it can read. |
| a refusal | **Undecided.** Report its code; nothing follows. |

Each "not as it stands" goes back to the run coordinator: write shapes on this book, or a ruling on the half.

## Stop rules
- Any dialog or prompt on Tally's screen.
- Any call with no answer in 10 minutes.
- Education mode.
- m2 not byte-equal to m1: record both, report, and send nothing more.

## Code admits the trigger (187e882b, the build's head)
- **The tool:** `masters` is dispatched at agent.rs:3394 and defined at agent_masters.rs:15.
- **The kind:** `"voucher_types"` maps to `NativeMasterKind::VoucherTypes` (tally/runtime_masters.rs:27). Its request is `render_native_voucher_type_export_request` (bridge-tally-protocol native_outstandings/request.rs:411). It is a master export and sends no voucher read.
- **The answer's numbering:** each row carries `series_numbering` from `series_numbering_json` (agent_masters.rs:268).
  - **observed:** `method` is manual, automatic (with `sub_method` auto_retain, auto_renumber or absent) or automatic_manual_override, and `duplicates` is prevented or not_prevented;
  - **series_count:** more than one series;
  - **field_invalid:** a field it could not read.
- **The presence rule this feeds:** a type declared `manual` is checked against the book's numbering before the window is read (agent_presence.rs:285, `read_presence_numbering`).
- **No write is possible:** writes are not enabled (`BRIDGE_AGENT_ENABLE_WRITES` unset), and the only raw request is the marks read.

## Who sends each request (187e882b)
| step | request | sent by | the count |
|---|---|---|---|
| 1, 4 | marks, raw | the operator's curl on 9001, not Bridge | one COMPANY row. V is its voucher mark (ALTVCHID) and M its master mark (ALTMSTID); byte-equal answers mean nothing in the book changed between them. |
| 2 | the company list | `list_companies`, agent.rs:3380 | one company with this GUID |
| 3 | the voucher-type export | `masters`, agent.rs:3394 | one row per voucher type the book holds |

## Keep
On `lab/1029-t-answers`, with `SHA256.txt`:
- every tool answer whole, with its size and sha256;
- m1 and m2;
- a RESULTS.md with the Sales line first, then the table of every row;
- a notes file with UTC times, one line per step.

Screens stay on the lab PC, listed by name, size and sha256. Send the sha to the run coordinator before any push.
