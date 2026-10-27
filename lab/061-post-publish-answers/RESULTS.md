# Results: post-publish check of release mcp-v0.6.1 (Windows x64) on TallyPrime 7.2, 27 Oct 2026. Hashes as published; 0.6.1 answers the reads

Release `mcp-v0.6.1` (published 2026-10-27T12:10:25Z, not a draft, not a prerelease), source commit 0e1467ec3a07a301edae89661d65899fe4d6c16f (its provenance file).

## The download
| file | bytes | sha256 |
|---|---|---|
| bridge-tally-mcp-v0.6.1-windows-x64.mcpb | 15,060,419 | 7812d8ad7b0ff6a07eac20828dfedaef4d50104bd5d679db3fc03c26e784c47a |
| its `.sha256` (committed here) | 107 | names the same sha256 for the same file |
| its `.smoke.json` (committed here) | 710 | `archive_sha256` the same; `binary_sha256` bdd428e4...; `server_version` 0.6.1 |
| its `.provenance.json` (committed here) | 352 | `sha256` the same; `source_sha` 0e1467ec |

The archive holds 9 entries. Unpacked:
- `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` sha256 bdd428e4806bdbf70925956f90d410df48a0470b1a2f78c746c4c18088471d8d, equal to the smoke file's `binary_sha256`, and the same before the launch and after it;
- `manifest.json` version **0.6.1**, name `bridge-tally`.

The archive itself stays on the lab PC, listed here by sha256.

## The reads
One launch over MCP stdio (`client-log.txt`, 12:15:02 to 12:15:27 UTC). The settings were `BRIDGE_TERMS_ACCEPTED` true, host 127.0.0.1, port 9001 and a new data folder. `BRIDGE_AGENT_ENABLE_WRITES`, `_ENABLE_IMPORT`, `_MAX_BYTES` and `_REDACTION` were unset. The server's error stream stayed empty.

| call | answer |
|---|---|
| initialize | `bridge-tally` **0.6.1** |
| `tools/list` (000) | **29** tools; `audit_test_run` absent |
| `tally_status` (001) | TallyPrime 7.2, silver, `education_mode` false, endpoint 127.0.0.1:9001, one company loaded |
| `list_companies` (002) | one company, BRIDGE NESTED STOCK LAB, synthetic, number 100086, GUID 4b6094e2-d6db-46b6-a4ad-90c283cd1b52, `verified_tuple` |
| `ledger_masters` (003) | `complete`; `snapshot.master_alter_id` 219, `voucher_alter_id` 2, `reused` false |

The smoke file gives `default_tool_count` 31 and `opt_in_tool_count` 33. The 29 here is the list with writes and import both unset. The smoke run's default configuration is not recorded here, so the two counts are not compared.

## Files
- `answers/000-003`: the whole answers; sha256 in `client-log.txt` and `SHA256.txt`.
- The release's `.sha256`, `.smoke.json` and `.provenance.json` files, as downloaded.
- `notes.txt`: UTC times.
