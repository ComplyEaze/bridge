# Windows item 293-P v0.1 DRAFT (owner at the screen): the packaged desktop app's existing-ledger catalogue flow against live Tally (25 Oct 2026; for issue #293, the Windows half of #287). SYNTHETIC ONLY: THE COMPANY = BRIDGE CATALOG LAB, created fresh by the owner in step 1 (books from 1 Apr 2026, India, GST No, inventory No); its company number and GUID are recorded there and every later step is checked against that GUID. No client book is loaded at any time. LAB-COMMON v3 applies in full, except where "Differences" says otherwise; where the two differ, the stricter holds. This item is DRIVEN BY THE OWNER at the Windows desktop of the lab PC; it cannot be run over the gateway tunnel. Needs the run coordinator's go-ahead, and a packaged Windows build of the current head installed for this item (the go-ahead names the installer's file name, size and sha256; an installer with another hash is not run).

Why: the flow (Prepare file, then Load existing ledgers, bind a target, save and reopen the draft, change the company scope) has been proven on macOS only. Nobody has driven the packaged app on Windows against live Tally. The macOS run found a first-run credential-store prompt that blocked until answered; Windows will have its own first-run behaviour, and only a real packaged run shows it. A step that fails is the more valuable result.

## The build and its settings
- TallyPrime lab release, licensed (Educational mode anywhere = STOP), port 9001. The desktop app is the PACKAGED Windows build the go-ahead names, installed by the owner with the installer's own screens (screenshot each; accept only the default install folder). Posting and every write feature are not part of this item and the app is never asked to post.
- Optional wire record: if the go-ahead names the serial read-only relay (as GS1-L1 used; its sha256 is recorded there), the app's Tally endpoint is set to the relay's port in step 3 and the relay's recording is kept; a request the relay refuses = STOP. Without the relay the item has no wire record and says so in its result.

## Differences from LAB-COMMON
- C1: THE COMPANY does not exist before step 1; the owner creates it there. From step 2 on, C1 applies as written. The desktop app is never started while another company is loaded.
- C3: the operator sends only MARKS (below); every other request is the app's own. C5: one backup, P0, after step 1 (masters only). C6: no restore.
- OWNER STEPS: every step is the owner's. One act at a time, with a screenshot of the whole screen after each act (the file dialogs, the credential-store prompt and any error included). Screenshots are never pushed.
- The first-run credential-store prompt (and any prompt the installer, Windows or the app raises) is answered ONLY if the step says so; otherwise screenshot it, copy its text, press no key and report. It stays on the screen until the run coordinator answers.

## Files (bytes and sha256 in SHA256.txt)
- MARKS: `requests/catalog-marks.utf16le.xml`, 980 bytes, sha256 592df2367f730f6fadd81c6ac3aa46749ddc597391390ac70563ca87d98c7725.
- The source file the app opens in Prepare file: `inputs/source-receipt.xml`, 957 bytes (UTF-8, no line break at the end), sha256 8cf07c3458d3c9f65da9f061c54fb1fbfaeaddc8b5da4854a6650a8f0217f799. One Receipt dated 10 April 2026, `Catalog Bank` Dr 250.00 / `Catalog Customer` Cr 250.00. The book has NO ledger called `Catalog Customer` (it has `Catalog Party`), so the file names a ledger to bind. The owner copies this file to `C:\bridge-lab\293-P\inputs\source-receipt.xml` and checks its size and sha256 (Get-FileHash) before opening it.

## Steps (LAB-COMMON start, then step 0 before every step that sends)
- 1. OWNER: create BRIDGE CATALOG LAB. Ledgers: `Catalog Bank` (Bank Accounts), `Catalog Party` (Sundry Debtors), `Catalog Sales` (Sales Accounts), `Catalog Cash` (Cash-in-hand), `Catalog Expense` (Indirect Expenses), `Catalog Creditor` (Sundry Creditors). No opening balances, no vouchers. RECORD the company number; MARKS: RECORD the GUID.
- 2. BACKUP P0 (C5) to the NEW folder `C:\bridge-lab\293-P\backups\P0\`. Step 0 (C2).
- 3. OWNER: install and start the packaged app (screenshots). RECORD every prompt the installer, Windows and the app raise, with its exact text, BEFORE answering; answer only a prompt that is the installer's own accept screen. If the app asks for a credential-store access, screenshot and STOP for the run coordinator's word. (This is the likeliest finding; it is a result.) Settings: endpoint 127.0.0.1 port 9001 (or the relay's port if named). Check the connection from the app: RECORD what it says and the company list it shows.
- 4. OWNER: select BRIDGE CATALOG LAB in the app (record how the company is shown: name, number, GUID if shown). Open `C:\bridge-lab\293-P\inputs\source-receipt.xml` in Prepare file (the file dialog: screenshot it, record the folder it opens in).
- 5. OWNER: Load existing ledgers. EXPECT the six ledgers of step 1 (plus Tally's built-in `Cash` and `Profit & Loss A/c`, which may or may not be listed: record). A missing ledger, an extra ledger or an error is a result. RECORD the count shown.
- 6. OWNER: bind the file's `Catalog Customer` to `Catalog Party`. EXPECT it to report as re-read and bound, and still unapproved. Screenshot.
- 7. OWNER: save the draft; close and reopen it. EXPECT the proposal text to persist and the binding to show as a saved UNVERIFIED target (not as verified). Screenshot both states.
- 8. OWNER: change the company scope (select another company in the app, or none; record which). EXPECT the catalogue capture to be invalidated and the proposal text to survive. Screenshot. Do NOT select any client book: if the list shows one, leave it alone.
- 9. Close the app by its own window close; step 0's tasklist/netstat (C2 a, b) again; MARKS once: EXPECT both marks equal to P0's after-backup marks (the flow only reads), else RECORD and report. Close (C10).
- THE RESULT is the seven-step table: step; what was observed; pass or fail against the EXPECT; plus every prompt and platform-specific behaviour from step 3 (installer, credential store, file dialog folder, first-run screens).

## Stop rules (beyond LAB-COMMON)
A credential-store or other prompt not named here (STOP, press nothing); a client book visible and selectable in the app's list (do not select it, report); the app asking to post, import or write anything; an installer hash that differs from the go-ahead's.
WHAT THE BOOK HOLDS IF THE ITEM STOPS: masters only (P0 exists); the app's flow reads and never writes, so the book is unchanged after step 1. The app's installation and its local draft stay on the lab PC and are listed in the notes.
Not covered: macOS, a split-GUID pair, a company with thousands of ledgers, anything the app does with posting on.
End state: BRIDGE CATALOG LAB shut; the app closed and left installed; Tally on 9001 with no company loaded.

Evidence branch: `lab/293-p-answers`. Report after step 3 (the first-run behaviour) and at the end, or at a STOP.
