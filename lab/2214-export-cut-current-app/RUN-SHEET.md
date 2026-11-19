# 2214 export cut on the current desktop app: the party/ledger master export, with its mailing read cut at the 20 s limit (Part of #2214)

Not for merge. Draft for the run coordinator: **the build block below is not yet filled, and the row does not start until it is.** About 70 minutes, one sitting. Form and rules are those of `lab/2074-2214-export-cut` at `7a49b5c1` (the sheet of the W row), whose Part W4 this row runs again, alone, on a current desktop app: `LAB-COMMON.md`, `relay.py`, `scripts/make_front.py` and `requests/marks.utf16le.xml` are that sheet's files, byte for byte (see `SHA256.txt`).

## The one question
**With a front of Journals queued ahead of the export's mailing read, does the export on the current desktop app fail closed with its sentence, write no file, and send no master post to Tally until the drain clears?**

Why this row: #2214 was reopened on 9 Nov 2026 because the 8 Nov export step ran a 0.6.1 app, older than the export's re-read without the details (#2188) and the hold after a cut (#2204). The server rows of 8 Nov ran on that build's own server and stand. The 18 Nov W row (`lab/2074-2214-w-answers` at `b8422a36`) ran W1 to W3 on the 0.7.0 app and ran no cut. The decision to keep `Refuse` on a cut (`tally/connection.rs`, `MailingReadDecision::Refuse`) is not moved by this row, whatever it reads.

**One variable.** Against W4 of 8 Nov, this row changes the app (the current one) and, on purpose, the book's size: W4's book had been rebuilt to the X rows' state (175 ledgers, about 32,000 dated entries) and this row does not rebuild it, because the cut is made by queueing and the decision under test does not read the size. The book stays at its rest state plus the two front ledgers. W0 below shows, before the cut, that the app asks for the mailing details on this book; if it does not, the row stops there.

