# #2173: the tax audit's read of a period over 5,376 vouchers, before and after PR #2191, on BRIDGE CORPUS DENSE

Not for merge. READS ONLY, on one synthetic company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6) applies in full, with THE COMPANY below. Where this sheet and it differ, the stricter holds. `SHA256.txt` lists every file here with its bytes; check it before step 0. About 30 to 60 minutes.

**WHY.** The tax audit's read (`audit_test_run`) reads one period and cannot narrow it. Before #2191 it refused any period holding more than 5,376 vouchers before reading a single one, because it counted its reads at the heaviest voucher weight ever measured. #2191 refuses such a period before reading only when no measured weight could fit it in 128 requests: at most 10,837 vouchers, and fewer when days are dense. Otherwise it reads the first part, measures it, and reads on. This item records, through ComplyEaze Bridge's own tool calls:
- the refusal before #2191 (step 0);
- the read after it (step 1);
- a period still over the new limit, refused with no data part sent (step 2).
It also records the time of every census request over a quarter and over the year, which is not yet measured inside 20 seconds.

**THE COMPANY:** `BRIDGE CORPUS DENSE` (synthetic). Its GUID is `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3`, and its voucher mark was 29,900 at the last sittings (28 and 29 Oct). Its financial year is 1 April 2025 to 31 March 2026, with about 82 vouchers a day. Licensed mode only; Education mode = STOP. TallyPrime 7.2 expected: record the release.

## Differences from LAB-COMMON
1. **No raw requests.** The operator sends no request file to Tally. Every read is a call of ComplyEaze Bridge's built tool, one at a time, through the relay (Difference 2). C2's step 0 runs WITHOUT its MARKS request. Its GET /status checks still run, straight to 127.0.0.1:9001.
2. **The relay.** `read_relay.py` sits between ComplyEaze Bridge and Tally, on 127.0.0.1:9012. It forwards only `GET /status` and Export reads to 127.0.0.1:9001, and answers anything else 403 without forwarding it. It records every request whole, and every answer by size, sha256, rows and seconds; an answer of at most 1 MiB is kept whole. Read its header. Before step 0, run `python -I read_relay.py --self-test`. EXPECT exactly `self-test: ok (7 forwarded unchanged, 12 refused, none of the refused reached Tally; rows by structure; an abandoned answer read to its end)` and exit code 0. Anything else = do not use the relay; STOP.
3. **Size bound.** The bound is the product's own (32 MiB an answer). The relay reads every answer to its end, with no time limit, as LAB-COMMON C3 does.
4. **Time.** ComplyEaze Bridge gives up on a request at 20 seconds. The relay keeps reading Tally's answer to its end, so its `seconds` column is Tally's real time. Each tool call has 15 minutes. A call with no answer by then: STOP, leave it running, press no key, send nothing more, and report.
5. **Held gateway.** After each tool call answers, `GET /status` straight to 9001. No answer within 60 seconds = the gateway is held: STOP, leave everything running, press no key, send nothing more, report, and wait (no state read and no shut until the run coordinator says).

## The builds
- **BUILD-A (step 0):** candidate A, the release-candidate bundle of master `026ba163d`, CI artifact `12928034390`. The release-candidate sitting downloads it; no second download is needed. Use its `bridge_mcp.exe`. Record its path, bytes and sha256 (Get-FileHash) before the first call. If the release-candidate sitting recorded a hash for the same file, the two must be equal, else STOP.
- **BUILD-B (steps 1 and 2):** the bundle of a CI run on `fix/2173-audit-floor-ceiling`, the head of PR #2191, at `a4711a6a1155dc985ea703736d607ff3d7a59d4b` or a later head the run coordinator names. The run coordinator names the run, the artifact and the head before step 1. Record the same for its `bridge_mcp.exe`. Steps 1 and 2 wait for that word; step 0 does not.

## The environment (one ComplyEaze Bridge process per step; record each value; nothing else set)
Run `bridge_mcp.exe` as the stdio server with exactly:
- `BRIDGE_TERMS_ACCEPTED=true` (the owner allowed the lab to set it on 25 Oct 2026);
- `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9012` (the relay);
- `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`;
- `BRIDGE_AGENT_DATA_DIR` = a new empty folder for this step: `C:\bridge-lab\2173l\s<step>\data`;
- no `BRIDGE_AGENT_REDACTION` (a redaction setting refuses `audit_test_run`), no lab-write, import or posting setting.

The process is stopped after its step, and a new one is started for the next.

## The steps (one at a time; the next only when this one's EXPECT is recorded)
Before step 0: C2 step 0 with THE COMPANY loaded alone, as Difference 1 says. Then the self-test (Difference 2).

**For each step:**
1. Start the relay in its own console, with a new recording folder:
   `Start-Process cmd.exe -WorkingDirectory <this folder, absolute> -ArgumentList '/k','python -I read_relay.py --listen-port 9012 --record C:\bridge-lab\2173l\s<step>\relay'`
   EXPECT its first line: `relay: 127.0.0.1:9012 -> 127.0.0.1:9001, reads only, recording into C:\bridge-lab\2173l\s<step>\relay; Ctrl+C to stop`.
