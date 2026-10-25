# Windows lab sitting 2: a Microsoft Store (MSIX) install of Claude Desktop, with the extension and the desktop app, on the release-candidate build (#1459, #1584, #293; 25 Oct 2026). Synthetic companies only. No Tally write.

Not for merge. This is the ONE sheet of the sitting. It holds the order, the build, the owner's acts, the substitutions and the new part. Two earlier sheets are kept beside it as the step text of Parts 1 and 2, changed only by replacing the word "session" (by "lab" or "conversation"): `part1-source\WINDOWS-1569-P.md` (v2.2) and `part2-source\WINDOWS-1641-ROWS.md` (v5). WHERE THIS SHEET AND A PART'S SOURCE DIFFER, THIS SHEET HOLDS. The standing rules are LAB-COMMON v3 (LAB-COMMON.md, branch lab/626-1038-1050-run-sheets, commit 8c602c15): it applies to every Tally-side act here, in full; where an item sheet is stricter it holds. Do not start without LAB-COMMON in front of you.

## What the sitting decides
1. PART 1, #1459: on a Store install, whether `build_import_xml` prints a path that File Explorer opens (`result.path`, `result.path_state`); which data folder the extension really uses; what `local_data_report` names against that folder; and what the 1,403 lock files of the plain-shell folder are (counted, names grouped by pattern only, nothing opened or deleted). The fix (#1569) is merged; this is its live row.
2. PART 3, #293 (OPTIONAL, see below): the seven steps of the packaged desktop app on Windows against live Tally, with the first-run behaviour recorded, failures included.
3. PART 2, #1584: whether a Tally port lock held on one side of the package boundary refuses a request from the other. The fix (#1641) is merged; its Windows row has never run.

**PART 3 IS OPTIONAL.** It stays in the sheet but is run only when the owner is at the screen and the time is there; Parts 1 and 2 and CLOSE decide #1459 and #1584 without it, and a sitting that skips it is complete. If it is skipped: (a) in ORDER below steps 4 and 6 are not run and step 5, the COMPANY SWITCH, follows O5; (b) O5 still runs, because Part 2 needs the desktop app installed and not running; (c) the Part 2 row "0g" below does not apply: Part 2's 0g and C2 run as its source says (the endpoint is NOT set, C2 sets it and records it); (d) the report says "Part 3 (#293): not run". #293 stays open.

## What was stale in the two source sheets, and what this sheet puts in its place
| in the source | stale because | in its place |
|---|---|---|
| Part 1: the build under test (PR head, run id, artifact id, expiry); S1's run and zip digest | #1569 is merged (da656d6b2) and that artifact has expired | the BUILD block below; S1 compares against it, and its zip-digest anchor uses the Build block's artifact id and digest |
| Part 1: manifest `version` 0.5.1; keys `accept_terms_2026_10_1`, `host`, `port`, `redaction`, `enable_writes` | the manifest at master is 0.6.0 with `accept_terms_2026_10_1`, `host`, `port`, `redaction`, `enable_posting_2026_10`, `enable_tax_audit_2026_10` | the version and keys S1 reads from the build's own manifest. The posting key that stays UNTICKED is `enable_posting_2026_10`, and `enable_tax_audit_2026_10` stays unticked too (found ticked: the owner unticks it, his act). Read `enable_writes` as `enable_posting_2026_10` everywhere |
| Part 1: B5's tool list | the list is longer today | record the list whole. STOP only on `post_import`, `acknowledge_post_review`, a `lab_` tool or a Bridge listed twice; other read tools are recorded, not a STOP |
| Part 1: D6 "(b) NOT MET at dff9beee" | master's `docs/legal/privacy.md` (about line 118) and `docs/security-and-privacy.md` (about line 317) now name the Store-install location | D6 reads both paragraphs at the build's commit and decides MET or NOT MET against what C0, D2 and D3 found |
| Part 1: E1 to E3 "put back the earlier build" at the end of Part 1 | Parts 3 and 2 need the release candidate installed | E1 to E3 run ONCE, at CLOSE below. PRIOR is whatever S4 found installed (its hash is the one S4(b) records) |
| Part 2: "built ON THIS PC from commit 0b2bd2de"; 0b's `git rev-parse`; A1 and A2's lab-built installer and `.mcpb` | #1641 is merged and the CI bundle holds both files | 0b is replaced by hashing the BUILD block's installer and `bridge-tally.mcpb` against it; A1 is O5 below; A2 is already done by Part 1 (the same `.mcpb`, hash recorded); the premise "nothing of the lab build installed yet" (Order of work) becomes "the extension of Part 1 is installed and idle, the desktop app is installed by O5 and not running" |
| Part 2: 0g "the endpoint is NOT set here" | Part 3 has already run Check Tally Endpoint and saved port 9001 | 0g records the saved endpoint; its C2 still sets and records it |
| Part 2: company BRIDGE PILOT LAB | unchanged | unchanged; the company of Parts 1 and 3 is BRIDGE BATCH LAB |
| Part 2: step 7 `--local-data-report --show-paths` | unchanged | unchanged; Part 1's D6c and D6d are the other halves of the same question |

