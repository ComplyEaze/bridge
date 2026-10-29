# Results: #1650 lost answer, 29 Oct 2026, through #2073's build (exe be79cf12…, a build WITH #2073). Row L ran on its second sitting, after the first halted at step 3 on a relay that refused the build's upper-case `EXPORT` read (fixed in the sheet at 87c08f4c). With Tally's answer to the create withheld, the sending call failed after 23.8 s with `import_dispatch_outcome_unknown`. The next call settled the batch as **`posted_unattributed`**, and ten minutes later it answered the same from the journal alone. The ledger is in the book, with ALTERID = M_I + 1. No answer said `posted_verified`, and none told the caller to create the ledger by hand. Row B: with a BUSY read holding the gateway at the click, the call refused after 20.3 s with `company_collection_invalid`, before any send. The same batch then posted `posted_verified`. Lab faults recorded below: in row B, one approval box was clicked with no call after it (nothing sent), and B2 was run again

The sheet is `lab/1650-masters-lost-answer`. Row B and row L's first sitting ran at 9e88962a7754eb3aa2c8ed81d3f60ee51830075b. Row L's rerun ran at 87c08f4c68bdc49058eb9396bbf7f45b97280310. `SHA256.txt` matched 5 of 5 at both commits, and the relay's self-test passed at both.

- **Build:** #2073's head 902ccc22a0eb44a5a57dc92cd76498b2ee169e53, run 40648570083, artifact 12799361804.
  - `bridge_mcp.exe` sha256 be79cf12056e4da3cfdb941aa98b5ace3a002a82f3b7f775326c27d329479fe3, equal to the package's smoke file and the same before and after.
  - Server 0.6.1.
  - `post_ledger_creates`' description names `posted_unattributed`, so the **[with #2073]** EXPECT lines apply.
- **Tally:** 7.2, Silver, licensed, `education_mode` false. One `tally.exe` (13992) owned 9001 before every post call.
- **Approval box:** a standard dialog titled "ComplyEaze Bridge — create one ledger?", owned by a child `bridge_mcp.exe` of the server, with Yes, No and Cancel buttons.
  - Each box was answered by a mouse click at the centre of its button. No key was typed while a box could be open.
  - Each box opened about 2 s into the call, and the call answered `pending` after about 40 s with `retry_after_s` 15.

## Row L: the relay, BRIDGE LOSTANS LAB (new, number 100097, `<company guid>`)

**Setup.** BRIDGE LOSTANS LAB was created on Tally's screen with books from 1 Apr 2025. The Features screen was accepted unchanged; its values are in the notes.

| | |
|---|---|
| BK1 | TDBK1800_100097.001, 23,390 B, sha256 8304321d… |
| M0, M0′ | 207 / none (no vouchers) |
| Ledgers | Cash, Profit & Loss A/c |

**First sitting (halted).** `prepare_ledger_creates` was refused with `agent_runtime_read_failed` (`http_status_failure`). The relay had answered 403 to a read, `refused:not_an_export`: the build sends `<TALLYREQUEST>EXPORT</TALLYREQUEST>` for its List of Ledgers read, and the relay compared with `Export` case-sensitively.
- Nothing was prepared or posted, and the marks stayed at 207.
- It is kept in `row-l/sitting1/`.

The sheet's relay was fixed (87c08f4c), and row L was run again on the same book with a new data folder and recording folder (`row-l/relay2/`).

| step | observed |
|---|---|
| 3 | B1 bridge-d9b0be48…, B2 bridge-3111f45f… (a second open batch for the same name was allowed); previews of nine lines; M 207 |
| 4 | names match the relay's (BRIDGE LOSTANS LAB, LAns Suspense); 118 relay lines, all `forwarded`, every answer returned |
| 5 | Box; `pending` after 40.1 s; Yes clicked 01:34:10.6Z; the call 01:34:11.5Z answered after **23.8 s**: failure `import_dispatch_outcome_unknown`, "The create may have been sent to Tally. Do not send this batch again: check the company's ledgers in Tally.", `attempt_recorded` true, `dispatch` {`reconciliation_required`, `resent` false}, `next_step` (says **not** to create the ledger by hand). No `posted_verified`. |
| 5, relay | `create matched` (request sha256 7504bbe1…, expected text sha256 9b18bfba…); manifest 183 `forwarded_answer_withheld`, Tally's answer 538 B (sha256 1b72152a…), `answer_returned` no; "held connection ended: client_closed, 20.0 s after the create was sent". **Tally's withheld answer: CREATED 1**, every other counter 0. |
| 7 | at once, no box, 2.8 s: **`posted_unattributed`**, `verification.moment` `reconcile`, `settled` true, `checks.first_change_after_intent` `passed`, `checks.counters` `not_checked`, `reasons` ["counters_unavailable"], `guid` …-000000ce, `answer_recorded` false, `dispatch` {`dispatched`, `resent` false}, no `alert`; `next_step`: found as sent, cannot show the change was its own create (no answer from Tally on file), check the ledger in Tally, a create Tally still holds could change it again, do not post again. Relay: reads only. |
| 8 | M1 **208 = M0′ + 1**; `LAns Suspense` present (Suspense A/c), GUID …-000000ce, **ALTERID 208**. Journal: the intent's `pre_post_master_marks` altmstid **M_I = 207 = M0′**; ALTERID = M_I + 1. No `master_dispatch_response`; one `master_verdict` `posted_unattributed` with the GUID and `counters_unavailable`, nothing after it. |
| 9 | B2: no box, 1.5 s: refusal **`master_name_taken`** (not `master_ledger_open_dispatch_overlaps`), `attempt_recorded` false, nothing sent; relay: reads only, no `refused:` line |
| 10 | ten minutes later, no box, 0.1 s, **no request to Tally**: `posted_unattributed` from the journal (`journal` {`response_recorded` false, `verdict` `posted_unattributed`}), the same `next_step` as step 7; M 208 unchanged |
| 12 | relay stopped (last line 274, a forwarded read); write settings removed; company shut and kept |

