# Lab run sheet: Tally's own GSTR-1, GSTR-3B and HSN/SAC summary, requested by report name (#2011, #2012, #2013). SYNTHETIC COMPANIES ONLY. Not for merge.

About 60 minutes; the owner is needed at the screen steps (P4, P9, P10 and their Part 2 twins). Over the gateway this sheet only reads: no ComplyEaze Bridge program runs, nothing is keyed, nothing is imported. Two synthetic companies, one at a time: Part 1 `BRIDGE PILOT LAB` (August 2026), then Part 2 `BRIDGE GST LEDGER LAB` (November 2025), which sends its report requests only if that book has a GST registration (step L4).

**WHAT IT DECIDES:**
1. Whether TallyPrime 7.2 answers a request for its GSTR-1, GSTR-3B or HSN/SAC summary report under the names in `requests/` (the built-in report envelope of protocol reference §12a.1, which Cash Flow, Negative Stock and others answer, §12a.16), and with what structure. If every name is refused, the sheet shows only that these names are refused, not that the reports cannot be read.
2. Whether such a request changes any voucher's GST status or the company's marks. On Tally's screen, opening a GST report can ask to update the GST status of vouchers (recorded on 7.2 for GST Annual Computation; PARTIAL: one book, one run). The sheet reads that question's number before and after the gateway requests: if it falls, something updated those vouchers.
3. The screen figures each future reader must tie to: Total Vouchers counts and Return View rows of GSTR-1, the table totals of GSTR-3B, the HSN/SAC rows.

No code reads these reports yet; nothing is built before this capture (AGENTS.md P1).

**EXPECT (an expectation, not a fact):** the unknown name is refused in-band, as on 7.1; each report either answers under a name below or is refused the same way; every status compare says `same rows`; the marks do not move. Report what happened, not what is expected.

## FILES

`requests/`: each file is the product's own rendering, unedited: UTF-16LE with a byte-order mark, no final line break. `answers.py`: reads an answer by structure (see its header). `check-request-ids.py`: the request-ID census. Before step 0's first send:
- every file against `SHA256.txt` with `Get-FileHash -Algorithm SHA256`; any mismatch = STOP;
- `python -I check-request-ids.py requests`: EXPECT exit 0 and `census: ok 5, skipped 12, FAIL 0` (the 12 report requests are `TYPE Data`, which the census skips; the 5 collection requests, two marks and three status reads, each define the collection their header ID names);
- `python -I answers.py --self-test`: EXPECT `self-test: ok`, exit 0.

