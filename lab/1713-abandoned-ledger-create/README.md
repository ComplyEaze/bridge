# Does Tally still run a ledger Create its client gave up on, and when? A busy gateway, a create abandoned at ComplyEaze Bridge's 20-second limit, and a read queued behind it (#1713), on BRIDGE ABANDON LAB

**Synthetic company `BRIDGE ABANDON LAB` only, NEW for this sheet.** If a company or a data folder of that name already exists on the PC: STOP and report it; never reuse or overwrite one. It is loaded alone, and nobody else uses this PC's Tally from step 0 to the last step.

**This sheet WRITES.**
- By import: two ledgers, 24,000 to 48,000 Journals (96 to 192 files of 250), and seven probe ledgers `Lab Probe L0`, `L1`, `L1B` and `L2` to `L5`, each name used once.
- On Tally's own screens: the company's creation.

Licensed TallyPrime 7.1, not Educational mode, a single currency. No ComplyEaze Bridge build is used. Every request is sent by curl, as the ruling of 26 Oct 2026 allows for lab requests, writes included. Allow about four hours, most of it waiting:
- up to 1.5 hours of bulk import;
- about 35 minutes per row L2 to L5, run by a script that also does its observation.

LAB-COMMON v3 applies in full: file `lab/626-1038-1050-run-sheets/LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets`, commit 8c602c15. Where this sheet and LAB-COMMON differ, the stricter holds, except for the deliberate differences listed next.

## Deliberate differences from LAB-COMMON C3 (each one stated, nothing else differs)

1. **Overlapping requests.** In rows L1 to L5 only, `row.ps1` sends A, B and C so that they overlap, which is the point of the sheet. Everywhere else, one request at a time.
2. **Client limits.** The row script sends some requests with `--max-time 20` (marked LIMIT 20 below). That is ComplyEaze Bridge's own request limit (`DEFAULT_REQUEST_TIMEOUT`, 20 s, in `bridge-tally-transport`), so curl gives up as Bridge does. Curl's exit codes 28 (limit reached), 7 (connection refused), 52 (empty reply) and 56 (connection reset) are **results** of those rows, recorded, never STOPs.
3. **Timing fields.** Every send uses C3's line with `-w "%{http_code} %{size_download} %{time_connect} %{time_starttransfer} %{time_total}"`, and the row script logs each send's start and end in UTC milliseconds.
4. **The BUSY read's bound is 400 MB, not 2,000,000 bytes.** BUSY's answers are many megabytes each. Before M0, the disk space available on the evidence drive must be at least 3 GB, else STOP. A BUSY answer whose size differs from its CONTENT-LENGTH, or whose VOUCHER row count differs from the Journals imported, is a STOP. Measurements part 11c.1 records that answers this large can fail partway.
5. **The bulk import loop** (S4) takes a screenshot after the first and the last file, not after every file, and reads MARKS before and after the loop, not before every file.
6. **Reads sent again.** Inside the row script, the ledger read C is re-sent every 10 seconds, up to 30 times, until one is answered: it is a read, and a refused or reset one has not reached Tally's work. The observation reads are the same files sent again, each answer saved separately. Outside the script, C3's rule holds: a failed request is a STOP and is not sent again.
7. **Waiting.** A send that has not finished stops the sheet after D + 10 minutes, not 10 minutes. D is BUSY's measured time (M1).

THE COMPANY in LAB-COMMON is `BRIDGE ABANDON LAB`; record its company number and GUID at S1.

## Why, and what each answer decides

