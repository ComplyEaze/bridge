# Row 738-A: a ledger import carrying an opening balance on a new synthetic company, then a small debtor and creditor book and its kept audit read: results

**Part A: MET.** On the new company BRIDGE ACCURACY LAB (TallyPrime 7.2 Silver):
- `a2-ledger`, sent once, created one ledger: CREATED 1, every other counter 0, no LINEERROR. ALTMSTID went 207 → 208, and ALTVCHID stayed unchanged (empty).
- `trial_balance` reads `ACC Creditor 01` under Sundry Creditors with opening 1000.00 (the credit side in Tally's sign), debit and credit present_empty, and closing 1000.00. (verified)

**Part B: PASS.** Every step from B0 to B9 was recorded and every EXPECT met:
- **The imports:** b2 created 14 ledgers and b3 created 6 Journals.
- **The book read back (B4):** all 15 rows of the sheet's table matched, and the totals balance.
- **The backups:** BK0 and BK1 were taken. BUILD-L was built on the lab PC from master 44be49d3.
- **The audit read:** it answered `completed` with no failed book check. B7 found it kept as one folder. B8 kept a second read whose parts are byte-equal to the first's, with manifests that differ only in `created_at` and `read_at`.
- **The screen:** no dialog showed and no stop rule fired. (verified)

The sheet is `lab/738-a/RUN-SHEET.md` at 7795e0b9; its SHA256.txt is 7 of 7 equal by sha256 and size, and LAB-COMMON v3 applies. C5 was waived for the company's creation and A2's one ledger, on the run lead's word. Every step and stamp is in `notes.txt` (UTC). THE COMPANY's GUID appears as captured, `3c21f661-5517-4809-8e17-0ea4830a1453`. The company number stays on the lab PC.

## Part A

| step | result |
|---|---|
| A0 | **Before:** Select Company listed no company named BRIDGE ACCURACY LAB.<br>**Create Company:** Company Name BRIDGE ACCURACY LAB; Country India; financial year and books beginning 1-Apr-25; base currency ₹ / INR. Every other field was left as Tally offered it: State Not Applicable, every contact field empty. No password, security or TallyVault was set.<br>**After "Accept ?" y:** Tally showed "Company created successfully" with the Company Features screen at its defaults, accepted unchanged with Ctrl+A. The Gateway then showed THE COMPANY alone, period 1-Apr-25 to 31-Mar-26, Current Date Tuesday 1-Apr-2025.<br>**m-a0:** 3,310 B, sha256 bccbcc8b35c4c91f13523f4c8dfd8ab1cadf742683ac318709f2dc8133b724e5; one COMPANY row; ALTVCHID empty; **ALTMSTID 207**.<br>**Dates:** PC 4-Nov-2026. (verified) |
| A1 | **list_companies:** exactly THE COMPANY, this GUID, `books_from` 20250401.<br>**tally_status:** release 7.2, licence silver, education mode false. (verified) |
| A2 | **Send:** `a2-ledger` once; curl exit 0, 200, 538 B = CONTENT-LENGTH.<br>**Counters:** CREATED 1; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED and EXCEPTIONS 0; no LINEERROR; LASTVCHID 0, LASTMID 0.<br>**Tally's screen:** the Gateway, no dialog.<br>**m-a2:** 3,310 B, sha256 48d9e287…; ALTVCHID empty (unchanged); **ALTMSTID 208** = M-a0 + 1. (verified) |
| A3 | **`trial_balance`**, 2025-04-01 to 2026-03-31: state observed, basis tally_native_trial_balance, 3 ledgers on one page, no `movements_differ_by` member.<br>**`ACC Creditor 01`**, parent `Sundry Creditors`: opening `{"state":"present","value":"1000.00"}`, debit `{"state":"present_empty"}`, credit `{"state":"present_empty"}`, closing `{"state":"present","value":"1000.00"}`.<br>**Also listed:** Cash (Cash-in-Hand) and Profit & Loss A/c (parent given as `&#4; Primary`), each with opening 0.00 and debit, credit and closing present_empty.<br>**totals:** opening sum 1000; closing sum 1000 (2 empty); debit and credit sums 0 (3 empty each). (verified) |
| A4 | **MET.** (verified) |

## Part B

| step | result |
|---|---|
| B0 | **C2 step 0:** the Gateway with THE COMPANY alone, no dialog; one tally.exe owns 9001; GET /status 200.<br>**m-b0:** byte-equal to m-a2. (verified) |
| B1 BK0 | **Folder:** `<LAB_DIR>\backups\bk0\`, new and empty before.<br>**Path:** Tally shows it as `<LAB_DIR>\backups\bk0`, because it capitalises the letter after a hyphen however it is typed. Windows paths ignore case, and the listing after shows the file in the named folder.<br>**The screen before accepting:** this path and one company. "Accept ?" y at 13:43 IST.<br>**File:** `TDBK1800_<company number>.001`, 23,662 B, sha256 aa4fa55e2b92aa7cab5a07e06c1679e12ee91d72d644a7cc46cbf39882120220.<br>**m-b1:** equal to m-b0. (verified) |
| B2 | **Send:** `b2-ledgers` once; 200, 540 B = CONTENT-LENGTH.<br>**Counters:** CREATED 14, every other counter 0, no LINEERROR.<br>**Tally's screen:** the Gateway, no dialog.<br>**m-b2:** ALTVCHID empty (unchanged); **ALTMSTID 222** = 208 + 14. (verified) |
| B3 | **Send:** `b3-vouchers` once; 200, 538 B = CONTENT-LENGTH.<br>**Counters:** CREATED 6, every other counter 0, no LINEERROR; LASTVCHID 6.<br>**Tally's screen:** the Gateway, no dialog.<br>**m-b3:** **ALTVCHID 6** (empty + 6); ALTMSTID 222. (verified) |
| B4 | **`trial_balance`**, same period: 17 ledgers on one page, no `movements_differ_by` member.<br>**The 15 rows:** each matches "What the book must hold after B3" by parent, opening, period debit, period credit and closing. (The tool gives the period debit as a negative figure and the credit as a positive one, and an empty field reads as nil.)<br>**`ACC Creditor 06`:** listed, with opening 0.00 and closing present_empty.<br>**Tally's own ledgers:** Cash and Profit & Loss A/c are also listed, each nil.<br>**totals:** opening 0, closing 0, debit −4600 and credit 4600; they balance. (verified) |
| B5 BK1 | **Folder:** `<LAB_DIR>\backups\bk1\`, new and empty before (shown as `<lab folder>\…`, as at BK0).<br>**Accept:** "Accept ?" y at 13:48 IST.<br>**File:** `TDBK1800_<company number>.001`, 30,188 B, sha256 724b20fea1feb8faf9b53a928e072008544ff154a122575ab27f82d8938ce57e. BK1 stays on the lab PC.<br>**m-b5:** byte-equal to m-b3. (verified) |
| B6 | **Switch:** BUILD-M quit; BUILD-L started.<br>**tools/list:** 38 tools, kept whole. `lab_import_masters` and `lab_import_vouchers` are among them; neither was called.<br>**list_companies:** this GUID.<br>**`audit_test_run`:** company_guid THE COMPANY's, 2025-04-01 to 2026-03-31, test `cash_44ab`, cash_groups `["Cash-in-Hand"]`, bank_groups `["Bank Accounts"]`. Answered in **5.997 s**, 7,278 B, no refusal.<br>**State:** **`completed`**. Book checks evaluated: ID-1, MAP-0, MAP-1, POP-0, POP-1, POP-2, POP-3 and POP-5; **failed: none**.<br>**Figures** (paise, or bp):<br>• applicable_threshold 1,000,000,000<br>• bank_payments, bank_receipts, cash_payments and cash_receipts 0<br>• cash_share_payments and cash_share_receipts null<br>**Finding:** `cash_44ab/applicability`, confidence needs_document, clauses s.44AB(a) and 3CD-8.<br>**Versions:** rules 2026-09-17.1, test 1.<br>The test's own result is recorded, not judged. egress_log was taken after it. (verified) |
| B7 | **What was kept:** `<data dir>\lab\` holds `audit-reads\` and `lab-manifest.jsonl`.<br>**The folder:** exactly one, `07eb38a479393e777cfd910b7c2cab96e01f6bfc01fa9b25cbb09577e22a1fb0`. It holds `manifest.json` and `parts\` and nothing else. `parts\` holds 8 plain files: company_object, groups, high_water_after, high_water_before, ledgers, trial_balance, voucher_types and vouchers-2025-04-01, each `.xml`.<br>**The manifest:** its sha256 **equals the folder's name**.<br>**`lab-manifest.jsonl`:** exactly one line, with THE COMPANY's `company_guid` and `manifest_sha256` equal to the folder's name.<br>The listing (`evidence/b7-lab-listing.txt`) gives every file by name, bytes and sha256. Nothing was copied out. (verified) |
| B8 | **The repeat:** `audit_test_run` again with B6's arguments, 23.5 s after B6; 5.941 s, 7,278 B, state completed. Its `result` member is equal to B6's (parsed).<br>**Second folder:** `0faacfff87c4…`.<br>**First folder:** its files are unchanged since B7 (sha256).<br>**The manifests:** the two `manifest.json` differ only in `created_at` and `read_at` (parsed).<br>**The parts:** both `parts\` folders hold the same 8 names, and each part is byte-equal to its namesake.<br>**No difference to report as a finding.** (`evidence/b8-lab-listing.txt`, 19 files) (verified) |
| B9 | **m-b9:** byte-equal to m-b5 (ALTVCHID 6, ALTMSTID 222).<br>**BUILD-L:** stopped.<br>**C10:** Shut Company by Tally's own menu. Click on the title band, Alt+K, Down ×4 to "SHut", Enter, then Enter on THE COMPANY (the only row). Select Company with no company loaded.<br>**After:** one tally.exe (pid 1200) owns 9001; GET /status 200.<br>THE COMPANY is kept, shut, not restored. (verified) |

## Builds and client

- **BUILD-M:** CI run <CI run id>, artifact <CI artifact id>. The zip's sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a equals the digest. `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0, equal to `mcpb-smoke.json`'s, before and after. (verified)
- **BUILD-L:** built on the lab PC in a clean detached checkout of master **44be49d3bc146c84ddcbf6e25920defe360b4a60**, which contains cd100807. The command was `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp --features lab-writes` (cargo and rustc 1.96.0). `bridge_mcp.exe` is 35,630,080 B, sha256 **21a66eabf2ca09d1baf708f2379343ba4e0b9b2626f49ff9571efbb114182edf**, the same before B6 and after B9. (verified)
  - The first build stopped in openssl-sys's build script: Git Bash's perl could not load `Locale::Maketext::Simple` for OpenSSL's Configure.
  - The same command was then run in the same checkout with `PERL` and `OPENSSL_SRC_PERL` set to the PC's Strawberry Perl. It finished in 27 min 07 s. No source changed. (The build log stays on the lab PC, listed in `kept-local.txt`.)
- **The client:** the 852-C1 inbox stdio client (`run/client.py`). It never cancels and has no call limit of its own. One process per build, each with a new empty data folder.
  - **BUILD-M:** TERMS true, host 127.0.0.1, port 9001; WRITES, BATCH_POST, TAX_AUDIT and REDACTION unset.
  - **BUILD-L:** the same, plus ENABLE_TAX_AUDIT true, LAB_WRITES 1 and LAB_TARGET_GUID THE COMPANY's; LAB_DENY_GUIDS, WRITES, IMPORT and REDACTION unset.
  - bridge-tally 0.7.0 both times. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number, or a run of 10 or more digits that is not inside a whole 64-hex or 40-hex hash value. Such files are listed in `kept-local.txt` by size and sha256, with structure lines in `evidence/kept-structure.txt`. The BK0 folder's listing before the backup and the build log also stay local, whole: each holds a word the evidence gate refuses. (verified)

Seven tool answers stay local because they carry the company number: both `list_companies`, `tally_status`, both `trial_balance` reads and both `audit_test_run` answers. The audit answers also carry the 10-digit threshold, 1,000,000,000 paise. Every value quoted above was read from these files by the lab. Each read's `result` member is also hashed as canonical JSON (`json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False).encode("utf-8")`, leaving out company, evidence, read_at and truncated); those bytes are written to no file. (verified)

| answer | canonical bytes | sha256 |
|---|---:|---|
| A3 `trial_balance` | 1,667 B | 1cf76d85c20d2880d0cdd38408877de59c4247d7350432fb55ba9ad21f5fc605 |
| B4 `trial_balance` | 5,593 B | 32f40dd3849f72d6606fd785eb821eaf6cac0cab1885b4182fa4c2d57dcf4b4b |
| B6 `audit_test_run` | 2,645 B | 4d878d770d5317d5538960df865e94c768ec4e513f3ee86e1115696011bd3bc9 |
| B8 `audit_test_run` | 2,645 B | the same sha256 as B6's (B6's and B8's results are equal) |

Everything else captured is pushed whole: the marks and import answers, the tools lists, `initialize`, both `egress_log` answers and the data folders' egress logs. They carry THE COMPANY's GUID as captured.

Also on the lab PC:
- the screens (`SCREENSHOTS.txt`);
- BK0 and BK1;
- the two kept audit reads (by name, bytes and sha256 in `evidence/b7-lab-listing.txt` and `evidence/b8-lab-listing.txt`);
- BUILD-L's exe. (verified)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
