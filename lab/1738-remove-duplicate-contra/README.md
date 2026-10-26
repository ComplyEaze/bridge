# Taking one Contra out of a book: cancel and delete by REMOTEID, a reload and a restore over them, absent IDs, and two numbering behaviours (#1738), on BRIDGE REMOVE LAB

**Synthetic company `BRIDGE REMOVE LAB` only, NEW for this sheet.** If a company or a data folder of that name already exists on the PC: STOP and report it; never reuse or overwrite one. It is loaded alone, and nobody else uses this PC's Tally from step 0 to the last step.

**This sheet WRITES.**
- By import: two ledgers and ten Contras, then five deletes and five cancels.
- On Tally's own screens: the company's creation, up to three numbering changes of the Contra type, one shut-and-load and one restore.

Licensed TallyPrime 7.1, not Educational mode, a single currency. No ComplyEaze Bridge build is used: every request is sent by curl (owner's ruling of 26 Oct 2026: lab requests by curl are allowed, writes included, to discover shapes that Bridge will then send itself).

LAB-COMMON v3 applies in full: file `lab/626-1038-1050-run-sheets/LAB-COMMON.md` on branch `lab/626-1038-1050-run-sheets`, commit 8c602c15. That means C1 to C11:
- the request line of C3;
- the reading rules of C4;
- the backups of C5 and the restores of C6;
- the screens rule of C7;
- the STOP rules of C8 and C9;
- the close of C10.

Where this sheet and LAB-COMMON differ, the stricter holds, and this sheet's five-minute rule (STOP rules below) supersedes C3's ten minutes. THE COMPANY in LAB-COMMON is `BRIDGE REMOVE LAB`; record its company number and GUID at S1.

## Why, and what each answer decides

ComplyEaze Bridge detects a transfer booked twice: two Contras, one from each bank's statement, both in Tally. It cannot clear one today. The owner decided on 26 Oct 2026 that Bridge may take one of the two out through its approval window, but only after this evidence:
- a **cancel** by default (the voucher and its number stay, shown cancelled);
- a **delete** only on voucher types that keep their numbers (Retain Original Voucher No.);
- a rectifying entry in a locked period.

Nothing of that is built yet.

**What is known** (public protocol reference, voucher-writes part):
- **Delete:** a delete by the REMOTEID a voucher was created with removed one voucher on Silver (a Journal) and one on Gold (its type not recorded). Neither was Bridge's own Contra body (sections 9.12b and 9.14).
- **Cancel:** a cancel with the full body cancelled a Journal in place once on Silver (`ALTERED 1`, `CANCELLED 0`, entries emptied, same GUID; 9.14). On an older Education instance a hand-built cancel created a new cancelled voucher instead (9.6).
- **Not tried:** either action on a Contra, on the bodies Bridge sends, or under Renumber Vouchers.
- **Not measured:** a reload or a restore after a removal, a delete of the voucher that holds the company's voucher mark, and a cancel or delete of an ID Tally never saw.

Each row answers one question, changing one thing against the row it is compared with:

| rows | compared with | the one thing that differs | decides |
|---|---|---|---|
| L1 | L2 | the voucher deleted holds the company's top ALTERID (the voucher mark) | whether a delete moves ALTVCHID, keeps it, or lets it fall (a fall would read as a restore to Bridge) |
| L2 | L1 | the voucher (a repeat, P5) | whether the candidate delete removes a native Contra and leaves every other voucher as it was |
| L3 | L2 | the body: the one a Bridge hand-import file carries (tagged narration), sent by curl | whether the delete holds for a voucher created from a Bridge file's body (not through Tally's Import menu) |
| L4, L5 | each other | the voucher (a repeat) | whether Bridge's native body with `ACTION="Cancel"` cancels a Contra in place, and ALTVCHID's step for it |
| L6 | L4 | the body: the file's | the same for the file body |
| L7, L8 | L4, L2 | the REMOTEID: one Tally never saw (two different ones) | whether a cancel or a delete of an unknown ID fails loudly or writes something (P7) |
| P1 | the state after L8 | the company shut and loaded again | whether every result survives a reload |
| R1 | the state before L1 | a restore of the backup taken just before L1 | whether a restore brings back what was cancelled and deleted, and where ALTVCHID ends |
| L9 | L2 | the Contra type's numbering behaviour: Renumber Vouchers | whether a delete renumbers other vouchers, and whether their ALTERIDs move |
| L10 | L4 | the same | whether a cancel keeps its number, and the others theirs, under Renumber Vouchers |

