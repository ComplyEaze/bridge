# Hand-over note for the next lab session (25 Oct 2026, end of this sitting)

"Lab root" below means the lab's working folder on the Windows lab PC. Its full local path is in the lab's running hand-over file on that PC; the item sheets name the same folders.

## Tally
- TallyPrime 7.2 (Latest), Silver Rental (expires 24-Nov-2026), `tally.exe` 1.1.7.2.
- One process, owning port 9001, started normally (not elevated).
- Data folder: the TallyPrimeEditLog data folder.
- State now: Select Company, no company loaded. Tally's session date may read a November 2025 date from the last voucher entry; it is not data.
- Desktop control: keys reach Tally only when its window is focused. Click its title band (the blue bar) in the same batch as the keys, and never click inside the Select Company list.
- An elevated Task Manager window can cover Tally. It cannot be moved by the lab; ask the owner to close it.

## Items finished today (pushed)
| Item | Branch | sha |
|---|---|---|
| 72-REG | lab/72-read-regression-answers | fc83a758 |
| RD21 part A | lab/72-reads-rd21-answers | f3c9c736 |
| #1879 box-fit B1 | lab/1879-box-fit-answers | a1ac2391 |
| LR7 #698 | lab/698-gst-ledgers-read-answers | 0e8106e1 |
| LR5 #697 | lab/697-0.7.0-answers | dcf07c00 |
| GS1-L1 (partial) | lab/gs1-l1-answers | this commit |

## GS1-L1: where it stopped
See RESULTS.md. In short:
- BRIDGE GST LEDGER LAB, GUID aae372c5-a894-491c-8843-5532e43d2d2f.
- Marks 54 / 225.
- Step 5 is 40 of 50 copies done; the next copies are dated 26 Nov.
- Backup GS0: `<lab root>\gs1\backups\gs0\TDBK1800_<company number>.001`, sha256 3c804b2a5cd72a890bb61fd21338236e78dbd118a6c9a5a85792ba05f9e0bdde.
- Working files are in `<lab root>\gs1\`:
  - `c3-send.ps1` (the C3 sender);
  - `mk.sh` (one MARKS read);
  - `state.ps1`;
  - `ledgers.py`;
  - `evidence\notes.txt` (one line per step);
  - `shots\` (screenshots, never pushed);
  - `about-PRIVATE\`.
- Recorder: the original 5f36ad42 is lost. The replacement the replacement recorder's branch (name in the local hand-over file) @ 5b6fbfca (its script 00bd8411…) is not yet fetched or self-tested.

## Staged builds (nothing installed; each unpacked from a copy of its .mcpb)
| Folder under `<lab root>\builds\` | What | bridge_mcp.exe sha256 |
|---|---|---|
| `w1-2306d86d` | master 2306d86d, CI run 40077056130, artifact 12553782422 (zip 74ae2daa…) | 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 |
| `1879-dafa1c9f` | PR #1879 head dafa1c9f, run 40171649980, artifact 12591232885 (zip 77f1c4cc…) | d50abce7eaf9ce06b48563cc4b01ce8a013eac11071ab687390c0e79639b7014 |
| `1867-796e94f1` | PR #1867 head 796e94f1, run 40167319272, artifact 12589749758 (zip 1f9e76d0…) | bf8cdf82e10bf22f693d3c5d6d89bd074b35b2dbf6dc2cbad748ac8609916f46 |

The box-fit B1 re-run waits for #1879's new build 62bf7126; it is not downloaded.

## The lab entry and the stdio route
- The assistant's own Bridge tools are the installed 0.6.0 extension; they were not used for any row.
- The desktop app's config entry (named in the local hand-over file) points at the `w1-2306d86d` exe, with:
  - data folder `<lab root>\rc\data`;
  - import false, writes false;
  - `BRIDGE_TERMS_ACCEPTED` true;
  - localhost:9001, redaction none.
  A backup of the config as it was is kept under `<lab root>\72-step0\`.
- That entry's tools are not visible to the assistant session, so rows run through stdio clients that start the staged exe with the same environment:
  - `<lab root>\rc\mcp-call.py`: one call per process (sha256 dd6cb3f3…, used for 72-REG);
  - `<lab root>\lr5\lr5-run.py`: a jsonl file per process.
- `BRIDGE_TERMS_ACCEPTED=true` is set for these runs, with the owner as the person accepting the Terms for the lab.

## Queue (from the run's review)
1. Finish GS1-L1: step 5 (10 copies), recorder self-test, steps 6 to 10, optional 11 to 14, then C10.
2. #628 C0 (`lab/628-purchase-c0-run-sheet`).
3. LR6 (`lab/699-sft-candidates-sheet` 322b2a13).
4. #724 rows (`lab/724-numbering-rows`).
5. #1039 row A (`lab/1039-live-rows`; B and C stop on Silver).
6. RD21's #1897 live row (`lab/1240-empty-window-live` 43f7a026), when its build digest lands.
7. Box-fit B1 re-run on #1879's build 62bf7126.

These wait for the owner's word: RD21 part B (needs the release-candidate build), RD21's GST-status write row, and NONINT LAB (finish or restore).

## Standing rules (unchanged)
- Synthetic BRIDGE companies only.
- One request or one keyed act at a time.
- A backup before any write.
- Any unlisted dialog, Education mode or migration prompt is a stop.
- Push only after the pre-push checks and the run's review's yes, then read back with ls-remote.
- Never commit company or folder numbers, personal names, machine paths, screenshots or About captures.
