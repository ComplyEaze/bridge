# 1050-B: what identifies a natively posted voucher after a restore (synthetic company only; not for merge)

**Why.** After a backup is restored and vouchers are keyed again, Tally reuses a native post's MasterID, AlterID, GUID and voucher number (#1050 results). This run looks for a field that a hand re-key cannot reproduce, and that a harmless restore keeps:
- the REMOTEID ComplyEaze Bridge sent;
- the voucher key;
- creation facts such as entered-by or an audit trail.

It sends reads only, apart from one native post. Nothing here is a fix.

**Company.** A NEW company, `BRIDGE IDENTITY LAB`:
- books from 1 Apr 2026, no GST, no inventory, automatic numbering;
- ledgers `Synthetic Bank Ledger` (Bank Accounts), `Party A` (Sundry Debtors, bill-by-bill No) and `Lab Seed Income` (Indirect Incomes);
- a seed Receipt keyed on 1 Jul 2026: Cash Dr / Lab Seed Income Cr 1,000.00, narration `Seed voucher`.

Only this company is loaded at every step.

**Build.** Any installed CI build with voucher posting on. Posting is the owner's toggle in Claude Desktop. Record the build's `bridge_mcp.exe` SHA-256 and `tally_status` first.

**Request files.** In this folder:
- `b1` is ComplyEaze Bridge's own voucher read for 1-31 Jul 2026.
- `b2` is the same read, asking for more fields.
- `b3` to `b5` are templates.

Check every file against `SHA256.txt` before use; do not send one that differs. To fill a template, replace only its placeholder, in PowerShell:

`$t = [IO.File]::ReadAllText("<template>", [Text.Encoding]::Unicode); $t = $t.Replace("{GUID}", "<value>"); [IO.File]::WriteAllText("<out>", $t, [Text.Encoding]::Unicode)`

For `b5`, the placeholder is `{SENT_REMOTEID}`. Record each filled file's size and SHA-256.

**How to send.** One request at a time, never piped:

`curl.exe -sS -w "%{http_code} %{size_download}" --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o <answer>`

Record curl's exit code and the printed line for each. A failed or empty answer is kept and reported as failed, never as "not found".

## Steps

1. Take Tally backup I0 (file, time). Write `restore-plan.txt` first: the backup folder and file, the company and its number, and the Restore Data Path.
2. Native post N1 through ComplyEaze Bridge: `build_import_xml` with ONE Receipt dated 2026-07-10, Dr Synthetic Bank Ledger 301.00 / Cr Party A 301.00, narration `Lab identity probe 1`. Then `post_import` (approve in the window), then `verify_import`. Expect `posted_verified`, bound. Save each result.
3. N1's sent REMOTEID: in ComplyEaze Bridge's data folder, open `agent-import-ledger.jsonl`. Find the line carrying N1's `batch_id` and a `native_remote_id`, and copy that value (a UUID) into `sent-remoteid.txt`. Take N1's GUID from step 2's verify result.
4. Take Tally backup I1 (it holds N1). Add it to `restore-plan.txt` before taking it.
5. **Reads A:**
   - `b1` → `a1-window.xml` and `b2` → `a1-candidates.xml`.
   - `b3` filled with N1's GUID → `a1-by-guid.xml`. This is the positive control: it must hold exactly one voucher.
   - `b4` filled with N1's GUID → `a1-by-own-remoteid.xml`.
   - `b5` filled with the sent REMOTEID → `a1-by-sent-remoteid.xml`.
6. **A harmless restore:** restore I1 over BRIDGE IDENTITY LAB itself, exactly as `restore-plan.txt` says. Then repeat step 5's five reads → `a2-*`.
7. **A restore and a re-key:** restore I0 over the same company. Then key K by hand: Receipt 10-Jul-2026, Party A Cr 301.00 / Synthetic Bank Ledger Dr 301.00, narration `Lab identity probe 1`, automatic number. If Tally opens a bank allocation, accept what it shows and say so. Screenshot K.
   - Then repeat the five reads → `a3-*`. Fill `b3` and `b4` with K's GUID, read from `a3-window.xml`, and `b5` with N1's sent REMOTEID.
8. Take Tally backup I2.

## What to report

- For each answer: its size, SHA-256 and HTTP status, read by structure (parse the XML; never search the text).
- For each voucher in `a1`, `a2` and `a3`: the VOUCHER element's attributes (`REMOTEID`, `VCHKEY` and any others) and every child element of `b2`'s answer, as a table. Then mark each field equal or different, N1 (a1) against N1 after the harmless restore (a2), and N1 (a1) against K (a3).
- For `b3`, `b4` and `b5`: the number of vouchers each returned. If `b3` returns no voucher for a voucher that exists, the filter itself failed, and `b4` and `b5` answers from that step prove nothing. Say so.
- Which field, if any, holds the sent REMOTEID.

## STOP

Change nothing, screenshot, save and report on any of:
- any Tally dialog other than the restore prompts this list expects ("Accept ?" and the "already exists … Replace" prompt), and the bank allocation of step 7;
- a restore screen that differs from `restore-plan.txt`;
- a post answer other than step 2's expectation;
- no answer within 10 minutes;
- anything that differs from these steps.

Never post N1 again, and never rebuild it.

## Not measured by this run

Tally security (users and passwords) stays off, so entered-by may be blank on both vouchers. The lab's install keeps an edit log, so any audit trail found here may not exist on an install without one. Both are needed before a field is chosen.

## Evidence

The answers, `sent-remoteid.txt`, `restore-plan.txt` and the screenshots' hashes go on a never-merged `lab/1050-identity-answers` branch, through a push request to the orchestrator.
