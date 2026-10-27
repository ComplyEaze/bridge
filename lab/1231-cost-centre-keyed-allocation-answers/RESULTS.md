# Results: #1231 cost-centre allocations keyed on Tally's screen, TallyPrime 7.2 Silver, 27 Oct 2026 (UTC)

The sheet is `lab/1231-cost-centre-keyed-allocation/RUN-SHEET.md` at 23bca8b9 (SHA256.txt 9 of 9 equal; census ok 5). The company is BRIDGE COST KEY LAB, new and synthetic: number 100088, GUID ddf3a557-704c-4f61-ae8d-46a3380062a1, books from 1 April 2025. It was created, set up and keyed by the lab on Tally's screen. Raw reads used the sheet's five requests by curl. The built-tool reads used build 609776ae (bridge_mcp.exe sha256 4c9bace3...df44), which holds #1707.

## What was captured
**1. The book's setting is Yes.** The F11 field is "Enable Cost Centres" (Company Features Alteration > Accounting). It was set from No to Yes and nothing else changed. `company_features` reads `cost_centres: "yes"`.

**2. A centre with a parent, and a second category.** The raw cost-centre read gives:

| NAME | PARENT | CATEGORY |
|---|---|---|
| CK North | `&#4; Primary` | Primary Cost Category |
| CK North East | CK North | Primary Cost Category |
| CK South | `&#4; Primary` | Primary Cost Category |
| CK Retail | `&#4; Primary` | CK Region |

The raw categories are CK Region (ALLOCATEREVENUE Yes, ALLOCATENONREVENUE No) and Primary Cost Category (Yes, Yes).

**3. K1, a Payment keyed on the screen, 15 May 2025, one category with two centres.** On the CK Expense entry (AMOUNT -1000.00, ISDEEMEDPOSITIVE Yes) there is one `CATEGORYALLOCATIONS.LIST` with CATEGORY Primary Cost Category and ISDEEMEDPOSITIVE Yes. Inside it are two `COSTCENTREALLOCATIONS.LIST`:
- CK North, AMOUNT -600.00
- CK North East, AMOUNT -400.00

Each also carries PAYHEADSORTORDER 0 and EMPLOYEESORTORDER 0. The CK Bank entry carries no category or cost-centre allocation.

**4. K2, a Journal keyed on the screen, 16 May 2025, two categories on one entry.** On the CK Expense entry (-500.00, Yes) there are two `CATEGORYALLOCATIONS.LIST`:
- Primary Cost Category, holding CK South -500.00
- CK Region, holding CK Retail -500.00

The CK Bank entry carries none.

**5. The built tool against the raw reads.** `vouchers` for 2025-05-15..16 (state complete, total 2) returns `cost_centre_allocations` on each CK Expense entry, equal to the raw allocations:
- K1: Primary Cost Category / CK North -600.00 and Primary Cost Category / CK North East -400.00.
- K2: Primary Cost Category / CK South -500.00 and CK Region / CK Retail -500.00.
- There is no `cost_centre_allocations` on the CK Bank entries, and no `cost_centre_allocations_unreadable` anywhere.

`masters` gives these rows:
- `cost_centres`: total 4, with the same names, categories and the CK North East parent CK North.
- `cost_categories`: total 2, with the same allocate flags.

**6. FINDING: the masters read renders the root parent as U+FFFD followed by `#4; Primary`.** For the three top-level centres (CK North, CK South, CK Retail) Tally's raw answer holds PARENT `&#4; Primary`, and `masters` (kind `cost_centres`) returns `parent` as the replacement character U+FFFD followed by `#4; Primary` (`answers/002-masters-cost-centres.json`). CK North East's parent reads `CK North` in both.

## Observed on the screen (7.2)
- **The ledger screen has no cost-centre field, but the voucher still allocates.** Ledger Creation for CK Expense under Indirect Expenses showed no "Cost centres are applicable" field, nor one under any other name. The ledger was saved as offered. On the Payment, its Dr line still opened "Cost Centre Allocations", and that screen offered a second category on one entry.
- **The masters list.** Cost Category appears in Create's List of Masters only after Show More. Choosing it asked "Primary Cost Category exists by default ... alter Primary Cost Category or create a new Cost Category?" (Create New was chosen).
- **The Journal's ledger list leaves out bank and cash.** On K2's To line the list offered only CK Expense and Profit & Loss A/c until Show More added Cash and CK Bank.
- **A prefilled allocation.** When K2's allocation screen opened, Tally had already filled in Primary Cost Category / CK North 500.00 (the first centre used on K1). It was changed to CK South before anything else was keyed.
- **Bank Allocations.** On both vouchers the Cr CK Bank amount opened "Bank Allocations for: CK Bank" (Cheque, the amount, Inst Date the voucher date). The offered values were accepted with Enter only.
- **The masters tool's parent text.** For the three top-level centres, `masters` returns `parent` as U+FFFD followed by `#4; Primary`, where Tally's raw answer holds `&#4; Primary`.

## Control and book state
- **Marks.**
  - After creation: ALTMSTID 207, no voucher mark. F11 took it to 208, the category to 209, the four centres to 213, and the two ledgers to 215 (M6).
  - The vouchers set ALTVCHID 2 (M7), which is above M6 by two, as expected.
  - M8 after every read is byte-identical to M7.
- **Backups** (they stay on the lab PC):
  - BK0 (empty book): `TDBK1800_100088.001`, 23,415 B, sha256 5acbfb5c221872e0606965207967bafa8bdb194da898a3640b1658e19e3663bf.
  - BK1 (before vouchers): `TDBK1800_100088.001`, 24,505 B, sha256 0f82213e9181ee2db758857cad843ede4b2b825e045a5dc0ec4e4294cb310f64.
- **The book is left as step 7 made it**, with two vouchers, and is shut.

## PASS / FAIL
Every read came back whole: 200, CONTENT-LENGTH equal to the bytes, each answer parsed, and each one below its bound. No tool call was refused. Two vouchers were keyed, as the sheet expects. Every screen or prompt the sheet does not name was stopped on, with no key pressed, and then handled as decided at each stop:
- the cost-category prompt (Create New)
- the missing ledger field (saved as offered)
- the Bank Allocations sub-screen (accepted with Enter)
- the Journal's ledger list (Show More)

## Files
- `evidence/`: the four curl answers with their headers, and `tables.txt` (the parsed tables).
- `marks/`: every MARKS answer with its headers.
- `answers/`: every tool answer, and `client-log.txt`.
- `send-log.txt`: every curl send.
- The read-only lab helpers (MARKS and curl sends, the stdio client, the parser) stay on the lab PC.
- `notes.txt`: UTC, one line per step.
- `SCREENSHOTS.txt`: the captures and both backups, by name, bytes and sha256. They stay on the lab PC.