2. Start the build named for the step with the environment above. Then call:
   - `list_companies`: EXPECT exactly THE COMPANY, with the GUID above and `identity_state` `verified_tuple`;
   - `tally_status`: record the release, licence and Education mode.
3. Call `audit_test_run` ONCE, with:
   - `company_guid` = the GUID above;
   - `from` and `to` as the step says;
   - `test` = `cash_44ab`;
   - `cash_groups` = `["Cash-in-Hand"]` and `bank_groups` = `["Bank Accounts"]`.

   Note the UTC time at the call and at its answer. Keep the whole answer as a file, with its size and sha256. A refusal is an answer: record its code and any cause. Do not call it again.
4. `GET /status` straight to 9001 (Difference 5). Stop the build's process. Stop the relay (Ctrl+C in its console).
5. Run `python -I summarise.py C:\bridge-lab\2173l\s<step>\relay` and keep its output as a file. Check the EXPECT lines below against it and the answer.

| Step | Build | `from` to `to` | Vouchers in the period |
|---|---|---|---|
| 0 | BUILD-A | `2025-04-01` to `2025-06-30` | about 7,460 (91 days of about 82) |
| 1 | BUILD-B | `2025-04-01` to `2025-06-30` | the same |
| 2 | BUILD-B | `2025-04-01` to `2026-03-31` | about 29,900: the year, the negative control |

### EXPECT (recorded; the lab judges only the STOP rules)
**Step 0 (before #2191).**
- The answer is a refusal with the code `voucher_window_too_many_reads`.
- The summary shows `data parts: 0`: no `Bridge Agent Vouchers` request was sent.
- The census lines show the period's census:
  - Spans are of 8,192 AlterIDs from 0, ending at the mark: (0, 8192], (8192, 16384], (16384, 24576], (24576, 29900].
  - It stops once its count passes 5,376 with spans left. If the quarter's AlterIDs follow its dates, that is after the first span (rows about 7,460).
  - Record how many census requests were sent and each one's rows and seconds.
- Other branches:
  - The answer is a refusal with cause `census_deadline_exceeded`: a census span took over 20 s. Record the relay's seconds for it; this is a finding, not a STOP.
  - Any other answer: record it whole and report before step 1.

**Step 1 (after #2191, the same quarter).**
- The answer is a result (`completed` or `completed_with_failed_book_checks`, either), or a refusal by the audit engine after the read. Record which.
- The summary shows:
  - **census** lines for all four spans, nothing stopped;
  - **data parts:** between 89 and 128 (about 92: the first part holds at most 42 vouchers, then about one part a day of at most 85);
  - **first part rows:** at most 42;
  - **most rows:** at most 85;
  - **most bytes:** at most 16,777,216;
  - **total rows:** equal to the census rows in the quarter;
  - **mean bytes per row:** this book's voucher weight on the wire, recorded for the first time;
  - **marks:** at least 2 (the window opened and closed on its marks);
  - **over:** none.
- Other branches:
  - **Refused `voucher_window_too_many_reads` with data parts 1:** the first part measured heavier than about 200 KiB a voucher, so a part holds fewer than a day. Record the mean bytes per row. The change behaves as designed; this book is too heavy for the quarter.
  - **Refused with cause `census_deadline_exceeded`:** a census span over the quarter took over 20 s. Record the relay's seconds. Then this read also needs its census divided by date (#899), and that is the finding.
  - **More than 128 data parts sent, or any data part over 16,777,216 bytes:** a defect in the build. Record it whole and report at once.
  - **The call takes longer than 240 s:** record its time. The host limit on a desktop chat (240 s) would bound this read, not the read count.

**Step 2 (after #2191, the year: the negative control).**
- The answer is a refusal with the code `voucher_window_too_many_reads`.
- The summary shows:
  - `data parts: 0`;
  - census lines for the first two spans only, (0, 8192] and (8192, 16384], with all their rows in the year, about 16,384;
  - the census stopped there with spans left, because the count passed 10,837.
- Other branches:
  - **Refused with cause `census_deadline_exceeded`:** a span over the year took over 20 s; the year's census was measured at 13 to 25 s a span on this book. Record the relay's seconds. Still no data part may be sent.
  - **Any data part sent:** a defect in the build. Record it and report at once.

After step 2: C2 step 0 again (Difference 1), then C10.

## Evidence and close
For each step:
- the tool answers whole, with sizes and sha256;
- the relay's folder: `manifest.tsv`, every request file, and the kept answers. Answers over 1 MiB were not kept; their sizes and sha256 are in the manifest;
- the summary output;
- the build's exe path, bytes and sha256, and the environment as set;
- UTC times.

Also: the notes, one line per step, and the screenshots, which stay on the lab PC, listed by name, size and sha256.

The evidence goes to a new branch, `lab/2173-audit-floor-ceiling-answers`, with `SHA256.txt`. The data folders stay on the lab PC: list their files by name, size and sha256, and push none of them. Push only after the pre-push scan and the run coordinator's yes.

## Not measured by this item
- Any book but this one.
- An inventory-heavy book, whose voucher weight and census time differ.
- A period between 10,837 vouchers and the year.
- Gold, and Education mode.
- The read through a desktop chat's own host limit.
