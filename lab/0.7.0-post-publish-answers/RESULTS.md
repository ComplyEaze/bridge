# Results: post-publish check of release mcp-v0.7.0 (Windows x64) on TallyPrime 7.2, 30 Oct 2026. The hashes match what was published, and 0.7.0 answers the reads

Release `mcp-v0.7.0` was published 2026-10-30T10:10:03Z and is not a draft or a prerelease. Its source commit is fcfb5ff6958a60af3d713420bf1b488089d35cdb, per its provenance file. The check reads only: no post, no ledger create, no import.

## The download
| file | bytes | sha256 |
|---|---|---|
| bridge-tally-mcp-v0.7.0-windows-x64.mcpb | 15,598,419 | 291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec |
| its `.sha256` (committed here) | 107 | 0abc378e…; it names the same sha256 for the same file |
| its `.smoke.json` (committed here) | 710 | 599d4550…; `archive_sha256` is the same; `binary_sha256` 48d2f743…; `server_version` 0.7.0; `default_tool_count` 33; `opt_in_tool_count` 35 |
| its `.provenance.json` (committed here) | 352 | 4799753a…; `sha256` is the same; `source_sha` fcfb5ff6… |

Each file's bytes and sha256 equal the digest GitHub records for it. The `.mcpb` sha256 equals the sheet's value, the `.sha256` file, the provenance file and the smoke file.

The archive holds 9 entries, none with an absolute or parent path. Unpacked:
- `bin/x86_64-pc-windows-msvc/bridge_mcp.exe` has sha256 48d2f7430b26da0d8f670eb74a27df46b99b7d45e05e102504482ddf24c8ba2d. That equals the smoke file's `binary_sha256`, and it was the same before the launches and after them.
- `manifest.json` gives version **0.7.0**, name `bridge-tally`.

The archive and the unpacked folder stay on the lab PC, listed in `kept-local.txt` by sha256.

## The reads
One launch over MCP stdio, 12:42:59 to 12:43:16 UTC (`client-log.txt`):
- Settings: `BRIDGE_TERMS_ACCEPTED` true, host 127.0.0.1, port 9001, a new data folder.
- Unset: `BRIDGE_AGENT_ENABLE_WRITES`, `_ENABLE_MASTER_WRITES`, `_ENABLE_IMPORT`, `_ENABLE_BATCH_POST`, `_MAX_BYTES` and `_REDACTION`.
- The launch exited with code 0, and its error stream stayed empty.

| call | answer |
|---|---|
| initialize | `bridge-tally` **0.7.0** |
| `tools/list` (000) | **30** tools; `post_import`, `post_ledger_creates`, `prepare_ledger_creates` and `audit_test_run` absent |
| `tally_status` (001) | TallyPrime 7.2, silver, `education_mode` false, endpoint http://127.0.0.1:9001; exactly one company loaded, BRIDGE CORPUS DENSE |
| `list_companies` (002) | one company, BRIDGE CORPUS DENSE (synthetic), `verified_tuple`, books from 20250401 |
| `ledger_masters` (003) | `complete`; `snapshot.master_alter_id` 328, `voucher_alter_id` 29900, `reused` false; 123 ledgers |

**Marks:** a second launch with the same settings (`l2/`, 12:43:32 to 12:43:38 UTC) read `ledger_masters` again: 328 / 29900, unchanged. It exited with code 0, and its error stream stayed empty.

**Tool count:** the smoke file gives `default_tool_count` 33 and `opt_in_tool_count` 35. The 30 here is the list with every write, import and batch setting unset. The smoke file does not record the smoke run's settings, so the two counts are recorded and not compared.

## Not run
- the macOS file;
- the desktop client's extension install;
- posting, ledger creation and import;
- `audit_test_run` with its setting on;
- any other read.

## Files
- `answers/000-003` (launch 1) and `l2/answers/000` (launch 2): the whole answers. Their sha256 values are in the client logs and `SHA256.txt`.
- The release's `.sha256`, `.smoke.json` and `.provenance.json` files, as downloaded.
- `notes.txt`, with UTC times.
- `kept-local.txt`: the files kept on the lab PC.
- `substitutions.txt`: each change made to a pushed copy.
