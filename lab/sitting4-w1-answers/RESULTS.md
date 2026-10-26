# Results: W1 (#917), the longest ledger, group and company name TallyPrime 7.2 accepts, 26 Oct 2026. PASS: all three stop at 150 characters, counted as characters, cut silently at input

The sheet is `lab/sitting4-run-sheets` at a14799fc3ac98fdeaaba1e718e14415f7458aae5 (`SHA256.txt` 6/6 OK). Its README's no-XML-by-curl paragraph applied, so step 0 ran without the MARKS request and every mark comes from the tools. TallyPrime 7.2, Silver, Licensed, not Educational mode.

The build is the staged `w1-2306d86d`: `bridge_mcp.exe` sha256 **92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524**, checked before every call. It ran over stdio with `BRIDGE_TALLY_HOST=127.0.0.1`, port 9001, a new data folder kept for this row, `BRIDGE_TERMS_ACCEPTED=true`, redaction unset, and posting and import both unset.

The owner authorised the lab, in the lab's own chat, to key W1's screen steps (new test companies, masters), one key per action with a screenshot after each. The closing shut was keyed by the owner at the PC (see C10).

## THE COMPANY
`BRIDGE NAMELEN LAB`, company number 100082, GUID **eb8d828d-9d4e-4169-b659-dbea27170730**, books from 1-Apr-2025, no voucher. Select Company showed no company of that name first. Features were accepted as offered.

BK1 was taken before the first keying, with the safe sequence: a new empty folder, Specify Path, the one company, End of List, Enter, Accept? Yes. It holds one file: `TDBK1800_<company number>.001`, 23,386 bytes, sha256 **f06eb8b20ea5e0710f79430f35549a12c9b15bdabb0c53a0c4d8f68b872d5491**. It is kept on the lab PC.

Marks are taken from `ledger_masters`' first-page snapshot, with `reused` false every time. **M0 207 / V0 null**; **M1 222 / V1 null** (+15 = 9 ledgers and 6 groups).

## Names, as keyed and as returned
Every name was written in `notes.txt` (length, UTF-8 bytes, sha256) before it was keyed. Each was typed in pieces of at most 14 characters, and each was accepted on its screen with Ctrl+A. No message appeared at any point.