ComplyEaze Bridge creates a ledger with one `Create` behind an approval window. When Tally does not answer within 20 s, Bridge stops waiting, and its later reads can find the name absent. Today Bridge then holds the name in every company of its data folder, because nothing it can read rules out the create still waiting inside Tally (#1713).

The public protocol reference measured an abandoned **read** (measurements part, section 11b.2): the gateway stayed busy about nine more minutes after the client left, and every other request failed meanwhile. No abandoned **write** has been measured. The rule Bridge wants to rely on is: a request answered after the create was sent proves the create has either run or been dropped. This sheet tests that rule.

| row | compared with | the one thing that differs | decides |
|---|---|---|---|
| L0 | | an idle gateway | the control: one Create, its answer, its mark step |
| L1 | L0 | the gateway is busy with a long read (A); the create (B) is waited out | whether a request sent while the gateway is busy is accepted and queued, and in which order it is answered |
| L1B | L1 | a ledger read (C) is also sent, 5 s after B | the queue-order control: whether B and C are answered in the order they were sent, with A ahead of both |
| L2 | L1 | B is abandoned at 20 s, and C is sent the moment B's curl exits | THE QUESTION: is the probe ledger present in C's answer, and does anything change afterwards? |
| L3 | L2 | the probe name only (a repeat, P5) | that L2's answer repeats |
| L4 | L2 | A is abandoned at 20 s too | the same, when no client waits for A either |
| L5 | L4 | B is sent only after A's curl has exited, as ComplyEaze Bridge does: one request at a time, the next after the last was given up | the same, on the sequence Bridge itself produces |

**PASS for the rule (decided before the sitting).** In each of L2 to L5 that is not void:
- C is answered after A's answer ends (L2, L3), or after A's and B's curls exited (L4, L5);
- whatever C shows for the probe ledger, present or absent, every observation read after it shows the same.

**FAIL:** a probe ledger absent in C appears in a later read, or C is answered before A's answer ends in L2 or L3 (C did not queue behind A). Either way, no read rules out a waiting create, and ComplyEaze Bridge keeps the name held.

**VOID:** a row in which B was answered within its 20 s. Nothing was abandoned; the row is recorded and not counted.

**Confidence.** L2 and L3 are a repeated pair. L1B, L4 and L5 are single observations, PARTIAL by the reference's convention, until repeated.

**Builders.** None is new.
- The create: `LedgerCreate::render_message` (`src-tauri/crates/bridge-tally-protocol/src/ledger_create.rs`), inside `render_import_envelope` (`import_envelope.rs`, same crate), as `LedgerCreateItem::render_import` renders it for a Suspense ledger.
- The marks: `render_agent_company_high_water`.
- The year's vouchers: the verification window read (`src-tauri/src/agent_import.rs`).
- The ledgers: `ledger_name_index_read`, the read Bridge's post and reconcile send.

What this sheet decides is the release rule in `post_ledger_creates`, not a request shape.

## Files

- `requests/` (11 requests).
- `row.ps1`: rows L1 to L5. `-DryRun` prints a row's plan and sends nothing; it was not run before the sitting.
- `gen.py`, `BULK-SHA256.txt`, `.gitattributes`, `SHA256.txt`.

Check every file against `SHA256.txt` first. The bulk files are not in the branch: S4 writes them on the lab PC with `python -I gen.py bulk`, which checks each against `BULK-SHA256.txt` and prints `bulk ok` only when all 192 match.

| request | what |
|---|---|
| `marks.xml` | MARKS: GUID, ALTVCHID, ALTMSTID |
| `ledgers.xml` | LEDGERS: every ledger's name, GUID, parent, bill-wise flag and ALTERID |
| `busy-year.xml` | BUSY: every voucher from 1 April 2025 to 31 March 2026, by Bridge's verification read |
| `s3-bulk-ledgers.xml` | two ledgers: `Lab Bulk Expense` (Indirect Expenses) and `Lab Bulk Income` (Indirect Incomes) |
| `l0-create.xml`, `l1-create.xml`, `l1b-create.xml`, `l2-create.xml` … `l5-create.xml` | one Suspense ledger each, named for its row |
| `bulk/bulk-001.xml` … `bulk-192.xml` (made at S4) | 250 Journals each, dated across the year, 1.00 to 100.00 |

**Reading the answers.**
- **LEDGERS:** read by C4. Find the probe ledger by its NAME attribute exactly; record its GUID, parent and ALTERID.
- **BUSY:** count VOUCHER rows by structure, and record the size, CONTENT-LENGTH and `time_total`.
- **`times.txt`:** the row script's `times.txt` is the record of order.
- **Connections:** record the `Connection` header of every answer, from its `.headers.txt`. If Tally closes the connection after each answer, Bridge's pooled client cannot reuse one, and curl's fresh connections are faithful to it.

## Steps

- **S0.** LAB-COMMON step 0 (C2), with NO company loaded: C2 d is replaced by a screenshot showing no company loaded. Check every hash. Record the disk space available (EXPECT at least 3 GB). Run `powershell -NoProfile -File .\row.ps1 -Row L2 -DryRun` and keep its output. It must print five plan lines, else STOP.
- **S1.** Create THE COMPANY on Tally's screen: name `BRIDGE ABANDON LAB`, books beginning 1 April 2025, base currency INR, nothing else changed from the defaults. Screenshot the Gateway showing it loaded alone, and record the Current Period. MARKS: record the GUID and the company number.
- **S2.** Backup BK0 (`C:\bridge-lab\1713\backups\bk0\`).
- **S3.** `s3-bulk-ledgers.xml`: EXPECT `CREATED 2`. MARKS.
- **S4.** `python -I gen.py bulk`: EXPECT `bulk ok` (else STOP). Send `bulk-001` to `bulk-096`, one at a time, in order. EXPECT each answer `CREATED 250`, every other counter 0, no LINEERROR.
  - A PowerShell loop may do the sending: it saves each answer, reads `CREATED 250` and every other counter 0 by an XML parse, sends the next only then, and stops at the first other answer.
  - Screenshot Tally after the first and the last file, then MARKS.
- **M0.** BUSY alone, no limit. EXPECT 24,000 rows. Record its time as D0.
  - If D0 is 60 s or more, go to S5.
  - Otherwise send `bulk-097` to `bulk-192` as in S4, then MARKS.
- **S5.** Backup BK1 (`…\bk1\`): the bulk book.
- **M1.** BUSY alone, no limit. Record its size, CONTENT-LENGTH, rows (EXPECT 250 times the files sent) and time D. Then `GET /status`.
  - EXPECT D between 40 s and 300 s. Below 40 s, a create sent 5 s into BUSY may be answered within Bridge's 20 s: STOP, and the run coordinator decides. Above 300 s: STOP.

After every row, before the next: `GET /status` answers, then LEDGERS and MARKS, by hand.

- **L0** (control). `l0-create.xml`, no limit. EXPECT `CREATED 1`. LEDGERS shows `Lab Probe L0` under `Suspense A/c`. RECORD ALTMSTID's step (reference 9.4i saw +1).
- **L1.** `row.ps1 -Row L1`. RECORD from `times.txt`: B's start, its end and its exit code; whether B ended after A; and B's answer. EXPECT `CREATED 1`, and LEDGERS then shows `Lab Probe L1`.
- **L1B.** `row.ps1 -Row L1B`. RECORD the order of A's, B's and C's ends, and whether `Lab Probe L1B` is in C's answer.
- **L2, L3, L4, L5.** `row.ps1 -Row L2` (then `L3`, `L4`, `L5`), each run to its end, about 35 minutes.
  - RECORD from `times.txt`: A's and B's ends and exit codes, every C attempt with its exit code, which attempt is C, C's end, and A's end.
  - RECORD from C's answer and from each `ledgers-plusN` answer: the probe ledger, present or absent, with its GUID and ALTERID.
  - RECORD ALTMSTID from each `marks-plusN` answer, and every `:9001` line of each netstat file.
  - A row that prints VOID is recorded as void.
- **E1.** Backup BKE (`…\bke\`), MARKS, C10 close. THE COMPANY stays on the PC as BKE left it.

## STOP rules (beyond LAB-COMMON C8 and C9)

- **A dialog** or half-entered screen before a send: STOP.
- **A probe ledger absent in C that appears in a later read:** a FINDING, the decisive one. Let the row script finish. If it was L2, run L3 once, then STOP and report. Otherwise STOP and report.
- **A probe ledger that appears twice, or a ledger no row created:** STOP after recording.
- **A row script that prints STOP, or fails:** STOP. Do not run it again for that row.
- **`GET /status` not answering** within D + 10 minutes after a row's script ended: C9. Do nothing at all, report and wait. A restart of Tally is outside this sheet: the run coordinator decides it, and the sheet does not resume after one without a new word.
- **Hazard detail:** any finding that Tally refused every connection, or stopped, goes to the run coordinator, and its detail goes through `SECURITY.md`'s private channel, not into the pushed evidence.

## Evidence

- Every request and answer with size and sha256 (C11), with these exceptions kept on the lab PC:
  - the bulk answers, one line each in `bulk-answers.txt` with their counters;
  - BUSY's answers, listed by size, sha256 and row count.
- Each row's evidence folder: `times.txt`, the answers, the headers, and the netstat files.
- `notes.txt`, one line per step.

**Not covered here:**
- a create sent on a connection Bridge reused (if Tally keeps connections alive);
- a busy gateway caused by anything other than a read;
- Gold;
- a Tally restart during a row;
- an abandoned create of another master type.

Each is a later row if the rule needs it.
