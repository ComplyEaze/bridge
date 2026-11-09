# Row 2377-NB: for a bill-wise party with no open bills, Tally's Ledger Outstandings report answers no bytes and no Content-Type, twice, and the snapshot's closing is empty; OUTCOME: response_content_type_unsupported

Run on 9 Nov 2026, 22:53:18Z to 22:58:28Z (04:23 to 04:28 IST on 10 Nov), on TallyPrime Silver (the Gateway on 9001; `tally.exe` file version 1.1.7.2), by the sheet at `lab/2377-nb-sheet` (03d7763a), on `BRIDGE PILOT LAB` (GUID `6b43e498-430c-4d5c-bfef-d32e2ab93c85`, synthetic, in master's registry), loaded alone in a sitting of its own. **Reads only:** eight raw sends, one at a time through `run.py`, with bytes rendered by master's builders; no write, no backup, no restore, and no Bridge build. Every value below is M, measured by this run, in one run: **partial** (one book, one party, one release). Tally's current period was 1-Apr-26 to 31-Mar-27 and its current date Tuesday 15-Sep-2026 throughout; neither was changed. Every send got an HTTP 200 reply, each well under the 5-minute wedge rule; two of them, by the sheet's one exception, carried no bytes.

This sitting ran after row 683-OP (Tally quit at 22:44:48Z), on the run lead's word, with a Tally start of its own. Tally was started once from its desktop shortcut and closed by its own Quit at the end.

**Result.**
- **m0:** one COMPANY row, the book's, ALTVCHID 68, ALTMSTID 245. EXPECT met.
- **The control, at 20260804:**
  - `s1`: 18 LEDGER rows. `BRIDGE Supplier RJ`: PARENT `Sundry Creditors`, CURRENCYNAME the rupee sign, ISBILLWISEON `Yes`, OPENINGBALANCE `0.00`, **CLOSINGBALANCE `2100.00`**, BRIDGECOMPANYGUID the book's.
  - `l1a` and `l1b`: each 1,232 B, HTTP 200, `CONTENT-TYPE: text/xml; charset=utf-16`, `RESPSTATUS: 1`. **ANSWER: ROWS 1 (0 On Account):** `SUP/0001`, BILLDATE `4-Aug-26`, BILLOP `1050.00`, BILLCL `2100.00`, BILLDUE `4-Aug-26`, BILLOVERDUE `0`, with the two Purchase vouchers (numbers 1 and 2, 1050.00 each, 4-Aug-26) as its BILLVCH* scalars. The whole window right after `l1a` showed the Gateway, no dialog.
  - `l1a` and `l1b` are byte-equal (sha256 `024a6630ad018b9d0e16b472ee71b2ab05e8f3b59c747906d6e9e8df2f4fb657`).
  - **`pb.py --outcome`: `OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL 2100.00 equals closing 2100.00)`.** For this payable party, the bill's BILLCL and the snapshot's closing carry the same sign.
- **The case, at 20260731:**
  - `s2`: 18 LEDGER rows. `BRIDGE Supplier RJ`: as in `s1`, except **CLOSINGBALANCE empty** (`pb.py`: "EMPTY (master: None, not zero)").
  - `l2a`: **0 bytes.** `run.py` printed "empty answer on L-lo-rj-20260731.xml: the request failed; stop, do not resend". The request was recorded first as failed. Its headers file holds, whole: `HTTP/1.1 200 OK`, `RESPSTATUS: 1`, `CONNECTION: KEEP-ALIVE`, `CONTENT-LENGTH: 0`, **with no Content-Type**. That is HTTP 200 with CONTENT-LENGTH 0, so the sheet's one exception applied: the whole window was captured at once (`after-l2a`: the Gateway, no dialog); `GET /status` was sent once (200, 51 B); and `pb.py` printed **ANSWER: NO BYTES**.
  - `l2b`: the same, by the same exception: 0 bytes, the same four header lines (its headers file is byte-equal to `l2a`'s), `after-l2b` the Gateway with no dialog, `GET /status` 200 (51 B), **ANSWER: NO BYTES**.
  - `l2a` and `l2b` are byte-equal (both empty).
  - **`pb.py --outcome`: `OUTCOME: response_content_type_unsupported (0 Content-Type headers)`.**
- **m1:** SAME MARKS (68/245), and m1 is byte-equal to m0 (both sha256 `8a0c42682c958e4850d6893438d41ffeb62f6c26eb2e7eb29c46c6f2d51076e6`). Nothing moved.

**By the sheet's Decides:**
- **No bytes, twice** (both ANSWER lines NO BYTES, and OUTCOME `response_content_type_unsupported`): measured. On this book and release, the report answers a bill-wise party with no bills as it answers an unknown name: HTTP 200, `CONTENT-LENGTH: 0`, no Content-Type, no bytes. The no-rows refusal stays. The next step, as the sheet says, is that only the snapshot can tell that the party was found and is bill-wise. Partial: one book, one release.
- **An envelope with no row** (closing `0.00` or empty): not seen. No envelope came.
- **No bytes once and an envelope once, or the pair not byte-equal:** not seen.
- **Rows in the case:** not seen.
- **Any other OUTCOME in the case:** not seen.
- **The control not told:** not so. The control was told.

**The sheet's estimates:**
- **The control:** the closing 2100.00 and one row `SUP/0001` with BILLCL 2100.00: as estimated. The bill is dated `4-Aug-26`, one of the two dates the estimate named. OUTCOME told.
- **The case:** the snapshot's `CLOSINGBALANCE` is empty, as for `Cash` on BILLTRAIL and `BRIDGE Output CGST XML` on PILOT. The answer was refused before the tie, by its headers, so `party_ledger_closing_not_read` was not reached.

## Step 0: start, check and load
- Sheet files: all 11 equal to `SHA256.txt` at 03d7763a. `LAB-COMMON.md` and `.gitattributes` are byte-equal to row 683-OP's (683-LM's v3). `M-marks.xml` is byte-equal to row 2162-RP's. `run.py` differs from row 683-OP's only by its one `-D` change and the comment line about it. The book's GUID is in origin/master's synthetic-uuid registry. One data folder's `Company.1800` names the book: it holds the book's GUID and not the excluded GUID.
- Tally was started once, at 22:53:18Z, from its desktop shortcut (`TallyPrime.lnk`, `C:\Program Files\TallyPrime\tally.exe`, 1.1.7.2), after tasklist showed none. Its start-up company loaded by itself, as the install is set to, and was shut first by C9 (the K: Company label, Shut, Enter on it, no prompt), as at every Tally start of this lab. Three Bridge processes were running; none was ended and no Bridge tool was called (a standing rule of the lab).
- C2 step 0 at 22:54:27Z, with no company loaded: the window capture, one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B).
- At Select Company the book's name was typed. The field showed `BRIDGEPILOT`: the space did not register. The list held one row, `BRIDGE PILOT LAB`, highlighted, 1-Apr-26 to 31-Mar-27; it was captured (`s0-select-pilot`), then Enter, no prompt. The Gateway showed (`s0-gateway`):
  - BRIDGE PILOT LAB alone, TallyPrime with SILVER on its title (licensed: no Educational mode anywhere);
  - current period 1-Apr-26 to 31-Mar-27;
  - current date Tuesday 15-Sep-2026;
  - date of last entry 15-Sep-26;
  - "(e)  (Schedule Backup)" on the company row, and the footnote "(e) Data exceptions exist", as on this book in earlier rows.
