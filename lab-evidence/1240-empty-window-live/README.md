# Lab run sheet: the empty-window check by census on a live Tally (reads only)

Written 25 Oct 2026, revised after an independent read of the first version. It measures one change, pull request #1897 (branch `empty-window-census-only`; the head to run is named in the Builds section). The same change was merged earlier as #1452 and reverted for want of a live run. Until this sheet has been run, the change is checked on test doubles only.

**What changes.** When `vouchers` (also `voucher_presence`, `ledger_movement` and the registers, which share the code) finds a window empty on a book that is counted, it looks again at the window widened by a day on each side. Before, that second look counted the widened window and then read its vouchers; now it only counts it, because the count carries each voucher's day and that is all the check uses. The answer should be the same. A book counted in one request is expected to need fewer requests; a book counted in several requests is expected to need the same number, with the data read replaced by a second read of the company's marks (the bracket of a divided count).

**What this run decides.** (1) Does the answer stay the same on a real Tally, for the windows the 12 October run used (`lab/1452-empty-window-evidence`, TallyPrime 7.1)? (2) On a book counted in one request, are the requests fewer? (3) On a book counted in four requests, is the answer the same, and does the bracket hold when nothing else touches the book?

Reads only. Nothing is keyed into a book, imported, backed up, restored, created or renamed, and no setting is changed. The only keys are those the steps name: Select Company and Shut Company, the Day Book period, and screen navigation. Never Enter on a voucher row in the Day Book (it opens the voucher). On a master or voucher-type screen never Escape. Synthetic BRIDGE companies only. Posting in the built tool stays OFF.

## Rules

`LAB-COMMON v3` (as sent for the 626, 1038 and 1050 items) applies in full, including C1, C2, C4, C7, C8, C9, C10 and C11, except C5 and C6 (nothing is backed up or restored) and the parts of C3 that name curl: the `vouchers`, `list_companies`, `tally_status` and `egress_log` calls go through the built tool; the company list and marks reads are raw requests sent with curl exactly as C3 says. Where this sheet and LAB-COMMON differ, the stricter holds.

Added for this sheet:
- **Who does what.** Every act on Tally's screen (loading and shutting a company, the Day Book readings, screenshots) is done by the lab operator, one act at a time, and the Gateway of Tally is shown with no dialog before each call. Calls are sent by the same operator, one file at a time.
- **Nobody touches Tally while a call runs.** From the moment a file is started until its answer is complete, no key, no mouse and no Day Book use. The screenshot and the Day Book readings come after the answer, never during it.
- **Timeouts.** The built tool has its own deadline of 20 seconds for each request it sends to Tally, and abandons a request that passes it (the request keeps running in Tally). Any answer holding `request_deadline_exceeded` or `response_size_limit_exceeded`, any call that returns no answer within 10 minutes, and any dialog on Tally's screen is a STOP: nothing is re-run or re-sent, no key is pressed, the screen is photographed, and the answer is reported whole. A call without an answer is never sent again.
- **Run command.** In cmd.exe, from a fresh empty folder: set `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, `BRIDGE_AGENT_DATA_DIR=<a new empty folder per file>`, and `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use; the tool refuses every call as `terms_not_accepted` without it, and writes an acceptance record `terms-acceptance.jsonl` into the fresh data folder); leave `BRIDGE_AGENT_REDACTION` and the posting settings unset (redaction off, posting off). Then `bridge_mcp.exe < <filled file> > <name>.out 2> <name>.err`, and keep both outputs byte for byte. Record the exit code.
- **Filled files.** A template is filled by replacing only the placeholders. Diff each filled file against its template (only the placeholder lines differ), and record the sha256 of the filled file before it is sent. A different diff is a STOP.
- **Bounds.** Each call's answer is at most 2,000,000 bytes (the tool's own cap is 200,000 for the result); larger is a STOP before it is used. Afterwards, read the trail's `response_bytes` for every send and report any above 25,000,000.
- **Egress readback.** The `egress_log` answer must hold this call's `response_prepared` record and its `stdio_write_completed` record, `truncated` false. Otherwise report it and do not compare that call.
- **Marks.** A marks read before and after each group must be equal. If they are not equal: STOP and report both answers, because nothing in this sheet writes.
- **Release.** Record the TallyPrime release and edition at step 0 (the lab reports 7.2 Silver; the 12 October windows were measured on 7.1). STOP if Education mode shows, if the edition is not Silver, or if a company other than the two named is loaded.
- **Pushing the evidence.** The evidence branch is pushed only after the pre-push scan and the coordinator's yes (C11). Notes and `.err` files carry no user names and no machine paths beyond the lab folders named here.

## Builds (one variable: the build)