## BUILD (filled in by whoever relays this sheet, before it goes to the lab)
The release candidate's Windows bundle: artifact `unsigned-bundle-smoke-windows-latest` of the Bridge CI run on the release-candidate head. It holds, under `bundle\`, `bridge-tally.mcpb`, `mcpb-smoke.json` and the desktop installers (`nsis\*-setup.exe`, `msi\*.msi`); the installer to run is the `*-setup.exe`.
- head sha: ______  run id: ______  artifact id: ______  zip digest: ______
- `bridge-tally.mcpb` bytes and sha256: ______  `bridge_mcp.exe` (inside it) sha256: ______  `*-setup.exe` name, bytes and sha256: ______

The lab downloads into an EMPTY folder, hashes, and compares with this block. A different hash, an expired or missing artifact, or a head that is not the release candidate's is a STOP before anything is installed. Nothing is built on the PC.

## THE OWNER'S ACTS (every other act is the lab operator's; each is marked OWNER in the steps)
| act | where | about (min) |
|---|---|---|
| O1 Uninstall the installed ComplyEaze Bridge extension (Desktop, Settings, Extensions) | Part 1 B1 | 2 |
| O2 Install `bridge-tally.mcpb` (Settings, Extensions, Advanced settings, Install Extension) and answer Desktop's prompts | Part 1 B2 | 4 |
| O3 Tick the Terms box in the extension's own settings screen (never ticked by the lab; no value written). Untick the posting and tax-audit boxes if found ticked | Part 1 B3 | 1 |
| O4 Quit Claude Desktop fully (tray icon, Quit) and start it | Part 1 B4 | 2 |
| O5 Run the desktop `*-setup.exe`; approve the administrator prompt if one appears; UNTICK "start ComplyEaze Bridge" on the last page (quit it from the tray if it starts anyway) | after Part 1 | 5 |
| O6 Be at the screen for Part 3: answer the OS prompts (SmartScreen, the Windows credential store) and the two native file dialogs; the owner or the person he names | Part 3 | 30 |
| O7 Approve the operator's permission rehearsal (Part 2 A0b) and click Allow, the option that does NOT ask again, on Desktop's tool prompt for the first Bridge call (A5) | before Part 2 | 10 |
| O8 At CLOSE: say whether to put the earlier extension build back (E1: uninstall the candidate, install the earlier `.mcpb`, quit and start Desktop) and what to do with the two ports | CLOSE | 12 |
| total | | about 66; about 40 in Parts 1 and 3, 10 before Part 2, 12 at CLOSE; he is on call (back within 10 minutes) while Part 2 runs |

STANDING RULE: Terms acceptance, posting, batch posting, ledger creation and the tax-audit box are the owner's acts in every form (the extension's settings, an environment variable, a Claude Desktop config entry, a script). This sheet writes the marker word OWNER-ACCEPTS-TERMS-HERE and never a value. The lab never touches `claude_desktop_config.json`. Posting found ON is a STOP in all three parts (nothing here posts). A tool answering `terms_not_accepted` is a STOP (in Part 2 it makes the setup VOID, as its source says).

