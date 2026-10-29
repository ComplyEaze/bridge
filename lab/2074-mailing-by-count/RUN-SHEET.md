# Windows item 2074-L v1: the master workbook's mailing details for a book with many master alterations and few ledgers, and what a further dated mailing entry costs, on #2188's build. (Part of #2074)

**Writes to the book: YES, to THE COMPANY only, undone at the end by restoring BK0.**
- One party ledger, "Lab Mark Probe", is created by request.
- It gets ten dated mailing entries on Tally's own screen, one save each.
- Its name-on-PAN field is then altered back and forth to raise the company's master-alteration mark (ALTMSTID) past 4,266.
- A second party, "Lab Mark Probe Two", is created on Tally's screen with five dated mailing entries in its one creating save.

Nothing else is changed. No voucher is posted. No ComplyEaze Bridge write tool is called.

## Why
- **The change.** #2188 (a draft) lets the desktop app's party/ledger master workbook read each party's dated mailing details for a book whose mark is past 4,266, whose ledgers are counted before the read. It reads them where the counted ledgers × 5,000 bytes, plus (mark − ledgers) × 950 bytes, fit the 16 MB budget. The 950 allows one further mailing entry for every alteration past the ledgers. See `tally/connection.rs`, `fetch_party_ledger_master_source` and `resolved_for_count`.
- **Before it.** Master reads the mailing details only for a mark of at most 3,200, so such a book shows "not observed". Both figures are UNVERIFIED.
- **Where the code is tested.** Only the workbook (`ComplianceRead::WithMailingDetailsWhereWholeReadFits`) runs `resolved_for_count`. The `ledger_masters` tool with `include_mailing_details` sends the same mailing request through the same read, but sizes it by the count alone. So the tool's reads below measure bytes, and **only the workbook (W1) tests the decision.**
- **What this row decides:**
  - L1. On #2188's build, does the workbook of a book with a mark past 4,266 and few ledgers show the mailing details? Each probe's status should read "in force", and the header should carry no not-observed note.
  - L2. What does one further dated mailing entry add to the mailing read, against the 950 bytes allowed?
  - L3. Does each read stay within its time? Does the mark move by exactly one per alteration?
  - L4 (optional, the run coordinator's call). Pushed past the largest mark the bound admits for this book, does the workbook go back to the standard read: every status "not observed" and the note shown?
  - L5. For a party given five dated entries in ONE save, does the mark move by one while its entries number five? If so, the bound's further-entry term undercounts such a party; record by how much.

## Who acts
- **The lab does every step, on the owner's standing words:**
  - **The raw requests** (create, and the repeated alterations). The owner's word of 26 Oct 2026 allows the lab's curl requests, writes included, for shape discovery. The owner's word of 28 Oct 2026 allows master writes on synthetic books. This is one synthetic book, restored at the end.
  - **The screen edits**, the clicks and keys in Tally, and the app's export, on the owner's standing word of 27 Oct 2026.
  - **The backup and the restore** (C5, C6); the restore only on the run coordinator's word.
  - Record each as done by the lab on the owner's standing word, naming which.
- **Downloading the CI bundle and installing the desktop app** from it need the owner's OK in the lab chat. Until it is given, nothing in this row is run.
- The go-ahead and any STOP ruling come from the run coordinator.

## Build and inputs (hash before the first step and at the end; a different hash = STOP; `SHA256.txt` lists every file of this folder with its size)
- **BUILD-2188:** the `unsigned-bundle-smoke-windows-latest` artifact of a CI run started by hand on #2188's branch (`gh workflow run ci.yml --ref fix/2074-mailing-by-count`). A draft pull request's own run builds no bundle.
  - The run's head SHA must equal the head the run coordinator names. Download it within the artifact's 7 days.
  - Record the run id, the artifact id and its GitHub digest. EXPECT the zip's sha256 to equal the digest.
  - **The server:** the `.mcpb`'s `bridge_mcp.exe`, extracted whole. EXPECT its sha256 to equal `mcpb-smoke.json`'s `binary_sha256`.
  - **The desktop app (D1):** installed from the bundle's Windows installer. Record the installer's file name, size and sha256, and accept the unsigned-installer warning. The app's version reads as the release's, so the build is identified by the installer's sha256 and the run's head, not by the version.
  - **Desktop app scope:** use the app only to connect (Tally's port set to 9001 in its connection settings) and to export the workbook.
- **The server is driven over stdio** by the lab's client, with an environment that sets:
  - `BRIDGE_TALLY_PORT=9001`;
  - `BRIDGE_TERMS_ACCEPTED`;
  - `BRIDGE_AGENT_MAX_BYTES=5000000`, so no answer is cut.

  `BRIDGE_AGENT_ENABLE_WRITES` and `BRIDGE_AGENT_ENABLE_MASTER_WRITES` are not set. Record the environment file's key names and sha256.
- **`requests/create-probe.utf16le.xml`, `alter-a.utf16le.xml`, `alter-b.utf16le.xml`:** UTF-16LE with a byte-order mark, sent by `LAB-COMMON.md` C3.
  - They are the captured create-then-partial-Alter shape of `tests/fixtures/native/master_fields_lab_partial_alter*.request.xml`, with three changes: the company and the ledger renamed to THE COMPANY and "Lab Mark Probe"; the create carrying no `PARTYGSTIN`; and the alter changing `NAMEONPAN` (a field the captured create sets) instead of `INCOMETAXNUMBER`, so that no identifier-shaped value is sent.
  - `create-probe` creates the probe under Sundry Debtors with PAN `ZZZZZ0000Z` and name on PAN `LAB MARK PROBE`. `alter-a` sets its name on PAN to `LAB MARK PROBE A`, and `alter-b` to `LAB MARK PROBE B`.
  - EXPECT the create's answer to read `CREATED` 1, and an alter's to read `ALTERED` 1, with `ERRORS` 0 and `EXCEPTIONS` 0.
- **Folders and company.**
  - Working folder `C:\bridge-lab\2074\`; evidence in `C:\bridge-lab\2074\evidence\`.
  - THE COMPANY is BRIDGE BATCH LAB, guid `a30f23c3-4819-4334-96ab-ffa6fba70976`.
  - BK0 goes to the new empty folder `C:\bridge-lab\2074\bk0\`.
- **The runner.** The lab's runner sends the alterations and checks each before the next (see the named exception). Its sha256 goes into `notes.txt` before step A5, and it is committed with the answers under `scripts/`.

## Rules (they add to `LAB-COMMON.md`; where they differ, the stricter holds)
- **Named exception to C3: repeated alterations.**
  - **What may repeat.** C3 sends each request file once. In A5 and Part L, `alter-a` and `alter-b` may be sent again and again, alternately, up to the count the step needs.
  - **Before every alteration** the runner:
    - sends the MARKS read (C2d);
    - goes on only if the answer holds exactly one COMPANY row, THE COMPANY's, with the voucher mark unchanged and the master mark exactly the one expected;
    - checks the alter file's sha256.
  - **After every alteration**, the answer must read `ALTERED` 1, `ERRORS` 0 and `EXCEPTIONS` 0.
  - **On any other reading** the runner halts and sends nothing more.
  - **Screenshots** of the whole Tally window at the 1st, the 10th, every 500th and the last alteration, and on any halt.
- **One company, two ledgers.** Only THE COMPANY is loaded. Only "Lab Mark Probe" and "Lab Mark Probe Two" are created or changed.
- **WRITES OFF.** A request whose answer reads anything but the EXPECT counters = STOP and WRITES OFF: no more requests but reads. Record, and wait for the run coordinator.
- **Marks.** Read MARKS at each step named. A master mark that moves other than as the step says = STOP: by one for each alteration, and by one for each screen save of the probe. The exception is A4b, whose step is what it measures, so record it whatever it is. A voucher mark that moves at all = STOP.
- **Screen edits** are made only in A3 (the probe's alteration screen) and A4b (probe two's creation screen). Escape is never pressed on a master screen (C7). Step 0 (C2) runs before each screen step and before W1.
- **Server reads only.** The server is used only for read tools: `tally_status`, `list_companies` and `ledger_masters`. For each `ledger_masters` read, record its evidence bytes and its duration.
- **Leftovers (B5)**, before, after part A, and at the end: no Bridge server or desktop app process the lab started is left running.
- **Privacy.**
  - Before committing evidence, search it for the user's name and for paths outside the lab folders. Replace each by `<user>` or `<path>`, and record the edit.
  - Screenshots stay local, with name, size and sha256 in `SCREENSHOTS.txt`.
  - The workbook stays local. Record its sha256 and the cells it reports.

## What the byte counts mean
- **What a read's evidence bytes total.** A `ledger_masters` read's evidence bytes are the sum of every answer in it: the company and currency reads, the master, balance and group reads (each read twice, as a pair, so counted twice), the extent brackets, and, for a counted book, the catalogue (read twice).
- **A change that grows only one party's master row** therefore grows the evidence by twice that row's growth.
- **A further entry's cost** is the evidence growth divided by twice the number of further entries.
- **What these totals cannot measure.**
  - A ledger's own cost cannot be read from these totals. R0 / (2 × ledgers) is an upper bound only, so only a result at or under 5,000 is conclusive.
  - The 16 MB comparison is not made with these totals.

## Before the sitting
- **P0.** `LAB-COMMON.md` C1 and C2: THE COMPANY in Select Company's list and loaded alone, else STOP. Record ALTVCHID and ALTMSTID (m0) from C2's MARKS read. **EXPECT m0 at most 3,150**, so that every read before A5 is sized by the mark, with no count. Otherwise STOP.
- **B1.** BUILD-2188 fetched and checked as above. Every file of this folder hashed against `SHA256.txt`.
- **D1.** The desktop app installed, as above.
- **S0.** BK0 by C5, before any write. Record the marks after it (ms0).
- **B5** before.

## Part A: the probes, their entries, and the mark
- **A1.** With the server:
  - `ledger_masters` (basic): record the ledger count c0;
  - then `ledger_masters` with `fields=compliance` and `include_mailing_details=true`, recorded as **Rpre**.
- **A2.** Send `create-probe` (C3). EXPECT `CREATED` 1, and MARKS m0 + 1.
- **A2b.** `ledger_masters` (compliance, mailing), recorded as **R0**. EXPECT the probe present, with `mailing_details` entries `[]` or `not_observed`; record which.
- **A3.** On Tally's screen, alter "Lab Mark Probe" to give it ten dated mailing entries, **one save each**, oldest first, using the screen's multiple mailing details.
  - Entry n (n = 1 to 10) is dated in turn 1 Apr, 1 May, 1 Jun, 1 Jul, 1 Aug, 1 Sep, 1 Oct, 1 Nov and 1 Dec 2026, and 1 Jan 2027.
  - Each has address lines `<n> Lab Street` and `Lab Area`, PIN 560001, state Karnataka, and country India.
  - MARKS after each save: EXPECT +1.
  - **After the first save** (one entry), read `ledger_masters` (compliance, mailing) as **Ra**.
  - After the tenth save, read it as **R1**, and record the mark m1.
- **A4.** From those reads:
  - **the first entry's cost:** (Ra − R0) / 2;
  - **a further entry's cost:** (R1 − Ra) / 18 (nine further entries, each counted twice);
  - **the upper bound on an average ledger's cost:** R0 / (2 × (c0 + 1)).
  - Check that the probe reads ten entries, oldest first, and that every other ledger's entries are unchanged.
- **A4b.** On Tally's screen, create "Lab Mark Probe Two" under Sundry Debtors with five dated mailing entries (1 Apr, 1 May, 1 Jun, 1 Jul and 1 Aug 2026, each as in A3), all entered before its ONE save.
  - Record the MARKS step of that save, s.
  - Then read `ledger_masters` (compliance, mailing) as **R1b**. EXPECT probe two with five entries, oldest first.
  - **The undercount** is 5 − s: the entries the bound's further-entry term does not see.
  - **Probe two's growth** is (R1b − R1) / 2. Compare it with (R0 − Rpre) / 2 + 4 × the further-entry cost.
- **A5.** The mark push, under the named exception. The runner sends `alter-a`, `alter-b`, `alter-a`, … until MARKS reads at least **4,400**.
  - **A5a.** After the first 10 alterations, stop. Read `ledger_masters` (compliance, mailing). EXPECT both probes' entries unchanged, and the marks m1 + s + 10. A partial Alter that dropped the mailing list = STOP, and report it before any further alteration.
  - Then go on to 4,400. Record the count sent, the time taken and the last mark m2.
- **A6.** Read `ledger_masters` (compliance, mailing) as **R2**. EXPECT:
  - the ledger count c0 + 2, with both probes' entries unchanged;
  - R2 − R1b about twice the catalogue (about 2 × (c0 + 2) × 1,100 bytes) and nothing more, which shows the count path ran. If the read was in parts, record that instead.

Run B5 after part A.

## Part W: the workbook on #2188's build
- **W1.** In the desktop app, after step 0, export the party/ledger master workbook for THE COMPANY with the host's date. Record:
  - the time taken and the workbook's sha256;
  - the "Balances as of" date the workbook states, d;
  - whether cell C4 holds the note "No ledger's mailing details were observed in this read …" (EXPECT empty). If it is shown, record whether its text includes "the count and the alterations beyond it", which only #2188's build says;
  - the probe's five Mailing cells. EXPECT the entry on or before d with the latest date: its address `<n> Lab Street, Lab Area`, PIN 560001, state Karnataka, its date, and status "in force".
  - one other party's Mailing status: "no mailing details" or "in force", never "not observed".
- **W1b.** Probe two's Mailing cells in the same workbook. EXPECT its latest entry on or before d (1 Aug 2026 for d after that date), with status "in force".

## Part L (optional, only on the run coordinator's yes; it is long)
- **Ln.** Compute the first mark the bound refuses for this book, with n = c0 + 2: L = n + floor((16,000,000 − n × 5,000) / 950) + 1. For example, for 51 ledgers that is 16,625.
- **L1.** Continue A5 under the named exception until MARKS reads at least L.
- **L2.** W1 again. EXPECT cell C4's note shown, including "the count and the alterations beyond it", and every Mailing status reading "not observed".

## Close
1. WRITES OFF: no more requests but reads.
2. Re-hash the inputs.
3. **Restore BK0 by C6**, on the run coordinator's word.
4. MARKS: EXPECT ALTVCHID and ALTMSTID as after S0 (ms0). `ledger_masters` basic: EXPECT no ledger named "Lab Mark Probe" or "Lab Mark Probe Two".
5. Record the installed app as left. Uninstall nothing.
6. B5 at the end.
7. C10: MARKS before the shut, then Shut Company, then GET /status only.

## Reading it (for the requester)
| reading | meaning |
|---|---|
| W1: both probes "in force" with the expected cells, other parties not "not observed", C4 empty | #2188 works on a live book at a mark past 4,266: L1 yes. |
| W1: C4's note shown with "the count and the alterations beyond it", every status "not observed" | #2188's build read the standard request: L1 no. Report c0, m2, R2 and the time. |
| W1: C4's note shown without that phrase | Not #2188's build: check D1 and B1. |
| W1: a probe "in force; some content of the entry was not read" | Read, with content skipped: record the cell whole. |
| A4: a further entry costs at most 950 | The entry allowance held on this book (one book, one release: partial). |
| A4: a further entry costs more than 950 | The allowance is too small for this book's addresses. Report the figure; the bound must grow before ready. |
| A4: the upper bound on an average ledger is at most 5,000 | The ledger figure held here (partial). |
| A4: the upper bound is over 5,000 | Not conclusive (it is a bound). Say so. |
| A4b: s = 1 | The undercount is real: one save added five entries, and the bound sees none of the four further ones. Report probe two's growth against the slack between 5,000 and the measured ledger cost. |
| A4b: s = 5 | Each entry counted as an alteration: the bound's premise held here. |
| A4b: s another value | Record it; the premise is partly true. |
| A5: a step other than one per alteration | The runner halts. Report it: the bound's reading of alterations as entries rests on it. |
| A5: the first `alter-a` answers other than `ALTERED` 1 | `NAMEONPAN` is not altered by this shape: STOP, report; the push needs another field. |
| A5a: an entry lost after a partial Alter | STOP: the push itself changed the history. Report it before anything else. |
| Part L: the note shown and the statuses "not observed" at L | The bound's refusal works live: L4 yes. |
| Anything else | Record it whole and infer nothing. |

**FAIL of the row:**
- a write other than the named ones, or to another company;
- a voucher mark moved;
- the restore not returning the marks to ms0;
- a Bridge write tool called;
- B1's checks not met.

## Evidence (stays on the lab PC except the files listed)
- **RESULTS.md:** every step, stamp, mark, byte count, duration and cell reading.
- **The C3 answer files** for the create and the alterations: the alterations aggregated (counted, with the first, the last and any differing answer kept whole, each with its size and sha256).
- **The `ledger_masters` answers** Rpre, R0, Ra, R1, R1b, the A5a read, R2 (and Part L's), whole.
- **`notes.txt`**, the runner under `scripts/`, and `SCREENSHOTS.txt`.
- **The workbook's sha256 and its cell readings.** The file itself stays local.
- **`SHA256.txt`** over the committed files, with sizes.

Evidence goes on a new branch `lab/2074-mailing-by-count-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.

## Not measured (say so in the results)
- Mailing entries added by request: no import shape for `LEDMAILINGDETAILS.LIST` is captured, so they are added on the screen.
- A book whose alterations went into mailing entries on many parties.
- The census path, at a mark past 22,857.
- The master body's own bytes: the evidence totals hold every answer of a read.
- Other releases, Gold, a second monitor.
