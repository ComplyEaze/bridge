# 2433 name: the longest company name Tally stores, and whether it keeps and matches edge spaces, read raw with no Bridge build

Not for merge. Part of #2397 (row T1) and #2433 (row T2, its part (b)). Written 2026-11-09. About 60 minutes. No owner act: no Bridge build, no Terms, no installed extension, no tool call. Raw reads only, sent by `send.py` with LAB-COMMON C3's own curl line, and two renames on Tally's own Company Alteration screen, each undone by a restore of one backup.

`LAB-COMMON.md` in this folder applies in full (v3, sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6`, 12,062 bytes). Where it and this sheet differ, the stricter holds. `SHA256.txt` lists every file here with its bytes; check it before step 0.

Marks: V = read at a named source; U = not measured. **Every EXPECT in this sheet is U, written "record as found":** nothing here has been read live. Every outcome is recorded as found, never explained away.

## What it decides

One synthetic book, loaded alone, the same book at the start of each row (a restore after each, checked by the COMPANY row's GUID and marks). One variable per row: the company's name.

- **T1 (#2397): Tally's stored name limit.** When the 300-character letters-only pattern is typed as the company's name, how many characters and bytes does Tally store, and in which block does the stored name end? Bridge's parser and #2397's identity check both refuse a name over 255 bytes; whether Tally can store one is not known (bridge#917 recorded only that 150 characters are stored whole: `docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md:880-884`).
- **T2 (#2433 part (b), marked unverified there): edge spaces.** When the name is typed with two spaces before and two after, does Tally store the spaces? And does a company-scoped ledger list whose `SVCURRENTCOMPANY` holds the trimmed name, which is what Bridge sends (#2433 (b): the company-list parser holds each name trimmed), answer with the book's ledgers, and what does the same read with the spaced name answer? A ledger list is used, not the marks read: a `Company` collection answers one row per loaded company whatever `SVCURRENTCOMPANY` names (bridge#938, `docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md:866`), so its row cannot show a match. A ledger list's rows can: each ledger's `GUID` begins with its company's GUID, and the request computes the company's GUID into every row (`BRIDGECOMPANYGUID`).

**What is already known, and not re-measured here** (`docs/tally/TALLY_PROTOCOL_REFERENCE_COMPANY_IDENTITY_AND_CREATION.md`, §9.11, lines 45-75):
- **Finding 2, a name Tally does not match:** a read answers HTTP 200, `STATUS` 1 and zero rows, with no error. §9.11's table is marked VERIFIED. It was read again on licensed TallyPrime 7.1 Silver on 2026-10-13 (one run, Confidence: PARTIAL; branch `lab/m1-create-lands-where-answers`, `7c2ff6a1`) with a `List of Ledgers` read: a name carried by no loaded company answered HTTP 200, `STATUS` 1, a `COLLECTION` with no rows and every `CMPINFO` count 0, and no error line.
- **Finding 3, how names match:** matching is case-insensitive and whitespace-trimmed. Its confidence marker is §9.11's **VERIFIED**, with no date or run of its own. It therefore rests on the reference's primary baseline (`docs/tally/TALLY_PROTOCOL_REFERENCE_ENVIRONMENT_AND_REQUESTS.md`, §0): 2026-07-29, TallyPrime Edit Log 7.0 in **Educational** mode, gateway port 9000, one synthetic company, raw captures kept outside the tree. It entered the tree in #106 (`7bd6123a`). The 2026-10-13 re-read did not test it.
- **The direction finding 3 measured:** the extra space was **in the request**. A `Ledger` read whose `SVCURRENTCOMPANY` carried a trailing space ("Trailing space": the table does not say how many) answered the same 86 rows as the correct name, for a company stored **without** one. Leading spaces were not in the table, and neither was a stored name with spaces.
- **What T2 adds:** a **stored name with spaces**, if Tally keeps them, read by a request naming it **without** them (Bridge's trimmed name) and **with** them (byte-exact), on licensed Silver, with each row's GUID checked against THE COMPANY's. If Tally trims on entry, the stored name has no spaces. The spaced read is then finding 3's own direction, with two leading and two trailing spaces, on this release.

**What T1 covers of `lab/2397-b1-name-sheet`, row N3.** T1 makes N3's Tally half without Bridge: N3's steps 0 and 1 (load, MARKS, backup), step 3 (the rename to the same pattern), step 4 (the native read of the stored name: characters, bytes, block, attribute and element), and steps 6 to 8 (restore, the GUID-and-marks check, shut). T1 does not cover N3's step 2 and step 5: the two Bridge builds' answers before and after the rename. If T1 runs first, N3 can keep only its build launches around its own rename, and record at its step 4 only that the stored name equals T1's (same sha256), in place of measuring it again.

## THE COMPANY and the book

`BRIDGE SHAPE LAB` (synthetic, seeded by this project: V `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/PROVENANCE.md`, Provenance, Book), GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, books from 2025-04-01 (V: that folder's `shape_lab_company_list_response.utf16le.xml`). Loaded alone whenever anything is sent or keyed (C1). Licensed mode only; Education mode = STOP. Write down the Tally release shown on screen.

- **One backup, BK,** taken at step 0 before the first change (C5), to the new empty folder `C:\bridge-lab\2433\bk\`. Both restores are from BK.
- **The book at the start of each row** is the book BK holds. Each restore is checked by the marks read: the COMPANY row's GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346`, and ALTVCHID and ALTMSTID equal to step 0's M1 (C6, and the unchanged-marks rule earlier sheets used: the GUID, then both marks).
- **Its company number** is the one step 0's native read shows (`COMPANYNUMBER`); C5 and C6 use it.

## Files

Each is checked against `SHA256.txt` before step 0; another hash = STOP. Work in `C:\bridge-lab\2433\`, a new folder: copy every file there. Answers land in `C:\bridge-lab\2433\answers\`.

| file | source |
|---|---|
| `RUN-SHEET.md` | this sheet |
| `LAB-COMMON.md` | byte copy of `lab/2397-b1-name-sheet:lab/2397-b1-name/LAB-COMMON.md` (itself a byte copy of `lab/2246-split-journal`'s v3), sha256 `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6` |
| `.gitattributes` | byte copy of the B1 sheet's, sha256 `d13bfb7ad438ed990b12090ee49e38d6d751345e951ff6048bc7ae5136a74149` |
| `measure.py` | byte copy of the B1 sheet's, sha256 `86087ec7f20fda8965fc0ee68ff5181baff632148b1f128be789ff3e21ec8604`. Used here for `pattern`, `native` and `marks` only; its `tool` and `diff` read Bridge answers, which this sheet has none of |
| `C-company-list.utf16le.xml` | byte copy of the B1 sheet's, sha256 `9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8`, 2,524 bytes: Bridge's own company-list request, captured on BRIDGE SHAPE LAB (`note-days/shape_lab_company_list_request.utf16le.xml`). It names no company: the Company collection with `NAME`, `GUID`, `COMPANYNUMBER`, `BOOKSFROM` and four computed licence facts |
| `M-marks-shape-lab.utf16le.xml` | byte copy of the B1 sheet's, sha256 `f556551d09682fd94d41b40bcf539d5d34bf06260d6da4bb44ad7023c598fcb1`, 976 bytes (`note-days/shape_lab_company_marks_request.utf16le.xml`): `SVCURRENTCOMPANY` `BRIDGE SHAPE LAB`, `FETCH` `GUID,ALTVCHID,ALTMSTID`. Used only for M0, M1 and the restore checks |
| `L-ledgers-shape-lab.utf16le.xml` | byte copy of `src-tauri/crates/bridge-tally-protocol/tests/fixtures/note-days/shape_lab_ledgers_1_request.utf16le.xml`, sha256 `61c9b17f265e3f1e9dc9d6997488dc41e65ae49a2850233e16df523abd041373`, 1,736 bytes. That file's `PROVENANCE.md` lists it as wire bytes captured from Bridge on BRIDGE SHAPE LAB (licensed TallyPrime 7.1 Silver, 2026-10-01). It is a `List of Ledgers` collection with `SVCURRENTCOMPANY` `BRIDGE SHAPE LAB`, `SVFROMDATE` 2025-04-01 to `SVTODATE` 2026-03-31, a `FETCH` that includes `GUID`, and `COMPUTE` `BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany`. Its captured answer in that folder (`shape_lab_ledgers_1_response.utf16le.xml`, 85,746 bytes) holds 44 `LEDGER` rows |
| `L-ledgers-spaced.utf16le.xml` | written by `t2.py gen` from the file above: `SVCURRENTCOMPANY` holds two spaces, `BRIDGE SHAPE LAB`, two spaces; 1,744 bytes, sha256 `515693fa634387575955e411cdca0bc61ee831c648ea3be01408c61fb604c3f0`. `t2.py gen` asserts that the only change is those four spaces (eight bytes) |
| `send.py` | new; C3's sends, modelled on `lab/2162-bc-sheet:lab/2162-bc/run.py` (below) |
| `t2.py` | new; `gen`, `name` and `ledgers` (below) |
| `SHA256.txt` | sha256 and bytes of every other file |

**`send.py`** (`python -I send.py <label> <file>...`): before the first send it checks every file named: each must be one of the four request files in the table above, with the sha256 and bytes of its line in `SHA256.txt` (else nothing is sent). Then, for each file, one at a time, it refuses if its answer already exists (never sent twice), and sends it to `http://127.0.0.1:9001` with C3's line exactly: `curl.exe -sS -w "%{http_code} %{size_download}" --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -D <headers file> http://127.0.0.1:9001 -o <answer file>`. It records curl's exit code, the printed status and bytes, `CONTENT-LENGTH` from the headers file, the bytes on disk and the answer's sha256 in `log-send.txt`, and stops, as "the request failed", at: an exit other than 0, a status other than 200, an absent `CONTENT-LENGTH` or a size that differs from it, an empty answer, an answer over 2,000,000 bytes (kept, not used), or `Unknown Request` in the answer. It sets no timeout: a send not answered in 10 minutes is left running and reported (C3). Answers are `answers\<label>-<file>`.

**`t2.py`** (`python -I t2.py ...`; exit 3 = STOP):
- `gen`: writes `L-ledgers-spaced.utf16le.xml`, as above, and prints both files' sha256. Run it at step 0 and check its output equals the committed file's sha256.
- `name <answer> <guid>`: reads a raw Company collection answer as C4 says (UTF-16LE, forbidden characters dropped, parsed as XML, rows under `ENVELOPE/BODY/DATA/COLLECTION`). For the GUID's one row, prints its `NAME` attribute and each `NAME` element quoted, so edge spaces show, with characters, UTF-8 bytes, the first and last 4 bytes in hex, and sha256. The text is printed only if, without edge spaces, it is `BRIDGE SHAPE LAB`.
- `ledgers <answer> <guid>`: reads a raw `List of Ledgers` answer the same way and prints:
  - `HEADER/STATUS`, the number of `BODY/DATA/COLLECTION` elements, and the rows under them with their tags;
  - how many rows hold one `GUID`, and how many of those begin with `<guid>-`;
  - how many rows hold a `BRIDGECOMPANYGUID` equal to `<guid>`, and the distinct `BRIDGECOMPANYGUID` values;
  - the `CMPINFO` counts that are not 0, and how many are 0;
  - every `LINEERROR`, quoted.

  It prints no ledger name and judges nothing: 0 rows is printed, not a STOP.

**Checked when the sheet was written,** on captured files, with these results:

| command | file | printed |
|---|---|---|
| `t2.py gen` | the source above | the sha256 above |
| `t2.py ledgers` | `note-days/shape_lab_ledgers_1_response.utf16le.xml` | `STATUS` 1; 44 `LEDGER` rows; all 44 GUIDs begin with THE COMPANY's; all 44 `BRIDGECOMPANYGUID` equal to it |
| `t2.py ledgers` | `7c2ff6a1`'s `answers/m1-s6-led-a-while-A-closed.xml` (finding 2's zero-row answer) | `STATUS` 1; one collection; 0 rows; all 35 `CMPINFO` counts 0 |
| `t2.py ledgers` | `7c2ff6a1`'s `answers/m1-s2-led-a.xml` (another synthetic book) | 2 rows; 0 GUIDs beginning with THE COMPANY's; one other `BRIDGECOMPANYGUID` |
| `t2.py name` | `shape_lab_company_list_response.utf16le.xml` | `"BRIDGE SHAPE LAB"`, 16 characters, 16 bytes, sha256 `54510f216f9b0854d58ad42e7d5b7fbbc38b130d9a6038ae89aab528dc1e3f7b` |