## Files (check size and sha256 first; a different value is a STOP)
| file | bytes | sha256 |
|---|---|---|
| hold-lock.ps1 | 3564 | e9cac8ce447ab3be72bd25419c0d0a8b83db6c5f42aa9473c3975f832076162d |
| requests\r01-company-extent.xml | 2524 | 9df2a53f085dac2636e9435462b612c1487ec6f903677815036c9f39163f7dd8 |
| requests\r02-marks.xml | 976 | 81892a744009b6cf462a6f8d2fd505e650f8ad022e1462b23414cf2b8b3d2d7c |
| requests\build-1569.json | 306 | e5e60a118598c9a5d275fcea4b25202394fd570089b4d03617e51732b76b8039 |

Every other file is listed in SHA256.txt. Working folders: Part 1 `C:\bridge-lab\1569p\`, Part 2 `C:\bridge-lab\1641\`, Part 3 `C:\bridge-lab\sitting2\p3\` (drafts in `p3\draft\`, evidence in `p3\evidence\`). Screenshots stay on the PC and are never pushed.

## ORDER, and where it may be split
1. SETUP (S0 to S5 of Part 1, with the Build block). 2. PART 1 (B1 to D6d, no E). 3. O5 (the desktop installer). 4. PART 3, steps P3-0 to P3-6 (BRIDGE BATCH LAB loaded alone). 5. COMPANY SWITCH to BRIDGE PILOT LAB (the lab operator's act at the Tally screen: Select Company and Shut Company, nothing typed; LAB-COMMON C1, C2 and C10 around it). 6. PART 3, step P3-7. 7. PART 2 (BRIDGE PILOT LAB). 8. CLOSE.

Length: Part 1 about 150 minutes (its source's plan, without E), Part 3 about 45, Part 2 about 180 plus the owner's 10, CLOSE about 30: about 7 hours. It may be split once, after P3-6 and before the COMPANY SWITCH: day A is steps 1 to 4 (about 3 h 30), day B is steps 5 to 8 (about 3 h 30). The state left between days: the candidate extension and the desktop app installed and idle, no holder running, BRIDGE BATCH LAB shut by Tally's own Shut Company (LAB-COMMON C10) and the saved draft on disk. Day B starts with LAB-COMMON C1 and C2 (BRIDGE BATCH LAB loaded alone), starts the app, runs Check Tally Endpoint, selects BRIDGE BATCH LAB and opens the saved draft (**Open saved draft**; EXPECT the draft as saved, binding unverified), and then does the COMPANY SWITCH.

## PART 1: #1459 (step text: part1-source\WINDOWS-1569-P.md; the substitutions above apply)
Run S0 to S5, B1 to B6 (OWNER at B1 to B4), C0 to C3, D1 to D6b as written, then:
- D6c. `local_data_report` from the extension, `{}`, one call (Desktop's tool prompt: its RULES, DESKTOP TOOL PROMPT; this tool is added to the four it names, for this step only). RECORD the answer whole: the folder it names, its counts, and whether it says the folder is missing, empty or complete. COMPARE with C0 and D3: which folder (real or package) holds the journal and `imports\` after D1, and which folder the answer names. EXPECT nothing; the contributor's prediction, unverified, is that the answer names the real folder while the files are in the package. Record whichever is found: "it names the folder that holds the files" and "it names the other one" are both results.
- D6d. From a Start-menu PowerShell (outside the package): `& "<the exe of B6>" --local-data-report --show-paths > <evidence>\d8-out.json 2> <evidence>\d8-out.err; $LASTEXITCODE`, then read it with `ConvertFrom-Json`. RECORD `folder`, `folder_path`, `lock_folder_path`, the file counts and the count of lock files (the contributor saw 1,403 in the real folder; a different number is a result, not a STOP), and the stderr line. Then list the real `agent` folder's lock files by COUNT and by name PATTERN only (group by the text before the first `.`, at most 20 groups; never open, rename or delete one). RECORD which lock names exist. Nothing is deleted.
- Additional STOPS: `post_import` or `acknowledge_post_review` listed; an approval window opening; the extension writing to neither data folder at C0 (its own STOP).

## OWNER STEP O5, between Parts 1 and 3
The lab operator hashes the `*-setup.exe` against the Build block BEFORE the owner runs it (a mismatch = STOP, nothing installed). The owner runs it (UAC) and unticks "start". RECORD: each installer prompt as a screenshot before the owner answers it; whether the install is per-user or per-machine (the Programs and Features entry and the install folder, by name); `Get-Process` by the app's process name: NONE running. A running desktop app at this point = STOP.

## PART 3 (OPTIONAL): #293, the packaged desktop app against live Tally (OWNER at the screen)
The owner (or the person he names) is at the screen; the operator reads each step aloud and records; it drives the screen itself only if the owner granted screen control at O7's rehearsal. The OS prompts are the owner's alone.

Preconditions: Part 1 done; the desktop app installed (O5) and NOT running; Tally with BRIDGE BATCH LAB loaded alone (LAB-COMMON C1 and C2 a to d first, with the screenshot of the whole Tally window first; any dialog = STOP); the file of Part 1's D1 exists (its path and sha256 from `d1-build-answer.json`: `result.path`, `result.sha256`); nothing typed in Tally. A STEP-LOG.txt line per step in `p3\` (step, UTC to the second, one word).

- P3-0. FIRST RUN. The owner starts the desktop app from the Start menu. The operator screenshots the whole screen before and after every prompt the OS or the app draws (SmartScreen, Windows security, a credential-store prompt, a firewall prompt, a Settings screen asking for a secret) and records for each its exact text, which button the OWNER pressed, and how long the app waited on it. The macOS run met a Keychain prompt on first Settings access that blocked until answered: a Windows equivalent, if any, is the likeliest surprise and is a RESULT, not a STOP, unless it asks for a password, key or token to be typed (then the owner declines, screenshot, STOP). RECORD: `p3-0-first-run.txt`, the app's version (the window or About), and `Get-Process` for the app's process name (names and PIDs; no path in public evidence). EXPECT: the app opens to its first screen; nothing has been sent to Tally.
- P3-1. Check the connection. The app's endpoint is 127.0.0.1:9001 (the operator reads the field first and sets it in the app's Settings if it shows 9000). Click **Check Tally Endpoint**. EXPECT success; the endpoint 127.0.0.1:9001; BRIDGE BATCH LAB with the GUID that `requests\build-1569.json` carries; Education mode No. RECORD the screen's text and a screenshot. Any other company, or two companies, = STOP.
- P3-2. Select BRIDGE BATCH LAB in the app. EXPECT it is selected as the one company, with an observed GUID. RECORD.
- P3-3. **Prepare file**, then **Choose source XML**. In the native file dialog the OWNER types the printed `result.path` of Part 1's D1 into the file-name field (this also proves that a person can open the printed path). EXPECT the file loads: one voucher with two ledger entries. If it is refused, RECORD the exact text (the screen refuses symbolic links, Windows reparse points and hard links; a package-redirected folder is the case to watch), then copy the same file's bytes (size and sha256 equal) to `C:\bridge-lab\sitting2\p3\source.xml` and open that, and RECORD both outcomes. RECORD the dialog's starting folder.
- P3-4. **Load existing ledgers**. EXPECT the catalog read returns the company's real ledgers: the two ledgers of Part 1 (`Synthetic Bank Ledger`, `Party A`), which Part 1's C3 found exact in Tally, are in the list. RECORD the count and the two names; the full list stays on the PC.
- P3-5. Bind a target: for a row's ledger entry choose `Party A` (or `Synthetic Bank Ledger`). EXPECT the app reports it as re-read and bound, and still UNAPPROVED. RECORD the exact state text.
- P3-6. **Save draft** (native save dialog; the OWNER types `C:\bridge-lab\sitting2\p3\draft\sitting2.bridge-draft.json`), then **Open saved draft** and open that file. EXPECT the proposal text persists and the binding is demoted to a saved unverified target ("Saved unverified target: <the ledger>"). RECORD the exact text and the saved file's size and sha256. The saved JSON holds the original XML and stays on the PC.
- COMPANY SWITCH (between P3-6 and P3-7): the lab operator, at the Tally screen: screenshot; Shut Company (menu path and key written down); a screenshot showing NO company loaded; LAB-COMMON C2 a and b; Select Company BRIDGE PILOT LAB; LAB-COMMON C2 c and d (its GUID is recorded); then Part 2's `r02` read, sent under Part 2's curl rule, its `ALTVCHID` 170 or less (else STOP). The app is left running at its Prepare-file screen with the draft open.
- P3-7. Change the company scope. In the app: **Check Tally Endpoint**, then select BRIDGE PILOT LAB. EXPECT the app invalidates the ledger capture of BRIDGE BATCH LAB (the bound target is gone or marked stale, and it says so) and the proposal text SURVIVES. RECORD the exact text and a screenshot. Then close the app from its window (never by ending the process); the draft was saved at P3-6, so no unsaved-edit prompt is expected; if one appears, screenshot, the owner presses Cancel, and the operator reports (STOP).

Result per step: MET, NOT MET or VOID, with the screen's text. Confidence: one run, PARTIAL.

## PART 2: #1584 (step text: part2-source\WINDOWS-1641-ROWS.md; the substitutions above apply)
Run its 0a to 0g, A0 to A5 (A1 is O5 and A2 is Part 1's B2, both done; A3's port 9001 and Terms box were set in Part 1 B3 and are only re-recorded; A0, A0b, A4 and A5 are run, with the owner at O7), steps 1 to 8, STOP, RESTORE and REPORT as written, BRIDGE PILOT LAB being loaded alone from the COMPANY SWITCH. Its 0f listing will show the sets as Parts 1 and 3 left them (the extension's `native-dispatch-leases` under the package copy, the real one if the desktop app made it, the profile folder if either made it); its renames in 1a handle exactly that and nothing is deleted. Its 0c requires the single `bridge_mcp.exe` process to be the extension's, with a hash equal to the Build block's exe hash.

## CLOSE
1. Part 2's RESTORE (every holder ended, the app closed, the sets put back as its text says).
2. The Tally screen: BRIDGE PILOT LAB shut (LAB-COMMON C10: one `tally.exe` owning 9001, GET /status 200, a screenshot with NO company loaded, the date as recorded at the sitting's first step 0). No XML request after the shut.
3. OWNER O8: the owner decides E1 (the earlier extension build back: Part 1's E1 to E3 as written, PRIOR from S4) or leaves the candidate installed; the two ports (extension 9001, desktop app 9001) are left as they are unless he says. Recorded as said.
4. EVIDENCE: Part 1's E4 as written, for the three parts together: README from `README-evidence.template.txt` (filled from the files; "not measured" where a step did not run), MANIFEST.txt, the public-bound set with `<home>` and `<package>` in place of the real strings, and the PUBLIC-TEXT grep of Part 1's rules over every public-bound file. Screenshots, the saved draft, `source.xml` and the full ledger list stay on the PC, listed by name, size and sha256.

## STD STOP (the standing stop of all parts)
Change nothing, take a whole-screen screenshot, save what exists with size and sha256, report to the run coordinator, call no tool, key nothing. In Part 2 first let any running Core Accounting read or tool call finish (its text). In Part 3 the desktop app is left as it is and not closed. A STOP in a part ends the sitting for the parts after it unless the run coordinator says otherwise; the parts before it stand.

## REPORT (one results file)
The Build block as filled and the hash checks; per part the source's own REPORT; for Part 1 also the D6c and D6d answers, the C0 folder finding and the D2 path findings; for Part 3 the first-run table (prompt, text, the owner's button, the wait) and the seven results; for Part 2 its row table and listings. Say plainly what was not run. Confidence: one run, PARTIAL.
