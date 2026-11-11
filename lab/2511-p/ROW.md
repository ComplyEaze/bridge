# Row 2511-P: on the several-currency lab book, do three plain rupee Receipts built by master's `build_import_xml` import and read back plain, and does the ledger catalogue answer a ledger's currency

DRAFT, not for merge, and not queued. Part of #2511. **Three writes, behind one backup**, and one read of a new shape. Lines are at master 06268abe. LAB-COMMON v3 (`LAB-COMMON.md`, byte-equal to the 683 rows' copy) applies in full. Where this sheet and it differ, the stricter holds.

## Question

#2511 needs lab rows before any build (P1). This sitting covers the case a per-ledger currency rule would admit: plain rupee vouchers on a book with two Currency masters. It also covers the one read that rule needs.
- **P1:** a plain rupee Receipt from `BRIDGE INR Customer` into `Cash`. Both ledgers are kept in `₹` and carry no dollar entry. Does it import (CREATED 1), and do its entries read back plain?
- **P2:** P1 with the money ledger `BRIDGE Lab Bank`, a `₹`-kept bank ledger whose entries and closing are dollar composites. Does the new rupee entry read back plain, or as a composite?
- **P3:** P1 with the party `BRIDGE USD Customer`, a bill-wise `₹`-kept party holding dollar bills. Does it read back plain, and where does the amount land among its bills?
- **P4a:** the import's ledger catalogue request (V2) with one more native method, `CurrencyName`. Does that collection answer each ledger's currency on 7.2, and as the same text the currency Trial Balance gives (`₹` for all eight)?

Each write row changes one thing from P1. Each is dated on a day of its own (2, 3 and 4 April 2025), so its read window holds that voucher alone. Every voucher already on the book is dated 1 April, and eight of the eleven carry composites, which `verify_import` refuses (`CompositePolicy::Refuse`).

The writes are cumulative: P2 runs with P1's voucher saved, and P3 with both. Each row's own checks compare the book before and after that row's send.

## Book

