# Results: #1237, is a party's PAN returned by the compliance listing, 26 Oct 2026. PASS: a keyed PAN comes back as `INCOMETAXNUMBER` text exactly as typed; a party with no PAN, and a party whose PAN was cleared, both answer with no `INCOMETAXNUMBER` element at all

The sheet is `lab/1237-pan-compliance-read` at 44658ff164fcd115d6b5438f826bf0e7422c9c48. Preflight:
- `SHA256.txt` checked 6/6 OK on sha256 and bytes;
- `check-request-ids.py` gave `census: ok 2, skipped 0, FAIL 0`;
- each request was checked against `SHA256.txt` before every send.

TallyPrime **Release 7.2 (Latest), Silver Rental**, not Educational mode, from Tally's About screen earlier in the same sitting.

The book is BRIDGE PILOT LAB (100033, GUID 6b43e498-430c-4d5c-bfef-d32e2ab93c85). Every request was a read by curl, one at a time.

The three ledgers were made on Tally's own screen. The owner gave the go-ahead before any keying. The owner typed the PAN value itself, and the lab keyed the rest one key at a time.

## The steps as run
| Step | What | Result |
|---|---|---|
| 1 | load alone; `p01` → `m1` | GUID as EXPECTed; **ALTVCHID 68, ALTMSTID 245** |
| 2 | Chart of Accounts > Ledgers | 29 groups, 18 ledgers; no `BRIDGE PAN Party A/B/C` |
| 3 | `p02` → `c0` | 38,278 B, sha256 4976bbec…7216; 18 rows; 0 `INCOMETAXNUMBER` elements |
| 4 | C5 backup | `TDBK1800_100033.001`, 90,051 B, sha256 26276fa11541d523e65d1b514efc809a7dea7910bb64dc3ff8ff522172978661, 21:38 IST. The marks read after it equals `m1` |
| 5 | create Party A, PAN `ZZZZZ0000Z` | saved (details below) |
| 6 | create Party B, PAN untouched | saved |
| 7 | create Party C, PAN `ZZZZZ0000Z`; then alter C, PAN cleared | saved; altered |
| 8 | `p01` → `m2` | ALTVCHID 68 (= m1); **ALTMSTID 249** (m1 + 4: three creates and one alter) |
| 9 | `p02` → `c1` | 45,232 B, sha256 c143d51b…33e5; **21 rows** (c0 + 3, each under Sundry Debtors) |
| 10 | `p02` → `c2`; `p01` → `m3` | `c2` byte-identical to `c1`; `m3` byte-identical to `m2` |
| 11 | the staged build's `ledger_masters`, fields `compliance`, group `Sundry Debtors` | rows below; snapshot master_alter_id 249 |
| 12 | C6 restore of step 4's backup; `p01` → `m4`; `p02` → `c3` | **`m4` byte-identical to `m1` (68 / 245); `c3` byte-identical to `c0`** (18 rows; the three parties gone) |
| 13 | C10 | none loaded; one tally.exe; 9001 LISTENING; /status 200 |

A read-back was taken after each party was saved:
- `cA-readback`, `cB-readback` and `cC-readback`: the listing after A, after B and after C was created;
- a marks read after each: ALTMSTID 246, 247, 248, one more each time.

## The PAN field (steps 5 to 7)
- **Label "PAN/IT No.", in the section "Tax Registration Details"**, on the Ledger Creation screen. Below it: Registration type, GSTIN/UIN, and Set/Alter additional GST details.
- **Tally 7.2 warns on `ZZZZZ0000Z` and stores it.** Leaving the field raised a box titled "Warning": "Invalid PAN format." / "The PAN must contain 5 alphabets, followed by 4 numbers and then 1 alphabet." / "For example:" followed by an example PAN / "Press any key to continue". One key (Space) closed it, and the value stayed in the field. The warning is advisory, and the value was not refused. Why Tally flags this value is not established: the idea that it checks the 4th character is unverified.
- Registration type was **Regular** on all three:
  - on A it was the owner's choice on screen when accepting;
  - B and C were kept the same, so the PAN is the only thing that differs.
