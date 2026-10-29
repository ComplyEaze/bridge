# Results: #1490 mailing captures, 29 Oct 2026, through pull request #1675's build at 9045b9f0 (corrected 29 Oct: not d6d89545), on Bridge Lab Win 0.5.1 (TallyPrime 7.2 Silver). The 4c read on 7.2 is byte for byte the 7.1 answer of 12 Oct. The tool reads the dated mailing details only with `include_mailing_details` true (a 1,792-byte party-master read); with it the probe's entries come back as keyed, and `mask_parties` masks the name and address lines only. A second dated entry (1 Jul 2026) comes back from Tally and from the tool, oldest first, the same at both period ends and both `as_of` dates. Tally would not hold an entry without a date: the screen dropped the row. `GSTDETAILS.LIST` is empty on this book, so the combined lists were not shown. The restore of BK1 returned the book to step 1. Not established: the book's last voucher date is 7 Aug 2026, not 31 Aug, so no raw file here is "the tool's request"

The sheet is `lab/1490-mailing-captures` at 3e0722198b632ea60bc43af3b4267c1825f37963. `SHA256.txt` 12 of 12 equal; `gen_requests --check`, the census controls and the census printed exactly the EXPECT lines.

- **Build:** #1675's branch `feat/1490-party-mailing-details` at 9045b9f012c06ee89665bb3419871ffa2a9e6ad3, Bridge CI run 40641009295, artifact 12795987228 `unsigned-bundle-smoke-windows-latest`; its `mcpb-smoke.json` carries the exe's sha256 below and archive_sha256 f21e71f2. (Corrected 29 Oct: the first record named the sheet's head d6d89545 and run 40555011762, which is not the build that ran; d6d89545 has no `include_mailing_details`.)
  - `bridge_mcp.exe` sha256 0d17a112dd7f7e349afeeae8dcfe5aba38ec2cb8c0e1ae7b45e0880c455bc345, equal to the smoke file's `binary_sha256` and the same before and after.
  - The server reports 0.6.1.
- **Tally:** release 7.2 (About screen), Silver, licensed, `education_mode` false.
- **Company:** Bridge Lab Win 0.5.1, number 100031, `<company guid>`, `verified_tuple`, books from 1 Apr 2026, loaded alone.
- **Marks:** ALTMSTID / ALTVCHID 217 / 13 at step 1 (M0, M0′), 218 / 13 after step 5 (M1), 218 / 13 after step 7, and 217 / 13 after the restore. `reused` false every time.
- **Dialogs:** none.

## The book's dates (step 1)
- **Gateway:** period 1-Apr-26 to 31-Mar-27; current date 7-Aug-2026; last entry 7-Aug-26.
- **Day Book for the year:** 7 vouchers, the last on 7-Aug-26.

**Differs from EXPECT (31 Aug 2026).** `gen_requests.py` accepts a period end whose day is 1, 2 or 31 only, so it cannot write a file for 7 Aug. The committed files were sent as they are. None of them is called the tool's request.

The request sizes still match by bytes:
- the tool's mailing read is 1,792 B, the size of `q-to-20260831`;
- its answer is 20,896 B, the size of the raw answer.

The period the tool named was not seen.

## C5: the 4c read on 7.2 (step 2)
`q-to-20260831` answered:
- 200, 20,896 B;
- sha256 ceaa6d789d1ce9a41c7bcf233e5d26cbe383dcd261e57fb804935d8845cc75d5.

These are **the same bytes as the 12 Oct answer on 7.1** (`lab/1490-mailing-answers` 4c), so the structure is the same. It has 8 ledgers, each with one `LEDMAILINGDETAILS.LIST`.

The probe has one mailing entry:
- `APPLICABLEFROM` 20260401;
- `ADDRESS.LIST` "1 Test Street", "Test Area";
- `PINCODE` 560001, `STATE` Karnataka, `COUNTRY` India;
- `MAILINGNAME` Lab Party Duplicate Probe.

The other ledgers hold whitespace placeholders. The probe has two `LEDGSTREGDETAILS.LIST` entries (20260401 and 20260701).

The flat fields carry nothing:
- `LEDADDRESS.LIST` is a placeholder;
- `LEDPINCODE` and `LEDGSTPINCODE` are empty;
- there is no `STATENAME`.

## C7: the tool on the book (step 3)
| call | answer | party-master send |
|---|---|---|
| 005: `compliance`, `as_of` 20260831, as the sheet writes it | result; **no `mailing_details` key**: this build names the mailing details only with `include_mailing_details` | 1,744 B, answer 19,094 B (the 4b sizes) |
| 007: the same plus `include_mailing_details` true | the probe: `observation` `entries`, one entry: `applicable_from` 20260401, `address_lines` ["1 Test Street","Test Area"], `mailing_name` Lab Party Duplicate Probe, `pin_code` 560001, `state` Karnataka, `country` India, `other_content_skipped` false; the other seven `entries` []; flat `address` "", `pin_code` "", `gst_pin_code` "", `state` null | **1,792 B** (sent twice), answer 20,896 B |
| 009: 007 under `mask_parties` | `mailing_name` "La…be", `address_lines` "1 …et", "Te…ea"; ledger names masked ("Cash" → "…"); PIN code, state and country as in 007; flat `address` "" as in 007 | 1,792 B, redaction `mask_parties` |

