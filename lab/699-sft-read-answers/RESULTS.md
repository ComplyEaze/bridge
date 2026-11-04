# Row 699-R1: the audit read of BRIDGE SFT LAB, through a recording relay: results

**First line: every EXPECT met; no refusal, no prompt, no `refused` line in any manifest, no answer over 2,000,000 bytes. `audit_test_run` answered `completed` with no failed book check. One recorded difference, not a stop: s1's answer is not byte-equal to the 26 Oct a4 answer (4,878 B against 4,888 B), with the same GUID, V and M.** (verified)

Sheet `lab/699-sft-read/ROW.md` at 66cc762c (SHA256.txt 5 of 5, by sha256 and size). Every step and stamp is in `notes.txt` (UTC).

| step | result |
|---|---|
| s1: V, M, sha256; equal to the 26 Oct a4 answer or not; Tally's release; the dates | One COMPANY row, GUID 00f0c618-b0df-43bd-bd69-b4c7b92c616f, **V 34, M 219**, 4,878 B, sha256 6d3aebaa05b2d03b8384df58fb940adae098fb82e33275c8834599fce3ff933a. **Not** equal to the 26 Oct a4 answer (4,888 B, sha256 5431ca42…), though GUID, V and M are equal. TallyPrime 7.2 Silver. Tally's Current Date was Tuesday 10-Mar-2026; the PC's date was 2026-11-04. (verified) |
| build: zip digest; `bridge_mcp.exe` sha256 at steps 3 and 7 | CI run <CI run id> (push of master d77d0682, success). Zip sha256 bd7e796174a185b1523258c3052acad67c29a7d8b2e68d99d5b12d6ce60cc2bf = the digest. `bridge_mcp.exe` c41fe5df31b3dc6a81c1eb90848837012579c041b6034531721c80c2cabb531c at step 3 and at step 7, equal to `mcpb-smoke.json`'s. (verified) |
| relay self-test | `self-test: ok (7 forwarded unchanged, 1 answer withheld, 15 refused, none of the refused reached Tally)`. (verified) |
| r1: `list_companies`, seconds; manifest lines | One open company, BRIDGE SFT LAB, this GUID, verified_tuple, books from 20250401; 0.373 s. Tools list: 33 tools, `audit_test_run` among them. Manifest: 4 lines, each forwarded, 200 and answer returned. (verified) |
| r2: `audit_test_run` answer, seconds; refusal code if any; manifest lines, each with method and bytes; largest answer | Answered with a result, 6.681 s, 7,315 B. No refusal. Manifest: **34 lines**, each forwarded, 200 and answer returned (2 GET /status, 32 POST). Request and response bytes per line are in `notes.txt` and the manifest. Largest answer 1,103,236 B. No `events.txt`. (verified) |
| r2: `result.state`, figures, findings, failed book checks | `completed`. Book checks evaluated: ID-1, MAP-0, MAP-1, POP-0, POP-1, POP-2, POP-3, POP-5; **failed: none**. Figures (paise, unless bp): applicable_threshold 1,000,000,000; bank_payments 0; bank_receipts 40,000,000; cash_payments 5,000,000; cash_receipts 461,000,001; cash_share_payments 10,000 bp; cash_share_receipts 9,202 bp. Finding: `cash_44ab/applicability`, confidence `needs_document`, clauses s.44AB(a) and 3CD-8. Rules version 2026-09-17.1, test version 1. (verified) |
| `read_evidence`: records, their states | 2 records, both `complete`: one for `list_companies`, one for the audit read. (verified) |
| r3: vouchers by type and in all; pages; manifest lines | **34 in all**: 13 Sales, 19 Receipt, 1 Payment, 1 Contra, each with this GUID's prefix. One page (no next offset), 2.963 s. Type, number and date equal A5-VOUCHERS.md's 34 rows (`evidence/s6-date-check.txt`). Manifest: 22 lines, each forwarded, 200 and answer returned. (verified) |
| s9: V, M, sha256 equal to s1's or not | V 34, M 219. The answer is **byte-identical to s1's** (6d3aebaa…), read one minute after `bridge_mcp` stopped. (verified) |

## Kept on the lab PC (the run lead's decision (c))

No captured byte is altered. A captured file is pushed whole unless it holds the company number or a digit run of 10 or more outside VOUCHERKEY. Each such file stays on the lab PC, listed in `kept-local.txt` by size and sha256. Each kept file's structure is below, with no values given (tags and keys from `evidence/kept-structure.txt`). (verified)
- **The company and marks reads in the relay folders** (r1 2, r2 22, r3 8, 32 in all): byte-equal, 1 distinct sha256, 3,992 B each. XML: ENVELOPE/HEADER (VERSION, STATUS) and ENVELOPE/BODY (DESC, DATA); one COLLECTION/COMPANY with NAME, GUID, COMPANYNUMBER, BOOKSFROM, BRIDGERELEASE, EDUMODE, GOLD, SILVER and PRODUCTNAME, one each. The requests that drew them are pushed. (verified)
- **`list_companies`' answer** (`run/answers/001-s3-list-companies.json`): 1,561 B. JSON; the tool result's key `companies`, a list of 1. (verified)
- **`audit_test_run`'s answer** (`run/answers/002-s4-audit-test-run.json`): JSON. The tool result's keys are book_checks, figures (7), findings (1), limitations (2), population_note, state and test. Besides the company number, it holds the 10-digit threshold, 1,000,000,000 paise. (verified)
  - whole file: 7,315 B, sha256 f020268b79d7ca9aa95da35aa9d524f2b548ca3ca5a9fd46cfa41cccbe1a9ffe
  - audit result (canonical JSON): 2667 B, sha256 f41152587c9b287c4413881bbad67fc4049aa640d61b377724f0f5d0c360a3e4

  The second line is the `result` member (`structuredContent.result`) alone, as `json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False).encode("utf-8")`. It leaves out `company`, `evidence`, `read_at` and `truncated`, which change from run to run. Its bytes are written to no file. (verified)
- **`vouchers`' answer** (`run/answers/004-s6-vouchers-p0.json`): 40,318 B. JSON; the tool result's keys are items (34), offset, profile, reason, snapshot, state, total and window. It also carries the company number, so it stays here. r3's relay pages, which the build read to make it, are pushed whole. (verified)
- **s1 and s9** (`reads/s1-marks.xml`, `reads/s9-marks.xml`): 4,878 B each, byte-equal. XML: one COLLECTION/COMPANY with NAME, GUID, COMPANYNUMBER, ALTVCHID, ALTMSTID, BOOKSFROM, LASTVOUCHERDATE and the NUM… counts, one each. (verified)
- **The 26 Oct a4 answer, copied for the comparison** (`evidence/a4-marks-26oct.xml`): 4,888 B, the same shape. (verified)

## Decides (the sheet's)

r2 holds every answer, `audit_test_run` answers with a result, `read_evidence` is all complete, r3 counts 34, and s9 equals s1. So `r2`'s exchanges become the fixtures of clause 42's tests, measured against the synthetic lab book only. (verified; the reading is the sheet's)

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