| file | what it is | builder that renders it (or that it becomes) |
|---|---|---|
| `p-marks` / `l-marks` | company marks (`Bridge Agent Company High Water`): GUID, ALTVCHID, ALTMSTID of every loaded company | `render_agent_company_high_water` (`company_high_water_read`), as sent today; byte-equal to the `r02-marks.xml` of the PILOT LAB status fixtures and to GS1-L1's `marks.utf16le.xml` |
| `p-status-all` / `l-status-all` | every voucher dated in the window, any type: date, number, type, MASTERID, ALTERID, the three GSTR-1 status flags, `ISGSTOVERRIDDEN`, cancelled, optional, post-dated | `render_gst_status_window` with its voucher-type condition left out. No builder renders it today; a narrower untyped variant (without the cancelled, optional and post-dated fields; its request not kept) answered on 10 Oct 2026 (§9.20). If kept, it becomes a type-less variant of that builder |
| `p-status-sales` | the same read for type `BRIDGE Sales` only | `render_gst_status_window`, as `gst_status` sends it today; byte-equal to the committed fixture `pilot-lab-gst-status-august.request.utf16le.xml` |
| `p-unknown` / `l-unknown` | the built-in report envelope with a name Tally has no report for (`Bridge Unknown Report Probe`) | `render_built_in_report_request`; the refusal shape on 7.2, the reference every report answer is classed against |
| `p-gstr1-a` / `-b`, `l-gstr1-a` / `-b` | the envelope with ID `GSTR-1` / `GSTR1` | `render_built_in_report_request` with that ID, for a GSTR-1 reader (#2011) |
| `p-gstr3b-a` / `-b`, `l-gstr3b-a` / `-b` | ID `GSTR-3B` / `GSTR3B` | the same, for a GSTR-3B reader (#2012) |
| `p-hsn-a`, `l-hsn-a` | ID `HSN/SAC Summary` | the same, for an HSN/SAC summary reader (#2013) |

Windows: Part 1 2026-08-01 to 2026-08-31; Part 2 2025-11-01 to 2025-11-30 (a day-30 end is accepted by a licensed release, §12a.16; the product's date check admits it on a licensed profile only).

## RULES

1. **Release and edition.** TallyPrime **Release 7.2**, licensed **Silver**, not Educational mode, read from Tally's own About screen and recorded. Anything else = STOP, nothing sent. Port 9001 only.
2. **Companies.** Only the two named synthetic companies, each loaded **alone** (Select Company; shut every other). Never a client company. `BRIDGE PILOT LAB`'s GUID is the `GUID` in the committed capture `src-tauri/crates/bridge-tally-protocol/tests/fixtures/agent/gst-status/pilot-lab-gst-uncertain-august-marks.utf16le.xml`; `BRIDGE GST LEDGER LAB`'s is the `GUID` in `lab/gs1-l1-answers/answers/s10-state-marks.xml` (branch `lab/gs1-l1-answers`; that answer also holds ALTVCHID 64). Copy both into `notes.txt` before step 0. A marks answer with another GUID or more than one COMPANY row = STOP for that part.
3. **Loading.** If loading a company shows any question, message or upgrade prompt (PILOT LAB was made on 7.1), press nothing, screenshot, and STOP that part: the owner decides; the operator answers nothing. The two prompts of a restore (step P11) are the only exception, as LAB-COMMON v3 C6 lists them.
4. **One request at a time; never abandon one; no `--max-time`.**
5. **Send with exactly:** `curl.exe -sS --data-binary "@requests\<file>.utf16le.xml" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download} %{time_total}\n" http://localhost:9001/` and record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. `<name>` is the name the step gives.
6. **The check after every report request (the item's gate).** Send the part's `status-all` file, then its `marks` file, each with the next name in the step. Run `python -I answers.py compare evidence\<baseline status>.response evidence\<new status>.response` and keep its whole output in `notes.txt`. EXPECT exit 0 (`same rows`; a `CMPINFO` difference is recorded, it is not part of the rows). Exit 1 = a voucher row changed: STOP. Exit 2 or 4 = the compare could not be made: STOP. The marks: EXPECT both equal to the baseline marks; a different mark = STOP.
7. **What STOP stops.** The **whole item** (both parts; nothing more is sent; screenshot; call the run coordinator): any dialog, prompt or message box on Tally's screen during or after a gateway request (if it is Tally's question to update the GST status of vouchers, record its exact wording; the run coordinator decides the answer, the operator answers nothing); 10 minutes without an answer; a transport failure (a non-zero curl exit or a non-200 answer: report it as a failure, never as "no report"); an answer over 1,000,000 bytes (record its size first); a rule-6 STOP. **That part only:** rules 2 and 3, and the registration checks of P4 and L4. A command that errors is reported as an error, never as "absent", "unchanged" or "not found".
8. **Backup.** Before the screen step of each part (P3, L3), Tally's own backup of that company into `C:\bridge-lab\2011\backups\p\` or `...\l\` (absent or empty BEFORE, exactly one file AFTER). A restore only where step P11 or L11 orders it.
9. **On Tally's screens:** look only, with the keys a step names. Never press Accept, Alt+J, Ctrl+J, or anything that resolves, accepts, ignores or edits a voucher. Leave each report with Escape after its screenshots.
10. **Read every answer by structure.** For every response: `python -I answers.py shape evidence\<name>.response`, output kept whole in `notes.txt`. It says whether the answer parsed, its class (`refused`, `empty` or `answered`), its root, its `COLLECTION/VOUCHER` row count, any `LINEERROR` text, and every element name with its count. An `empty` answer cannot be told from a report that was not rendered (§12a.11): it is recorded as such, never as "nothing to report". An answer that does not parse (exit 3) is still evidence. Never count by substring: `<VOUCHER` also matches `<VOUCHERNUMBER>`.
11. Evidence in `C:\bridge-lab\2011\evidence\` (absent or empty BEFORE): every response and headers file unedited, `log.txt`, `notes.txt`. Do not trim or re-save a response. Screenshots stay on the lab PC, listed by name, size and SHA256.

## STEPS

0. Rule 1 (About screen); the checks under FILES; the evidence and backup folders empty. No company loaded.

**Part 1: `BRIDGE PILOT LAB`, August 2026**

P1. Load the company alone (rules 2, 3).
P2. Send `p-marks` (name `p-m0`); record ALTVCHID and ALTMSTID (the committed reading of 13 Oct 2026 was 68 and 245, restored to after its run; a different value is recorded, not a STOP). ALTVCHID above 25,000 = STOP the part. Send `p-status-all` twice (names `p-s0`, `p-s0r`) and `p-status-sales` (name `p-x0`); `answers.py shape` on each; `answers.py compare` `p-s0` with `p-s0r` (EXPECT exit 0: two reads in a row agree; anything else = STOP the item). `p-s0` must hold at least one row (else STOP the part: the window is not the one this sheet was written for).
P3. Backup (rule 8).
P4. **OWNER, SCREEN, BEFORE ANY REPORT REQUEST.** Display More Reports > GST Reports: screenshot the menu; record every entry's exact title. Then open GSTR-1, period 1-8-2026 to 31-8-2026:
   - if Tally says GST details or a registration are missing, or asks to choose a registration: record it, Escape, and STOP the part;
   - if Tally asks to update the GST status of vouchers: record the exact wording and the number it names (N1), answer **No**, screenshot. On 7.2 the question is whether to open the report, so No leaves it closed; record "not opened". (That No changes nothing was recorded for GST Annual Computation only; P5 checks it here);
   - if the report opens without a question: record N1 = none; record the Total Vouchers counts and every Return View row (counts and totals) as printed, and the HSN/SAC Summary section if the report shows one.
   Escape. Then GSTR-3B for the same period, the same way (on an open report: every table total as printed: 3.1 rows, 4A to 4D, 5, 6.1). Then, if the menu has its own HSN/SAC summary entry, the same way.
P5. Send `p-status-all` (name `p-s1`) and `p-marks` (name `p-m1`). `compare p-s0 p-s1`: a difference here, or `p-m1` different from `p-m0`, is a FINDING (the screen step changed it), recorded, not a STOP; from here `p-s1` and `p-m1` are the baseline of rule 6.
P6. Send `p-unknown` (name `p-unknown`); `shape`. Then rule 6 (names `p-s2`, `p-m2`).
P7. Send each report file in this order, each followed by `shape` and rule 6 (status and marks names `p-s3`/`p-m3`, `p-s4`/`p-m4`, and so on): `p-gstr1-a`, `p-gstr1-b`, `p-gstr3b-a`, `p-gstr3b-b`, `p-hsn-a`. Every file is sent whatever the one before it answered; each answer's evidence name is its file's name (`p-gstr1-a` and so on); record each answer's class.
P8. Send `p-status-sales` (name `p-x1`); `compare p-x0 p-x1` (EXPECT exit 0; a difference = FINDING, recorded).
P9. **OWNER, SCREEN AGAIN.** As P4 (GSTR-1, then GSTR-3B, then the separate HSN/SAC entry if any), with the same No rule. Record N2 for each report. **N2 lower than N1, or N2 none where N1 was a number = FINDING of the first rank**: something during P5 to P8 updated vouchers' GST status. N2 higher than N1 is also a FINDING (recorded). If the reports opened in P4, record the figures again (EXPECT the same). If N1 was none, record that this run cannot show whether a request would update a pending status (none was pending).
P10. **OWNER, OPTIONAL, ONLY ON THE RUN COORDINATOR'S YES GIVEN AT THIS STEP** (recorded in `notes.txt` with the time), and only if P4 or P9 asked the update question: open GSTR-1 again and answer **Yes**; record the Total Vouchers counts and every Return View row as printed; then GSTR-3B (its table totals) and the HSN/SAC entry if any. Escape. Send `p-status-all` (name `p-s9`) and `p-marks` (name `p-m9`); `compare p-s1 p-s9` and record every row it names: this is what Tally's own update wrote. Then, so that a gateway answer and the screen figures come from the same book state, send again each report file that P7 classed `answered`, in P7's order, each with evidence name `<file>-y` and `shape`, each followed by rule 6 with `p-s9` and `p-m9` as the baseline. Without the coordinator's yes, record "P10 not run" and go to P12.
P11. **OWNER, only if P10 ran:** restore the P3 backup by LAB-COMMON v3 C6 (shut the company first; Restore under Data, Backup & Restore; the source identified by folder, file, bytes, sha256 and time; the only prompts answered are "Accept ?" = Yes and the "Company data folder ... already exists" prompt = Replace; any other prompt = press no key, screenshot, STOP the item). Load the company alone, send `p-status-all` (name `p-s10`) and `p-marks` (name `p-m10`). `compare p-s0 p-s10` (the backup was taken before the P4 screen step, so `p-s0` is what it holds): EXPECT exit 0, and the marks equal to `p-m0`; anything else = STOP the item and report "the restore did not bring the book back".
P12. Shut the company. No company loaded.

**Part 2: `BRIDGE GST LEDGER LAB`, November 2025**

L1 to L12 are P1 to P12 with the `l-` files and names (`l-m0`, `l-s0`, ...), the period 1-11-2025 to 30-11-2025, the backup folder `...\l\`, and these differences:
- L2: the last committed marks (GS1-L1's close) were 64 and 225; a different value is recorded, not a STOP (other items may have keyed since). This part has no `status-sales` file: skip its `x0` read and step L8.
- **L4 is the registration check.** The book was created with GST on but no registration entered (GS1-L1 step 1: no State, no GSTIN), unless a later item has added one. If the GSTR-1 screen says GST details or a registration are missing, asks to choose one, or opens with no GSTIN in its header: record it, Escape, and **skip L5 to L11**: no report request is sent to this book (a gateway request might raise the same message as a dialog). Record "Part 2: no registration".

## PASS / FAIL

- PASS: every request the steps reached was answered in full (HTTP 200), every rule-6 check said `same rows` with equal marks, and the screen steps recorded what the screens print (or that a report did not open). The sheet passes whatever the reports answer, including a refusal of every name: that is also an answer.
- FAIL: a transport failure, a dialog, a rule-6 STOP, a count made by substring, or a request sent out of order.

## REPORT

One message to the run coordinator: release and edition; per part every marks reading; per response its name, size, SHA256 and `answers.py shape` summary (class, root, row count, LINEERROR); for each report name, its class; every rule-6 result; the screen steps (menu titles; N1 and N2 per report with the wording; the figures as printed; whether P10/L10 ran and what `compare` named); any STOP and its scope; the evidence zip SHA256. Mark each finding verified, partial or unverified. The responses, headers, `log.txt` and `notes.txt`, unedited, go on a NEW branch `lab/2011-gst-report-names-answers` under `answers/`, with their SHA256 list: send its SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes. Nothing is committed to this branch.