- **P**, the change: the Windows bundle built from the head of #1897 named below, from the Bridge CI `pull_request` run named below, artifact `unsigned-bundle-smoke-windows-latest`. Verify the digest before unpacking. Filled in on 25 Oct 2026 from the run named:
  - head sha: `3367a1b9981f03a34aa4644b23f64eaa23ee8f99` (the PR's head when the run started); run id: `40182690146` (event `pull_request`, conclusion `success`); artifact id: `12595393056` (61,202,241 bytes, expires 2026-11-01 10:35 UTC); zip digest: `sha256:9056a6d42adcfb73a0a331a31c971c4d2c1a3cee47c901766f050f6f6a37048b`.
  - Inside it: `bridge-tally.mcpb` 16,387,518 bytes, sha256 `35debe4fc9da85ff3b265612f98df16a77d1b4882e79a678f463eccc05132783` (equal to `archive_sha256` in its `mcpb-smoke.json`); `bridge_mcp.exe` inside the `.mcpb`, 33,703,424 bytes, sha256 `1f5dd92d43b3ffacb83d3bf72026be22794521f3bb35f481ae2ab9a7438993fd` (equal to `binary_sha256`); manifest version 0.6.0. These were measured on a Mac from the downloaded artifact on 25 Oct 2026. To get the exe: unpack a COPY of the `.mcpb` with a zip tool into a new folder (the exe is under `bin\x86_64-pc-windows-msvc\`).
  - COMPILED TREE: a `pull_request` run builds GitHub's merge of the head into `master` as it stood when the run started, not the head alone; the hashes above, not the commit, decide whether the file is the build. The lab records the hash of the exe it stages and compares it with the line above; a different hash, or a head that is not the PR's current head, is a STOP.
- **M**, the control: the build already staged at the lab, the bundle of master `2306d86df` (artifact 12553782422). Between M and the base the change is built on, the window-read code (`agent_voucher_window.rs`, `agent_vouchers.rs`, `agent.rs`) is identical; only tests and files elsewhere differ. M already carries the rename of the reason an empty window beside a voucher gives (it said `nonempty_uncorroborated` on 12 October; it says `empty_window_wider_read_nonempty` since). Confirm M's full `bridge_mcp.exe` sha256 at step 0 and record both builds' digests before the first call.

Run each window with P first, then M, on the same book state.

## Books (identities)

| book | company number | GUID |
|---|---|---|
| `BRIDGE BATCH LAB` | 100050 | `a30f23c3-4819-4334-96ab-ffa6fba70976` |
| `BRIDGE CORPUS DENSE` | the number in the company list fixture `company_identity_fields_after_split.utf8.xml` (books from 1 Apr 2025) | `d45bc1b0-e5e3-4261-b3b2-cce3915f42d3` |

Each is loaded ALONE. Step 0 (C2) begins every part: the company list request `c0-company-list` (path `lab-evidence/72-reads/requests/c0-company-list.utf16le.xml` of branch `lab/72-reads-rd21`, sha256 `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8`) must show exactly the one company, with the number and GUID above; any other company, or another GUID, is a STOP. The marks read is `c1-marks-TEMPLATE` (path `lab-evidence/72-reads/requests/c1-marks-TEMPLATE.utf16le.xml` of the same branch, sha256 `4bcdcfd4b06f60fb6ee705cb3cb05c73037345a0ac123077071392f011d64c37`, placeholders `@@COMPANY@@` and `@@GUID@@` only); keep every answer whole, with its CMPINFO block. The answer holds `LastVoucherDate`.

## Part B: BATCH LAB (a count of one request)

The voucher mark must be above about 42 for the book to be counted at all (below it the old path is taken and this part measures nothing); on 12 October it was 82 and has only risen.

B0. Step 0 for BATCH LAB, then the tool: `a-list.jsonl` with build P. EXPECT `list_companies` one company, 100050, the GUID above; `tally_status`: record release and edition. Marks read `M-B0`. A different mark than on 12 October (82 voucher, 224 master, while three keyed test vouchers stood in the book) is recorded, not a STOP.
B1. Oracle, one window at a time, each with a screenshot, on Tally's own Day Book: 21 to 22 Jul 2026, 10 to 12 Nov 2026, 19 to 20 Aug 2026, each also with the day before and the day after. Write down the number of vouchers on each day. EXPECT (the 12 October book): 20 Jul holds 10 vouchers and 21 and 22 Jul none; no voucher near 10 to 12 Nov; 18 Aug and 21 Aug hold vouchers, 19 and 20 Aug none. If the book differs, the readings decide what to expect, and the sheet's EXPECT lines for that window are replaced by them in writing before the call.
B2. Build P, one file at a time, each filled with the GUID: `b-w1-batch-lab.TEMPLATE.jsonl`, then (after its answer, a screenshot of the whole Tally window, and the marks read) `b-w2-batch-lab.TEMPLATE.jsonl`, then `b-w3-batch-lab.TEMPLATE.jsonl`. Time each file. After the first file's answer take a screenshot of the whole Tally window before anything else is sent: the census-only check is a new request pattern on a live Tally, and a dialog there is a STOP.
B3. Marks read `M-B1`, equal to `M-B0`.
B4. The same three files with build M (a fresh data folder), in the same order and with the same checks. Marks read `M-B2`, equal.

## Part C: DENSE (a count of four requests; the heavy part)

DENSE holds about 29,900 vouchers at 82 a day from 1 Apr 2025. If the generator left no day empty, its last voucher `L` is near 31 Mar 2026, the end of a financial year, every day up to `L` holds vouchers, and an empty window exists only after `L`, in a financial year the book has no entries in. No read outside the book's last financial year has been shown on this book. Part C is therefore conditional, and each condition below is a gate, not a formality.

C0. Shut BATCH LAB; load DENSE alone by Select Company (the operator). Step 0 for DENSE (company list, marks `M-C0`, `a-list.jsonl` with build P). EXPECT ALTVCHID about 29,900; a mark differing by more than a few hundred is reported before C1.
C1. `L`: the date of the last voucher, from the marks answer's `LastVoucherDate`, and cross-checked on the Day Book for the last month of the book (not the whole year: it renders about 30,000 rows). The two must agree, else STOP and report.
C2. Windows: **D1** = `L`+1 to `L`+2 (both empty, the day before holds vouchers); **D2** = `L`+30 to `L`+32. For the oracle, set the Day Book period explicitly to `L` through `L`+34 and read it: it must show vouchers on `L` only, and none on `L`+1 to `L`+34. If Tally will not show that period, or the days after `L` are not empty: STOP, do not send D1 or D2, and report what the screen showed; that is a finding (this book has no usable empty window), not a failure of the change. D2 needs `L`+29 and `L`+33 empty too; they are inside the period above.
C3. `c-d1-dense.TEMPLATE.jsonl` (filled: GUID, `<D1_FROM>`, `<D1_TO>` as YYYY-MM-DD) with build P, once, timed. Marks read `M-C1`, equal to `M-C0`. Screenshot of the whole Tally window.
C4. `c-d2-dense.TEMPLATE.jsonl` (filled) with build P, once, timed. Marks read `M-C2`, equal.
C5. D1 with build M, once, timed; marks read `M-C3`. D2 with build M, once, timed; marks read `M-C4`. (M goes after P so the lighter read goes first on the heavy book.)
C6. C10: shut DENSE (the operator), no company loaded, current date as at step 0.

## What to record for every call

The tool's whole result (state, reason, `items`, `total`, `window` with its census, marks and parts, `evidence`), the `egress_log` records of that call (`response_prepared` and `stdio_write_completed`), the wall-clock seconds, the exit code, and the sha256 of the file sent. From the trail: `sent` (the total of requests sent), `failed` and `omitted`; the trail keeps only the last 32 sends, so a total is taken from `sent`, never counted from the tail. Classify the retained requests by their `kind` field (`post` or `status`) and by size within one run; do not infer a request's meaning from its size alone.

## EXPECT (a prediction from test doubles and the 12 October table; every line UNVERIFIED until run)

| call | state and reason (P and M) | `sent`, P against M |
|---|---|---|
| W1, 21-22 Jul | partial, `empty_window_wider_read_nonempty`, `items` empty, `total` 0 | P 28, M 34 |
| W2, 10-12 Nov | partial, `empty_uncorroborated` | P 34, M 40 |
| W3, 19-20 Aug | partial, `empty_window_wider_read_nonempty` | P 28, M 34 |
| D1, `L`+1 to `L`+2 | partial, `empty_window_wider_read_nonempty` | equal (both about 30 more than the first read): M reads the widened window whole (one data read), P sends a second read of the company's marks instead; P's trail holds no data read of the widened window and one extra marks read |
| D2, `L`+30 to `L`+32 | partial, `empty_uncorroborated` | equal (the widened window is empty, so the company mark is read in both) |

The 34 and 40 for M are what the 12 October run recorded for the old build, so the arithmetic of the doubles (28 and 34 for P) is checked against a live count at once. The saving is on a book counted in one request; on a book counted in several requests none is expected, and an equal count there is a pass, not a miss. A refusal `voucher_window_changed_during_read` in P on a book nothing else touched is a FINDING.

## Not measured by this sheet

`voucher_presence`, `ledger_movement` and the registers by name (they share the code); a window that holds a voucher (refused as `window_contradicted`; it cannot be produced without writing); Education mode; Gold; a client book; a count that moves during the read (it needs a hand edit mid-read, a write); the time saved by P (timed, but one run on one machine).

## Report

The `.out` and `.err` files, the marks answers with their CMPINFO blocks, the Day Book readings, the filled files and their diffs, the screenshots (kept on the lab PC, listed by name, size and sha256), both builds' digests, the Tally release and edition, wall-clock times, and a notes file with UTC times and one line per step. Evidence on a new branch `lab/1240-empty-window-live-answers` with `SHA256.txt`; send the SHA to the requester before pushing, and push only after the pre-push scan and the coordinator's yes. Confidence: one run on one release, PARTIAL.
