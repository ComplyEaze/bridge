# Post-publish check of release mcp-v0.7.0 on Windows: reads only (Part of #2056)

Not for merge. This sheet is the release process's check of the published Windows file after publication (step 17 of the 0.7.0 cut). It sends **no write** to Tally: no post, no ledger create, no import. It is the same check as `lab/061-post-publish-answers` ran for 0.6.1, on the 0.7.0 file.

**Before step 1:** the owner's OK to download the release's Windows assets, typed in the lab's chat. No OK = do not start.

## What is checked

The release `mcp-v0.7.0` was published on 30 October 2026 (10:10:03 UTC) and marked latest; it is not a pre-release. GitHub records these digests for its two archives, and `registry/server.json` on #2213 carries the same values (its `--published` check passed):

```
bridge-tally-mcp-v0.7.0-windows-x64.mcpb   15,598,419 bytes   sha256 291a4ed95c758fad9fa04cd21a66be889e90a2fd03960fbceeec2c2a7242d4ec
bridge-tally-mcp-v0.7.0-macos-arm64.mcpb   13,943,477 bytes   sha256 4895dfa1a567526aa74114424af7769b597852fad1af8a6bfd7e6ad72b245a5f   (not run here)
```

## Steps

1. **Download.** Into a new empty folder `C:\bridge-lab\rc070-post\`: the four Windows assets of `mcp-v0.7.0` (the `.mcpb`, `.mcpb.sha256`, `.mcpb.smoke.json`, `.mcpb.provenance.json`). Record each file's bytes and sha256.
   - EXPECT the `.mcpb` sha256 = `291a4ed9…` above = the `.sha256` file's value = the provenance file's `sha256` = the smoke file's `archive_sha256`. Any difference = STOP.
   - Record the provenance file's `source_sha` (EXPECT it to start `fcfb5ff6`, the merge of #2207) and the smoke file's `server_version`, `binary_sha256`, `default_tool_count` and `opt_in_tool_count`.
2. **Unpack** the whole `.mcpb` into a new empty folder. Record the entry count, and that no entry has an absolute or parent path. Record `bin\x86_64-pc-windows-msvc\bridge_mcp.exe`'s sha256 (EXPECT = the smoke file's `binary_sha256`) and `manifest.json`'s `version` (EXPECT 0.7.0) and `name`.
3. **Tally.** One `tally.exe` owns port 9001; `GET /status` answers 200. Exactly one company is loaded, a synthetic one of the lab's own (record its name; the gate's synthetic family). Nothing is changed on Tally's screens.
4. **One launch over stdio** of the unpacked `bridge_mcp.exe`, hashed just before, with `BRIDGE_TERMS_ACCEPTED=true`, `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001` and a new empty `BRIDGE_AGENT_DATA_DIR`; `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_ENABLE_MASTER_WRITES`, `BRIDGE_AGENT_ENABLE_IMPORT`, `BRIDGE_AGENT_ENABLE_BATCH_POST`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_REDACTION` unset. Calls, in order, each answer kept whole with its sha256:
   - `initialize`: EXPECT server `bridge-tally` **0.7.0**;
   - `tools/list` (000): record the count and the names; EXPECT no `post_import`, `post_ledger_creates`, `prepare_ledger_creates` or `audit_test_run`. Compare the count with the smoke file's `default_tool_count` and say whether the smoke run's settings match these; if they are not recorded, record both counts without comparing;
   - `tally_status` (001): record the release, edition, `education_mode`, the endpoint and the companies loaded;
   - `list_companies` (002): EXPECT exactly step 3's company;
   - `ledger_masters` (003), basic: EXPECT `complete`; record the master and voucher marks.
   Then stop the launch by closing its input. Record the exit code, the error stream's size, and the exe's sha256 after the run (EXPECT unchanged).
5. **Marks.** A raw MARKS read after the launch, as before step 4 (or `ledger_masters` again from a second launch with the same settings): EXPECT the marks unchanged.

A STOP at any step ends the sheet there: record it, change nothing, and send it to the run coordinator.

## Evidence

On a new branch `lab/0.7.0-post-publish-answers`, folder `lab/0.7.0-post-publish-answers/`:
- `RESULTS.md`: one table for the download (each file, bytes, sha256) and one for the calls (each answer's key fields), as `lab/061-post-publish-answers` did, with what was not run: the macOS file, Claude Desktop, posting, ledger creation, `audit_test_run` with its setting on, any other read;
- `answers/000-003` whole, the client log, `notes.txt` with UTC times, and `SHA256.txt` (sha256, bytes, path for each file);
- the release's `.sha256`, `.smoke.json` and `.provenance.json` files as downloaded. The `.mcpb` and the unpacked folder stay on the lab PC, listed by sha256.

Before the push: the pre-push scan; any machine path in an answer replaced by `<LAB_DIR>` or `<DATA_DIR>`, each replacement recorded; no `Co-Authored-By` or "Generated with" line in the commits. The SHA goes to the run coordinator, and the push waits for its yes.

## What the result is for

The post-release pull request (#2213) says what has been run on the published file, in `README.md`, `docs/agent/INSTALL.md` and the Download and home pages, from this sheet's `RESULTS.md`. Until it runs, they say the published 0.7.0 file has not been run against TallyPrime.