`BRIDGE FOREXGROUP LAB` (synthetic), GUID `1bb48be5-baba-45b2-90d8-7593244915a9`, TallyPrime 7.2. THE COMPANY in LAB-COMMON means this book.
- **Synthetic:** it is in master's registry, `scripts/merge-gate-synthetic-uuids.json`, as row 683-VD records. It is not the register lab book, and not the one the private exclusion list names. No client book is used at any step.
- **Ledgers** (row 683-TW's `tb3` answer): eight, all with `CURRENCYNAME` `₹`. `BRIDGE INR Customer` and `Cash` read plain. `BRIDGE Lab Bank`, `BRIDGE USD Customer`, `BRIDGE Export Sales` and `BRIDGE Import Purchases` read composite. `BRIDGE USD Supplier` and `Profit & Loss A/c` read the zero composite.
- **Vouchers** (row 683-TW's `mvy` answer): eleven, all dated 20250401.
- **Its last state:** the run coordinator names, in the kit line below, the marks answer of the last row's restore. m0 must be SAME MARKS with it.

KIT LAST MARKS: <to fill: file, its row and branch, sha256>

The lab fills this line by a new commit on this branch. **While it reads `<to fill>`: STOP. This row does not run.**

## Build

The build is the Windows bundle of a Bridge CI run on master whose commit holds 06268abe, unexpired, checked by the three hashes of the 1761-S4 sheet: the zip, the `.mcpb` and `bridge_mcp.exe`. The run coordinator fills the line by a new commit on this branch, as for 1761-S4.

RC COMMIT: de54a0f26e554f574727a0bc4914b0ad6ca88583  CI RUN: the Bridge CI `merge_group` run of #2502 at that commit, conclusion success (its id is written in words only, in `notes.txt` as fetched)  ARTIFACT: `unsigned-bundle-smoke-windows-latest` of that run, 60,522,001 bytes, expires 18 Nov 2026  ZIP DIGEST: sha256:5c30b5b7d62b32077d30d06aabceecc6a5b4f3bc3f3ac409f6f204c780b47106  EXE SHA256: be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce

**While any field of this line reads `<to fill>`: STOP. This row does not run.**

One launch over stdio by the lab's own client, from step 0 to the close:
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
- one new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\2511-p\data\`;
- `BRIDGE_TERMS_ACCEPTED=true`;
- **`BRIDGE_AGENT_ENABLE_IMPORT=true`, and `BRIDGE_AGENT_ENABLE_WRITES` absent.** Step 0 records both. `BRIDGE_AGENT_ENABLE_WRITES` present = STOP. This row builds files and never posts through Bridge.
- `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset.

No other ComplyEaze Bridge calls port 9001 during the row.

## Kit

**How the request bytes were made: a lab-only render test, not a template edit.** `src-tauri/src/agent_lab_2511_p_render_tests.rs` on this branch, one `#[ignore]` test, run once in a build slot:
`LAB_2511_P_OUT=<this folder> cargo test -p bridge --lib lab_2511_p -- --ignored --exact agent::lab_2511_p_render_tests::lab_2511_p_renders_the_row_requests`. It writes each request as the build sends it, UTF-16LE with a BOM. Before writing, it checks the year window render byte for byte against `V-mvy-20250401-20260331.xml`, which rows 683-TW, 683-LD and 683-VD sent on this book, so the encoding and the builder are shown equal. It also checks that V2 holds the bill-wise line once and no `CurrencyName`. The test and its one `mod` line in `agent.rs` are on this branch only and are never merged.

Check every file against `SHA256.txt` (sha256, bytes, name) first, and each request file again just before it is sent (C3).

| file | what |
|---|---|
| `M-marks.xml` | The marks request, byte-equal to the 683 rows'. |
| `V-mvy-20250401-20260331.xml` | Row 683-TW's book-scoped movement window, byte-equal (`render_agent_movement_vouchers`, `agent_read_profiles.rs:100`). |
| `V-day-20250402.xml`, `V-day-20250403.xml`, `V-day-20250404.xml` | The same builder, book scope, from and to that day. Each is 1,770 B and differs from `V-mvy-20250401-20260331.xml` in its dates alone: `SVFROMDATE`, `SVTODATE` and the two `$$Date` literals of the window's filter. |
| `C-cat2.xml` | The import's ledger catalogue request as master sends it (V2): `render_import_ledger_catalog_request` (`src-tauri/src/tally/standard_ledger_catalog.rs:98-101`), reached from `agent_import.rs:3105` through `import_ledger_catalogue_read`, for the book's name. Kept for the diff only: it is not sent. |
| `C-cat4.xml` | **NEW SHAPE.** `C-cat2.xml` with one line added after `<NATIVEMETHOD>IsBillWiseOn</NATIVEMETHOD>`, at the same indent: `<NATIVEMETHOD>CurrencyName</NATIVEMETHOD>`. `C-cat2-to-cat4.diff` is the diff of the two decoded texts: that one line and nothing else. The builder it is meant to become is a `StandardLedgerCatalogV4` profile in `src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs`, unbuilt (V3 is V2 plus `AlterID`). |
| `a-twmvy-V-mvy-20250401-20260331.xml` | Row 683-TW's answer to that window, byte-equal to row 683-VD's copy: the book as found, which step 2 compares against. |
| `run.py`, `lm.py`, `tb.py`, `cmpinfo.py`, `LAB-COMMON.md`, `.gitattributes` | The 683 rows', byte-equal. |
| `vcmp.py` | New for this row. `vcmp.py A B [--only-a GUID]... [--only-b GUID]...` compares two movement answers VOUCHER row by VOUCHER row, matched by GUID, parsed as C4 says, never by a text search. See **Reading `vcmp.py`** below. |

**Reading `vcmp.py`.** Run it alone, never into a pipe (`| head` or `| tail` would hide its exit code), read `$?`, and copy the whole output. It prints each answer's bytes and sha256 first.
- **STOP (exit 1):** a read that failed, never "0 rows". That is an answer that cannot be read, does not decode as UTF-16LE or does not parse; HEADER/STATUS other than exactly 1, or none; any LINEERROR; no BODY/DATA/COLLECTION; a COLLECTION child that is not a VOUCHER; no VOUCHER row; or a GUID that is absent, not this book's GUID plus `-` and 8 lowercase hex digits, or on two rows.
- **Forbidden characters** (C4's list, raw or as a reference) are never deleted: each becomes the character U+FFFD, then `#` and its code in two hex digits (`&#4;` reads as U+FFFD `#04`), and the count per answer is printed. Two answers that differ only there still differ.
- **The bar.** A row in both answers is SAME when the two VOUCHER elements, each serialised by the parser without its own tail, are equal: equal after the parse's own normalisations, not byte-equal. Three are accepted for this row: a line break written CR LF reads as LF (this book's answers are written with CR LF); an empty element reads the same as `<X/>` or `<X></X>`; and a character reads the same written raw or as a reference (`&amp;` and `&#38;`). Within one sitting, by one Tally, none of the three should move.
- **Rows in one answer only.** `--only-a` and `--only-b` name the GUIDs the step EXPECTs in that answer only; a side not named EXPECTs none.
- **Result.** It prints the SAME rows as one count, then a block for each row that DIFFERS (each differing child, the character where it first differs and the text around it, or that the row differs only between its children), then each row in one answer only, marked named or NOT NAMED. Detail lines stop after 300. The result is decided before anything is printed, so the cap never changes it. The last line is `COMMON SAME (...)`, exit 0, only when every common row is SAME and the rows in one answer only are exactly the ones named; otherwise `COMMON DIFFERS (...)`, exit 2.

The three import files are not in the kit. Steps 3 to 5 build them in the sitting through `build_import_xml`, and the row records each one's sha256 before its send.

## The builds and the writes

Each import file is built by `build_import_xml` with one inline voucher. Its arguments are written into the notes before the call:
- **P1:** Receipt, date `2025-04-02`, party `BRIDGE INR Customer` credit `100.00`, `Cash` debit `100.00`, narration `2511-P1`.
- **P2:** as P1, date `2025-04-03`, `BRIDGE Lab Bank` in place of `Cash`, narration `2511-P2`.
- **P3:** as P1, date `2025-04-04`, `BRIDGE USD Customer` in place of `BRIDGE INR Customer`, narration `2511-P3`.

For each:
1. **The first call.** EXPECT a refusal naming the party as bill-wise, with the approval to return (`on_account_approvals`). Record it whole. Any other refusal is recorded whole, and that row is not sent: go on to the next row. If the refusal names `import_multi_currency_unsupported`, master has changed since 06268abe: STOP before any send.
2. **The second call,** with `on_account_approvals` exactly as the refusal gives it, after the run coordinator's yes in chat. EXPECT a built file. Record the result whole: `batch_id`, the file's path, its sha256 and `verification_preflight`.
3. **The file.** Parse it (C4), and record that it holds one `VOUCHER` with the two entries as written above. Record also its `REMOTEID` and every element present (names only). Any other content = that row is not sent.
4. **The send.** By C3, ONCE, with `run.py`, as label `p1w`, `p2w` or `p3w`, only if the marks read just before showed THE COMPANY's row alone and no key was pressed since. EXPECT CREATED 1 and every other counter 0. Then screenshot the whole Tally window before any other request (C3). A dialog = C7.

## Steps

Run each send as `python3 -I run.py http://127.0.0.1:9001 <label> <file>`. Each label is used once.

0. **Step 0 by C2, with no company loaded.** Load the book alone. Record the Gateway's current date, period and date of last entry. EXPECT 1-Apr-2025, 1-Apr-25 to 31-Mar-26, and 1-Apr-25. Record the release, the licence mode and the PC's date. Launch the build, then record the two settings and the exe's hash against the build line.
1. **`m0`:** `M-marks.xml`, then `lm.py --same-row <KIT LAST MARKS> a-m0-M-marks.xml`. EXPECT SAME MARKS. Otherwise STOP.
2. **The book as found, and P4a,** one send at a time:
   - **`mv0`:** `V-mvy-20250401-20260331.xml`, then `lm.py --capture a-mv0-V-mvy-20250401-20260331.xml a-twmvy-V-mvy-20250401-20260331.xml` (row 683-TW's answer, in the kit). EXPECT CAPTURE SAME, with 11 and 11. Then `vcmp.py a-twmvy-V-mvy-20250401-20260331.xml a-mv0-V-mvy-20250401-20260331.xml`, recorded whole and not an EXPECT: it shows whether the restores since row 683-TW left every voucher equal at `vcmp.py`'s bar. Steps 4 to 6 compare only reads of one sitting.
   - **`c4`:** `C-cat4.xml`. Size bound 200,000 B. It is a NEW SHAPE, so screenshot before the next request. Record the answer whole. For each LEDGER row record NAME, PARENT, ISBILLWISEON and CURRENCYNAME, or that CURRENCYNAME is absent, read by an XML parser as C4 says. `lm.py` is not run on this answer: its `C-` kind reads currencies.
   - **`m0b`:** `M-marks.xml`, `lm.py --same-row a-m0 a-m0b`. EXPECT SAME MARKS: the reads wrote nothing.
3. **P1's build** (calls 1 to 3 above). Bridge reads and writes only its own data folder here. Then **BK** by C5, to a new empty local folder `bk-2511p`. Record the file and the time. Then **`mb`:** `M-marks.xml`, `lm.py --same-row a-m0b a-mb`. A difference is recorded (C5), not a STOP.

   **The restore's word, given now (C8):** from here on, step 8 runs after the last read, after any STOP, and after any write that does not read as EXPECT. The word waits, and the run coordinator is asked first, in three cases: an unnamed dialog still up, a send past the wedge rule, or Tally closed (C9).
4. **P1's send** (`p1w`), then, one at a time:
   - **`m1`:** EXPECT MARKS DIFFER from `mb`. Record ALTVCHID and ALTMSTID.
   - **`d1`:** `V-day-20250402.xml`. Record the voucher whole: each entry's LEDGERNAME, AMOUNT and ISDEEMEDPOSITIVE, and every bill allocation's NAME, BILLTYPE and AMOUNT. EXPECT one voucher.
   - **`mv1`:** `V-mvy-20250401-20260331.xml`, then `vcmp.py a-mv0-V-mvy-20250401-20260331.xml a-mv1-V-mvy-20250401-20260331.xml --only-b <P1's GUID, copied from d1>`. EXPECT exit 0 and `COMMON SAME (common 11, differ 0, only in A 0 (0 not named), only in B 1 (0 not named), named but missing 0)`. The row only in B is dated 20250402 with narration `2511-P1`.
   - **`verify_import`** for P1's `batch_id`, through Bridge. Record the result whole: `verification_status`, and any refusal code.
5. **P2's build and send** (`p2w`), as step 4 with `m2`, `d2` (`V-day-20250403.xml`), `mv2` against `mv1` with `--only-b` P2's GUID from `d2` (EXPECT exit 0, common 12, differ 0, only in B 1, named), and `verify_import` for P2's batch.
6. **P3's build and send** (`p3w`), as step 4 with `m3`, `d3` (`V-day-20250404.xml`), `mv3` against `mv2` with `--only-b` P3's GUID from `d3` (EXPECT exit 0, common 13, differ 0, only in B 1, named), and `verify_import` for P3's batch.
7. **`m3r`:** `M-marks.xml`, `lm.py --same-row a-m3 a-m3r`. EXPECT SAME MARKS.
8. **The restore** from `bk-2511p`, by C6. Then **`mr`:** `lm.py --same-row a-m0 a-mr`, EXPECT SAME MARKS. Then **`mvr`:** `V-mvy-20250401-20260331.xml`, `lm.py --capture a-mvr a-mv0`, EXPECT CAPTURE SAME, with 11 and 11, and `vcmp.py a-mv0-V-mvy-20250401-20260331.xml a-mvr-V-mvy-20250401-20260331.xml`, recorded whole and not an EXPECT. Otherwise STOP: "the restore of bk-2511p did not return the book to m0".
9. **Tally's date:** EXPECT as at step 0. If it differs, record it and press nothing; the run coordinator decides.
10. **Close** by C10. End the Bridge launch from its client, by closing stdin. `GET /status` after the last request (C11).

## The wedge rule

As row 683-VD's. One request at a time by `run.py`, which never abandons one. The book is small: its largest answer, the book window, was 59,524 B as found. A send not answered within 5 minutes: press nothing, send nothing more, screenshot, and report (C9). A Bridge call not answered within 5 minutes is treated the same. Nothing is re-sent on this row's own judgement.

## Stop

Press no key, screenshot the whole window, and report before anything else, if:
- an unnamed dialog appears, or a second `tally.exe` starts;
- KIT LAST MARKS or any field of the build line reads `<to fill>` (then nothing is sent at all);
- step 0 is not as EXPECT, `BRIDGE_AGENT_ENABLE_WRITES` is present, or the exe's hash differs;
- m0 is not SAME MARKS, or `mv0` is not CAPTURE SAME;
- a send fails (C3), or an import answer is not CREATED 1 with every other counter 0;
- a build refuses with `import_multi_currency_unsupported`;
- `mr` or `mvr` is not as EXPECT after the restore.

From step 3 on, a STOP is followed by the restore (step 8) on the word given there, except in the three cases it names.

## Record

- Each request's and answer's bytes and sha256. The whole output of each `lm.py`, `vcmp.py` and Bridge call.
- For P4a: per ledger, NAME, PARENT, ISBILLWISEON and CURRENCYNAME, or absent; and whether CURRENCYNAME equals the `tb3` answer's for that ledger.
- For each write: the build arguments, both build results, the file's sha256 and its element names, the import answer's counters with LASTVCHID and LASTMID, the day read's voucher, the `vcmp.py` lines and `verify_import`'s result.
- The marks at m0, m0b, mb, m1, m2, m3, m3r and mr.
- The BK, `restore-plan.txt`, the data-folder listings and every restore prompt word for word. Tally's date at steps 0 and 9.
- Every screenshot by name, bytes and sha256 (never pushed).
- Staging the evidence, as for the 683 rows: the start-up check allows a hit only inside the amount `$1000000.00`, and the check for 10 or more digits sets aside this book's registered GUID matched whole.

## Expected (estimates, not EXPECTs)

- **P4a:** CURRENCYNAME `₹` on all eight rows, or the field absent. Either is a result.
- **P1:** CREATED 1, entries plain, `vcmp.py` exit 0 with P1's GUID named, `verify_import` `posted_verified`.
- **P2, P3:** not known. That is the question.

## Decides

- **P1 plain and verified:** on this book and release, the per-ledger rule's admitted case imports and reads back plain, through master's own build: PARTIAL.
- **P2's bank entry plain:** a `₹`-kept ledger's dollar history did not change a new rupee line: PARTIAL. **Composite:** a plain rupee import onto such a ledger is stored or read as a composite, and the per-ledger rule must refuse any ledger that reads composite, not only one kept in `$`.
- **P3 plain:** as P2, for a bill-wise party; record where the amount landed among its bills. **Composite, or a bill moved that no voucher touched:** record it whole. It decides nothing until the run coordinator reads it.
- **P4a answers CURRENCYNAME as the Trial Balance does:** the catalogue can carry the ledger's currency on 7.2: PARTIAL. **Absent:** the per-ledger rule reads the currency Trial Balance's `CURRENCYNAME` instead, which `classify_ledger_currencies` already reads.
- **Any row where an earlier voucher changed** (`vcmp.py` COMMON DIFFERS): record it whole; nothing is built on any row until it is read.

Any answer that becomes a fixture is parsed again by master's own parser first.

## Not in this row

- Any foreign amount in an import (rows F0 to F11, a later sheet).
- A ledger kept in `$`, so rows P4b and P5: the 7.2 book has none, and keying one, or using the 7.1 book as a second book, is a maintainer's choice.
- `post_import`: it refuses this book, and writes are off.
- Another book, another release, a Journal, Payment or Contra.
