# Results: L4b (#1787, #1909), the not-loaded refusal on a build that names its remedy, TallyPrime 7.2, 26 Oct 2026. PASS: `company_identity_not_found` with its remediation, no cause, no rows

The sheet is `lab/72-read-regression-run-sheet` at 1de867736f58560265b9cf66ab15c5be83ab3c33, file `L4B-RERUN-AFTER-1909.md` (sha256 c9d3b67e... = the sheet's `SHA256.txt`). The argument template `args/l4-ledger-masters-not-loaded.json` was d5d6c975... (= the sheet's `SHA256.txt`). Its one placeholder was filled; the filled file is `args/l4-ledger-masters-not-loaded.json` here.

## Build and setting
- Bridge CI run 40362694215 on master fd6f7c0d032ec51f9d9de9c80e8bcd33d930ad77, artifact 12668144368. The zip's sha256 equals the digest 0a06f072...
- `.mcpb` 9572c5941b7045c374aaf3bb903ac073481b8e203b146ad14caf527da7fe3230.
- `bridge_mcp.exe` 63641231787492201133443cb1a887705c7137f495a9205ffd33c2d0068f7484, the same at the start (18:27 UTC) and at the end.
- `tools/list` (`answers/00-tools-list.json`): server `bridge-tally` **0.6.0**, 28 tools.
- Over MCP stdio, with `BRIDGE_TERMS_ACCEPTED=true`, port 9001 and a new empty data folder. `BRIDGE_AGENT_MAX_BYTES`, `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_IMPORT` were unset.
- TallyPrime Release 7.2, Silver Rental, not Educational mode; `tally.exe` FileVersion 1.1.7.2.

## Steps (UTC, 26 Oct 2026)
1. **BRIDGE ALIAS LAB** (100070) was loaded alone, with no question or prompt. `list_companies` (18:27:18, `answers/01-list_companies.json`) returned exactly that company: GUID 021bda0c-b1b1-44c8-8279-5212ce87c9c7, `identity_state` verified_tuple.
2. `ledger_masters` (18:27:26, `answers/02-ledger_masters-not-loaded.json`, saved whole) was called with `company_guid` cbe97898-88e6-42ee-b48f-5e1c3246105c. That is BRIDGE MANUAL NUMBERING LAB 1 (N1 of 25 Oct), which exists on this PC and was not loaded. `fields` was basic.
3. The answer:
   - `isError` true;
   - `result.error.code` **`company_identity_not_found`**;
   - `result.error.message` "ComplyEaze Bridge refused this operation.";
   - **`result.error.remediation` present**, beginning "The company list Tally returned held no company with this company_guid, so nothing from its books was returned." It goes on: "The company may not be open in Tally, or may have been closed while the read ran. Call list_companies and use a company_guid it returns; if the company the user named is not listed, ask the user to open it in Tally, then call list_companies again before repeating the request." It names `list_companies`;
   - **no `cause`** (the error carries `code`, `message` and `remediation` only);
   - no rows;
   - `evidence.state` partial, `evidence.reason_code` company_identity_not_found.
4. Egress `response_prepared` for this call (`answers/02-egress-response_prepared.line.txt`, data only):
   - `rows_prepared` 0, `bytes_prepared` 2010, `response_sha256` a7c947b1071a77026b9882760230468e11cf52e3cfb785fef325f479d9e5ddff;
   - the request trail: 4 sent, 0 failed. They are 2 posts (2,524 bytes out, 4,000 bytes back each: the company list) and 2 status reads. Nothing was read from N1's books.

**Compared with 25 Oct** (`lab/72-read-regression-answers`, L4, build 2306d86d): the code is the same (`company_identity_not_found`), and the remediation that was missing then is now present. That is #1909's change, seen on 7.2.

**Close.** BRIDGE ALIAS LAB was shut by the Company menu (Alt+K) > Shut > the one row, leaving Select Company with no company loaded, as found. One `tally.exe` owns port 9001, and GET /status returns 200 (51 bytes). Nothing was written; there was no dialog.

| file | bytes | sha256 |
|---|---|---|
| answers/00-tools-list.json | 148,286 | 9865488cb5538be26dcd9e165e602639ca1a9c83d5c6d73cbf08ecd8ef7140c1 |
| answers/01-list_companies.json | 1,580 | 0990ca004be042e76a398ed0c40ab1c2f2596da7e401af30235fe10aa0577bab |
| answers/02-ledger_masters-not-loaded.json | 2,350 | 3ac86d21bcad5c3f594340aec3c3d20d36d03a5dd51fc3419e69cfab510149d7 |
| answers/02-egress-response_prepared.line.txt | 1,215 | e2b9b414753f35faaf6561529eb76d60490a2d59ca3161d0d75d746b6eba2cef |

Screenshots (the company loaded, after the shut) stay on the lab PC.
