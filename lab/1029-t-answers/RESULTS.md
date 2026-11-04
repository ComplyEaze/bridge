# Row 1029-T: how BRIDGE CORPUS DENSE numbers its voucher types: results

**Sales: `series_numbering.observation` observed, `method` automatic (`sub_method` auto_retain), `duplicates` not_prevented.** No other row has Sales as its parent. (verified)

**Reading (the sheet's Decides, third row): not as it stands. The tool decides nothing from a number under automatic, so row A's presence half cannot run read-only on this book as it stands.** (verified; the reading is the sheet's)

Sheet `lab/1029-t/ROW.md` at cc43826d (SHA256.txt 3 of 3 equal). Every EXPECT was met, no prompt or dialog showed, and no stop rule fired. Every step and stamp is in `notes.txt` (UTC).

## Every voucher-type row (`masters`, `kind=voucher_types`; 24 rows, one page)

| name | parent | observation | method | sub_method | duplicates |
|---|---|---|---|---|---|
| Attendance | Attendance | observed | automatic | auto_retain | not_prevented |
| Contra | Contra | observed | automatic | auto_retain | not_prevented |
| Credit Note | Credit Note | observed | automatic | auto_retain | not_prevented |
| Debit Note | Debit Note | observed | automatic | auto_retain | not_prevented |
| Delivery Note | Delivery Note | observed | automatic | auto_retain | not_prevented |
| Job Work In Order | Job Work In Order | observed | automatic | auto_retain | not_prevented |
| Job Work Out Order | Job Work Out Order | observed | automatic | auto_retain | not_prevented |
| Journal | Journal | observed | automatic | auto_retain | not_prevented |
| Material In | Material In | observed | automatic | auto_retain | not_prevented |
| Material Out | Material Out | observed | automatic | auto_retain | not_prevented |
| Memorandum | Memorandum | observed | automatic | auto_retain | not_prevented |
| Payment | Payment | observed | automatic | auto_retain | not_prevented |
| Payroll | Payroll | observed | automatic | auto_retain | not_prevented |
| Physical Stock | Physical Stock | observed | automatic | auto_retain | not_prevented |
| Purchase | Purchase | observed | automatic | auto_retain | not_prevented |
| Purchase Order | Purchase Order | observed | automatic | auto_retain | not_prevented |
| Receipt | Receipt | observed | automatic | auto_retain | not_prevented |
| Receipt Note | Receipt Note | observed | automatic | auto_retain | not_prevented |
| Rejections In | Rejections In | observed | automatic | auto_retain | not_prevented |
| Rejections Out | Rejections Out | observed | automatic | auto_retain | not_prevented |
| Reversing Journal | Reversing Journal | observed | automatic | auto_retain | not_prevented |
| **Sales** | **Sales** | **observed** | **automatic** | **auto_retain** | **not_prevented** |
| Sales Order | Sales Order | observed | automatic | auto_retain | not_prevented |
| Stock Journal | Stock Journal | observed | automatic | auto_retain | not_prevented |

Each row also carries `numbering_method` default. The answer reports `state` observed, `total` 24, `offset` 0, `next_offset` null, and `verification` stable_paired_source_with_company_mode_and_extent_guards. It carries no `first_pair_differed` member. Its snapshot read V 29,900 and M 328. The answer took 0.876 s. Since no `next_offset` was given, it was called once. (verified)

## The rest of the Record

| | |
|---|---|
| m1 (`reads/m1-marks.xml`) | 3,406 B, sha256 fcc9086dfa112813c447d23e98eb4500c9a1badbe983d3f8d0c0ceff11dc8430; one COMPANY row, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, **V 29,900, M 328** (EXPECT met) |
| m2 (`reads/m2-marks.xml`) | 3,406 B, the same sha256: **byte-equal to m1** (EXPECT met) |
| data folder, company shut, before | 20 files (EXPECT 20) |
| data folder, loaded | 23 files (EXPECT 23) |
| data folder, shut, after | 20 files (EXPECT 20) |
| `list_companies` | this GUID, `books_from` 20250401, verified_tuple (EXPECT met) |
| `tally_status` | TallyPrime release 7.2, licence silver, education mode false |
| dates | Tally's Current Date: Tuesday 31-Mar-2026. The PC's date: 4-Nov-2026. |
| client | the 852-C1 inbox stdio client (`run/client.py`): no cancel, no call limit of its own. `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9001, one new empty data folder. `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_BATCH_POST` and `BRIDGE_AGENT_REDACTION` unset. One `bridge_mcp` (bridge-tally 0.7.0), stopped by the client with exit code 0. |
| build | BUILD-M: CI run <CI run id>, artifact <CI artifact id>; zip sha256 1940bd9736edc2aa76d811b3bcd0d6229ee69b5e01f5f59321a46b6ca28f691a = the digest; `bridge_mcp.exe` 9bda7144f54e5ef68477b5ca30be33a17759de2e1ad756f840785c67bcab2dc0 before and after, equal to `mcpb-smoke.json`'s |
| close | Shut Company by Tally's own menu: a click on the title band, Alt+K, Down ×4 to "SHut", Enter, and Enter on BRIDGE CORPUS DENSE (the only row). Tally then shows Select Company with no company loaded. One tally.exe (pid 1200) owns 9001. |

The only keys pressed in Tally were the load and the Shut. Nothing was written, backed up or restored. (verified)

## Kept on the lab PC

No captured byte is altered. A captured file is pushed whole unless it holds the company number, or a run of 10 or more digits that is not inside a whole 64-hex or 40-hex hash value. Such files stay on the lab PC, listed in `kept-local.txt` by size and sha256, with structure lines in `evidence/kept-structure.txt`. Screens stay local, listed in `SCREENSHOTS.txt`. (verified)

- **Three tool answers carry the company number:** `list_companies` (result key companies[1]), `tally_status` (loaded_companies[1]) and the `masters` read (`run/answers/003-s3-voucher-types-p0.json`, 28,943 B). The `masters` read's result keys are basis, company_guid, kind, limitations[14], masters[24], next_offset, offset, snapshot, state, total and verification.
  - The table above was read from that file by the lab, field by field. Its `result` member, as canonical JSON (`json.dumps(obj, sort_keys=True, separators=(",",":"), ensure_ascii=False).encode("utf-8")`, leaving out company, evidence, read_at and truncated), is 12,836 B with sha256 074569b236247acd2fa526a86691a351a502942de22b9d49d6cd3d51cb029f9e (`evidence/kept-canonical.txt`). Those bytes are written to no file.
- **m1 and m2, the tools list, `initialize`** and **the data folder's files** hold neither the company number nor such a run. They are pushed whole.

The four desktop-client connector servers stayed running: no call was made to them from this sitting, and their calls were not measured.