- The release on screen: TallyPrime Silver, with no release number; `tally.exe` file version 1.1.7.2. The PC's date was 2026-11-10 (IST).

## Steps 1 to 4: the sends
Each send's start time is in `evidence/sends-utc.txt` (UTC) and `log-<label>.txt` (local time). Each request's hash was checked against `SHA256.txt` by `send.sh` just before it was sent. For `l1a`, `l2a` and `l2b`, `send.sh` took the whole-window capture before it wrote the "answered" or "run.py exit" line, so those three times include the capture (about 3 s).

| Step | Label | Request | Sent (UTC) | Line after (UTC) | Answer bytes | HTTP, Content-Type |
|---|---|---|---|---|--:|---|
| 1 | m0 | `M-marks.xml` | 22:55:35.653 | 22:55:36.044 | 3,394 | 200, text/xml; charset=utf-16 |
| 2 | s1 | `S-snap-20260401-20260804.xml` | 22:55:42.458 | 22:55:43.067 | 25,502 | 200, text/xml; charset=utf-16 |
| 2 | l1a | `L-lo-rj-20260804.xml` | 22:55:50.704 | 22:55:53.650 (after the capture) | 1,232 | 200, text/xml; charset=utf-16 |
| 2 | l1b | `L-lo-rj-20260804.xml` | 22:56:07.351 | 22:56:07.753 | 1,232 | 200, text/xml; charset=utf-16 |
| 3 | s2 | `S-snap-20260401-20260731.xml` | 22:56:16.520 | 22:56:16.952 | 25,298 | 200, text/xml; charset=utf-16 |
| 3 | l2a | `L-lo-rj-20260731.xml` | 22:56:23.287 | 22:56:26.238 (after the capture) | 0 | 200, none |
| 3 | l2b | `L-lo-rj-20260731.xml` | 22:56:57.040 | 22:57:00.868 (after the capture) | 0 | 200, none |
| 4 | m1 | `M-marks.xml` | 22:57:25.010 | 22:57:25.593 | 3,394 | 200, text/xml; charset=utf-16 |