- With GSTIN/UIN left blank, Tally showed a second Warning, "GSTIN/UIN not specified", dismissed the same way.
- In the alteration of C, a first Backspace on the whole-selected PAN value moved the cursor back one field and changed nothing. End, then ten Backspaces, cleared the value.

## Reading the answers (C4, by structure; `tables/`)
| Answer | Rows | `INCOMETAXNUMBER` elements | Party A | Party B | Party C |
|---|---|---|---|---|---|
| `c0` | 18 | 0 | (none) | (none) | (none) |
| `cA-readback` | 19 | 1 | text `ZZZZZ0000Z`, ALTERID 246 | (none) | (none) |
| `cB-readback` | 20 | 1 | text `ZZZZZ0000Z` | **absent**, ALTERID 247 | (none) |
| `cC-readback` | 21 | 2 | text `ZZZZZ0000Z` | absent | text `ZZZZZ0000Z`, ALTERID 248 |
| **`c1`** (= `c2`) | 21 | 1 | **text `ZZZZZ0000Z`** | **absent** | **absent** (after the clear), ALTERID 249 |
| `c3` (= `c0`) | 18 | 0 | (none) | (none) | (none) |

- `PARTYGSTIN` and `NAMEONPAN` are **absent** on all three parties in every answer.
- The raw characters of A's element are `<INCOMETAXNUMBER TYPE="String">ZZZZZ0000Z</INCOMETAXNUMBER>`, stored as typed. `notes.txt` has the UTF-16LE bytes.

**What this measures.** In this listing, a party whose PAN was never keyed and a party whose PAN was keyed then cleared both answer **no `INCOMETAXNUMBER` element**, never an empty one. So, on 7.2, "no PAN" reads exactly as an element Tally did not send. A keyed PAN comes back as its text.

## Step 11: the build's compliance read, as returned
| Ledger | `compliance.income_tax_number` | `party_gstin` | `party_gstin_status` |
|---|---|---|---|
| BRIDGE PAN Party A | `"ZZZZZ0000Z"` | `null` | `"no_gstin_in_force"` |
| BRIDGE PAN Party B | `null` | `null` | `"no_gstin_in_force"` |
| BRIDGE PAN Party C | `null` | `null` | `"no_gstin_in_force"` |

- `snapshot.master_alter_id`: 249, with `reused` false.
- Each party's `gst_registrations` holds one entry: applicable_from 20260401, gstin null, registration_type Regular.

## Deviations, recorded
- There is no lab screenshot of Party A's final screen before it was accepted; the owner accepted it. The last lab screenshot is just before the Registration type field.
- Extra reads beyond the sheet's steps:
  - C5's marks read after the backup;
  - a marks read and a listing read-back after each party was saved;
  - each was a read, and none changed the book.
- Shutting the company for the restore, with a blank Ledger Creation form open, raised Tally's question "This will close all the open Reports, Vouchers, and Masters, without saving changes you might have made. Do you want to shut the Company?". The owner said yes, and it was answered Yes.
- The company's data-folder listings before and after the restore stay on the lab PC, by sha256: before d7d9558f0cf95dc54cf1192d297f966abb98a5d2eafc726da88f2421417c4420, after 3b4cd163e86be9737792f71f49b397792a21d89892ceb1dd2f2cbb70a6af8c96.

## In this commit
- `responses/`: every answer whole, with its headers. That is m1 to m4 and c0 to c3; the marks read after the backup; the three check marks reads and three read-backs; step 11's two tool answers; the C2 and C10 /status answers and the C10 observation.
- `send-log.txt`: every request, with request sha256, send and answer times (UTC), curl output, bytes, answer sha256 and bound.
- `notes.txt`: UTC, one line per step. The lab PC's own folder is written `<lab folder>`.
- `tables/`: the structural reading of each listing.
- `read1237.py` and `call1237.py`: the lab's reader and tool client, not product code.
- `restore-plan.txt`.
- `SCREENSHOTS.txt` (name and sha256) and `SHA256.txt`.

Not committed: the screenshots, the backup and the client's data folder.