## Writes to the book
**YES, to THE COMPANY only, undone at the end by restoring the backup taken at S0 (C6).**
- Ledgers `Lab Bulk Expense` and `Lab Bulk Income`, and Journals between them, by raw request (`make_front.py`): 500 in F0 and `n50` in the front of W4, as another client's imports.
- No ComplyEaze Bridge write tool is called (writes off in the server's launch configuration). The app's export is a read.
- A workbook file may be written to the user's Downloads folder (W0): the app's own output, kept.

## Needs, and what needs a person
- **The installed desktop app** whose two files' hashes equal the build block. This sheet installs nothing: an install (the NSIS setup from Explorer, as on 8 Nov) is an owner act outside this sheet. If the hashes differ: STOP and tell the run coordinator.
- **No setting of the app is changed.** The app's Tally endpoint must already read `127.0.0.1:9002` when it starts (the W row found it so). If it reads anything else: STOP. Choosing THE COMPANY in the app's company list is the one click that is not an export; the W row made it.
- **Acts on application windows, all by the lab, none by the owner:** starting the app by `helpers\app-start.cmd`; the clicks of W0 and W4 (the company choice, the export button); closing the app at Close by its window's close button, as the W row did. No application is restarted during the row and no process is ended by name or number. If closing the app at Close is not wanted, leave it running and say so in the notes: nothing after it needs it closed.
- **The run coordinator's word** for the restore (C6), as in every row.
- **Standing words the lab names in its notes:** raw requests, writes included (26 Oct 2026); master writes on a synthetic book (28 Oct 2026); the backup (C5).

## Rules and company
`LAB-COMMON.md` (v3) applies in full; where it and this sheet differ, the stricter holds. The lab's standing rules stand above both.

**THE COMPANY** is `BRIDGE BATCH LAB` (synthetic), GUID `a30f23c3-4819-4334-96ab-ffa6fba70976`, marks at rest `ALTMSTID` 224 and `ALTVCHID` 87, 11 ledgers. Anything else at P0: record it and ask.

**Named exceptions to LAB-COMMON C3.** Nothing raw is sent but: MARKS, C2 c's and C10's `GET /status`, `front-ledgers.xml` and `front-probe.xml` by C3's line (F0, through `helpers\raw.py send`), and `front-w4.xml`, which only `relay.py` sends, once, when armed.

**Marks.** A master mark that moves other than as a step says = STOP. A voucher mark that moves = STOP, except by exactly the Journals of a front that was answered.

**A refused call is a result, not a STOP,** when the step expects it: `tally_endpoint_draining` with `retry_after_s`; for the app, the export's failure at W4. Any other refusal is a STOP.

**No raw request while a drain may be owed.** From the cut until a STANDARD call has answered, no MARKS and no `GET /status`. `helpers\w4.py` copies the drain record (`<key>.wire.debt` beside the port's wire lock, as `lab/2142-drain-live` describes; its key is port 9002's) and sends nothing raw before its last step.

**If Tally stops answering.** A drain still refused after 20 calls or 3 minutes, or any sign of a hang, is C9: STOP and report. No probing, and no restart until the run coordinator says.

**Keys.** The Gateway is the only Tally screen used: Select Company at P0, Backup at S0, Restore and Company > Shut at Close. Never Ctrl+Q or Escape at the Gateway. Licensed mode only; Education mode = STOP.

## Build (NOT YET FILLED: the run coordinator fills this block after picking the build)
| line | value |
|---|---|
| head commit of the build | NOT FILLED |
| CI run (name, id, trigger, result) | NOT FILLED |
| artifact name and GitHub digest | NOT FILLED |
| bundle zip: bytes and sha256 | NOT FILLED |
| app version string | NOT FILLED |
| installed `bridge.exe` sha256, read from the real install path | NOT FILLED |
| installed `bridge_mcp.exe` sha256, read from the real install path | NOT FILLED |
| the bundle's `bridge_mcp.exe` sha256 (the server of every tool read) | NOT FILLED |
| TallyPrime release and edition at the lab | NOT FILLED |
| the app's file lines this sheet cites, rechecked at that head | NOT FILLED |

**File lines to recheck at that head:** the export's failure message, `commands.rs` (`MAILING_DETAILS_READ_FAILED_SENTENCE`, and the sentence that opens "ComplyEaze Bridge withheld the party/ledger master:"); the decision, `tally/connection.rs` (`MailingReadFailure::decision`); the button, `OutstandingsScreen.tsx` ("Ledger master + group subtotal trace"). At master `72c2408b8` the sentence and the button read as quoted in this sheet; any difference at the picked head is recorded before W0.

## Inputs
- **The kit:** this folder, copied whole to `<lab folder>\sheet\`. `SHA256.txt` lists every file with its bytes; check each before P0.
- **Helpers carried from the answers branches** (in `helpers\` and `args\`, byte copies of the pushed files: `client.py`, `call.py`, `raw.py`, `rd.py`, `note.py`, `app-start.cmd`, `close-hash.cmd`, `c6prep.py`, `setup-run3.py` and `launch-env.json` from `lab/2074-2214-w-answers` at `b8422a36`; `w4.py` from `lab/2074-2214-w4-answers` at `a16b7cc7`; `args\standard.json` and `args\basic.json` from `lab/2074-2214-export-cut-answers` at `b8422a36`; see `SHA256.txt`). `note.py "<text>"` appends one UTC-stamped line to `evidence\notes.txt`. They are the pushed copies, whose machine-specific strings were replaced by placeholders before they were pushed. The lab sets each placeholder in a **copy kept in the lab folder** (never in `sheet\`), records the sha256 of every copy it runs and the count of each replacement in its notes:

  | placeholder | set to |
  |---|---|
  | `<lab folder>` | this row's working folder, `C:\bridge-lab\2214-cut` (a new folder; it must not exist at P0) |
  | `<local app data>` | the per-user folder the app is installed in, read from the app's real install path at P0 |
  | `<Tally data folder>` | Tally's data folder, as C6's Restore Data Path |
  | `<lab root>` | the lab's own tools root, where its window-capture tool stays (`raw.py`'s screenshot call); not a kit file |
  | `<lab root name>` | left as it is (a client name string; the server does not read it) |

  - `helpers\c6prep.py` also holds the backup's expected bytes and sha256 (`WANT`) from the W row's backup. **Set them to this row's S0 backup** (its bytes and sha256 from S0's notes) before C6.
  - `helpers\w4.py` names `front-w4.xml`, `<lab folder>\fronts\`, `run2\`, `arm\` and `evidence\relay\`: the folders below. Its line that mentions "2026 ms" is a constant carried from an older row: compare each held time with this row's R1 threshold by hand.
  - `helpers\*` go to the lab folder's root, as `raw.py`, `rd.py`, `w4.py` and the others expect.
- **New in this row** (not run on Tally before): `scripts\cutlog.py` and `scripts\listdirs.cmd`. `cutlog.py` was run on a stand-in log only, which shows its own parsing and nothing about Tally.
- **The relay** (`relay.py`, unchanged): `python -I relay.py 9002 9001 <lab folder>\evidence\relay <lab folder>\arm`, with `arm` an existing empty folder and `evidence\relay` absent or empty. Started once before the first read, left up to Close. Its log is `relay.jsonl`; its events are `listening`, `open`, `forwarded`, `front_send`, `front_written`, `front_first_byte`, `front_answered`, `bridge_cut`, `tally_closed`, `relay_error` and `close`; an answered request is a line with no `event` and with `line`, `sent`, `answered`. A `relay.jsonl` with no `open` line means the row failed, not that nothing happened. A `relay_error` or `tally_closed` that a step does not name = NOTE: send it, do not explain it.
- **The server:** `helpers\client.py` over the bundle's `bridge_mcp.exe`, in `<lab folder>\run2\` (`inbox`, `answers`, `data`, `evidence` new and empty), with `helpers\launch-env.json` copied there: `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9002`, a new empty data folder, terms accepted, `BRIDGE_AGENT_MAX_BYTES=5000000`, writes off. `python -I -X utf8 client.py <lab folder>\run2 <exe path> <exe sha256>` once, left up to Close; it refuses if the hash differs. Tool calls: `helpers\call.py`, `helpers\rd.py` and `helpers\w4.py`; argument files `args\standard.json` and `args\basic.json`.
- **The reads.** STANDARD: `ledger_masters` with `fields=compliance`, as `rd.py <label> standard`. Telling a master post's kind: `cutlog.py` decodes the request kept in `bodies\` and says "mailing master" for a List of Ledgers request that names `LEDMAILINGDETAILS.LIST`, "standard master" for one that does not. No substring count of raw bytes is a grade.
- **The front:** `python -I make_front.py <lab folder>\fronts ledgers`, then `... front-probe.xml 0 500`, then (F2) `... front-w4.xml 500 <n50>`. Record what it prints and check `front_send`'s `sent_sha256` against it.
- **Folders:** working folder `C:\bridge-lab\2214-cut\`; evidence in `evidence\`; the backup in a new empty `bk\` there.

## Before the sitting
- **P0.** C1 and C2 through `helpers\raw.py step0 p0`: THE COMPANY alone. EXPECT MARKS 224 / 87. Hash every kit file against `SHA256.txt`; read the installed app's two files' hashes with `helpers\close-hash.cmd` and compare with the build block: any difference = STOP. Record the app's version string.
- **S0.** The backup by C5 into `bk\`; record the marks after it. Set `c6prep.py`'s `WANT` from it.
- **B5.** No ComplyEaze Bridge process of ours is running (the lab's own record of what it started; no process list is used to end anything).
- Start the relay, then the server. `list_companies` through `call.py`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`.
- **R1.** `rd.py r1a standard`, 120 s, `rd.py r1b standard`. EXPECT `total` 11, marks 224 / 87 both. The post threshold is the larger of twice the slower master post's `held_ms` and that plus 2,000 ms. EXPECT the drain record none or `{"state":"clear"}`.

## Part F: the front
- **F0.** `make_front.py` writes `front-ledgers.xml`; `raw.py send f0-front-ledgers` it: EXPECT `CREATED` 2, other counters 0, no `LINEERROR`, master mark up by two (226), voucher mark unmoved. Then `front-probe.xml` of 500 Journals; `raw.py send f0-front-probe` it, timed by the note line's seconds: EXPECT `CREATED` 500, every other counter 0, no `LINEERROR`, voucher mark up by exactly 500 (587), master mark unmoved. A capture of the whole Tally window after each: any dialog = STOP. The probe is the row's NEW SHAPE, and the book's first Journal between these ledgers.
- **F1.** With `t` the seconds the 500 took, `n50` = 500 × 50 / `t`, rounded down. If `n50` passes 3,000, use 3,000 and record it.
- **F2.** `front-w4.xml` of `n50` Journals from first index 500 (so no REMOTEID repeats the probe's). Record its bytes and sha256.

## Part W0: the baseline export (no front)
1. App start: `helpers\app-start.cmd`. EXPECT its output file to show one `bridge.exe` process at the real install path with a sha256 equal to the build block's. Check Tally in the app: EXPECT the endpoint `127.0.0.1:9002`, reachable, THE COMPANY alone offered; choose it. Anything else = STOP.
2. `scripts\listdirs.cmd w0-before`. Note `relay.jsonl`'s line count `L0`.
3. Press the Outstandings screen's "Ledger master + group subtotal trace" with the As of field at the host's date. No front is armed.
4. `scripts\listdirs.cmd w0-after`. `python -I -X utf8 cutlog.py <relay.jsonl> L0+1 > evidence\w0-cutlog.txt`. Save the workbook's name, bytes and sha256.
- **EXPECT (W0):**
  - `w0-cutlog.txt`: one "mailing master" request from the app's connection, state `answered`; no `front_send` and no `bridge_cut` line; the SUMMARY reads "no bridge_cut". A "standard master" request in the log is recorded, not graded: it would show another path of the app, not a failure of this step.
  - `w0-after-listing.txt` against `w0-before-listing.txt`: exactly one new file in the Downloads listing, named `party-ledger-master-` plus the company's name and a date, ending `.xlsx`.
  - **If the app sent no mailing master request, or wrote no workbook: the row cannot answer its question. Record it, go to Close.**

## Part W4: the export at the cut
1. `scripts\listdirs.cmd w4-before`. Note `relay.jsonl`'s line count `L1`. In a console the lab keeps open: `python -I -X utf8 w4.py` (it checks the drain record is clear, arms the relay with `front-w4.xml`, waits for the relay's `bridge_cut`, and then runs D1, D2, D3 and Whole through the server, as W4 of 8 Nov did). **Within its 300 s wait,** press the same button as W0 on the same screen. Nothing else is touched.
2. When the export ends: copy the app's message **whole** from the app's window into `evidence\w4-message.txt` (the lab reads the window's text and also keeps a capture of it that stays on the lab PC). Then `scripts\listdirs.cmd w4-after` and, once `w4.py` has finished, `python -I -X utf8 cutlog.py <relay.jsonl> L1+1 > evidence\w4-cutlog.txt`.
3. VOID: if the app's mailing post was answered (state `answered` in `w4-cutlog.txt`), Tally served it ahead of the front. Record it, keep the marks, and go to Close: a second go needs the run coordinator's word and new front indices.

**EXPECT (W4), each graded from the file named:**

| # | EXPECT | graded from |
|---|---|---|
| 1 | The arm, then in order in `relay.jsonl`: `front_send` (its `sent_sha256` equals the one F2 printed), `front_written`, `forwarded` for the app's mailing request with `front` 1, `bridge_cut` for it. | `evidence\relay\relay.jsonl`; `evidence\w4-cutlog.txt` |
| 2 | That request is a "mailing master" in `w4-cutlog.txt`, state `CUT`, 15 to 25 s after its forwarded event (19.687 s on 8 Nov; the older sheet's "about 20 s"). | `w4-cutlog.txt`, SUMMARY line |
| 3 | The app's connection forwards nothing after the cut: "on the cut's own connection after it: 0". | `w4-cutlog.txt`, SUMMARY line |
| 4 | The export failed: `w4-message.txt` opens "ComplyEaze Bridge withheld the party/ledger master:" and holds "The read that failed also asked Tally for each party's dated mailing addresses" and `[tally_request_deadline_exceeded]`. Record it whole, with whatever remediation follows. | `evidence\w4-message.txt` |
| 5 | No file written: `w4-after-listing.txt` has no file that `w4-before-listing.txt` lacks, in any of the three folders, and no listed file has a write time after the arm's UTC in `w4-log.txt`. | the two listings; `evidence\w4-log.txt` |
| 6 | No master post to Tally after the cut until the admitted STANDARD: every "master" line the SUMMARY lists after the cut has a UTC not before the sent time of the first call in `w4-log.txt` that was not refused (the call D3 names). | `w4-cutlog.txt`; `w4-log.txt` |
| 7 | D1: the drain record read at once after the cut is `owed`. A `clear` record is a result. | `debt\` copy; `w4-log.txt` |
| 8 | D2: each call before the admitted one is `tally_endpoint_draining` with `retry_after_s` and no master post sent (its trail's master posts: none). The admitted call comes within 20 calls and 3 minutes. | `w4-log.txt` |
| 9 | D3 and Whole: the admitted read's master posts held under this row's R1 threshold; `total` 13; the Whole read's rows equal the admitted read's, compared as parsed JSON with the `snapshot` block left out. | `w4-log.txt` |
| 10 | The front was answered: `front-1.answer` decodes (UTF-16LE) to `CREATED` equal to `n50`, every other counter 0, no `LINEERROR`. | `evidence\relay\front-1.answer` |
| 11 | Marks: master mark unmoved (226); voucher mark 587 + `n50` exactly. | `w4-log.txt` MARKS lines |
| 12 | Drain record `{"state":"clear"}` after the Whole read and the marks. | `debt\` copy |

## Close
1. Writes off: reads only. Close the app by its window's close button (or leave it, and say so). Stop the server by `run2\inbox\stop`. Stop the relay by the lab's own handle on the process it started, not by name or number. Parse `relay.jsonl` whole; each `relay_error` or `tally_closed` line, if any, is named in the notes.
2. Re-hash the installed app's two files (`helpers\close-hash.cmd`), the server exe and the kit: all equal to P0's.
3. **Restore the S0 backup by C6**, on the run coordinator's word, with `helpers\c6prep.py before` and `after`.
4. MARKS: EXPECT 224 / 87, as after S0. Then `helpers\setup-run3.py` and a basic `ledger_masters` through a new server on port 9001 (`args\basic.json`): EXPECT 11 ledgers.
5. C10. `listdirs.cmd` is not run after the shut.

## Decides
| reading | then |
|---|---|
| W0 as EXPECT, W4 1 to 12 as EXPECT, not VOID | #2214's reopening reason is met: on the current app the export fails closed on a cut, with its sentence, writes no file and sends no master post before the drain clears. `Refuse` stays as it is; this sheet moves nothing. |
| W4 4 differs (another sentence) | send the whole message: the maintainers decide whether the text or the build is what changed |
| W4 5 shows a file, or 6 shows a master post before the admitted call | #2214 stays open: send `w4-cutlog.txt` and the listings; nothing is built from one go |
| W4 VOID | no cut was seen on this build: #2214's reason stays open; record the order finding |
| W0 fails | the row cannot answer: record why |

## Limits
The cut is made by queueing, not by a slow read: `Deadline` is the same failure to Bridge, but the time Tally stays busy is the front's, so D1 to D3 measure the drain after a cut and not how long a slow mailing read runs. The book is small and its mailing answer is short; the X rows and W4 of 8 Nov used a large one. One synthetic book, one build, one Tally release. The relay adds a hop and holds each answer until whole. The app's message is read from its window; the sheet does not know whether the app keeps a file copy of it. `cutlog.py` and `listdirs.cmd` are new and were not run against Tally or on Windows.

## Keep
The tool answers, `relay.jsonl`, `bodies\` (under C11's size rule), `front-1.answer`, the drain record copies, `w4-message.txt`, the two cut-log files, the six listings, the workbook of W0 (name, bytes, sha256), the marks, the notes with the placeholder replacements and the sha256 of every helper copy run. Screens and captures stay on the lab PC, listed by name, size and sha256. Answers branch `lab/2214-export-cut-current-app-answers`, RESULTS graded verified, partial or unverified; check that no machine path and no lane name is in anything pushed.
