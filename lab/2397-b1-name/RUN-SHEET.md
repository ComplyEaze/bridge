# 2397 B1: a company's name through the identity check of #2397, on two builds, and the longest name Tally stores

Not for merge. Part of #2397 (part B1). Written 2026-11-09. About 90 minutes; the owner about 5 of them, at O1. One Tally write, in row N3 only, made on Tally's own screen and undone by a restore. Nothing is posted, imported or created.

`LAB-COMMON.md` in this folder applies in full (v3, a byte copy of `lab/2246-split-journal:lab/2246-split-journal/LAB-COMMON.md`, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, 12,062 bytes). Where it and this sheet differ, the stricter holds, except the differences stated under Rules. `SHA256.txt` lists every file here with its bytes; check it before step S0.

Marks: V = read at a named source; U = not measured. Every outcome is recorded as found, never explained away. Confidence of every EXPECT below until the sheet runs: unverified (they come from reading the code, not from a run).

## What it decides

The change on `fix/2397-identity-company-name` (head 85aad284, base master 10f38816, pull request #2431) makes a Bridge company identity hold its name as a `ValidatedCompanyName` and refuses, when a company is selected, a name that is empty after trimming, over 255 bytes untrimmed, or holds a control character, with the code `company_name_invalid`. This sheet asks three questions, one variable each:

- **N1:** on a synthetic book with an ordinary name, does the B1 build answer `tally_status`, `list_companies` and one bounded `trial_balance` exactly as the master build does, apart from times and ids?
- **N2:** on the lab company with the longest stored name (150 characters, 150 bytes, by the protocol reference), does the B1 build accept it, as master does?
- **N3:** what is the longest company name Tally stores, in characters and bytes, when 300 are typed, and what does each build answer for that company?

**Found by reading, before any run (V, the section "The code admits each trigger"):** through `bridge_mcp.exe`, no row here can be answered `company_name_invalid`. The company-list parser that every one of these tools goes through refuses a name over 255 bytes or holding a control character first, on both builds, and the whole list with it. A name over 255 bytes is therefore answered `company_collection_invalid` by `list_companies` and by `trial_balance` on both builds, and `tally_status` shows no loaded company. The head's own comment says so (`agent_company.rs:143-144`). N3 measures this live. `company_name_invalid` from either build would contradict the reading and is a finding in its own right.

## BUILD

Two builds, each the Windows CI artifact `unsigned-bundle-smoke-windows-latest` of a named workflow run, as `lab/2246-split-journal` used. Each is downloaded only on the run coordinator's yes, into an empty folder of its own. The zip's sha256 is checked against the digest written here; another digest = STOP.

**RUN-B1:** the full CI run dispatched by hand on `fix/2397-identity-company-name` at head 85aad284: the run whose id ends 54584. Pull request #2431's head is now 88470425, which adds only the acknowledgement file, so the B1 build is this 85aad284 run. To be filled by the run coordinator once the run finishes:
- run id, in words or split (never ten digits together): the run whose id ends 54584 (V, the run coordinator)
- artifact name: `unsigned-bundle-smoke-windows-latest`
- artifact id, bytes, and the time it expires: ________
- zip digest (`sha256:` and 64 hex digits): ________
- `bridge_mcp.exe` sha256: ________

**RUN-M (master):** a workflow run whose head is master 10f38816. To be filled by the run coordinator:
- run id, in words or split: ________
- artifact name: `unsigned-bundle-smoke-windows-latest`
- artifact id, bytes, and the time it expires: ________
- zip digest: ________
- `bridge_mcp.exe` sha256: ________

Whether a run at 10f38816 holds that artifact is U; if none does, the run coordinator names another way to a master build at 10f38816, and the sheet waits.

For each build, find `bridge_mcp.exe` in the unpacked bundle (inside its `.mcpb`, itself a zip, under `bin\`; `lab/1459-s3-run-sheet` found it at `bin\x86_64-pc-windows-msvc\`) and record the path found. Copy it to `C:\bridge-lab\2397-b1\<build>\bin\bridge_mcp.exe`, where `<build>` is `b1` or `master`, and write its sha256, lowercase and alone, to `C:\bridge-lab\2397-b1\<build>\exe-sha256.txt`. It must equal the value above; another = STOP. `client.py` checks it at every launch. Hash both again at the close; another hash = STOP. Nothing is built on the lab PC.

## The code admits each trigger (read, not run)

At head 85aad284 unless named. The files outside the pull request's diff are byte-equal at 10f38816 and 85aad284 (`git diff --quiet`, checked when the sheet was written: `agent.rs`, `agent_terms.rs`, `agent_trial_balance.rs`, `tally/runtime.rs`, `tally/connection.rs`, the protocol crate's `lib.rs` and `xml_read_profiles.rs`), so those lines are the same on both builds.

- **Where the identity is built for the tools this sheet sends.** `trial_balance` calls `verified_company` (`src-tauri/src/agent_trial_balance.rs:31`), which reads the company list (`src-tauri/src/agent_company.rs:92`) and builds the identity by `VerifiedCompanyIdentity::from_observed_companies` (`agent_company.rs:134-140`). `list_companies` (`src-tauri/src/agent.rs:3445-3457`) and `tally_status` (`agent.rs:3486-3494`) build no identity: they print the list (`agent_company.rs:182-220`, whose `identity_state` looks at no name length) and the probe (`agent_company.rs:6-63`).
- **The check.** `from_observed_companies` (`src-tauri/src/tally/mod.rs:80`) builds `ValidatedCompanyName` from the name first, before the number, books-from and ambiguity checks (`mod.rs:87-91`). `ValidatedCompanyName::new` refuses a name that is empty after trimming, over 255 bytes, or holds a control character (`src-tauri/crates/bridge-tally-protocol/src/xml_read_profiles.rs:51-57`). The identity holds that type (`mod.rs:35-43`). The error has one variant for a name, `InvalidCompanyName`, which carries no text (`mod.rs:45-56`).
- **Where the code is mapped, and that no other code is given for that variant.** The three production matches map `InvalidCompanyName` to `company_name_invalid` and to nothing else: the MCP server's (`agent_company.rs:145-147`), the desktop app's selection (`src-tauri/src/commands.rs:1969-1976`) and the sync connector's (`src-tauri/src/tally/connector.rs:430-432`). Only the first is in `bridge_mcp.exe`'s path for these tools: `commands.rs` is the desktop app's command layer, and `RuntimeTallyConnector` is built only there (`commands.rs:1473`, `:1702`) and held by the sync coordinator (`src-tauri/src/sync/coordinator.rs:46`).
- **What reaches it first, on both builds.** Every company list the MCP server reads is `fetch_agent_companies` (`src-tauri/src/tally/runtime.rs:3004-3024`), parsed by `parse_companies_from_collection` (`runtime.rs:902-913`; `src-tauri/crates/bridge-tally-protocol/src/lib.rs:353-356`). Each row's `NAME` attribute goes through `normalized_standard_value` (`lib.rs:523-526`), which trims it and refuses it if it is empty, over 255 bytes or holds a control character (`lib.rs:427-433`), failing the whole list. The server answers that as `company_collection_invalid` (`agent_company.rs:65-70`; `ToolFailure::from_runtime`, `agent.rs:2646`, replaces the code it is given only for a cancelled call, a wire refusal or the typed read errors it names, and otherwise keeps it, `:2763-2765`). The name that reaches the identity is that trimmed, already-checked name, so for these tools the identity check is never the first to refuse: the head says so at `agent_company.rs:143-144`.
- **`tally_status` with such a name.** The probe parses the same collection (`src-tauri/src/tally/connection.rs:1938`). On a parse failure of a status-1 answer its reason is `xml_export_shape_unrecognized` (`connection.rs:1963-1976`), the loaded-companies feature becomes Unknown with that reason (`connection.rs:1777-1796`), and `tally_status` shows `loaded_companies` empty with that `refusal_reason` (`agent_company.rs:33-43`, `:56-57`). The product and mode facts are read only after a parse that succeeds (`connection.rs:1939-1942`), so `product` is expected `not_observed` and `education_mode` null.
- **The request the native read sends.** `C-company-list.utf16le.xml` is Bridge's own company-list request: `ReadOnlyProfile::CompanyListV2` (`xml_read_profiles.rs:375-411`), encoded with a byte-order mark (`src-tauri/crates/bridge-tally-protocol/src/text_encoding.rs:42-49`), is byte-equal to the captured file (2,524 bytes, sha256 `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8`, computed when the sheet was written). So the native read in N3 asks Tally exactly what both builds ask it, and each build's `list_companies` evidence `request_sha256` is expected to be that value (`agent_company.rs:72-74`).
- **The Terms gate.** Every tool call is refused `terms_not_accepted` until `BRIDGE_TERMS_ACCEPTED` is `true` or `1` (`agent.rs:3392-3394`; `src-tauri/src/agent_terms.rs:66-74`). When it is, the server appends a line to `terms-acceptance.jsonl` in its data folder at start (`agent_terms.rs:13-16`, `:110-120`), and refuses `terms_record_unavailable` if it cannot. The data folder is `BRIDGE_AGENT_DATA_DIR` when set (`agent.rs:587-589`), created if absent.

**What a master build does with the same company (at 10f38816).** The identity holds the name as a plain `String` (`src-tauri/src/tally/mod.rs:35-40`) and `from_observed_companies` (`mod.rs:73-82`) checks the number and books-from but not the name. The MCP server's identity build is the same call (`src-tauri/src/agent_company.rs:134`) with no name arm. Master measured the name only where a request builder took it: the two Object requests (`src-tauri/src/tally/agent_read_request.rs:16-21`, called by `company_object` at `:114` and `invoice_company_hsn` at `:132`, which none of this sheet's tools sends) and builders such as the book-extent read (`src-tauri/src/tally/connection.rs:3089`), each on the parser's trimmed name. The parser, the list, the probe and the Terms gate are the lines above, unchanged. So by reading, master answers every row of this sheet as B1 does: an ordinary or 150-character name is accepted, and a name over 255 bytes is refused at the company list with the same codes.

## THE OWNER'S ACTS (every other act is the lab operator's)

STANDING RULE (as the earlier sheets): Terms acceptance is the owner's act in every form. This sheet writes the marker word OWNER-ACCEPTS-TERMS-HERE and never a value. Posting, imports and the tax-audit box are not touched by this sheet in any form.

| act | who | exactly what | the operator checks after | about (min) |
|---|---|---|---|---|
| O1 | OWNER | In each of `C:\bridge-lab\2397-b1\b1\launch-env.json` and `C:\bridge-lab\2397-b1\master\launch-env.json`, which the operator wrote at S3 with the marker in place of a value, set `BRIDGE_TERMS_ACCEPTED` to the value that accepts the Terms: OWNER-ACCEPTS-TERMS-HERE. Save each file as UTF-8 without a byte-order mark. | each file still parses as JSON with exactly the four keys of S3, the other three unchanged; record each file's sha256 and bytes (never its value in any note) | 3 |

That is how `lab/2246-split-journal` gave its build the Terms: the setting in the launch file, which the server records in its own data folder. Here each build has its own data folder under `C:\bridge-lab\2397-b1\`, so neither touches the installed extension's data folder or its record. The operator copies each owner file byte for byte into the row folders of that build (S4) and checks each copy's sha256 against O1's; it never writes the setting itself. A `terms_not_accepted` or `terms_record_unavailable` answer = STOP; the owner repeats O1 once.

## Files

Each is checked against `SHA256.txt` before S0; another hash = STOP.

| file | source |
|---|---|
| `RUN-SHEET.md` | this sheet |
| `LAB-COMMON.md` | byte copy of `lab/2246-split-journal:lab/2246-split-journal/LAB-COMMON.md` (v3), sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6` |
| `.gitattributes` | byte copy of `lab/2246-split-journal:lab/2246-split-journal/.gitattributes`, sha256 `d13bfb7ad438ed990b12090ee49e38d6d751345e951ff6048bc7ae5136a74149` |
| `client.py` | `lab/2246-split-journal`'s `client.py` (sha256 `248dce5c01784f939b4dc74701f9cde545946ac3b8708353308ef9da98dbd4e2`), NOT byte-equal: see below |
| `measure.py` | new; reads the saved answers and prints measurements (below) |
| `C-company-list.utf16le.xml` | byte copy of `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/shape_lab_company_list_request.utf16le.xml`, 2,524 bytes, sha256 `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8` (V: that folder's `PROVENANCE.md`, files table; captured on 2026-10-01 on BRIDGE SHAPE LAB) |
| `M-marks-shape-lab.utf16le.xml` | byte copy of `note-days/shape_lab_company_marks_request.utf16le.xml` from the same folder, 976 bytes, sha256 `f556551d09682fd94d41b40bcf539d5d34bf06260d6da4bb44ad7023c598fcb1` (V: the same table). It names BRIDGE SHAPE LAB in `SVCURRENTCOMPANY` and fetches `GUID, ALTVCHID, ALTMSTID` |
| `SHA256.txt` | sha256 and bytes of every other file |

**Why `client.py` is not a byte copy.** The 2246 client cannot run this sheet unchanged: it sends only the tools of that item, and `trial_balance` is not among them; it starts only `C:\bridge-lab\2246\bin\bridge_mcp.exe`; and it requires `BRIDGE_AGENT_ENABLE_IMPORT` on. Changed, and nothing else (a `diff` against the 2246 file shows only these):
1. the header comment;
2. it takes `<b1|master>` in place of `<in|out> [port]`, and starts `C:\bridge-lab\2397-b1\<build>\bin\bridge_mcp.exe`, checked against `C:\bridge-lab\2397-b1\<build>\exe-sha256.txt`;
3. `ALLOWED` is `list` (the protocol's `tools/list`, not a tool), `tally_status`, `list_companies`, `trial_balance`; any other tool named in a request file stops the client with nothing sent;
4. the launch file must hold exactly four keys: `BRIDGE_TALLY_HOST` `127.0.0.1`, `BRIDGE_TALLY_PORT` `9001`, `BRIDGE_AGENT_DATA_DIR` `C:\bridge-lab\2397-b1\<build>\data`, and `BRIDGE_TERMS_ACCEPTED`, whose value the client does not check (the owner's act; the server is the gate); no import or write setting is admitted;
5. its log line and each saved answer name the build (`build`) where the 2246 client named the side.

**`measure.py`** (run `python -I measure.py ...`; exit 3 = STOP; it never prints a company name except N3's pattern, which holds letters only):
- `pattern`: prints the N3 name (30 blocks, block k one letter repeated 10 times: `A` to `Z`, then `a` to `d`), its length (300) and its sha256 (`1d6366e1ccaa762b2da904319dc3a5385513fc401f1c814bb7a16bb0a03e433e`).
- `native <answer> <guid> [--pattern]`: a raw Company collection answer, read as LAB-COMMON C4 says (UTF-16LE, forbidden characters dropped raw and as references, parsed as XML, rows only under `ENVELOPE/BODY/DATA/COLLECTION`). Exactly one `COMPANY` row with that GUID, else STOP. For its `NAME` attribute and each `NAME` element: characters, UTF-8 bytes, sha256, control characters, edge spaces, and whether the two are equal; with `--pattern`, whether the name is a prefix of the pattern and which block and character it ends in.
- `marks <answer> <guid>`: one `COMPANY` row with that GUID, its `ALTVCHID` and `ALTMSTID`.
- `tool <answer> [<guid>] [--pattern]`: a saved client answer: `isError`, and every `code`, `cause`, `refusal_reason`, `reason_code` and `identity_state` value; with a GUID, the listed company's name as characters, bytes and sha256.
- `diff <a> <b>`: two saved answers compared by structure, after parsing any JSON held in a string. It prints the paths that differ, never the values; a path whose last key is a time or an id (`id`, `read_at`, `duration_ms`, `call_sent_utc`, `round_trip_seconds`, `server_pid`, `server_info`, `build`, `snapshot_id`, `today`) is listed as ignored. A key that is not a plain word prints as the first 8 hex digits of its sha256.

Checked when the sheet was written, on the captured files: `native` on `shape_lab_company_list_response.utf16le.xml` (one row, `NAME` 16 characters, attribute equal to element), `marks` on `shape_lab_company_marks_response.utf16le.xml` (ALTVCHID 111, ALTMSTID 289), and `marks` with another GUID (STOP, exit 3). `tool` and `diff` were checked on hand-made answer files only: the exact shape of a live answer is U until S5.

## Names and folders

- `L` = `C:\bridge-lab\2397-b1`. Build folders `L\b1\` and `L\master\`, each holding `bin\`, `exe-sha256.txt`, `launch-env.json` (O1) and `data\` (absent until the first launch; the server creates it).
- **A launch** = one client, one build, one folder `L\rows\<row>-<build>\` holding `inbox\`, `answers\` and a byte copy of that build's `launch-env.json`. Start it from the lab shell: `"<python>" L\client.py L\rows\<row>-<build> <build>`. Write request files one at a time into `inbox\` and wait for each answer in `answers\` before the next. End it with an empty file `inbox\stop`, and record the exit line of `client-log.txt`. Only one launch runs at a time; the next starts only after the last one's `session ended` line.
- **The requests of a launch**, as UTF-8 files:
  - `01-list.json` `{"tool": "list"}`
  - `02-tally_status.json` `{"tool": "tally_status", "arguments": {}}`
  - `03-list_companies.json` `{"tool": "list_companies", "arguments": {}}`
  - `04-trial_balance.json` `{"tool": "trial_balance", "arguments": {"company_guid": G, "from": F, "to": T}}`, written only after 03 has answered, with G the row's GUID as 03 printed it.
- **The comparison of a row:** `python -I L\measure.py diff L\rows\<row>-b1\answers\<n> L\rows\<row>-master\answers\<n>` for each of the four answers.
- **A raw request** (N3 only): LAB-COMMON C3 exactly, the files from this folder, answers to `L\raw\<step>-<file>`, headers beside them.

## Rules

- One Tally on port 9001, licensed mode (Education mode = STOP). Each row's company is THE COMPANY of that row, loaded alone.
- **Tools sent:** `tally_status`, `list_companies`, `trial_balance`, through `client.py` only, plus the protocol's `tools/list`. Never `post_import`, `build_import_xml`, a `lab_` tool, a master write, or any other tool. The installed extension is not used: no chat calls a Bridge tool during the sitting.
- **Raw requests:** only in N3, only `C-company-list.utf16le.xml` and `M-marks-shape-lab.utf16le.xml`, each a read, and LAB-COMMON C2's `GET /status`. No raw write is sent anywhere.
- **The one write** is N3's rename on Tally's Company Alteration screen, after its backup. It is undone by N3's restore.
- Differences from LAB-COMMON (the stricter rule holds everywhere else):
  1. **C2 d in N1 and N2:** the MARKS read is replaced by the first launch's `list_companies` holding exactly one company with the row's GUID, as `lab/1459-s3-run-sheet` did, because those rows send no raw request. N3 sends its MARKS read raw.
  2. **C1, synthetic companies only, in N2:** N2's company is the lab company with the longest name, which the protocol reference records as one the owner created; whether it is synthetic is U. N2 runs only on the run coordinator's word in this sitting that this company may be read. Its answers stay on the lab PC (below).
  3. **A tool refusal is a result:** record its `code`, `cause` and the whole answer; it ends the row only where the row says so.
- **Never written into any note or pushed file:** N2's company name. Record it only as characters, UTF-8 bytes and sha256 (`measure.py tool ... <guid>`).
- **Delete nothing.** Answers, logs, data folders and the backup stay where they land. No process is ended (C2).
- **STD STOP** = change nothing, screenshot the whole screen, save what exists with size and sha256, report to the run coordinator, call no tool, key nothing.

## Setup

- **S0.** `date` (UTC). The run coordinator's yes for this sitting and for each download is held, with its time. Windows version and build. `(Get-Command python).Source`: record it; `python --version`. No company loaded in Tally (LAB-COMMON C1 START).
- **S1.** The two builds (BUILD): download on the yes, check each zip digest, unpack, copy each `bridge_mcp.exe`, write and check each `exe-sha256.txt`. Record the path of the exe inside each bundle.
- **S2.** Copy every file of this folder to `L\`, and check each against `SHA256.txt`. `python -I L\measure.py pattern`: EXPECT 300 characters and the sha256 under Files.
- **S3.** Write `L\b1\launch-env.json` and `L\master\launch-env.json` (UTF-8, no byte-order mark), each:
  `{"BRIDGE_TERMS_ACCEPTED": "OWNER-ACCEPTS-TERMS-HERE", "BRIDGE_TALLY_HOST": "127.0.0.1", "BRIDGE_TALLY_PORT": "9001", "BRIDGE_AGENT_DATA_DIR": "C:\\bridge-lab\\2397-b1\\<build>\\data"}`
  with `<build>` written out (`b1` or `master`). Then **O1**.
- **S4.** For each launch named in the rows, create its folder with `inbox\` and `answers\` and copy that build's `launch-env.json` into it; check each copy's sha256 against O1's record.
- **S5.** `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Format-List ProcessId, ParentProcessId, ExecutablePath` (an error is an error, never "none found"): record every process found. None is ended. A process whose `ExecutablePath` is under `L` before the first launch = STOP.

## Row N1: an ordinary synthetic book, reads only

**THE COMPANY:** `BRIDGE CORPUS DENSE` (synthetic), GUID `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, as in `lab/1459-s3-run-sheet`. Books 2025-04-01 to 2026-03-31 (V: `docs/tally/TALLY_PROTOCOL_REFERENCE_READS_AND_DATES.md:341`). **The read:** `trial_balance` with F `2025-05-01` and T `2025-05-31`, one month inside the books, the month whose native Trial Balance the tree holds, 123 ledgers (V: `src-tauri/crates/bridge-tally-protocol/tests/fixtures/STATEMENT_CAPTURE_PROVENANCE.md:34`). No backup: nothing is written.

1. Load THE COMPANY alone (C1). LAB-COMMON C2 step 0, a to c, and its Gateway screenshot.
2. **Launch `N1-b1`:** 01, 02, 03, then 04 with G from 03, then stop.
   - 02: EXPECT `endpoint` on port 9001, THE COMPANY the one loaded company, `education_mode` false, no `refusal_reason`.
   - 03 (C2 d): EXPECT exactly one company, GUID as above, `identity_state` `verified_tuple`; `measure.py tool ... d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`.
   - 04: EXPECT a result, no error: rows whose count `measure.py` does not judge; record the answer's bytes and sha256.
   - Neither answer holds `company_name_invalid`.
3. Record that `L\b1\data\terms-acceptance.jsonl` exists (bytes, sha256; never edited).
4. **Launch `N1-master`:** the same four requests, the same G. The same EXPECT.
5. `measure.py diff` for 01 to 04. **EXPECT `EQUAL apart from time and id keys` for each.** Any `DIFFERS` line is recorded whole and goes to the run coordinator before N2.
6. Shut THE COMPANY by Tally's Shut Company (write down the menu path and key).

**Book state after each step:** unchanged throughout (reads only).

## Row N2: the longest stored lab name, reads only

Only on the run coordinator's word (Rules, difference 2). **THE COMPANY:** the lab company with the longest stored name. The protocol reference records one of 150 characters, 150 UTF-8 bytes, stored whole (V: `docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md:880-884`, bridge#917, one company, 7.1 Silver). Its GUID is not in this tree (U). **The read:** `trial_balance` with F = its `books_from` from 03, and T = the last day of that same month.

1. In Tally's Select Company list, pick the company whose name is visibly the longest, and load it alone (C1). C2 step 0, a to c, and its Gateway screenshot (stays on the PC).
2. **Launch `N2-b1`:** 01, 02, 03, then 04, then stop.
   - 03 (C2 d): exactly one company. `measure.py tool ...\03-list_companies.json <its guid>`: record the GUID (= G), the name's characters, UTF-8 bytes and sha256, and `identity_state`. EXPECT 150 characters and 150 bytes; another length is a result, recorded.
   - 02 and 04: EXPECT as N1, no `company_name_invalid`. **The B1 build accepts the name: 150 bytes is under 255.**
3. **Launch `N2-master`:** the same, the same G and dates. The same EXPECT.
4. `measure.py diff` for 01 to 04. EXPECT `EQUAL apart from time and id keys`.
5. Shut THE COMPANY.

**Book state:** unchanged throughout. **N2's answers, logs and screenshots stay on the lab PC**, listed by name, bytes and sha256. Only `measure.py`'s lines go into the record, and they hold no name.

## Row N3: Tally's own name limit, with a backup and a restore

**THE COMPANY:** `BRIDGE SHAPE LAB` (synthetic, seeded by this project: V `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/PROVENANCE.md`, Provenance, Book), GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, books from 2025-04-01 (V: `shape_lab_company_list_response.utf16le.xml`); its company number is the one step 2's native read shows, and C5 and C6 use that. **The read:** `trial_balance` with F `2025-04-01` and T `2025-04-30`. **The variable:** the company's name, and nothing else on the alteration screen. Control characters stay unmeasured: they cannot be typed in Tally's name field, so a name holding one is not a row here (U).

0. Load THE COMPANY alone. C2 step 0 in full, with its MARKS read: `M-marks-shape-lab.utf16le.xml` by C3, then `measure.py marks L\raw\N3-0-M.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: record ALTVCHID and ALTMSTID (= M0). Record the Tally release shown on screen.
1. **Backup (C5)** to the new empty folder `C:\bridge-lab\2397-b1\bk-n3\`. Record the file's name, bytes, sha256 and the backup's local time to the minute. Then the MARKS read again: record M1 (EXPECT equal to M0; a difference is recorded, not a STOP).
2. **The name as found.** `C-company-list.utf16le.xml` by C3, then `measure.py native L\raw\N3-2-C.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: EXPECT one row, `NAME` attribute and element each 16 characters, 16 bytes, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b`, equal. Then **launches `N3a-b1` and `N3a-master`**, each with 01 to 04, and `measure.py diff` of each pair: EXPECT as N1, equal answers.
3. **The rename (the one write).** Screenshot the Gateway. Open Company Alteration for THE COMPANY from Tally's own menu (write down the keys used). On the Name field only: clear it, then type the 300 characters `measure.py pattern` prints, in order, from the first. If the field stops taking characters, type the rest anyway and note where the screen showed it stop. Screenshot the field before accepting. Change no other field. Accept the screen (write down the key) and answer only its own accept prompt. Any other prompt or dialog = C7: press no key, screenshot, STOP. Then screenshot the whole Tally window (no dialog) and the Gateway.
4. **The stored name, natively.** `C-company-list.utf16le.xml` by C3, then a screenshot of the whole Tally window, as C3 asks after a new shape (the request is not new; the name it meets is), then `measure.py native L\raw\N3-4-C.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 --pattern`. Record, for the attribute and the element: characters, UTF-8 bytes, sha256, whether a prefix of the pattern, the block and character it ends in, and whether the two are equal. A name that is not a prefix of the pattern (a typing slip) is recorded: its length still stands, its cut point is U, and the name is not typed again.
5. **The builds.** Launches **`N3b-b1`** then **`N3b-master`**, each with 01 to 04 (G as in step 2), then `measure.py tool` on each answer and `measure.py diff` of each pair. Write 04 even if 03 was refused: it is the read the row asks about. EXPECT, by the length step 4 recorded:
   - **Stored at 255 bytes or less:** as step 2 on both builds: the company listed with `identity_state` `verified_tuple`, `measure.py tool ... --pattern` on 03 equal to step 4's attribute, and 04 answered. Each pair of answers equal apart from time and id.
   - **Stored over 255 bytes:** on both builds, 02 with `loaded_companies` empty, `refusal_reason` `xml_export_shape_unrecognized`, `product` `not_observed`; 03 and 04 refused with `code` `company_collection_invalid`. Answers equal apart from time and id. Not `company_name_invalid` on either build (the code section): if either build answers `company_name_invalid`, record it whole; it is the row's finding and goes to the run coordinator before step 6.
   - Record which case held, every code seen, and each pair's `diff` result.
6. **Restore (C6)** from `bk-n3\`, the backup of step 1, by the C6 procedure and its two prompts only. Then load THE COMPANY alone, screenshot the window and the Gateway.
7. **The check of the restore.** MARKS read by C3, `measure.py marks`: **EXPECT the COMPANY row's GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and ALTVCHID and ALTMSTID equal to M1 (step 1's).** Then `C-company-list.utf16le.xml` by C3 and `measure.py native`: EXPECT step 2's 16 characters and sha256. Either differs = STOP, and the run coordinator decides; the book is not touched again.
8. Shut THE COMPANY.

**Book state after each step:**

| step | name | marks |
|---|---|---|
| 0 | as found, 16 characters | M0 |
| 1 | unchanged; backup taken | M1, EXPECT = M0 |
| 2 | unchanged | not read |
| 3 | the typed pattern, as Tally stored it | not read: the marks request names the company by its old name, so it is not sent while the name differs (U) |
| 4, 5 | as step 3 (reads only) | not read |
| 6 | restored | not read before the screenshots |
| 7 | EXPECT as step 2 | EXPECT = M1 |
| 8 | shut | |

If a STOP comes between steps 3 and 6, the book holds the renamed company and the backup of step 1 is the way back, restored only on the run coordinator's word (C8).

## Close

LAB-COMMON C10. Hash both `bridge_mcp.exe` copies (EXPECT the BUILD values). `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'"`: EXPECT no process under `L`; record every process found and end none. List `L\b1\data\` and `L\master\data\` (names, bytes, sha256). Nothing is deleted.

## Result

**PASS (the sheet ran):** every row answered and was recorded, refusals included; N3's book is back to step 1's marks and step 2's name; no tool outside the list was sent; no dialog was answered but those named.

**FAIL:** a transport failure read as an answer; a launch that sent anything twice; a dialog answered that is not named; N3's restore not equal to step 1.

**What it decides:**
- N1 and N2 equal on both builds: the B1 change does not alter what these reads answer for an ordinary name or a 150-byte name. PARTIAL: two books, one release.
- N3's stored length: Tally's own name limit on this release, in characters and bytes, for ASCII letters (the first measurement of it; bridge#917 recorded only that 150 is stored whole).
- N3 over 255 bytes: what a person with such a company sees from each build, and whether it is the code this sheet's reading predicts. If it is `company_collection_invalid` on both, the identity check of #2397 is a second line behind the company-list parser for these tools, and #2397's changelog line ("refused once, when it is selected, with `company_name_invalid`") does not describe what the MCP server answers. That goes to #2397 as a finding.
- N3 at 255 bytes or less: Tally cannot store a name that reaches either refusal by typing ASCII letters, on this release.

## Not measured

Control characters in a name (cannot be typed). A name over 255 bytes in fewer than 256 characters (multi-byte letters), and whether Tally's limit counts characters or bytes (ASCII only here). A name with edge spaces. The desktop app's selection (`commands.rs`) and the sync connector, which this sheet does not run. Any other tool or read. Windows 11. Another Tally release.

## Evidence

A new branch `lab/2397-b1-name-answers` (LAB-COMMON C11), pushed only after the pre-push scan and the run coordinator's yes, with no AI attribution in any commit:
- `RESULTS.md`: the build identity (both digests and exe hashes, the exe paths inside the bundles); per row and launch, each answer's bytes and sha256 and the `measure.py` lines whole; each `diff` result whole; N3's stored length, block and character, the codes each build gave, the marks M0, M1 and step 7's, the backup file's name, bytes and sha256, every prompt word for word with the key that answered it; anything VOID or not run, and why. Each finding marked verified, partial or unverified.
- N1's and N3's answers and logs, each a substituted copy: the home folder by `<home>` in every form, the user name by `<user>`, by a JSON parser where the file is JSON, each replacement recorded with the original's sha256 and bytes. N2's answers and logs stay on the lab PC, listed by name, bytes and sha256 in `kept-local.txt`, and are never pushed.
- `notes.txt` (UTC times, one line per step: the act, what was read, the marks before and after), `SCREENSHOTS.txt` (names, bytes, sha256; the captures stay on the PC), `restore-plan.txt`, and `SHA256.txt` (`# sha256  bytes  file`).

## Report (one message)

The build identities; whether O1 ran and how long it took; N1's and N2's `diff` results; N2's name length and sha256 (no name); N3's stored length in characters and bytes, the block and character it ends in, the codes each build gave at step 5, and the restore check; a statement that nothing was posted, imported or deleted. Confidence: one run, one PC, one release: PARTIAL.