**What the row shows.** The "never settles" question (#2003): with #2073 the batch settles at the first read-back after the lost answer as `posted_unattributed`, and stays settled ten minutes later without reading Tally.

Things that did not appear:
- `posted_verified`;
- an instruction to create the ledger by hand;
- a second send.

## Row B: the busy gateway at the click, BRIDGE ABANDON LAB (100096, as row 19 left it)

**Setup.** Row 19's D was 187 s and disk free was 22.9 GB. BK1 is TDBK1800_100096.001, 13,255,549 B, sha256 6b375e1f…. M0 and M0′ are 216 / 48000. Batch BB is bridge-4f96e415…, and M stayed 216.

**B2, as the sheet sets it (attempt 2).**
- `pending` came after 40.1 s.
- BUSY started at 01:23:36.634Z with C3's line and the 1713 branch's `busy-year.xml` (ad2d4aa9…).
- Yes was clicked at 01:23:36.666Z.
- The call was made at 01:23:37.520Z: 0.9 s after BUSY, and 15.4 s after the pending answer.

**The call refused after 20.3 s:**
- code **`company_collection_invalid`**, "Nothing was sent. Review the error before requesting approval again.";
- no `cause`, no `next_step`;
- `attempt_recorded` false, `dispatch` `not_dispatched`;
- no journal intent.

`egress_log` shows that the call sent **1 request**: the 2,524-byte company read, held 20,016 ms, `request_deadline_exceeded`.

**Expected, and different.** As expected, the first read waited behind BUSY and the call refused before any create. **The code it gives is a collection fault, not a deadline.**

**B3.** BUSY ended by itself about 187 s after it started:
- 200, 238,211,254 B = CONTENT-LENGTH;
- 48,000 rows;
- `/status` 200;
- no dialog;
- M 216, and no `LAns Busy Suspense`.

**B4, the same batch again.** A new box opened, then `pending`, then Yes. The call answered in 6.0 s:
- **`posted_verified`**, `attempt_recorded` true, `dispatch` `dispatched`;
- `created` 1 and every other counter 0;
- GUID …-000000d7;
- M1 217, with one new row, `LAns Busy Suspense`.

The journal reads batch, intent, response, verdict `posted_verified`.

**Lab faults in row B, recorded rather than counted:**
1. The first box (01:14) was not found in time, because the lab looked for a window of the server process and the box belongs to a child process. It closed unanswered, and the next call answered `master_approval_timed_out` with nothing sent.
2. Attempt 1 (01:17):
   - BUSY started and Yes was clicked, but the lab's queue helper wrote an empty request file. The stdio client stopped on it, and the server exited, so **no call followed the click**.
   - Nothing was sent: the journal held only the batch line.
   - BUSY ran to its end (the same bytes as attempt 2's, 48,000 rows).
   - B3's checks were then made: M 216, no ledger.
   - The helper was fixed, and B2 was run again as above.

**Confidence:** one observation per row, PARTIAL until repeated.

## Files
- `answers/` and `client-log.txt`: row L's rerun, every tool answer whole, with UTC times.
- `row-l/`:
  - `relay2/`: the relay's whole recording (`manifest.tsv`, `events.txt`, every request, answer and header file);
  - `agent-master-ledger.jsonl`;
  - `checks/`;
  - `sitting1/`: the first sitting's answers, client log and relay recording.
- `row-b/`:
  - `answers/`, `client-log.txt`, `agent-master-ledger.jsonl`;
  - `checks/`: the timed sequences, the one-Tally record, and BUSY's printed status and headers.
- `notes.txt` (UTC).
- `substitutions.txt`: in the pushed copies, row L's company GUID is `<company guid>` (`&lt;company guid&gt;` inside XML) and the lab folder is `<lab folder>`. Each original is listed by bytes and sha256. The relay's `manifest.tsv` hashes are of the originals.
- `SCREENSHOTS.txt`: the captures, both BK1s, the BUSY answers, the full netstat output and process lists, by sha256. They stay on the lab PC.
