# 697 clause 21(a) synthetic book (#697): a new synthetic book that holds clause 21(a) candidates, then the parity run on it. WRITE sitting (keyed on screen), then READ-ONLY.

Not for merge. Written 26 Oct 2026. `SHA256.txt` lists every file here with its bytes; check it before step A0.

Why: the `clause21a_candidates` parity record (`lab/697-clause21a-parity`) needs a book with at least one candidate on each side, and no synthetic lab book has one: `BRIDGE SFT LAB` has no ledger under Purchase Accounts, Direct Expenses or Indirect Expenses. This item keys a small synthetic book whose debits hit the test's own rules, so the record needs no client book. What such a record proves: the Rust port and the reference engine give the same answer on a real Tally read of a book with candidates. What it does not prove: how well the keyword table finds candidates in a real client's books (this book is keyed from that table). Whether a synthetic-book record meets `audit_test_run`'s admission bar for `clause21a_candidates` is the run coordinator's ruling, given before the sitting. The candidates come from the test as the crate states it (`src-tauri/crates/bridge-tax-audit/src/clause21a_candidates.rs` and its keyword table `rules/clause21a_keywords.toml`, sha256 28cb01475ab4ca72b8f9fb1cfb29ef4dcace3f56614d6fb64703d6685b4721b3): a debit on a ledger under those three groups whose ledger name or narration holds a listed word, and a debit in a voucher that credits a Capital Account ledger.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is `BRIDGE C21A LAB`. It does not exist yet: Phase A creates it. If a company of this name exists on the PC, STOP and report it.