`send.py` was checked with a stand-in for `curl.exe` only: one send passed; a resend, a size mismatch, an `Unknown Request`, a changed file and a file that is not a request were each refused. It has not sent to Tally (U).

**`CONTENT-LENGTH`** is in Tally's answers on the lab PC: the 2026-10-13 run (`7c2ff6a1`, `lab/m1-create-lands-where-answers/send-log.txt`) recorded `content-length` from the headers on all 43 of its sends, each equal to the bytes downloaded. That includes the two zero-row ledger answers of 2,994 bytes and `GET /status`. `send.py` still stops if it is absent.

**Names to compare with:** the pattern of `measure.py pattern`, 300 characters, sha256 `1d6366e1ccaa762b2da904319dc3a5385513fc401f1c814bb7a16bb0a03e433e` (30 blocks of 10: `A` to `Z`, then `a` to `d`); the spaced name `  BRIDGE SHAPE LAB  `, 20 characters, 20 bytes, sha256 `44e67913851e522f7681c2de08b1b6b668a74da6b2caf0082c29c57488ee4aca`; the name as found, 16 characters, sha256 `54510f21` as above.

## Rules

- One Tally on port 9001, THE COMPANY loaded alone. No Bridge process takes part: before step 0, `Get-CimInstance Win32_Process -Filter "Name='bridge_mcp.exe'" | Format-List ProcessId, ExecutablePath` (an error is an error, never "none found"); record every process found, end none, and let no chat call a Bridge tool during the sitting.
- **Requests sent:** only `C-company-list.utf16le.xml`, `M-marks-shape-lab.utf16le.xml`, `L-ledgers-shape-lab.utf16le.xml` and `L-ledgers-spaced.utf16le.xml`, each a read, by `send.py`, and C2's `GET /status`. No raw write is sent.
- **The writes:** the two renames on Tally's Company Alteration screen, each after BK and each undone by a restore of BK. Nothing else in the book is changed: on the alteration screen only the Name field is touched.
- **New shapes (C3):** after the first send of each request in a renamed state, and after `L-ledgers-spaced.utf16le.xml`, screenshot the whole Tally window before the next request. A dialog = C7: press no key, send nothing, STOP.
- **A ledger read in T2 is a result whatever it answers.** 0 rows, another company's rows or a `LINEERROR` is recorded by `t2.py ledgers`, not a STOP. Only `send.py`'s own stops (the request failed) and an answer that does not parse are STOPs.
- **No marks read is sent in a renamed state.** The marks request names the company by its old name, and a `Company` collection's row does not show a match (bridge#938). T1 measures the name; T2 measures the matching with the ledger reads.
- Delete nothing. No process is ended. **STD STOP** = change nothing, screenshot the whole screen, save what exists with size and sha256, report to the run coordinator, key nothing.