The masking is as EXPECTED: the first two characters, an ellipsis and the last two, and a value of four characters or fewer becomes an ellipsis alone.

## C4: the combined read (step 4)
- **Raw `q-gst-to-20260831`:** a NEW SHAPE, sent for the first time.
  - 200, 23,568 B, which is 2,946 B a ledger against the pull request's estimate of 8,950.
  - No dialog afterwards.
  - **`GSTDETAILS.LIST` is a whitespace placeholder on all 8 ledgers, the probe included.**
- **Tool call 011** (`compliance`, `include_gst_details` true, `as_of` 20260831):
  - every row has `gst_details` `entries` [];
  - `gst_registrations` sits beside it (the probe's two entries);
  - there is no `mailing_details`.
  - The party-master send was **1,884 B, not the EXPECTED 1,932**, because this build's GST read does not name the mailing details. The answer was 21,766 B.
- **Tool call 013** (both flags): refused before any send with `ledger_masters_mailing_details_with_gst_details_not_captured`.

**This book does not show the two lists combined. The pull request's GST estimate is not tested.**

Bytes a ledger against the estimates:

| read | bytes a ledger | estimate |
|---|---|---|
| raw mailing read | 2,612 | 4,700 |
| raw combined read | 2,946 | 8,950 |
| tool's GST-only read | 2,721 | none |

## C1, C2: a second dated entry (steps 5 and 6)
**How it was keyed.** The entry was added under Alter Ledger > More Details > Show More > **Mailing Details (History)**, a table with these columns:
- Applicable From
- Name
- Update Address (Yes opens the address lines)
- State
- Country
- Pincode

The table's Name column took the ledger's mailing name and could not be edited there. The new entry's name was keyed on the main screen, which showed the 1-Jul entry once the table was accepted. The history then read:
- 1-Apr-26: Lab Party Duplicate Probe, 560001;
- 1-Jul-26: Lab Mailing Probe Two, 560002.

M1 was 218, above 217. V1 stayed 13.

| read | the probe's mailing list | other |
|---|---|---|
| raw `q-to-20260831` | **both entries**, 20260401 then 20260701, dates as YYYYMMDD | 21,840 B, sha256 b1ad4409… |
| raw `q-to-20260601` | the same, **byte for byte the same answer** | the period end does not change Tally's list |
| tool `as_of` 20260831 (016) | both entries, oldest first: 20260401 (Lab Party Duplicate Probe, 1 Test Street / Test Area, 560001), 20260701 (Lab Mailing Probe Two, 2 Probe Road / Second Floor, 560002) | `party_gstin_as_of` 20260831, `party_gstin` …Z2ZL |
| tool `as_of` 20260601 (017) | **equal to 016** | `party_gstin_as_of` 20260601, `party_gstin` …Z1ZM |

As EXPECTED, `as_of` moves only the GSTIN in force, not the mailing list. The marks did not move between (a) and (b).

## C3: an entry with no Applicable From (step 7)
**Tally does not hold a dateless entry, and gives no message.** The steps were:
1. The second row's date was cleared.
2. The table was accepted with Ctrl+A.
3. The table closed, with no message.
4. Reopened, it held only the 1-Apr-26 row: the screen had dropped the dateless row.

The ledger was not saved: the lab left with Ctrl+Q, and no prompt appeared. The marks stayed 218 / 13. Because no dateless entry exists, the `entry_without_date` defect was **not reached** on a live answer.

## Restore (step 8)
BK1 was restored by C6:
- source: TDBK1800_100031.001, 33,318 B, sha256 81f678c3…;
- the list showed 29-Oct-26 6:08 am;
- prompts: Accept Yes, then Replace.

After the restore:
- the marks were 217 / 13, which equals M0′;
- the 8 ledger rows equal step 1's;
- raw `q-to-20260831` is byte for byte step 2's answer (ceaa6d78…), with one entry.

## Not run, not measured
- **Step 9 (C8, the workbook):** not run. The package holds the desktop app only as installers.
- **Step 10 (C6, size on a populated book):** not measured. No book was named.
- Gold, other releases, a party with more than two entries, and `other_content_skipped`: not covered.

**Confidence:** one sitting, one synthetic book: PARTIAL until repeated.

## Files
- `answers/`: every tool answer whole, 000 to 020, with UTC send times and seconds.
- `raw/`: the five raw answers with their headers, `send-log.txt`, and a structure print of each, made from the originals.
- `client-log.txt`.
- `notes.txt` (UTC).
- `checks/`:
  - the process list, the port 9001 lines of netstat, the port listener and `/status` at step 0 and at the close;
  - `restore-plan.txt`.
- `substitutions.txt`: in the pushed copies, the company GUID is `<company guid>` (`&lt;company guid&gt;` inside the raw XML, so that it still parses) and the lab folder is `<lab folder>`. Each original is listed by bytes and sha256. The originals stay on the lab PC.
- `SCREENSHOTS.txt`: the captures, the backup and the folder listings by sha256. They stay on the lab PC.