Every send's curl exit was 0. Each answer's headers file is pushed beside it in `answers/`.

## Step 5: close (C10)
- At 22:57:33Z (C11, after the last request): one `tally.exe`, which alone owns 9001, and `GET /status` 200 (51 B).
- Shut by K: Company (clicked), h, BRIDGE PILOT LAB alone, Enter, no prompt. Select Company with no company loaded. At 22:58:03Z: one `tally.exe` owning 9001, and `GET /status` 200 (51 B). No XML request was sent after the shut.
- Tally was then closed by its own Quit ("Q: Quit" clicked at Select Company). No prompt appeared, and no `tally.exe` ran after.

## Deviations and notes
- **The empty answers.** `run.py` stopped on each empty `L-` answer, as it is written to. By the sheet's one exception, both were recorded first as failed requests and then classified, never as 0 rows. Each exception step was taken in order, and each `GET /status` gave 200.
- **No dialog came at any point.** No master, voucher or report screen was opened; only the Gateway and Select Company were used. The K: Company label was clicked for each shut, and Tally's own title bar (its empty part) was clicked before each key so that the key reached Tally.
- **`send.sh` runs `pb.py` for the marks and snapshot answers only.** `pb.py` was run by hand for each report answer, for `--outcome` and for `--same-row`, as the steps say; each output is in `evidence/`, whole.
- **The Gateway's right-hand panel** showed a red line "N: Manage Rental License"; it was not pressed.
- **The staging checks.** The start-up check found no hit in the answers, their headers files, the logs, the evidence or this file. The 10+ digit check sets aside this book's registered synthetic GUID, matched whole, and nothing else, as the sheet says.
- **The notes file:** the lines for m0, s1, the control's pair, s2, the case's pair and m1 were written after the close from `sends-utc.txt` and the evidence files; that block says so.

## Not measured
Every item the sheet lists as not in this row. Also not measured:
- Whether a party whose bills are settled (closing a written zero) gets an envelope with no row or no bytes.
- Whether a receivable party with no bills, or another release, answers the same.
- Why Tally sends no Content-Type with an empty answer.
- Master's own parsers and transport on these answers; by the sheet, that run is the one that counts when they become fixtures.

## Files
- `answers/`: the answers and their headers files, byte for byte, except any that the standing rules keep on the lab PC; those are listed by size and sha256 in `kept-local.txt`, with the reason. The two empty answers are pushed as the 0-byte files curl wrote.
- `logs/`: `run.py`'s own logs.
- `evidence/`: the notes (UTC), `pb.py`'s outputs for every answer, both outcomes and `--same-row`, the pair hashes and the send times. The process and port listings stay local.
- `scripts/`: the step 0 helper, the window capture helper and the send helper.
- `SCREENSHOTS.txt`: the captures by name, size and sha256; they stay on the lab PC.
- `substitutions.txt`: what changed in the pushed copies of the lab's own text.