## Steps

**0. Load, read and back up.**
1. `date` (UTC). Copy the files to `C:\bridge-lab\2433\` and check each against `SHA256.txt`. `python -I t2.py gen`: EXPECT the sha256 under Files (another = STOP). `python -I measure.py pattern`: EXPECT the pattern's sha256 under Files.
2. Load THE COMPANY alone (C1). LAB-COMMON C2 step 0 in full: a to c, and d with `python -I send.py s0 M-marks-shape-lab.utf16le.xml`, then `python -I measure.py marks answers\s0-M-marks-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: record ALTVCHID and ALTMSTID (M0).
3. `python -I send.py s0 C-company-list.utf16le.xml`, then `python -I t2.py name answers\s0-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`: record the name as found, and `COMPANYNUMBER` from the answer. Record as found; the tree's capture of 2026-10-01 read `"BRIDGE SHAPE LAB"`, 16 characters.
4. **BK by C5** to `C:\bridge-lab\2433\bk\`: the file's name, bytes, sha256 and the backup's local time to the minute. Then `send.py s0b M-marks-shape-lab.utf16le.xml` and `measure.py marks`: record M1. M1 is the mark every restore is checked against. A difference from M0 is recorded, not a STOP.

**T1. Tally's stored name limit.**
1. Screenshot the Gateway. Open Company Alteration for THE COMPANY from Tally's own menu (write down the keys). On the Name field only: clear it, then type the 300 characters `measure.py pattern` prints, in order. If the field stops taking characters, type the rest anyway and note by screenshot where it stopped. Screenshot the field before accepting. Accept the screen (write down the key) and answer only its own accept prompt; any other prompt = C7, STOP. Screenshot the whole Tally window (no dialog) and the Gateway.
2. `send.py t1 C-company-list.utf16le.xml`; screenshot the Tally window. Then `measure.py native answers\t1-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346 --pattern`. **Record as found:** for the `NAME` attribute and each `NAME` element, the characters and UTF-8 bytes, the sha256, whether it is a prefix of the pattern, the block and character it ends in, and whether attribute and element are equal; and whether the stored length is over 255 bytes. A stored name that is not a prefix of the pattern (a typing slip) is recorded: its length stands, its cut point is U, and it is not typed again.
3. **Restore R1 (C6)** from BK, its two prompts only. Load THE COMPANY alone; screenshot the window and the Gateway.
4. **The check of R1.** `send.py r1 M-marks-shape-lab.utf16le.xml` and `measure.py marks`: EXPECT the GUID `3a6bd6e1-b835-4bff-89dd-8a6af138c346` and ALTVCHID and ALTMSTID equal to M1. `send.py r1 C-company-list.utf16le.xml` and `t2.py name`: EXPECT step 0.3's name and sha256. Either differs = STOP; the run coordinator decides, and T2 does not start.

**T2. Edge spaces.**
1. **The baseline, before the rename:** the name is as found, restored and checked at T1.4. Run `send.py t2base L-ledgers-shape-lab.utf16le.xml`, then `t2.py ledgers answers\t2base-L-ledgers-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found:**
   - from `log-send.txt`: the HTTP status;
   - `STATUS`, the row count (B), how many GUIDs begin with THE COMPANY's, how many `BRIDGECOMPANYGUID` values equal it, and every `LINEERROR`.

   The tree's capture of 2026-10-01 held 44 rows. If B is 0, steps 4 and 5 still run, but they cannot tell a match from no match: record that.
