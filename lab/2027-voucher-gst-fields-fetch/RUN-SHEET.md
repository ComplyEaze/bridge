# 2027 voucher GST fields by FETCH (#2027 step 2): the shipped `vouchers` request, and the same request with six voucher-level words added, on keyed notes, a purchase and an inter-state sale, then restored

Not for merge. About 2 hours. This sheet WRITES one ledger and five vouchers by hand on Tally's screens, then restores the book from a backup. ComplyEaze Bridge only reads. `SHA256.txt` lists every file here with its bytes; check it before step 0.

**Required reading beside this sheet:** `lab/2027-gst-note-fields-capture` at 4cde7eb3adb752b4ca1494840a3415ec571aca38, `RUN-SHEET.md`, steps 1 and 2. Its answers' `notes.txt` (`lab/2027-gst-note-fields-capture-answers` at 07e3bc34) records every screen of those steps as keyed on 28 Oct. This sheet repeats those four vouchers with the same values and gives each key below.

## What it decides
#2027 asks for the fields a CA checks in GST review. The 28 Oct capture keyed a Sales, a Credit Note, a Debit Note and a Purchase on this book. It found six fields at each voucher's top level in Tally's native voucher read, and not in the answer to the request `vouchers` sends today (`Bridge Agent Vouchers`, whose FETCH is master's `AGENT_VOUCHER_FETCH`). The six are:
- `PLACEOFSUPPLY`;
- `STATENAME`;
- `GSTREGISTRATIONTYPE`;
- `GSTNATUREOFRETURN`, the reason for a note;
- `VATPARTYTRANSRETURNNUMBER` and `VATPARTYTRANSRETURNDATE`, the buyer's or supplier's note number and date.

Adding the six words to that FETCH makes a new request shape. It is sent here, on a lab book, before any code changes (AGENTS.md P1, P7). For each day, this sheet sends:
- **the control:** the shipped request, unchanged;
- **the candidate:** the same request with the six words appended to its FETCH.

It decides three things.
1. Whether the candidate's answer carries each of the six, as a direct child of the voucher, with the value typed in step 3.
2. Whether the candidate's answer, with those six elements cut out, equals the control's answer for the same day, in three layers (step 8). This shows that the words added only those elements, as the `REFERENCEDATE` word was shown to (protocol reference §8.2f; there the reads were equal with the company counter block `DESC/CMPINFO` set aside as well). One control is read twice (step 7), so whether two reads of the same day can be equal at all is measured, not assumed.
3. On an inter-state sale (SV3, Place of Supply set to Karnataka), whether `PLACEOFSUPPLY` carries a value other than the company's own state, so that it is told apart by value. `STATENAME` is taken to be the party's state (unverified), which is also the company's state and the consignee's on this book (Maharashtra). It therefore stays attributed by element name only, PARTIAL, unless it reads Karnataka; that would itself be the finding.

