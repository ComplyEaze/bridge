# Results: #1378 read row, `include_e_invoice` through a build, 2 Nov 2026

**Every EXPECT holds. With `include_e_invoice`, invoice A reports its IRN, acknowledgement number and date as `present` with the typed values, invoice B reports all three `absent`, and the Payment carries no `e_invoice` key. Without it, no row carries the key. Nothing else on a row changes, no read errored, no mark moved, and `not_reported` did not fire.**

Build: head c078b0ff on fix/1378-read-irn, `bridge_mcp.exe` sha256 acddff0ce837d1654a6853458b4f16f6248c2c6670fb425aee9bb76c195d6ec5, equal to the package's smoke file, before and after. The branch tip after it adds an ack file only: checked, one file changed. No write setting on. BRIDGE EINVOICE LAB, TallyPrime 7.2 Silver, licensed, loaded alone. Nothing touched the portal.

## Set-up
- Invoices A and B keyed again by the path in row 1, same customer, 1,000.00 + 180.00 IGST, 3 April 2025. A: the invented IRN, acknowledgement number 112233, date 3-Apr-25. B: none. A saved with no prompt; B drew "Do you want to generate e-Invoice?", declined.
- One Payment by direct XML the same day, 50.00, Dr BRIDGE IRN Customer, Cr Cash, set-up only.
- The GSTINs are invented and masked as 15 X in what is pushed here; originals by hash in `kept-local.txt`.

The build ran through a loopback relay on 9002 that passes every byte and saved each request. No request was changed; no answer was held.

## Steps
| step | call | `isError` | evidence | seconds |
|---|---|---|---|---|
| 1 | marks, `list_companies` | false | | 0.20 |
| 2 | `vouchers` 3 April 2025 | false | complete | 2.16 |
| 3 | the same, `include_e_invoice` true | false | complete | 2.31 |
| 4 | step 3 again | false | complete | 2.11 |
| 5 | marks | | | |

Marks V 3, M 215; steps 1 and 5 are the same bytes. `ALTERID` 1, 2 and 3 for A, B and the Payment. [verified]

## Step 2
All three rows; no `e_invoice` key on any. [verified]

## Step 3
| row | `is_invoice` | `irn` | `ack_no` | `ack_date` |
|---|---|---|---|---|
| Sales 1, A | true | present, a1b2c3d4e5f6a7b8 four times, 64 characters | present, 112233 | present, 20250403 |
| Sales 2, B | true | absent | absent | absent |
| Payment 1 | false | no `e_invoice` key | | |

Each value equals what was typed on the `e-Invoice Details` screen, and `ack_date` is YYYYMMDD. Every other key of each row equals step 2's. `not_reported` appears nowhere in the answer. [verified]

**The request as sent**, `relay/req-e0702cbb980e35f7.bin`, 1,794 B, UTF-16LE with a BOM. Its FETCH, read from the bytes, is exactly:

`DATE,VOUCHERNUMBER,VOUCHERTYPENAME,PARTYLEDGERNAME,NARRATION,GUID,ALTERID,MASTERID,ISCANCELLED,ISOPTIONAL,ISPOSTDATED,ALLLEDGERENTRIES.*,REFERENCE,ISINVOICE,PARTYGSTIN,REFERENCEDATE,IRN,IRNACKNO,IRNACKDATE`

Step 2's request, `relay/req-b7b2380def07cd2e.bin`, ends at `REFERENCEDATE`. The relay log shows 4 sends of the first over steps 3 and 4, and 2 of the second over step 2: a paired read, twice per call. [verified]

## Step 4
Not byte-equal as saved, and the only differences are time and identity of the read: `read_at` (three places), `evidence/duration_ms`, three `ms` timings under `window`, and `snapshot/id`. Every row is equal. All are listed in `evidence/check.txt`. [verified]

## Left behind
BRIDGE EINVOICE LAB now holds the two invoices and the Payment, V 3, M 215: no restore was asked for. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** one synthetic book, values keyed by hand, one sitting: PARTIAL. A voucher the portal issued an IRN for is not read here.