| Kind | Typed | Screen | Returned by the tool (chars / UTF-8 bytes) |
|---|---|---|---|
| Ledger (Sundry Debtors) | 64 × A | accepted as typed | 64 / 64 |
| Ledger | 97 × A + BB | accepted as typed (ending BB shown) | 99 / 99, ends BB |
| Ledger | 98 × A + CC | accepted as typed | 100 / 100, ends CC |
| Ledger | 126 × A + DD | accepted as typed | 128 / 128, ends DD |
| Ledger | 127 × A + EE | accepted as typed | 129 / 129, ends EE |
| Ledger | 158 × A + FF (160) | **cut at input**: the field stopped taking characters and FF never appeared | **150 / 150**, all A = the first 150 characters typed |
| Ledger | é (U+00E9) × 150 | accepted | **150 / 300**, all é |
| Ledger | é × 76 | accepted | 76 / 152 |
| Group (Primary) | 64 × G | accepted as typed | 64 / 64 |
| Group | 97 × G + BB | accepted as typed | 99 / 99, ends BB |
| Group | 98 × G + CC | accepted as typed | 100 / 100, ends CC |
| Group | 126 × G + DD | accepted as typed | 128 / 128, ends DD |
| Group | 127 × G + EE | accepted as typed | 129 / 129, ends EE |
| Group | 158 × G + FF (160) | **cut at input** (FF never appeared) | **150 / 150**, all G |
| Company | "BRIDGE NAMELEN COMPANY TEST " + 132 × C (160) | cut point not visible on screen (only C's) | **150 / 150** = the prefix + 122 × C |
| Ledger (under group G64) | ' × 150 | accepted | 150 / 150, all apostrophes |

**Limits observed on TallyPrime 7.2 Silver:**
- **Ledger names: 150 characters.**
- **Group names: 150 characters.**
- **Company names: 150 characters.**

For all three, the limit counts **characters, not bytes**: 150 × é is stored whole at 300 UTF-8 bytes. At the limit Tally **cuts silently at input**; it does not refuse and shows no message. Each cut name is the first 150 characters of what was typed, and the read returns it exactly as the screen accepted it.

The series stopped at the first cut, as the sheet says, so 255 was not tried. Step 3 used L = 150, the observed limit, rather than 129, the longest typed name accepted whole, so the byte test sits at the boundary.

## Step 6: the apostrophe name
A ledger named with 150 apostrophes was created under the 64-character group G64. Read with `ledger_masters` group=G64, limit 1:
- total 1; the item has 150 chars / 150 bytes, all apostrophes; its parent is G64.
- `evidence.bytes` 199,230.

`egress_log` for that call:
- `response_prepared` for `ledger_masters`: **bytes_prepared 2,773**, enforced_bytes 200,000, truncated false.
- `stdio_write_completed`: bytes_written 2,773.

## Step 5: the company name
THE COMPANY was shut first, and BRIDGE NAMELEN COMPANY TEST was created alone. Its name was typed to 160 characters. It is company number **100083**, GUID **1d9056df-032e-496e-8df6-2d6698158b91**, books from 1-Apr-2025, and its stored name is 150 characters. It was shut, and THE COMPANY was loaded alone again; `list_companies` then showed only 100082.

The test company is **left in place** in the data folder, as the run's review asked.

## Deviations (agreed by the run's review)
- **Order:** step 6 was run before step 5, so THE COMPANY was not reloaded twice.
- **Placement:** the apostrophe ledger sits under group G64, so that `ledger_masters` (which has no name filter) returns it alone with group=G64, limit 1. G64 was chosen by typing the filter "GGG": Enter filled the common 64-G prefix, and a second Enter took the exact G64.
- **Choices on screen:** ledgers were created with bill-by-bill Yes as offered; groups with Nature Liabilities as offered; the apostrophe ledger with "Include in Assessable Value calculation: Not Applicable" as offered.
- **AFTER:** no restore was made (the run's review: the owner's grant covered new companies and masters, not a restore). BK1 is kept.

## Finding repeated from W4
Every group returned by `masters` kind=groups has `parent` as U+FFFD followed by "#4; Primary": Tally's own "&#4; Primary" decoded with a replacement character. So does Profit & Loss A/c in `ledger_masters`. Recorded as a repeat of W4's finding.

## C10
After step 7, the desktop-control tool could not get keyboard focus. A recurring Windows "USB device not recognized" notification, and then an empty notification window, held the foreground although the screen showed Tally. So **the owner keyed the shut at the PC**: Company > Shut (Alt+K, H, Enter) on BRIDGE NAMELEN LAB.

Then:
- a screenshot: Select Company, no company loaded, no dialog;
- tasklist: one tally.exe, PID 29692 (exit 0);
- netstat: 9001 LISTENING with that PID, IPv4 and IPv6;
- GET /status: 200, 51 bytes.

No XML request was sent after the shut. End marks (step 7): **M 222 / V null**.

## In this commit
- `notes.txt`: UTC times, one line per step; every name's plan before keying.
- `answers/`: every tool answer with its arguments, and the C2/C10 observations with the /status answers.
- `SCREENSHOTS.txt`: name and sha256 only.
- `SHA256.txt`.

In `notes.txt`, local folder paths are replaced by placeholders (`<item folder>`, `<lab folder>`) and the demo company's name by "the demo company". The raw file's sha256 on the lab PC is 373184d0c8275470352fc6b10470124758124c195f894561fc10488a2f02e499. Every other file is as saved, except that text files are stored with LF line ends.

Not committed: the screenshots and BK1.
