# W22: does Tally keep the narration of a Sales invoice byte for byte (for the invoice readback's narration comparison)

Synthetic company `BRIDGE PILOT LAB` only. It WRITES: three invoices keyed on screen, then the book restored from a backup taken first. Nothing else is keyed, altered, accepted, cancelled or deleted, no company feature is changed, and no other company may be loaded. Never use Accept As Is. Tally's current date is never changed.

The invoice readback now compares the narration Tally holds with the one the code wrote, byte for byte, and reports any difference as a divergent invoice. The earlier sales rehearsal read back only empty narrations, so it is not measured whether Tally keeps a non-empty narration of an invoice exactly as typed. This set types three narrations on three invoices and reads them back.

`w22-window-0920.xml` is the one-day voucher read of `workorder/w-window-0802.xml` (the lab's earlier one-day window, SHA-256 `9254eb91…`) with its four dates changed from 20260802 to 20260920 and nothing else. `r02-marks.xml` is the marks request of the earlier captures. Check both against `SHA256.txt` first.

**General STOP rule.** Stop and tell us before going on if: Tally shows any dialog, message or warning that this list does not name; an answer is not a complete envelope with a `COLLECTION` or carries a `LINEERROR`; an answer is larger than 60 KB or takes longer than one minute (do not resend it; check Tally still answers `r02-marks.xml`); a backup file is missing or its SHA-256 cannot be taken; or the marks after the restore differ from the marks of step 2. Never press Escape on a voucher screen or any other screen: save the voucher, or STOP and tell us. Leaving a screen: after a save, use Ctrl+Q once; any prompt on leaving means press no key and STOP. Tally must be at the Gateway with no dialog open before every request. If the screen offers no way to type a narration on the invoice, do not save: STOP and say what the screen shows. If the restore fails, leave the lab as it is and send nothing further.

**How to send.** One request at a time, `curl.exe` with `-w "%{http_code} %{size_download}"`, each answer written with `-o` and never piped; note curl's exit code and printed line for every request.

## The sitting, in this order
0. Before anything: run `tasklist` to see the Tally processes and `netstat -ano` to see which process id owns port 9001; note both. EXPECT: exactly one `tally.exe`, it owns port 9001, and either no company is loaded or `BRIDGE PILOT LAB` is the only one. If not, STOP and tell us; start nothing and open no application. Record Tally's current date at the Gateway (look only, change nothing). If no company is loaded, load `BRIDGE PILOT LAB` alone from the Select Company list.
1. Take a backup of `BRIDGE PILOT LAB` as it is now (`W22-NOW`). Record its path, its size and the SHA-256 of the backup file. STOP if the file is missing, is 0 bytes, or its SHA-256 cannot be taken.
2. `r02-marks.xml` -> `aw22-marks-0.xml`. EXPECT: a complete envelope of 3,394 bytes; record the marks.
3. `w22-window-0920.xml` -> `aw22-a.xml`. EXPECT: a complete envelope with a `COLLECTION` holding no voucher (the book has no voucher dated 20 September 2026) and well under 20 KB. If it holds a voucher, STOP and tell us.
4. Key three `BRIDGE Sales` invoices dated 20 September 2026, numbered `BP/26-27/0050`, `BP/26-27/0051` and `BP/26-27/0052`, each the same shape as `BP/26-27/0001` (the same party, ledgers and amounts: BRIDGE Walk-in, 1,000.00 plus 25.00 and 25.00 tax, total 1,050.00), each with the narration below typed exactly as written between the marks (the marks are not typed), and saved with the plain save. Note any dialog on saving.
   - `0050`: `Probe one & two <three> "four" 'five' 6% [seven]`
   - `0051`: `Probe  two  spaces, and a trailing space ` (two spaces after "Probe" and after "two", one space after the comma, and one space at the end after "space")
   - `0052`: `Probe three [BRIDGE:probe-0052]`
5. `w22-window-0920.xml` -> `aw22-b.xml`. EXPECT: a complete envelope with a `COLLECTION` of exactly three vouchers, about 24 KB (the earlier one-day window of 1 August 2026 held eleven vouchers in 87,884 bytes, `lab/1342-w18b-answers`, about 8 KB a voucher); STOP above 60 KB.
6. `r02-marks.xml` -> `aw22-marks-1.xml`. Record whether its marks differ from `aw22-marks-0.xml` and how.
7. Shut `BRIDGE PILOT LAB`, then Data > Backup & Restore > Restore from `W22-NOW`: on the Restore screen confirm that the source is the path recorded at step 1, that its size and its SHA-256 are the values recorded at step 1 (re-check both on the file now), and that the destination is the existing `BRIDGE PILOT LAB` folder (the screen lists it as `BRIDGE PILOT LAB (100033)`); the only dialog expected is the one saying the company data folder already exists and that Replace will overwrite it: choose Replace. STOP before choosing anything if the screen names another company, offers a new folder or a new number, or asks anything this list does not name. Then load `BRIDGE PILOT LAB` alone, then `r02-marks.xml` -> `aw22-marks-2.xml`. EXPECT: byte-identical to `aw22-marks-0.xml`. If not, do nothing further, leave the lab as it is, send nothing, and report.
8. `w22-window-0920.xml` -> `aw22-c.xml`. EXPECT: a complete envelope holding no voucher, as at step 3, well under 20 KB.
9. At the Gateway, look only: Tally's current date. EXPECT: the step-0 value. List the processes again and run `netstat -ano` again: EXPECT exactly one `tally.exe`, owning port 9001. Leave the company loaded or shut exactly as step 0 found it: if no company was loaded at step 0, shut `BRIDGE PILOT LAB` now; if it was loaded, leave it loaded and alone. Then leave with Ctrl+Q once if any screen is open (never Escape), and do not start another Tally instance.

## What to post back
The six answers (`aw22-marks-0`, `-a`, `-b`, `-marks-1`, `-marks-2`, `-c`), their hashes added to `SHA256.txt`, the backup's path, size and SHA-256, for each request curl's exit code and printed line, the narration each of the three vouchers reads in `aw22-b.xml` exactly as the answer holds it, the current date at steps 0 and 9, the `tasklist` and `netstat -ano` lines of steps 0 and 9, whether a company was loaded at step 0 and at the end, every dialog seen and its exact text, and whether you did anything that differs from this list.

---

*For the reader (not for the lab):* the narration comparison of the invoice readback treats an invoice whose narration Tally holds differently from the text written as divergent. This set measures whether Tally keeps three kinds of invoice narration (special characters, repeated and trailing spaces, a tag-shaped suffix) as typed.

Pre-registered expectations (written before any answer):
- (a) `aw22-a.xml` and `aw22-c.xml` hold no voucher; `aw22-marks-2.xml` is byte-identical to `aw22-marks-0.xml`.
- (b) `aw22-b.xml` holds exactly the three vouchers `BP/26-27/0050` to `0052`, each with the narration of its row above after XML decoding. Whether the repeated spaces and the trailing space of `0051` are kept, collapsed or trimmed is recorded, not predicted; so is whether the `&`, `<`, `>` and quotes of `0050` read back unchanged.
- (c) The three vouchers carry the amounts of `BP/26-27/0001` (1,050.00 and the two tax lines); the marks of `aw22-marks-1.xml` differ from the first by the three vouchers.

The opposite of any of these is a finding, not an error. Not covered: a post made by ComplyEaze Bridge, a narration of another length, a multi-line narration, and any voucher type but a Sales invoice.