2. Screenshot the Gateway. Company Alteration as T1.1, the Name field only: clear it, then type two spaces, `BRIDGE SHAPE LAB`, two spaces. Screenshot the field before accepting (whether the spaces show is U). Accept as T1.1. Screenshot the window and the Gateway.
3. `send.py t2 C-company-list.utf16le.xml`; screenshot the Tally window. Then `t2.py name answers\t2-C-company-list.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found:**
   - each `NAME`, quoted, with its characters and bytes;
   - its first and last 4 bytes (a space is `20`) and its sha256;
   - which of these it equals: the spaced name (`44e67913`, 20 characters), the name as found (`54510f21`, 16), or neither.

   If it is the name as found, Tally trimmed the name on entry: record that, and run steps 4 and 5 all the same.
4. **The trimmed-name ledger read** (what Bridge sends): `send.py t2a L-ledgers-shape-lab.utf16le.xml`; screenshot the Tally window. Then `t2.py ledgers answers\t2a-L-ledgers-shape-lab.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. **Record as found,** as step 1: the HTTP status, `STATUS`, the row count against B, how many GUIDs begin with THE COMPANY's, the `BRIDGECOMPANYGUID` values, and every `LINEERROR`.
5. **The spaced-name ledger read:** `send.py t2b L-ledgers-spaced.utf16le.xml`; screenshot the Tally window. Then `t2.py ledgers answers\t2b-L-ledgers-spaced.utf16le.xml 3a6bd6e1-b835-4bff-89dd-8a6af138c346`. Record as found, as step 4.
6. **Restore R2 (C6)** from BK, as T1.3.
7. **The check of R2,** as T1.4: EXPECT the GUID, ALTVCHID and ALTMSTID equal to M1, and step 0.3's name. Either differs = STOP.

