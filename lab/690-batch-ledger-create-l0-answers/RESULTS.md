# Results: #690 L0, three ledgers created in one import request, and one name colliding in a batch, 29 Oct 2026. **Request A counted `CREATED 3` and nothing else; the master mark stepped by exactly 3 (208 to 211) and the three new ledgers took ALTERIDs 209, 210 and 211 in request order. Request B (two new names and one existing) counted `CREATED 2, ALTERED 1`: the existing ledger kept its GUID and its ALTERID moved (210 to 213) although no field changed; the mark stepped by 3 again (211 to 214), each master taking the mark's next value in request order. Nothing else moved. BK0 restored; the book reads back byte-identical to step 0.**

| question (the sheet's table) | answer | from |
|---|---|---|
| One request of three new ledgers counts `CREATED 3` and nothing else? | **yes**: CREATED 3; ALTERED, DELETED, COMBINED, IGNORED, ERRORS, CANCELLED, EXCEPTIONS 0; LASTMID 0, LASTVCHID 0; no LINEERROR | step 1 |
| `ALTMSTID` steps by exactly 3 (M1 = M0 + 3)? | **yes**: M0 208, M1 211 | steps 0, 2 |
| The three new ledgers take M0+1, M0+2, M0+3 in request order? | **yes**: Alpha 209, Bravo 210, Charlie 211 (GUID suffixes ...cf, ...d0, ...d1) | step 2 |
| Every ledger and group of step 0 unchanged? | **yes**: the 3 ledgers and 28 groups keep their GUIDs and ALTERIDs | steps 0, 2 |
| With one name already a ledger, does Tally create two and alter one, or refuse? | **creates two and alters one**: CREATED 2, ALTERED 1, every other counter 0, no LINEERROR | step 3 |
| Does the altered ledger keep its GUID, and does an alteration that changes no field move its ALTERID and the mark? | **keeps its GUID (...d0); its ALTERID moves 210 to 213, and the mark moves** | steps 2, 4 |
| Does the mark step by 3 for request B? | **yes**: M1 211, M2 214 | steps 2, 4 |
| On 7.2, does each master the requests write take the mark's next value? | **yes, in request order**: Delta 212, Bravo 213 (altered), Echo 214; groups unchanged | steps 0, 2, 4 |

By the sheet: "ComplyEaze Bridge builds one approval for several ledgers only if request A shows `CREATED 3`, a step of 3 and the span M0+1 to M0+3": all three hold, on one sitting. Request B adds that a batch carrying an existing ledger's name is not refused: Tally alters that ledger in place (here with identical fields), and the alteration takes a mark value in its place in the order.

## Steps and checks
- **BK0** (before step 0): Tally's Backup with the company only selected, to a new empty folder: `TDBK1800_100092.001`, 23,613 B, sha256 0385db092fbf3e48f5197b7863b0abca8d006ebdd5fe6f7e8e3b207417099dd8.
- **The company:** BRIDGE LEDGERBOX CI LAB (100092), GUID 5848d0ca-ed85-4c97-ab20-e045893a9803, loaded alone; licensed TallyPrime 7.2 Silver; Tally's date Tuesday 1-Apr-2025; no vouchers.
- **Request files:** 6 of 6 equal to `SHA256.txt` before each send; each sent once with `Content-Type: text/xml; charset=utf-16` and the answer written with `--output`; every answer parsed with an XML parser (`evidence/log.txt` has each request's times, size and sha256, and each answer's size and sha256).
- **Before each request:** Tally's Gateway with no dialog (captured).
- **Step 6:** THE COMPANY shut; Tally's Restore from BK0's folder (the list showed the company with the backup's date and time, 29-Oct-26 11:40 pm local = 18:10 UTC); the company selected only; "Accept ?" Yes and "Company data folder (100092) already exists ... Replace" by mouse; no other prompt. Data-folder listings before and after kept.
- **Step 7:** marks 208 and the ledger list are byte-identical answers to step 0's (sha256 83dda085... and a27a54e8...); no `L0 Batch` ledger. PASS.
- The company was shut at the end; no request after the shut.

## Deviations
1. **Step 6 by the lab**, not the owner: the restore was made by the lab on the owner's standing word on synthetic books (27-28 Oct).
2. **Aliases** at step 0 were not read separately: the ledger catalogue the sheet sends returned three ledgers, none an `L0 Batch` name.

**Confidence:** one sitting, one synthetic book, TallyPrime 7.2 Silver: PARTIAL until repeated.

**Who acted.** The lab, on the owner's standing word for synthetic books. The sheet is `lab/690-batch-ledger-create-l0` at 83a5c98970037d079bca7f4b9962743b31d9a8f0.
