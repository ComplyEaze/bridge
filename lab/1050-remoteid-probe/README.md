# 1050-C: can a read reach a voucher by the REMOTEID ComplyEaze Bridge sent? (synthetic company only; not for merge)

**Why.** Tally keeps the REMOTEID a native post sends, because a delete by that REMOTEID works (protocol reference §9.14). But no field the voucher reads fetched in 1050-B returned it:
- The `REMOTEID` attribute carries the GUID.
- A `$RemoteID` filter matched neither the sent value nor the GUID.

If a read-only request can reach the sent value, it identifies the posted voucher in a way a hand re-key cannot copy. That would be the fix for #1050. These are reads only, apart from two restores of this synthetic company.

**Company.** The existing `BRIDGE IDENTITY LAB` from 1050-B. Today it holds K (the hand re-key), not N1. N1, the voucher the sent REMOTEID belongs to, exists only in backup I1, so step 1 restores it. The sent REMOTEID is the value in 1050-B's `sent-remoteid.txt` (`5da1d4d5-fc22-424f-9955-776d9c8ee090`). N1's GUID is `dcac52b6-2168-43fa-be2a-95b1488c11b4-00000002` (1050-B's `a1-window.xml`, the 10 July voucher). Only this company is loaded at every step.

**Request files.** In this folder; check each against `SHA256.txt` before use, and never send one that differs.
- **`c1-report-remoteid.xml`:** a small report that prints, for each voucher dated 1-31 July 2026, its GUID, the value of `$RemoteID` as text, and its MasterID. The GUID in the same line is the positive control.
- **`c2`-`c5`:** object exports of one voucher, each with an `ID` of type `Name` or `GUID`.
  - `c2` and `c4` are filled with N1's GUID. These are the controls: they show whether that request shape can return a voucher at all.
  - `c3` and `c5` are filled with the sent REMOTEID.

To fill a template, replace only its placeholder (`{GUID}` or `{SENT_REMOTEID}`):

`$t = [IO.File]::ReadAllText("<template>", [Text.Encoding]::Unicode); $t = $t.Replace("<placeholder>", "<value>"); [IO.File]::WriteAllText("<out>", $t, [Text.Encoding]::Unicode)`

Record each filled file's size and SHA-256.

**How to send.** One request at a time, never piped:

`curl.exe -sS -w "%{http_code} %{size_download}" --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" http://127.0.0.1:9001 -o <answer>`

Record curl's exit code and the printed line for each. A failed or empty answer, or an error answer, is kept and reported as failed, never as "not found".

## Steps

1. Write `restore-plan.txt` first: backup I1's folder and file, the company and its number, and the Restore Data Path. Take a fresh backup I3 of the company as it is now (it holds K), then restore I1 over BRIDGE IDENTITY LAB itself, exactly as the plan says. A restore screen that differs from the plan is a STOP before confirming.
2. With N1 in the book:
   - `c1` → `n1-c1.xml`;
   - `c2` with N1's GUID → `n1-c2.xml`, and `c3` with the sent REMOTEID → `n1-c3.xml`;
   - `c4` with N1's GUID → `n1-c4.xml`, and `c5` with the sent REMOTEID → `n1-c5.xml`.
3. Restore I3 (K) over the same company, by the same plan. Then `c1` → `k-c1.xml`, and `c3` and `c5` with the sent REMOTEID → `k-c3.xml` and `k-c5.xml`.

## What to report

For each answer: its size, SHA-256, HTTP status and encoding, read by structure (parse the XML; never search the text).
- **`n1-c1` and `k-c1`:** the number of `VOUCHERIDENTITY` lines; for each, its GUID, `REMOTEIDVALUE` and MasterID; and whether N1's `REMOTEIDVALUE` equals the sent value, the GUID, or something else (quote it).
- **`c2` to `c5`:** whether the answer holds a voucher, and which (by GUID). When the control (`c2`, `c4`) returns no voucher, that shape cannot address a voucher at all, and its `c3`/`c5` zero proves nothing: say so.
- **Overall:** which request, if any, reaches N1 by the sent REMOTEID and does not reach K.

## STOP

Change nothing, screenshot, save and report on any of:
- a Tally dialog other than the two restore prompts 1050-B recorded ("Accept ?" and the "already exists … Replace" prompt);
- a restore screen that differs from `restore-plan.txt`;
- no answer within 2 minutes (never re-send; check that `/status` answers);
- anything not in these steps.

Nothing is posted, and no voucher is keyed.

## Evidence

The answers, the filled files with their hashes, `restore-plan.txt` and the screenshots' hashes go on a never-merged `lab/1050-remoteid-answers` branch, through a push request to the orchestrator.