It decides no code. The control's answers also become the committed fixtures for the parse that follows. Not covered:
- an unregistered party;
- an export;
- a note without an original invoice;
- a party registered in another state;
- Gold;
- another release.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2027-gst-note-fields-capture` at 4cde7eb3, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds, except for the named differences below.

**Who acts.** The lab does every step here, the keying included, on the synthetic company, on the owner's standing words of 27 and 28 Oct 2026. Each step is recorded as LAB-COMMON C11 directs; its rule on names applies to what is pushed. Two words are the run coordinator's:
- **The grant for the one ledger** (rule 5). The 28 Oct sheet created none, so this is confirmed before the sitting and written in the notes.
- **RESTORE**, before step 11.

**THE COMPANY** is `BRIDGE NOTES LAB` (synthetic; company number 100084). It is as the 28 Oct capture's restore left it:
- books from 1 April 2025; State Maharashtra; GST on and Regular; inventory off;
- the ledgers `CGST` and `SGST` (2.50 % each), `BRIDGE Svc Sales 5%` and `BRIDGE Svc Purchase 5%`. On 28 Oct the sales and purchase ledgers carried IGST 5 among their rate rows;
- the parties `BRIDGE Customer A` and `BRIDGE Supplier A` (Maharashtra, Regular);
- four vouchers dated 1 and 2 June 2025;
- `ALTMSTID` 215 and `ALTVCHID` 4.

If it is absent, or step 1's marks differ, STOP and report; record the difference. Only THE COMPANY is loaded, alone, from step 0 to the last step. No item, godown or batch is involved.

Named differences from LAB-COMMON (the stricter rule holds everywhere else):
1. **No raw MARKS request.** C2 d's MARKS read is made with `ledger_masters` (rule 3).
2. **Raw reads.** Raw reads by curl are allowed by the owner's decision of 26 Oct 2026 for shape discovery. Each is a committed request from `gen_requests.py`, with only the company, the day and the candidate's six words changed from the template.
   - One request at a time, as C3 sets out.
   - No other raw request is sent. `GET /status` and the tool calls also reach port 9001, as the steps say.
   - **Order:**
     1. the five controls;
     2. the built tool's read;
     3. the five candidates;
     4. last, the control of 4 June once more.

     That last one is a shipped shape that was answered before, so C3 allows sending it again; two reads of the same day are what step 8 compares.
   - The candidate is a NEW SHAPE: C3's screenshot of Tally's whole window follows each candidate send.
3. **Marks.** M (`ALTMSTID`) and V (`ALTVCHID`) are read with `ledger_masters` (basic, first page, no `snapshot_id`): `snapshot.master_alter_id`, `snapshot.voucher_alter_id`, `snapshot.reused` false.
4. **Backup and restore.**
   - BK1 is a backup of THE COMPANY alone, made before the first write (C5), into a new empty folder `C:\bridge-lab\2027b\backups\bk1\`, created and listed empty first.
   - On the word RESTORE, BK1 is restored on Tally's screen by C6. The book is then checked against the marks read after the backup.
   - BK1 is the only backup; a restore that does not complete is a STOP.
   - `restore-plan.txt` is written before the first write, as C6 and C11 ask.
5. **Grant.** One ledger and five vouchers, keyed by hand; nothing else is written, and no other master is created or altered. ComplyEaze Bridge writes nothing.
   - **The ledger:** `BRIDGE IGST`, under Duties & Taxes:
     - type of duty: GST;
     - tax type: integrated tax (`IGST` or `Integrated Tax`, as the list names it);
     - every other field on its defaults, its rate or percentage field included, which is recorded and not typed.
   - **The vouchers:** as step 3 gives them.
6. **One Tally.** At step 0 and at the end, as C2 a and b set out, plus `Get-NetTCPConnection -LocalPort 9001 -State Listen`: exactly one `tally.exe`, and it owns port 9001.
7. **Keys.**
   - A voucher or master is accepted with Ctrl+A.
   - After it, if a fresh creation screen is showing, take a screenshot of its title, then ONE Ctrl+Q.
   - **Never Ctrl+Q or Escape at the Gateway of Tally.** Never two Ctrl+Q in one call. Never Escape on a master or voucher screen (C7).
   - A voucher is not reopened for alteration.
   - A sub-screen inside a voucher is recorded (its title, every field label and value, by a screenshot before it is left) and passed on its defaults, except where a step fills it.
   - A message or prompt this sheet does not name is a STOP under C7. The one named here is SV3's quit confirmation (step 3).
8. **The candidate reads only.** For a candidate, each of these is a result, and the sheet goes on after a screenshot of Tally's whole window and `GET /status` 200:
   - an answer that is 200 with a body that parses (any number of rows, or an in-band refusal: STATUS 0 / LINEERROR);
   - 200 above its bound (kept, size and sha256 recorded, not parsed).

   Any other outcome is C3 and C7: STOP. That covers curl exit non-zero, a status other than 200, a size other than CONTENT-LENGTH, an empty body, no answer within 10 minutes, or a dialog on Tally's screen.
9. **A screen left half-entered** (SV3 waiting for a word) blocks every send. Nothing is sent to port 9001 while it shows, `ledger_masters` included.

Licensed mode only; Education mode = STOP.

## Build
`bridge_mcp.exe` of master `609776ae9f5cec48e6563f165480d3fe1adae445`:
- merge-queue run 40366984202;
- artifact `unsigned-bundle-smoke-windows-latest`, id 12670984633;
- `bridge_mcp.exe` sha256 `4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44`.

It is the build the 28 Oct capture used; its `vouchers` request is master's. `list_companies`, `ledger_masters` and `vouchers` absent from `tools/list` = STOP.

Hash it before the first call (step 0), and after the last call: step 9, and again in step 11 if it runs. Another sha256 = STOP.

Run it over stdio with:
- `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`;
- a new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2027b\data`, listed first: absent or empty, else STOP);
- `BRIDGE_TERMS_ACCEPTED=true` (allowed for the lab on 25 Oct 2026);
- redaction unset. Writes stay off.

