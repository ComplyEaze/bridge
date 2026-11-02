# Lab run sheet: the ledger catalogue and three kinds of transfer day, for the first real `not_found` (#1392). SYNTHETIC COMPANY ONLY. Not for merge.

About 40 minutes. Keying on Tally's screen, then reads. ComplyEaze Bridge writes nothing; the keyed vouchers are the set-up.

**WHAT IT DECIDES:** pull request #2164 holds its wiring until a lab run shows what its comment of 30 Oct 2026 asks for. A skipped bank-statement row may read `not_found` only from a whole day holding no voucher on both ledgers, and that match compares ledger names from the catalogue with names on voucher rows. This sheet captures:

1. the book's ledger catalogue as `read_import_ledger_catalogue` sends it, so the catalogue's spelling can be compared with the voucher rows' spelling;
2. a day with a voucher on `TLab Bank A` and a third ledger and none on both banks (the first `not_found` on a real book's ledgers);
3. an A-B Contra on the day after a skip's day, so a read of neighbouring days has something to find;
4. a transfer through a clearing ledger: A to the clearing ledger on one day, the clearing ledger to B the next.

**WHY THE KEYING IS BY HAND:** the five Contras on this book were keyed, and the question is how keyed rows read. Nothing here is seeded.

**THE CODE THAT ADMITS THIS:** a day counts as whole only if it holds at least one voucher (`NotWhole::Empty`, `agent_transfer_pairing.rs`, `WholeDay::from_read`, and #985), so each case's skip day below holds one. `read_day` reads a day through `read_entry_window_rows_with` with `from = to = the day`. `census_spans` (`agent_voucher_window.rs`) sends one census with no AlterID filter while the mark fits a single census read, so these files carry none, and one voucher read per day. The catalogue file is `ReadOnlyProfile::StandardLedgerCatalogV2`, the request `read_import_ledger_catalogue` sends (`agent_import.rs`).

**REQUEST FILES** (UTF-16LE with a byte-order mark, no final line break; check every file in this folder against `SHA256.txt` with `Get-FileHash -Algorithm SHA256` before the first send, and STOP on any mismatch):

| file | what it is |
|---|---|
| `m.utf16le.xml` | company marks, BRIDGE TRANSFER LAB; unchanged from `lab/1392-whole-day-read` |
| `cat.utf16le.xml` | the ledger catalogue; rendered by the builder at master 9821af8e |
| `dDD-c.utf16le.xml` | census of that one day, April 2026 |
| `dDD-v.utf16le.xml` | that day's vouchers, every ledger entry |

The day files are `d03-c1` and `d03-v` of `lab/1392-whole-day-read` with the date changed and nothing else. The same change reproduces that sheet's 4, 5 and 6 April files byte for byte, and the fetch list has not changed on master since.

Days `DD`: `12`, `13`, `14`, `19`, `20`, `21`, `22`, `26`, `27`, `28`, `29`. The days either side of each case are read as well, because the plan on #2164 reads neighbouring days; those days are expected to be empty.

**RULES:**
- Company BRIDGE TRANSFER LAB only (number 100051, the book of `lab/1392-whole-day-read`). Load only it. Port 9001 only. Never touch a client company. If it is not on the lab PC, STOP and report; do not recreate it.
- Before the first send: ComplyEaze Bridge's `tally_status` shows a licensed release (not Education) and exactly that company loaded. Record the release, the edition and the build's `bridge_mcp.exe` sha256. Redaction stays off.
- One request at a time; never abandon one; no `--max-time`. STOP on any dialog or message box you did not expect (press no key, send nothing more), a second company loaded, Education mode, or 10 minutes without an answer.
- Send with exactly: `curl.exe -sS --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download} %{time_total}\n" http://localhost:9001/` and record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP.
- Evidence goes in `C:\bridge-lab\1392-days\evidence\`; zip it and report its SHA256. Do not trim or re-save any response.
- Do not reconcile, alter, delete or re-save any voucher, on this sheet's days or any other. A voucher saved wrongly is not corrected: STOP and report. Vouchers 3 to 5 April stay as they are.

**STEPS**
1. Load the book alone; `tally_status` as above.
2. **Before keying.** Send `m.utf16le.xml` (name `m1`), then `cat.utf16le.xml` (name `cat1`). Report `ALTVCHID` and `ALTMSTID` exactly as sent. An earlier sitting on this book moved `ALTVCHID` from 5 to 7, so it is not expected to be 5. **If the catalogue has no row named exactly `Cash`, `TLab Bank A` or `TLab Bank B`, or any row already named `TLab Clearing`, STOP.**
3. **Keying (Tally's screen, double entry, as this book's Contras were keyed: `To` the credited ledger, `By` the debited one).** One new ledger: `TLab Clearing` under `Suspense A/c`, no opening balance, Tally's defaults. Then five vouchers, in this order, each with the date set with F2, no narration and no reference. On a bank-allocation screen accept what Tally offers and type nothing. A screenshot of each saved voucher.

| # | Date | Type | Dr (`By`) | Cr (`To`) | Amount | Case |
|---|---|---|---|---|---|---|
| T1 | 13 Apr 2026 | Contra | Cash | TLab Bank A | 7,000.00 | 2 |
| T2 | 20 Apr 2026 | Contra | Cash | TLab Bank A | 4,000.00 | 3, makes the skip's day whole |
| T3 | 21 Apr 2026 | Contra | TLab Bank B | TLab Bank A | 25,000.00 | 3, the day after |
| T4 | 27 Apr 2026 | Payment | TLab Clearing | TLab Bank A | 30,000.00 | 4, first leg |
| T5 | 28 Apr 2026 | Receipt | TLab Bank B | TLab Clearing | 30,000.00 | 4, second leg |

If a screen differs from this (a field missing, a type not offered), STOP, screenshot it, and report; do not improvise.

4. **Marks after keying.** Send `m.utf16le.xml` (name `m2`). **EXPECT (an expectation, not a fact):** `ALTVCHID` is `m1`'s plus 5, and `ALTMSTID` has moved. If the voucher mark is above 8,000, STOP: these census files are for one unfiltered census read.
5. **Each day, in the order above:** send `dDD-c` (name `dDD-c`), then `dDD-v` (name `dDD-v`).
6. **After.** Send `m.utf16le.xml` (name `m3`), then `cat.utf16le.xml` (name `cat2`). If `m3` differs from `m2`, the book changed during the reads: report it; the answers are void.
7. **Count by structure, not by text.** Decode each response as UTF-16LE and parse it as XML. For each census and vouchers response count the nodes named exactly `VOUCHER`, and list each one's `GUID`, `ALTERID`, `DATE`, `VOUCHERTYPENAME` and `VOUCHERNUMBER`. A substring count of `<VOUCHER` also matches other elements. For each vouchers response list, per `ALLLEDGERENTRIES.LIST`, `LEDGERNAME`, `AMOUNT` and `ISDEEMEDPOSITIVE`. A day that answers with no `VOUCHER` is reported as an answer with a count of 0; a failed send is reported as an error, never as 0.
8. **The spelling.** From `cat2`, list every ledger row's name as received. For each `LEDGERNAME` in step 7, say whether it equals a catalogue name exactly, comparing the decoded text by code point, with no trimming and no case folding. Report any that do not, with both spellings shown as code points.

**PASS / FAIL**
- PASS: all 27 answers received in full (HTTP 200, complete XML), `m2` equals `m3`, and steps 7 and 8 reported. The sheet passes whatever the answers hold: a name that does not match, or a day whose census and vouchers disagree, is also an answer.
- FAIL: a transport failure, an unexpected dialog, a changed mark between `m2` and `m3`, or a count made by substring.

**REPORT, one message:** the release, edition and build; the marks of steps 2, 4 and 6; per response its size, SHA256, `VOUCHER` count and the listed fields; the result of step 8; any STOP; the evidence zip SHA256. The responses, headers and log, unedited, with the screenshots, go on a NEW branch `lab/1392-transfer-days-answers`, never on this sheet's branch.
A command that errors is reported as an error, never as "absent" or "unchanged".
