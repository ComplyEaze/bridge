# #1231: cost-centre allocations keyed on Tally's screen, on a book whose Cost Centres setting is Yes (keyed by hand, then reads)

Not for merge. Written 26 Oct 2026. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full; where this sheet and it differ, the stricter holds. `SHA256.txt` lists every file here; check it before step 0.

**WHY.** Issue #1231's cost-centre reads were built from live captures (#1398; protocol reference §12a.17 and §8.2g), and the vouchers read returns `cost_centre_allocations` from one captured Journal whose allocation was written by an import, on a book whose Cost Centres setting reads No. Protocol reference §8.2g lists what that capture does not show: an allocation keyed on Tally's own screen, a book whose setting is Yes, a centre with a parent, several categories on one entry, and another voucher type. This item captures exactly those, so the read's parser is checked against them before the per-cost-centre summary of #1231 is built on it (AGENTS.md P1).

**THE COMPANY** is `BRIDGE COST KEY LAB`, synthetic and NEW for this item (step 1 creates it). If a company or data folder of that name exists on the PC: STOP and report it; never reuse one. It is loaded alone throughout.

**REQUESTS BY CURL (allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes; this item's requests only read).** Every request is an existing builder's committed request, used as an instrument, with only the company (and, for the two voucher reads, the day) changed; no new builder is made from them:

| file | what | source (committed fixture under `src-tauri/crates/bridge-tally-protocol/tests/fixtures/`) | builder |
|---|---|---|---|
| `requests/marks.utf16le.xml` | MARKS: the company's GUID, ALTVCHID, ALTMSTID | `note-days/shape_lab_company_marks_request.utf16le.xml` | `render_agent_company_high_water` (`src-tauri/src/agent_read_profiles.rs`) |
| `requests/cost-centres.utf16le.xml` | the cost centres | `masters_cost_centres_request.utf16le.xml` | the `masters` read, kind `cost_centres` (#1398) |
| `requests/cost-categories.utf16le.xml` | the cost categories | `masters_cost_categories_request.utf16le.xml` | the `masters` read, kind `cost_categories` (#1398) |
| `requests/vch-20250515.utf16le.xml`, `requests/vch-20250516.utf16le.xml` | one day's vouchers, every ledger entry (`ALLLEDGERENTRIES.*`) | `voucher_cost_centre_allocations_shape_lab_request.utf16le.xml`, its four date literals changed | the vouchers read (`Bridge Agent Vouchers`, §8.2g) |

The files are UTF-16LE with a byte-order mark and no final line break; send them as they are, with LAB-COMMON C3's line. Before the first send: `python -I check-request-ids.py requests` in this folder; EXPECT `census: ok 5, skipped 0, FAIL 0`; anything else = STOP, nothing sent. Read bounds (C3): MARKS 8,192 bytes; each other read 200,000 bytes.

**THE BUILD (for the built-tool reads of step 8).** A Windows build of master that holds #1707 (merged 26 Oct 2026 as `ca4acb9f`, the change that reads `cost_centre_allocations` on a voucher's ledger entries): the 0.6.1 cut build once its Build block elsewhere is filled, or another build of master after `ca4acb9f` that the run coordinator names, with its commit and exe sha256 recorded and the exe hashed before the first call (another hash = STOP). The build staged as `w1-2306d86d` predates #1707 and is NOT used here. If no such build is staged when the row runs, the built-tool reads of step 8 are recorded as not run and everything else runs. Run it over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026), and posting, import and redaction unset.

**WHO ACTS, AND DIALOGS.** Every master, setting and voucher is keyed on Tally's screen, one act at a time, with a screenshot before accepting and MARKS after. Before step 1 the lab asks the owner in its own chat whether to create and key this book, and keys nothing until the owner says yes. Any dialog or message box a step does not name = STOP (C7): press no key, send nothing more, screenshot, copy its text, report. Never Escape on a master or voucher screen (C7).

Licensed TallyPrime only; Educational mode = STOP. Record the release from Tally's About screen (the lab PC runs 7.2 since 25 Oct 2026).

## Steps
0. LAB-COMMON step 0 with no company loaded: C2 d is replaced by a screenshot showing none loaded. Check every file against `SHA256.txt` and run the request-ID check above.
1. OWNER'S YES first (above). Create Company `BRIDGE COST KEY LAB`: books beginning 1 April 2025, no password, base currency as offered, every other field as the screen offers; if the Features screen opens, accept it unchanged and record every value it shows. Load it alone. MARKS: record the GUID and both marks (EXPECT no voucher mark, or 0). Record the company number from the company list.
2. Backup BK0 (C5) into a NEW empty folder `C:\bridge-lab\1231cc\backups\bk0\`: name, size and sha256. Then MARKS.
3. F11 (Company Features): set "Maintain cost centres" (or the screen's own name for it) to Yes, nothing else. Record the screen path and the field's exact name; screenshot before accepting. MARKS.
4. Cost categories: create `CK Region` (allocate revenue items Yes, non-revenue items No, as the screen offers otherwise). Record the fields the screen shows. The predefined `Primary Cost Category` is used as it is. MARKS.
5. Cost centres, one at a time:
   - `CK North`, category `Primary Cost Category`, under Primary;
   - `CK North East`, category `Primary Cost Category`, under `CK North` (a centre with a parent);
   - `CK South`, category `Primary Cost Category`, under Primary;
   - `CK Retail`, category `CK Region`, under Primary.
   Record what the screen offers for each (category, parent). MARKS.
6. Ledgers: `CK Bank` under Bank Accounts (no opening balance); `CK Expense` under Indirect Expenses, with "Cost centres are applicable" (or the screen's own name) Yes. MARKS. Then backup BK1 into a NEW empty folder `C:\bridge-lab\1231cc\backups\bk1\`, then MARKS (M6).
7. Vouchers, keyed on the screen, each in voucher view, one at a time:
   - **K1, Payment, 15-05-2025:** Dr `CK Expense` 1,000.00 with its cost-centre allocation: category `Primary Cost Category`, `CK North` 600.00 and `CK North East` 400.00; Cr `CK Bank` 1,000.00. Narration `CK-K1`.
   - **K2, Journal, 16-05-2025:** Dr `CK Expense` 500.00 with two categories: `Primary Cost Category` `CK South` 500.00, and `CK Region` `CK Retail` 500.00; Cr `CK Bank` 500.00. Narration `CK-K2`. If the screen does not offer a second category on the same entry, record that, key only the first, and go on.
   The allocation screen opens on the Dr line; its prompts are answered only by keying the centres and amounts above. Screenshot each voucher before accepting, then the Day Book for each day. MARKS (M7): EXPECT ALTVCHID above M6's by at least two.
8. Reads, one at a time, in this order:
   - by curl: `cost-categories`, `cost-centres`, `vch-20250515`, `vch-20250516`, each answer to `evidence\<name>.xml` with headers (C3's line; record exit, status, bytes, CONTENT-LENGTH);
   - by the build's tools (only on a build holding #1707, above): `list_companies`; `company_features` (EXPECT the cost-centre setting `yes`); `masters` with `kind` `cost_centres` and then `cost_categories`; `vouchers` for 2025-05-15 to 2025-05-16. Record every answer whole.
   Then MARKS (M8): EXPECT equal to M7 (the reads wrote nothing).
9. LAB-COMMON C10. THE COMPANY stays on the PC as step 7 left it, for later reads; it is never used for another item.

STOP RULES: C7, C8 and C9; any dialog a step does not name; a voucher count other than two after step 7 (one if K2's second category was not offered, which is recorded); a mark that moves outside the steps that change it; an answer above its bound.

## What is recorded (nothing is judged at the lab)
- From each voucher answer, per entry of `CK Expense`: every `CATEGORYALLOCATIONS.LIST` (its `CATEGORY`, `ISDEEMEDPOSITIVE`) and every `COSTCENTREALLOCATIONS.LIST` inside it (`NAME`, `AMOUNT` as sent, and any other child), and whether the `CK Bank` entry carries any such element. Parsed, never grepped (C4).
- From the cost-centre answer: each centre's `NAME`, `PARENT` and `CATEGORY` (is `CK North East`'s `PARENT` `CK North`?).
- From the built tool: each entry's `cost_centre_allocations` (or `cost_centre_allocations_unreadable` with its reason) beside the raw allocation it came from; the `masters` rows; `company_features`' cost-centre setting.
The requester compares them with §8.2g and §12a.17; nothing here is built on until that comparison is read.

## Not measured by this item
Quantities in an allocation; a foreign-currency amount; an allocation on a Sales or Purchase invoice; the per-cost-centre summary report; Gold; any release other than the one recorded.

## Evidence
Every request's and answer's size and sha256, every answer whole (the curl answers with their headers), the MARKS answers, BK0's and BK1's names, sizes and sha256, the screen paths and field names, screenshots by name, size and sha256 (never pushed), and a notes file with UTC times and one line per step. THE COMPANY's GUID is new: it goes into the merge gate's synthetic-UUID registry when a pull request first cites it, or the evidence shows it as `<company guid>`. Evidence on a new branch `lab/1231-cost-centre-keyed-allocation-answers` with `SHA256.txt`; send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the run coordinator's yes.