## Files
- `gen_requests.py` and `templates/a07-vouchers-shape-20250424.xml`: the committed shipped request, a byte copy from the 28 Oct sheet; its FETCH equals master's `AGENT_VOUCHER_FETCH`.
- `requests/`: for each of 3 to 7 June 2025, `r-control-<day>.utf16le.xml` (1,740 bytes) and `r-candidate-<day>.utf16le.xml` (1,964 bytes).
- `check-request-ids.py`, `LAB-COMMON.md`, `.gitattributes`, `SHA256.txt`.

## Before the sitting
1. Check every file here against `SHA256.txt`.
2. `python -I gen_requests.py --check`: EXPECT exactly `gen_requests: ok (the template is reproduced byte for byte; only company, day and the six words change; 10 committed)`.
3. `python -I check-request-ids.py requests`: EXPECT `census: ok 10, skipped 0, FAIL 0`. Anything else = STOP, and nothing is sent.
4. The run coordinator's grant for the one ledger, written in the notes.

## Steps
0. **LAB-COMMON step 0 (C2), with no company loaded:**
   - the screenshot;
   - the process list and port 9001 (rule 6);
   - `GET /status`;
   - Tally's About screen: a screenshot shows its title, then leave by one Escape. Record the release (7.2 expected) and the licence;
   - hash the exe;
   - load THE COMPANY alone (arrow keys);
   - a screenshot of the Gateway with the Current Period and Tally's current date. Record the date as D0 (2-Jun-2025 expected).
1. **Before the first write.**
   - List `C:\bridge-lab\2027b\data`: absent or empty, else STOP.
   - Start the client.
   - `list_companies`: EXPECT exactly THE COMPANY, `identity_state` `verified_tuple`; record its `guid`.
   - `ledger_masters` (basic, first page): M0 and V0 (EXPECT 215 and 4). EXPECT no ledger named `BRIDGE IGST` in its list: if one exists, STOP before any write.
   - List the data folder again (C11), and Tally's own company data folder `100084` (C6, before).
   - Backup **BK1** (C5): Data > Backup & Restore > Backup; `Specify Path`, the folder above; THE COMPANY alone; End of List; Enter. Never Ctrl+A on that screen. Answer "Accept ?" Yes, as the 28 Oct notes record.
   - A screenshot of the Gateway with no dialog.
   - List the backup folder (EXPECT one file, `TDBK1800_100084.001`): name, bytes, time, sha256.
   - Write `restore-plan.txt` (C6): the backup folder and file with its bytes and sha256, the company number, the Restore Data Path, and the two prompts expected (`Accept ?` Yes; the folder exists: Replace).
   - `ledger_masters` again: M0′ and V0′ (EXPECT 215 and 4).