## pb.py outputs and the pair hashes, whole

### m0 (pb.py)
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (the book's: yes) ALTVCHID 68 ALTMSTID 245
exit 0
```

### s1 (pb.py)
```
a-s1-S-snap-20260401-20260804.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Supplier RJ': PARENT 'Sundry Creditors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '2100.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 2100.00
exit 0
```

### l1a (pb.py)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1232']
  BILLREF 'SUP/0001' BILLDATE '4-Aug-26' BILLCL '2100.00' BILLDUE '4-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 2100.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### l1b (pb.py)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1232']
  BILLREF 'SUP/0001' BILLDATE '4-Aug-26' BILLCL '2100.00' BILLDUE '4-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 2100.00
ANSWER: ROWS 1 (0 On Account)
exit 0
```

### l1a and l1b (sha256sum, cmp)
```
024a6630ad018b9d0e16b472ee71b2ab05e8f3b59c747906d6e9e8df2f4fb657 *a-l1a-L-lo-rj-20260804.xml
024a6630ad018b9d0e16b472ee71b2ab05e8f3b59c747906d6e9e8df2f4fb657 *a-l1b-L-lo-rj-20260804.xml
l1a and l1b byte-equal
```

### the control (pb.py --outcome)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1232']
  headers: 'HTTP/1.1 200 OK'; Content-Type ['text/xml; charset=utf-16']; CONTENT-LENGTH ['1232']
  BILLREF 'SUP/0001' BILLDATE '4-Aug-26' BILLCL '2100.00' BILLDUE '4-Aug-26' BILLOVERDUE '0'
  sum of BILLCL: 2100.00
ANSWER: ROWS 1 (0 On Account)
a-s1-S-snap-20260401-20260804.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Supplier RJ': PARENT 'Sundry Creditors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE '2100.00'; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads 2100.00
OUTCOME: told (ROWS 1 (0 On Account), sum of BILLCL 2100.00 equals closing 2100.00)
exit 0
```

### s2 (pb.py)
```
a-s2-S-snap-20260401-20260731.xml: HEADER/STATUS 1
  LEDGER rows: 18
  'BRIDGE Supplier RJ': PARENT 'Sundry Creditors'; CURRENCYNAME '₹'; ISBILLWISEON 'Yes'; OPENINGBALANCE '0.00'; CLOSINGBALANCE ''; BRIDGECOMPANYGUID '6b43e498-430c-4d5c-bfef-d32e2ab93c85'
    CLOSINGBALANCE reads EMPTY (master: None, not zero)
exit 0
```

### l2a (pb.py)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-l2a-L-lo-rj-20260731.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### l2b (pb.py)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
a-l2b-L-lo-rj-20260731.xml: 0 bytes
ANSWER: NO BYTES
exit 0
```

### l2a and l2b, with their headers files (sha256sum, cmp)
```
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 *a-l2a-L-lo-rj-20260731.xml
e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 *a-l2b-L-lo-rj-20260731.xml
547c40c5c5f4e011efd4f95175768b2ae81f0dfd351f7e36d0b3b9821a584e0b *a-l2a-L-lo-rj-20260731.xml.headers
547c40c5c5f4e011efd4f95175768b2ae81f0dfd351f7e36d0b3b9821a584e0b *a-l2b-L-lo-rj-20260731.xml.headers
l2a and l2b byte-equal (both 0 bytes)
```

### the case (pb.py --outcome)
```
  headers: 'HTTP/1.1 200 OK'; Content-Type absent; CONTENT-LENGTH ['0']
OUTCOME: response_content_type_unsupported (0 Content-Type headers)
exit 0
```

### m1 against m0 (pb.py --same-row)
```
a-m0-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (the book's: yes) ALTVCHID 68 ALTMSTID 245
a-m1-M-marks.xml: HEADER/STATUS 1
  COMPANY rows: 1
  GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85 (the book's: yes) ALTVCHID 68 ALTMSTID 245
SAME MARKS
exit 0
```