**The builders these shapes are meant to become.** None exists; each is a candidate for review after this sitting.
- **The cancel** (rows L4 to L7 and L10): `render_voucher_cancel_xml`, beside `render_voucher_xml` in `src-tauri/src/agent_import.rs`. It is the same body Bridge sent at creation, with `ACTION="Cancel"`, inside `render_import_envelope`. It is sent only through the one native dispatch path, behind the approval window.
- **The delete** (rows L1 to L3, L8 and L9): `render_voucher_delete_xml(remote_id, voucher_type, date)` in the same file. It carries the REMOTEID the voucher was created with, `VCHTYPE`, `DATE` and `VOUCHERTYPENAME`, and nothing else.
- **The reads:** today's builders, unchanged.
  - marks: `render_agent_company_high_water` (`src-tauri/src/agent_read_profiles.rs`);
  - the day's vouchers: the verification window read (`src-tauri/src/agent_import.rs`, the `Bridge Agent Import Verification` collection);
  - numbering: `render_voucher_type_numbering_request` (`src-tauri/crates/bridge-tally-protocol/src/native_voucher_type_numbering.rs`).

**PASS for the design (decided before the sitting).** A row passes or fails on its read-back. The counters are recorded beside it but do not decide it.
- **Cancel as the default:** L4, L5 and L6 each pass when the voucher keeps its GUID, VOUCHERNUMBER and MASTERID and reads ISCANCELLED Yes, no new row appears, and every other voucher of the day keeps its number and its ALTERID. Its counters are recorded: 9.14 saw `ALTERED 1` and `CANCELLED 0`.
- **Delete on Retain Original Voucher No.:** L2 and L3 each pass when the voucher is absent from the day's read and every other voucher keeps its number and ALTERID. L1 the same, plus its ALTVCHID reading as recorded.
- **What the slice needs recorded, in every row:**
  - ALTVCHID before and after;
  - for a cancel, the cancelled voucher's ALTERID next to the new ALTVCHID;
  - for a delete, ALTVCHID's step, especially in L1.
- **L9 and L10** decide what Bridge does under Renumber Vouchers. If L9 renumbered or re-ALTERIDed any other voucher, Bridge refuses a delete on such a type.
- **L7 and L8** are P7 evidence whatever they answer.
- **P1 and R1** are recorded as read. A restore over a removal is detected only if R1's ALTVCHID is below every value read after a removal.

## Files

`requests/` (19 requests), `gen.py`, `.gitattributes`, `SHA256.txt`.
- `gen.py` writes every request from the cited builders' format strings. `python3 -I gen.py` regenerates them byte for byte, and SHA256.txt with them.
- Check every file against `SHA256.txt` first (PowerShell `Get-FileHash -Algorithm SHA256 FILE`, compare all 64 characters).
- The requests are UTF-16LE with a byte-order mark and no final line break. Send them as they are, with C3's line.
- Every export request's `ID` equals its `COLLECTION NAME` (`Bridge Agent Company High Water`, `Bridge Agent Import Verification`, `List of VoucherTypes`).

| request | what |
|---|---|
| `marks.xml` | MARKS: the company's GUID, ALTVCHID and ALTMSTID |
| `vch-20250505.xml`, `vch-20250506.xml` | DAY5, DAY6: the day's vouchers, Bridge's verification read |
| `vtypes.xml` | VTYPES: the voucher types with their numbering series |
| `s3-ledgers.xml` | two ledgers, `Lab Bank A` and `Lab Bank B`, under Bank Accounts |
| `s4a-create-v1-v2.xml`, `s4b-create-v3-v4.xml`, `s4c-create-v5-v6.xml` | six Contras on 5 May 2025, Dr Lab Bank B and Cr Lab Bank A, 101.00 to 106.00. V1, V2, V5 and V6 use the native body; V3 and V4 the file body |
| `l1-delete-v6.xml` … `l8-delete-absent-x2.xml` | the Phase A rows, named by row |
| `b1-create-w1-w4.xml` | four native Contras on 6 May 2025, 201.00 to 204.00 |
| `l9-delete-w1.xml`, `l10-cancel-w3.xml` | the Phase B rows |