2. **The ledger.**
   - Gateway > Create > Ledger. Name `BRIDGE IGST`, under `Duties & Taxes`, with rule 5's choices. Record every field shown, and its rate or percentage field as offered.
   - A screenshot before accepting, then Ctrl+A.
   - If a fresh Ledger Creation screen shows: a screenshot of its title, then ONE Ctrl+Q.
   - A screenshot of the Gateway with no dialog.
   - Then `ledger_masters`: EXPECT M 216 and V 4. Anything else goes to the run coordinator before step 3.
3. **Key the vouchers** (accounting vouchers for services), in this order: SV2, CN2, DN2, PV2, SV3 last. For each:
   - Gateway > Vouchers, the key named;
   - F2 for the date;
   - the screens as the 28 Oct notes record them;
   - a screenshot before accepting;
   - Ctrl+A;
   - ONE Ctrl+Q at the fresh voucher screen (rule 7);
   - a screenshot of the Gateway with no dialog;
   - then `ledger_masters`: EXPECT M 216 and V 5, 6, 7, 8 and 9 after SV2, CN2, DN2, PV2 and SV3. Anything else goes to the run coordinator before the next voucher.

   Record every sub-screen's title, fields and values: Dispatch or Receipt Details, Party Details, Additional Details. Set Bill-wise Details to **New Ref** with the name given; a note defaults to Agst Ref, so change it. Leave the narration blank. Each typed value is entered exactly as written.
   - **SV2** Sales, F8, 3-6-2025.
     - `BRIDGE Customer A`, `BRIDGE Svc Sales 5%` 1,000.00, then CGST and SGST as Tally computes them (EXPECT 25.00 each).
     - Bill `SYN-BILL-S2`.
     - Party Details passed on its values; record them.
   - **CN2** Credit Note, Alt+F6, 4-6-2025.
     - `BRIDGE Customer A`.
     - Receipt Details: Original Invoice No. `SYN-ORIG-1`, Date 12-5-2025.
     - `BRIDGE Svc Sales 5%` 400.00, CGST and SGST (EXPECT 10.00 each).
     - "Provide GST details" Yes. On the Additional Details screen: Reason for Issuing Note `01-Sales Return`; Buyer's Debit Note No. `SYN-NOTE-1`, Date 13-5-2025.
     - Bill `SYN-BILL-C2`.
   - **DN2** Debit Note, Alt+F5, 5-6-2025.
     - `BRIDGE Supplier A`.
     - Dispatch Details: Original Invoice No. `SYN-ORIG-2`, Date 14-5-2025.
     - `BRIDGE Svc Purchase 5%` 400.00, CGST and SGST (EXPECT 10.00 each).
     - "Provide GST details" Yes. On the Additional Details screen: Reason `04-Correction in Invoice`; Supplier's Debit/Credit Note No. `SYN-NOTE-2`, Date 15-5-2025.
     - Bill `SYN-BILL-D2`.
   - **PV2** Purchase, F9, 6-6-2025.
     - Supplier Invoice No. `SYN-SUP-1`, Date 16-5-2025.
     - `BRIDGE Supplier A`, `BRIDGE Svc Purchase 5%` 1,000.00, CGST and SGST (EXPECT 25.00 each).
     - Bill `SYN-BILL-P2`; Tally pre-fills the supplier invoice number, so type over it.
   - **SV3** Sales, F8, 7-6-2025, the inter-state sale.
     - `BRIDGE Customer A`.
     - On Party Details, set **Place of Supply** to `Karnataka`. Leave the party's State and registration type as shown, and record them.
     - `BRIDGE Svc Sales 5%` 1,000.00, then `BRIDGE IGST`. EXPECT Tally to compute 50.00 (the sales ledger's IGST row was 5 on 28 Oct); CGST and SGST are not used.
     - Bill `SYN-BILL-S3`.
     - **If Place of Supply cannot be changed** on Party Details: take a screenshot, press no key, and wait for the run coordinator's word. The word is to abandon: one Ctrl+Q. Tally then shows a quit confirmation (record its exact text and choices by a screenshot); answer it to quit without saving, as the run coordinator says. Then a screenshot of the Gateway.
     - **If Tally does not compute IGST:** take a screenshot, press no key, and wait for the run coordinator's word. It is one of two:
       - type 50.00 as the IGST amount and go on;
       - abandon, as above.
       
       Altering `BRIDGE IGST` is a second master write, and is not granted.
     - **If Tally computes any other amount,** or CGST or SGST also appear: take a screenshot, press no key, and wait for the run coordinator's word.
     - **If SV3 is abandoned:** question 3 stays undecided, and the 7 June pair still runs, expecting 0 rows. Rule 9 holds while any of this waits.
   - **If Tally refuses a date or a value:** take a screenshot, copy the text, press no key, and STOP (C7).
4. **The controls.** For 3, 4, 5, 6 and 7 June in turn, send `requests/r-control-<day>.utf16le.xml`: bound 2,000,000 bytes (the 28 Oct answers were about 65 KB), C3's line, every answer saved with headers, size and sha256. EXPECT each answer to hold exactly 1 VOUCHER row of its day (0 for a day whose voucher was not saved). Any other count goes to the run coordinator before step 5.
5. **The built tool.** `vouchers` with `company_guid` from step 1, `from` 20250603 and `to` 20250607, saved whole: the control of what the tool returns today. Record its rows per day beside step 4's counts.
6. **The candidates.** For the same five days in turn, send `requests/r-candidate-<day>.utf16le.xml`, with rule 8.
   - After each, C3's screenshot and `GET /status` 200 (rule 8).
   - If no answer comes within 60 s, take a screenshot of Tally's whole window and press no key: a dialog is C7.
7. **Last, the 4 June control once more** (`requests/r-control-20250604.utf16le.xml`), saved as `control-repeat-20250604`.
8. **Reading the answers.** Read them by structure, never by grep. The 28 Oct answers are UTF-16LE with no byte-order mark, lines ending CRLF, one space of indent per level; check this on these answers and record it.
   - **The six, per day:** in the candidate's answer, for each of the six, the elements that are direct children of `ENVELOPE/BODY/DATA/COLLECTION/VOUCHER`. Record:
     - how many;
     - each one's form (`<X/>`, or with text) and its text as sent: absent, empty, or a value;
     - the raw value, and the value trimmed of ASCII space, tab, CR and LF at both ends. A leading or trailing space in a raw value is itself a finding for the later parse.
   - **Against the typed values,** in the forms the 28 Oct native answers used:
     - CN2: `GSTNATUREOFRETURN` `01-Sales Return`, `VATPARTYTRANSRETURNNUMBER` `SYN-NOTE-1`, `VATPARTYTRANSRETURNDATE` `20250513`;
     - DN2: `04-Correction in Invoice`, `SYN-NOTE-2`, `20250515`;
     - SV2, PV2 and SV3: the three note fields empty or absent;
     - every voucher: `GSTREGISTRATIONTYPE` `Regular`;
     - SV2, CN2, DN2 and PV2: `PLACEOFSUPPLY` and `STATENAME` `Maharashtra`;
     - SV3: `PLACEOFSUPPLY` against `Karnataka`, and `STATENAME` against the party's state.
   - **Cut and compare.** On the decoded text, cut out exactly those direct-child elements of the six names. Cut each whole element, together with the CRLF and indent before it. Count the characters cut. Check that the candidate's size equals the control's plus twice the characters cut, in UTF-16LE bytes. If it does not, repeat the check with `DESC/CMPINFO` cut from both (a counter can change its digit count) before recording a mismatch. Then compare with the control's answer for the same day, in three layers:
     - (a) byte for byte;
     - (b) byte for byte with `ENVELOPE/BODY/DESC/CMPINFO` also cut from both;
     - (c) structurally, by the parsed trees, ignoring only the six.
     
     Record "equal" or the first difference, with its path, for each layer.
   - **The two 4 June controls:** compare them in the same three layers. If they differ, record where. A candidate's difference at the same place is then not counted against question 2.
   - **The six in the control's answers:** record whether any appears there; none is expected. If one does, as a direct child of the voucher, cut the same elements from that control too before the comparison, and record it.
9. **Before the shut.**
   - `ledger_masters`: M1 and V1 (EXPECT 216 and V0′ plus the vouchers saved).
   - Stop the client by its stop file; record its exit code and error stream.
   - Hash the exe.
   - List the data folder (C11).
10. **Tally's current date.** At the Gateway, take a screenshot. Each saved voucher moved the current date (to 6-Jun-2025 on 28 Oct). If it is not D0, set it back by the Gateway's F2 (Change Current Date) to D0, and take a screenshot of the Gateway showing it.
11. **Restore,** only on the run coordinator's explicit RESTORE.
    - Shut THE COMPANY (Company > Shut from the Gateway's menu). Check BK1's folder still holds the one file with its sha256, then restore BK1 by C6, with the prompts of `restore-plan.txt` (record each, and a screenshot before any request). List Tally's company data folder `100084` (after). Load THE COMPANY alone, then start the client again on its data folder.
    - `ledger_masters` first (the GUID from step 1), then `list_companies`. EXPECT the marks equal M0′ and V0′ (215 and 4), and no `BRIDGE IGST` in the ledger list.
    - The Day Book, its period changed by Alt+F2 to 3-6-2025 to 7-6-2025, must be empty. Never press Enter on a Day Book line. Leave by one Escape per report level, after a screenshot of its title.
    - Stop the client by its stop file, and hash the exe.
    - Any other result: record it whole and STOP.
    
    On NO RESTORE, the book keeps the ledger and the vouchers, and the report says so.
12. **LAB-COMMON C10,** whether or not step 11 ran:
    - at the Gateway, a screenshot (the Current Period and the current date D0);
    - Company > Shut;
    - Select Company with none loaded;
    - C2 a and b (one tally.exe, 9001 LISTENING);
    - `GET /status` 200;
    - a screenshot showing no company loaded.
    
    No request is sent after the shut.

**STOP rules:**
- C7 and C8 of LAB-COMMON;
- any message or prompt this sheet does not name;
- a second `tally.exe`;
- Ctrl+Q or Escape at the Gateway. This is an incident: if a prompt shows, press no key, take a screenshot and STOP (C7). Only if the process list shows NO tally.exe, follow C9.
- a ledger or voucher saved other than as the steps say;
- a control above its bound, or a status other than 200 for a control or the repeat;
- a mark that moves outside steps 2 and 3;
- an exe sha256 that is not the one named.

## Evidence
- **Kept whole:** every tool answer and every raw answer (controls, the repeat and candidates, with headers, size and sha256). They are small; they are pushed.
- **Per day, a table:** each of the six (count, form, raw and trimmed), the value typed, the size check, and the three comparison layers.
- **The repeat:** the two 4 June controls compared.
- **Masters and vouchers:** the ledger's fields as shown; every voucher's sub-screens; SV3's outcome, and any quit confirmation's text.
- **Backup and data folder:** `restore-plan.txt`; BK1's name, bytes and sha256; the data folder's listings before and after.
- **The build's sha256** before and after.
- **Screenshots:** listed by name, size and sha256, never pushed.
- **A notes file:** UTC times and one line per step, as C11 sets it out (no roles and no people), with the grant of rule 5.

The typed values are synthetic. THE COMPANY's GUID is already in the merge gate's synthetic registry; no real figure is recorded.

**Confidence:** one sitting, one voucher per kind: PARTIAL until repeated.