Differences from LAB-COMMON, the stricter rule holding everywhere else:
1. **No raw requests.** The operator sends no request file. Every read is a call of ComplyEaze Bridge's built tool, one at a time. C2 step 0 runs WITHOUT its MARKS request; its GET /status checks still run.
2. **Marks.** `tally_status` carries no marks. M (ALTMSTID) and V (ALTVCHID) are read with `ledger_masters` (first page, no `snapshot_id`): `snapshot.master_alter_id` and `snapshot.voucher_alter_id`; record the whole answer and `snapshot.reused`, which must read false. A book with no voucher may give no V: record it.
3. **Writes.** Every write is keyed by hand on Tally's own screens, one act at a time, with a screenshot before accepting each, on the run coordinator's word. (The build's lab import tools are not used: this book's vouchers are the reference shapes a person keys, not an import.) Nothing is imported and nothing is posted by ComplyEaze Bridge. Grant: the company, the ledgers of A3 and the eight vouchers of A4; nothing else.
4. **Backup.** C5's backup before the first change cannot apply to a company that does not exist yet; the run coordinator waives it for the company's creation and its ledgers. One backup, BK1, of THE COMPANY after its ledgers and before its first voucher, into the new empty folder `C:\bridge-lab\697c21a\backups\bk1\`. No restore is planned: THE COMPANY is this item's own and is kept, shut, for later re-reads.

Licensed mode only (vouchers on ordinary days); Education mode = STOP. TallyPrime 7.2 expected: record the release.

## Phase A: build the book
A0. LAB-COMMON step 0 with no company loaded, without its MARKS read; C2 d's Gateway observation is replaced by Select Company's list (THE COMPANY does not exist yet). EXPECT no company named `BRIDGE C21A LAB` in Select Company's list.

A1. Create Company `BRIDGE C21A LAB`: financial year and books beginning 1-4-2025, no password, every other field as the screen offers; if the Features screen opens, accept it unchanged and record every value it shows. Screenshot before accepting. Load it alone. Run `list_companies` (Phase A's build and settings, below): EXPECT exactly THE COMPANY with `identity_state` `verified_tuple`; record its `guid` and company number. `ledger_masters`: M0, V0.

A2. Make no change to any other company in this sitting.

A3. Create these ledgers, one at a time (no opening balance, "Maintain balances bill-by-bill" No if the screen shows it, no GSTIN, every other field as the screen offers):

   | Ledger | Group |
   |---|---|
   | `C21 Bank` | Bank Accounts |
   | `C21 Capital` | Capital Account |
   | `C21 Sales` | Sales Accounts |
   | `C21 Software Expenses` | Indirect Expenses |
   | `C21 Club Charges` | Indirect Expenses |
   | `C21 Penalty Paid` | Indirect Expenses |
   | `C21 Office Rent` | Indirect Expenses |

   The company's own `Cash` ledger (Cash-in-Hand) is used as it is. Then `ledger_masters`: EXPECT the seven ledgers under those groups, and the company's default ledgers `Cash` and `Profit & Loss A/c`.

A3b. Backup BK1 (Difference 4): its name, size and sha256. `ledger_masters`: M1, V1 (a mark that moved across the backup is recorded, not a STOP).

A4. Key these eight vouchers, one at a time, each exactly as written (one line per ledger named, nothing else; the narration typed as given). Payment, Receipt and Contra open in TallyPrime's single-entry mode, with an "Account" field and "Particulars": for a Payment, Account = the ledger credited (Cash or C21 Bank) and Particulars = the ledger debited; for a Receipt, Account = Cash and Particulars = the ledger credited; for the Contra V2, after saving it, its Day Book row must read C21 Bank Dr 50,000.00 and Cash Cr 50,000.00: if the sides are reversed, record it and STOP. The Journal V8 opens with Dr and Cr lines:

   | # | Type | Date | Debit | Credit | Amount | Narration |
   |---|---|---|---|---|---|---|
   | V1 | Receipt | 1-4-2025 | Cash | C21 Capital | 2,00,000.00 | capital introduced |
   | V2 | Contra | 2-4-2025 | C21 Bank | Cash | 50,000.00 | cash deposited |
   | V3 | Payment | 10-4-2025 | C21 Software Expenses | Cash | 12,000.00 | annual software licence renewal |
   | V4 | Payment | 15-5-2025 | C21 Club Charges | Cash | 5,000.00 | club annual membership subscription |
   | V5 | Payment | 20-6-2025 | C21 Penalty Paid | C21 Bank | 1,000.00 | penalty for late return |
   | V6 | Payment | 1-7-2025 | C21 Office Rent | Cash | 20,000.00 | office rent for July |
   | V7 | Receipt | 5-8-2025 | Cash | C21 Sales | 30,000.00 | cash sales |
   | V8 | Journal | 31-3-2026 | C21 Office Rent | C21 Capital | 3,000.00 | rent paid by proprietor |

   What each is for (from the test's rules; the parity run, not this table, is the record): V3 a capital candidate (`software`, `licence`); V4 a club fee candidate (`club` with `membership`, `subscription`); V5 a penalty candidate (`penalty`); V8 a personal candidate (a debit in a voucher that credits a Capital Account ledger); V6 a control with no listed word; V1, V2 and V7 have no debit on the three groups. Cash never goes below zero.

   Then screenshots (they stay on the lab PC): the Day Book for 1-4-2025 to 31-3-2026 and the Profit and Loss account for the year (Alt+F2 for each period; record Tally's current date before and after, and set the period and the current date back before C10). `ledger_masters`: M2, V2 (record both: how far V moves for eight new vouchers is recorded, not predicted). `vouchers` for 2025-04-01 to 2026-03-31 (every page; the tool reads the window with its own census, and on an eight-voucher book the read is small): EXPECT eight vouchers as keyed (types, dates, ledgers, amounts, narrations).

A5. Record, as a table, each voucher's number, type, date, every line's ledger and amount, and its narration, all as keyed. Then LAB-COMMON C10.

STOP: on any dialog other than a screen's own accept (C7), the Company Features screen of A1 being one such screen to accept unchanged; on any question about a feature (GST, inventory, cost centres) asked while a ledger or voucher is keyed; a voucher count other than eight after A4.

## Phase B and C: the parity run on THE COMPANY
Run `lab/697-clause21a-parity` at `f0f366d8e13799dac3c8b53ddb10df9caf14fad4` (or a later head of that branch the run coordinator names) in full, with:
- THE COMPANY there = `BRIDGE C21A LAB`, its GUID as A1 recorded it, named before its B0 (a synthetic book: no client-book grant applies);
- `cash_groups` `["Cash-in-Hand"]` and `bank_groups` `["Bank Accounts"]` (the same lists in its C1 client config);
- its build and environment as that sheet says (a lab-writes build of master holding #1949, `BRIDGE_LAB_TARGET_GUID` = THE COMPANY's GUID).

Phase A does not wait for that build. Its reads use the build already staged at the lab as `w1-2306d86d` (master `2306d86dfd4c86a3c5f6c19ea9ddff6818c460fb`, artifact `12553782422`, `bridge_mcp.exe` sha256 `92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524`: hash it before the first call; another hash = STOP), run over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, a new empty `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026, as the parity sheet's environment also records), and nothing else (no lab writes, no posting, no redaction).

EXPECT of that run's C3 (recorded, not predicted): `identical_json=true`, `differences=0`, and `figures` at least 2. If it reports `PARITY REFUSED: both sides report zero figures`, this book's candidates did not register on either side: report it with A5's table; that is a finding about this sheet's design, not about the engines.

## Evidence and after
Phase A: every tool answer whole with its size and sha256, A5's table, BK1's name, size and sha256, the screenshots by name, size and sha256 (never pushed), and a notes file with UTC times and one line per step, on a new branch `lab/697-c21a-synthetic-book-answers` with `SHA256.txt`. THE COMPANY's GUID is new: it goes into the merge gate's synthetic-UUID registry before any evidence that names it is pushed, or the evidence shows it as `<company guid>`. Phase B and C: as `lab/697-clause21a-parity` says (a synthetic book's kept read may become a fixture candidate only after the privacy scan and the registry check). Send each SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured by this item
Every other clause 21(a) item (political advertisement, club services, late fees, statutory interest, compounding, benefits, settlements, the large-entry rule); a partner's interest or remuneration ledger (the s.40(b) exception); a client book's spread of words; Gold.
