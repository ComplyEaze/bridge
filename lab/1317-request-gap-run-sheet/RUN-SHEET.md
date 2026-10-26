# #1317: can the 500 ms gap between Tally requests be shortened? (the owner at the Tally screen)

Not for merge. READS ONLY, on one company the lab already holds. `LAB-COMMON.md` in this folder (v3, a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15) applies in full; where this sheet and it differ, the stricter holds. THE COMPANY is BRIDGE CORPUS DENSE (synthetic, voucher mark about 29,900). STOP if it is absent.

**WHY.** Before each census read, ComplyEaze Bridge waits out a fixed 500 ms gap (`SHIPPED_REQUEST_SPACING`). It was never measured as needed, and on a heavy book it is about 41% of the census time. Before the constant can change, one sitting must show two things:
- whether Tally holds each request longer when the gap is 250 ms;
- whether the Tally screen stays responsive for a person using it while the reads run.

**ONE VARIABLE.** The same fixed sequence runs four times: gap 500, 250, 500, 250 ms, in that order (runs A1, B1, A2, B2). The order guards against drift being read as an effect.
- **The sequence:** 40 census reads of THE COMPANY, one per day from 1 May to 9 June 2025.
- **The shape:** Bridge's committed day census (`note-days/credit_note_day_voucher_census_request`), with only the company and the day changed. Bridge already sends this shape, so it is not new to THE COMPANY.
- **The driver:** `driver.ps1` sends each request with LAB-COMMON C3's exact curl line, plus two timing fields appended to `-w` (`time_starttransfer` and `time_total`); the request bytes are unchanged. It sleeps the gap between the end of one answer and the next send. It checks every file's sha256 first. It stops at the first request that fails C3 or takes more than 20 s, and it never sends a request twice.

**THE SCREEN.** In LAB-COMMON terms, the owner's actions are display only: reports and the Gateway, never an entry or alteration screen and never a dialog. So C3's rule that nothing is sent while a dialog or half-entered screen is up stays true throughout. Any dialog, or any screen other than those listed, = STOP: whoever sees it at once creates an empty file named `STOP` in this folder (the driver checks for it before every send and then sends nothing more), presses no key in the dialog, takes a screenshot and reports; nothing further starts (C7).

**REQUESTS BY CURL (allowed by the owner's decision of 26 Oct 2026 that requests sent by curl may be used to discover shapes).** Every request here is an existing builder's request, used as an instrument; no new builder is made from them: `c01` to `c40` are the voucher census builder (`render_agent_voucher_census` in `src-tauri/src/agent_read_profiles.rs`, no span) for one day each, and `m00` is the company marks builder (`render_agent_company_high_water`, same file). Before the first send: `python -I check-request-ids.py requests` in this folder; EXPECT `census: ok 41, skipped 0, FAIL 0`; anything else = STOP, nothing sent. Read bounds (C3): a marks answer 8,192 bytes; a census answer 2,000,000 bytes. After each run, an answer in its `log.tsv` above its bound = STOP before the next run.

**OWNER steps** are marked [OWNER].

## Before the sitting
1. [OWNER] Present at the Tally screen for all four runs. Tally licensed. Educational mode = STOP (C1). Record the release from Tally's About screen (the lab PC runs 7.2 since 25 Oct 2026).
2. Check every file here against `SHA256.txt`.
3. **Parse the driver without running it:**

   ```
   powershell -NoProfile -Command "$e=$null; [void][System.Management.Automation.Language.Parser]::ParseFile((Resolve-Path .\driver.ps1),[ref]$null,[ref]$e); $e.Count"
   ```

   EXPECT `0`. Then the operator reads `driver.ps1` in full and confirms it sends only to `http://127.0.0.1:9001` and only the files under `requests/`.
4. [OWNER] Go-ahead.

## Steps
0. C2 step 0. Send `requests/m00_marks.request.utf16le.xml` by C3 (marks **M0**, one COMPANY row, THE COMPANY's).
1. **Run A1:** GET /status, then `.\driver.ps1 -GapMs 500 -RunName A1`. While it runs, [OWNER] performs the fixed actions of `RESPONSE-FORM.md`, in order, and marks each response. When it ends: GET /status, then marks (**M1**, `m00_marks` sent again; so for M2 to M4).
2. **Run B1:** the same, with `-GapMs 250 -RunName B1`. Marks **M2** after.
3. **Run A2:** `-GapMs 500 -RunName A2`. Marks **M3** after.
4. **Run B2:** `-GapMs 250 -RunName B2`. Marks **M4** after.
5. C10.

**Between runs:** at least two minutes with no request, and the owner back at the Gateway.

**STOP** (and no further run), in any of these cases:
- the driver ended with STOP in its `log.tsv`;
- any request over 20 s;
- any dialog on Tally's screen (the `STOP` file is created at once);
- a `STOP` file present before a run starts;
- a marks read with a COMPANY row count other than one, or another company's GUID;
- M0 to M4 differing (the book must not move: every request is a read);
- a screen action marked F for longer than ten seconds on the form.

## EXPECT (recorded; the lab judges only the STOP rules)
- **Per run, from its `log.tsv`:**
  - the 40 held times (`time_total`, and `time_starttransfer`), with their median and largest;
  - the run's total time from the first send to the last answer.
- **Between A runs and B runs:** whether the held time per request rises at 250 ms, and by how much against A's median. Whether the run time falls, and by about the 39 × 0.25 s the shorter gaps alone would save.
- **From `RESPONSE-FORM.md`:** each action's response in each run (immediate, a noticeable lag, or frozen over 3 s), as the owner marked it.
- The issue's rule is unchanged: only if the screen stays responsive and the saving holds does the constant change, with its own review. If not, it stays 500 ms, and these results say why.

## The book
It does not change. Every request is a read, and M0 to M4 must be equal.

## Close and evidence
- **C10.**
- **C11:** evidence to a new branch `lab/1317-request-gap-answers`, after the pre-push scan and the run coordinator's yes. It holds:
  - the four `runs/<run>/` folders (each `log.tsv`, the answers, headers and stderr files);
  - the five marks answers;
  - the filled `RESPONSE-FORM.md`;
  - the notes.

  Screenshots stay on the lab PC, listed by name, size and sha256.