**Reading the answers.** Read every voucher answer by C4: one line per VOUCHER row with DATE, VOUCHERTYPENAME, VOUCHERNUMBER, MASTERID, ALTERID, GUID, ISCANCELLED, NARRATION and each entry. Count the rows by structure, never by text search, and keep each table as a file. A row is first matched to its voucher by NARRATION (`Lab transfer V1` … `W4`; the file body adds ` [BRIDGE:` and its REMOTEID `]`), and then followed by GUID.

**THE COMPARISON**, used after every row: the new table against the table before it, BY GUID. Look for rows new, rows gone, and rows whose ISCANCELLED, VOUCHERNUMBER, ALTERID or entries changed. Write one line per row in the notes: the counters, those differences, and ALTVCHID and ALTMSTID before and after.

## Folders and backups

Working folder `C:\bridge-lab\1738\`, evidence in `C:\bridge-lab\1738\evidence\`. Backups (C5), each into its own new empty folder:
- BK0 `C:\bridge-lab\1738\backups\bk0\` (S2);
- BKA `…\bka\` (S5, just before L1);
- BKB `…\bkb\` (B0, before Phase B);
- BKE `…\bke\` (the end).

## The Contra type's numbering screen

Gateway of Tally, Alter, Voucher Type, `Contra`. The field "Method of Voucher Numbering" stays **Automatic**. The field "Numbering behaviour on insertion/deletion" is the one changed: **Retain Original Voucher No.** or **Renumber Vouchers**.

On a type that holds vouchers, earlier runs (protocol reference, voucher-writes part, the "Renumber Vouchers" paragraphs) saw Tally ask two confirmations at each switch, and on this sheet each is answered **Yes**, once:
1. on choosing the behaviour: "Changing the method of voucher numbering will affect voucher numbers of existing transactions. Continue? Yes or No";
2. on accepting the screen: "Are you sure you want to update the voucher numbers of existing vouchers? Y : Accept Master & Update Vouchers / N : Back".

A save of a type with no vouchers asked nothing on an earlier run. Any other prompt, or a different text: screenshot, answer nothing, STOP (C7). Leave the screen as C7 says.

**Reading VTYPES.** For the row `Contra`, read the VOUCHERNUMBERSERIES row's NUMBERINGMETHOD and NUMBERINGSUBMETHOD. The type-level NUMBERINGMETHOD reads `Default` or `None` and is recorded, never compared.

## Steps

- **S0.** LAB-COMMON step 0 (C2), with NO company loaded yet: C2 d is replaced by a screenshot showing no company loaded. Check the 19 request hashes and SHA256.txt.
- **S1.** Create THE COMPANY on Tally's screen:
  - name `BRIDGE REMOVE LAB`, books beginning 1 April 2025, base currency INR, nothing else changed from the screen's defaults; accept;
  - screenshot the Gateway showing it loaded alone, and record the Current Period it shows;
  - EXPECT a period that includes 5 and 6 May 2025 (an alteration outside the current period may fail without saying so, implementation guide 3.1a). If not, set the period to 1 April 2025 to 31 March 2026 by the Gateway's Period key, screenshot, and EXPECT that; else STOP;
  - MARKS: record the GUID, the company number (from the company list on screen) and both marks. EXPECT ALTVCHID absent or 0: a new company.
- **S2.** VTYPES. EXPECT the Contra series `Automatic` and `Auto Retain`. If the sub-method is anything else, set Retain Original Voucher No. on the numbering screen (above), then VTYPES again, and EXPECT `Auto Retain` (else STOP). Then backup BK0.
- **S3.** `s3-ledgers.xml`. EXPECT `CREATED 2` (C4's import rule). MARKS.
- **S4.** `s4a`, then DAY5; `s4b`, then DAY5; `s4c`, then DAY5 and MARKS. EXPECT `CREATED 2` for each.
  - The final DAY5 holds exactly six rows, numbered 1 to 6 in the order V1 to V6, none cancelled. V6 holds the highest ALTERID, and it equals ALTVCHID.
  - Record each row's GUID, MASTERID and ALTERID as table T4. Record any other numbering as it is, and STOP before S5 if the six rows are not there.
- **S5.** Backup BKA, then MARKS (M5).

**Phase A (Retain Original Voucher No.).** Send each request once. After each send, DAY5 then MARKS, then THE COMPARISON.

- **L1.** `l1-delete-v6.xml` (V6 holds the voucher mark). PASS: V6 gone, every other row unchanged. RECORD the counters and ALTVCHID against M5: moved up, unchanged, or down.
- **L2.** `l2-delete-v2.xml`. PASS as L1, for V2.
- **L3.** `l3-delete-v4.xml`. PASS as L1, for V4 (the file body).
- **L4.** `l4-cancel-v1.xml`. PASS: V1 keeps its GUID, number and MASTERID and reads ISCANCELLED Yes, with no new row and every other row unchanged. RECORD its entries as read (9.14 saw them emptied) and its new ALTERID beside the new ALTVCHID.
- **L5.** `l5-cancel-v5.xml`. PASS as L4, for V5 (the repeat).
- **L6.** `l6-cancel-v3.xml`. PASS as L4, for V3 (the file body).
- **L7.** `l7-cancel-absent-x1.xml` (a REMOTEID never sent). RECORD the counters, any LINEERROR, and whether a new row appeared (if so, its number, ISCANCELLED and entries). No PASS line.
- **L8.** `l8-delete-absent-x2.xml` (another REMOTEID never sent). RECORD as L7.
- **P1.** Shut THE COMPANY by Tally's own Shut Company (no process is ended), load it again by Select Company, then step 0's screenshot, DAY5 and MARKS. RECORD THE COMPARISON against the table after L8. EXPECT no difference.
- **R1. Restore BKA** (C6: the operator, on Tally's screen, on the run coordinator's RESTORE; the fallback is BK0). Then DAY5 and MARKS.
  - EXPECT DAY5 equal to T4 by GUID, number, ALTERID and ISCANCELLED: the six vouchers back, none cancelled, nothing from L7 or L8.
  - RECORD ALTVCHID against M5 and against every value read after L1 to L8.

**Phase B (Renumber Vouchers).**
- **B0.** Backup BKB. On the numbering screen, set Renumber Vouchers, then DAY5, DAY6 and MARKS. VTYPES: EXPECT `Auto Renumber` for Contra (else STOP). Keep this DAY5 as table T5: the save itself may renumber the type's vouchers into date order, and T5 records what it did.
- **B1.** `b1-create-w1-w4.xml`, then DAY6, DAY5 and MARKS. EXPECT `CREATED 4`; record the numbers W1 to W4 took, and THE COMPARISON of DAY5 against T5.
- **L9.** `l9-delete-w1.xml`, then DAY6, DAY5 and MARKS. W1 should be gone. RECORD the counters, and for every row of both days whether its number or ALTERID moved.
- **L10.** `l10-cancel-w3.xml`, then DAY6, DAY5 and MARKS. RECORD as L9, plus W3's GUID, number and ISCANCELLED.
- **B2.** On the numbering screen, set Retain Original Voucher No., then DAY5, DAY6 and MARKS. VTYPES: EXPECT `Auto Retain`.

- **E1.** Backup BKE, then MARKS. C10 close. THE COMPANY stays on the PC as BKE left it, for a later read. It is never used for another sheet.

## STOP rules (beyond LAB-COMMON C8 and C9)

- **A dialog:** any dialog, message box or half-entered screen before a request is a STOP before sending it. On the numbering screen, the two confirmations quoted above are answered and nothing else is.
- **A bad answer:** an import answer that does not parse, has header STATUS 0, or is above 64 KiB is a STOP.
- **A row that fails its PASS** (L1 to L6) or meets an unexpected effect of its own: a FINDING. Record it whole, take DAY5 and MARKS once, then STOP before the next row and report; the run coordinator decides whether to go on. A row whose voucher is already gone or already cancelled before it is sent is a STOP at once.
- **A voucher no row names changes:** after recording, STOP. That covers a voucher of the other day, or any voucher in Phase A other than the row's own. In Phase B, numbers of either day's Contras may change (Renumber Vouchers renumbers the type), and that is recorded, not a STOP. An ALTERID or a GUID that changes on a voucher the row does not name is a STOP in Phase B too.
- **A slow answer:** one that has not finished after five minutes. Do nothing at all: do not cancel it, do not send it again, do not send anything else, do not touch Tally. Report and wait.

## Evidence

Every request and answer with size and sha256 (C11), the tables, and `notes.txt` with UTC times and one line per step. Screenshots are listed by name, size and sha256 and kept on the lab PC. The answers go to the evidence branch the run coordinator names.

**Not covered here:** Gold, the Edit Log edition, Multi-user Auto numbering, a voucher keyed on Tally's screen, a locked period, two loaded companies, a Tally process restart, and a file imported through Tally's own Import menu. Each is a later row if the owner wants it.