**Close.** Shut THE COMPANY by Tally's own Shut Company (write down the menu path and key). LAB-COMMON C10. `log-send.txt` is kept whole.

**Book state after each step:**

| step | name | marks |
|---|---|---|
| 0 | as found | M0, then M1 after BK |
| T1.1, T1.2 | the typed pattern, as Tally stored it | not read |
| T1.3, T1.4 | restored from BK; EXPECT as found | EXPECT M1 |
| T2.1 | as found (R1) | not read; the ledger baseline B |
| T2.2 to T2.5 | the spaced name, as Tally stored it | not read; ledgers read at T2.4 and T2.5, through each name |
| T2.6, T2.7 | restored from BK; EXPECT as found | EXPECT M1 |
| close | shut | |

If a STOP comes after a rename and before its restore, the book holds the renamed company, and BK is the way back, restored only on the run coordinator's word (C8).

## Result

**PASS (the sheet ran):** every step answered and was recorded; both restores checked equal to M1 and to the name as found; no request was sent twice; no dialog was answered but those named.

**FAIL:** a send recorded as an answer when `send.py` stopped; a request sent twice; a dialog answered that is not named; a restore not equal to M1.

**What it decides:**
- **T1:** Tally's name limit for ASCII letters on this release, in characters and bytes. Over 255 bytes: a person can type a company name that Bridge's company-list parser refuses (#2397, #2433 (a)). At 255 bytes or less: typing ASCII letters cannot make such a name on this release.
- **T2, the stored name:** whether Tally keeps edge spaces. Kept: Bridge's list holds a name that is not the stored name (#2433 (b)). Trimmed: Tally makes a company name with edge spaces impossible by typing, and #2433 (b) cannot arise from Tally's screen on this release.
- **T2, the reads:** a read **matched** THE COMPANY when it answers B rows, each GUID beginning with THE COMPANY's and each `BRIDGECOMPANYGUID` equal to it. A read **did not match** when it answers zero rows with `STATUS` 1 (finding 2). Anything else (another count, other GUIDs, a `LINEERROR`) is recorded as found and named.
  - The trimmed read matched: Tally matches Bridge's trimmed name to a stored name with spaces, the reverse of finding 3's direction.
  - The trimmed read did not match: a company whose stored name has edge spaces is read through Bridge's trimmed name as an empty book, which is #2433 (b)'s risk.
  - The spaced read says whether the byte-exact name selects it.
  - If Tally trimmed on entry, the trimmed read is an exact match. The spaced read then tests finding 3's direction with two spaces on each side, on licensed Silver.

## Not measured

A name over 255 bytes in fewer than 256 characters (multi-byte letters), and whether Tally's limit counts characters or bytes. A control character (cannot be typed). One edge space, a tab, or spaces on one side only. A voucher read or a write through either name (only the ledger list is sent). Whether a `Company` collection's row can show a match (bridge#938 says it cannot). Two companies loaded at once (#2433 (a)). Any Bridge build. Another Tally release.

## Evidence

A new branch `lab/2433-name-answers` (LAB-COMMON C11), pushed only after the pre-push scan and the run coordinator's yes, with no AI attribution in any commit:
- `RESULTS.md`: M0, M1 and each restore's marks; the BK file's name, bytes and sha256; T1's stored lengths, block and character; T2's stored names as `t2.py name` printed them, and the baseline and both ledger reads as `t2.py ledgers` printed them, each with its HTTP status from `log-send.txt`; every prompt word for word with the key that answered it; each finding marked verified, partial or unverified (one run: partial at most).
- Every answer and headers file under `answers\`, `log-send.txt`, and the `measure.py` and `t2.py` output whole, each with its size and sha256. The book is synthetic, so the answers are pushed; the home folder or user name, if any file holds one, is replaced by `<home>` or `<user>`, with the original's sha256 recorded.
- `notes.txt` (UTC times, one line per step: the act, what was read, the marks before and after), `SCREENSHOTS.txt` (names, bytes, sha256; the captures stay on the PC), `restore-plan.txt` for each restore, and `SHA256.txt` (`# sha256  bytes  file`).

## Report (one message)

M0 and M1; T1's stored length in characters and bytes and the block it ends in; T2's stored name (quoted), the baseline B, and what each ledger read answered (HTTP status, `STATUS`, rows, GUIDs of THE COMPANY's); both restore checks; a statement that nothing but the two renames was written and nothing was deleted.
