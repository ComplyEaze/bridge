# Results: row 1761-S4 v2, a bank cash withdrawal paid to someone, on a book with one seed voucher, 2026-11-11

**Outcome: a stop at step 0.7, while keying the seed, and a FINDING for the sheet. Steps 0.8 to 6 were not run.**
- **What stopped the row:** the seed Receipt keyed to BRIDGE Bank B opened a sub-screen titled Bank Allocations on TallyPrime 7.2. The sheet's step 0.7 counts any extra screen during the seed as a stop.
- **Nothing was saved.** The voucher screen was left by Ctrl+Q, as the run lead decided, and Tally went back to the Gateway with no prompt. The marks read afterwards are the step 0 marks (M 214, V null), and the Gateway still read No Vouchers Entered.
- **No restore was needed or made.** The book is as row 1761-S4 left it: the masters only.
- **The run lead ended the row at this stop.** The seed is keyed again only in a later sitting, under a new go-ahead.

Steps 0.1 to 0.6 each PASS (below). Nothing was sent to Tally but reads, and no approval window opened.

**Who acted.** The lab did everything on the owner's standing word for synthetic books (2026-10-27 and 2026-10-28):
- loading the book;
- the launch with the posting setting present;
- keying the seed as far as the stop;
- leaving the voucher screen unsaved;
- the close.

The go-ahead, the decision at the stop and the route back came from the run lead in chat.

## FINDING for the sheet

On TallyPrime 7.2, a Receipt keyed by hand with a ledger under Bank Accounts on its debit side opens a **Bank Allocations** sub-screen once that line's amount is accepted. Its fields, as shown (shots/s0-seed-bank-alloc.png, kept on the lab PC):
- "Bank Allocations for: BRIDGE Bank B", "For: 1,000.00";
- Transaction Type `Cheque/DD`, Amount 1,000.00;
- Inst No. empty, Inst Date 1-Apr-25, Bank Name empty;
- side buttons `S: Set Status` and `V: View Recon Info`.

The sheet's step 0.7 names no such screen, so it reads as "any extra screen", a stop. A later sheet's seed step should either name this screen and say how it is answered, or use ledgers that do not open it. Either way, the seed stays clear of Cash and BRIDGE Bank A.

## Record

- **Sheet:** `lab/1761-s4v2-cash-paid-to-someone-sheet` at `cae1213d3c49e655377c4439ded0a71ade8337fc`, queue item 91 on `lab/queue` at `fd9328c4`. The sheet's three files are equal to its SHA256.txt. The row's copy of `statement.csv` has sha256 `ebca906726de102cb23dcf15767de2cee5bba0ff4c401debd6e7b9fb3bf776ff`, equal.
- **Charset check:** the run lead asked for a check of the sheet for any raw send of a Bridge-saved file under a mismatched charset. There is none: the sheet sends nothing by curl but C2's GET /status, and its only write is Bridge's own `post_import`.
- **Build:** the sheet's build line, the Windows bundle of the merge-group run at master `de54a0f26e554f574727a0bc4914b0ad6ca88583`. The three hashes are in evidence/build-hashes.txt.
  - `bridge_mcp.exe` sha256 `be931eb0bd0228d5765886491970da712cb91941bc2e0a980049107c4d8267ce` was hashed again by the client before the launch.
  - The server reported `bridge-tally` 0.7.0.
- **Tally:** TallyPrime 7.2 (`tally.exe` 1.1.7.2), the TallyPrime single-user edition, licensed. No Educational mark showed (C1). The PC date was 2026-11-11.
- **Company:** BRIDGE CASHPAY LAB, synthetic, GUID `b52de336-8552-4d40-9221-60aa64d3c1f1`, row 1761-S4's book. It is not the excluded GUID.
  - At step 0 the Gateway showed the period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025, and No Vouchers Entered.
- **Launch:** one stdio launch by the lab's own client, from step 0 to the close (server pid 13388, exit code 0). The launch file's keys:
  - `BRIDGE_AGENT_DATA_DIR` (new and empty);
  - `BRIDGE_AGENT_ENABLE_WRITES=true`, set by the lab on the owner's word of 27 and 28 Oct, synthetic books only;
  - `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT`, `BRIDGE_TERMS_ACCEPTED`.

  `BRIDGE_AGENT_REDACTION`, `BRIDGE_AGENT_MAX_BYTES` and `BRIDGE_AGENT_ENABLE_BATCH_POST` were absent.

## Checks

| step | what | result |
| --- | --- | --- |
| before | sheet files equal to SHA256.txt; the statement copy equal; the row folder absent before the row | PASS |
| 0.1 | C2: one tally.exe, PID 22572, owning 9001; GET /status 200; THE COMPANY loaded alone; exe hash equal to the build line | PASS |
| 0.2 | `tools/list` (answers-mcp/answers/001): 37 tools; `parse_bank_statement_csv` lists `cash_ledger` under `cash_answers` items; `post_import` present | PASS |
| 0.3 | `list_companies`: exactly BRIDGE CASHPAY LAB, its GUID, `verified_tuple` | PASS |
| 0.4 | MARKS: M0 214, V0 null, reused false | PASS |
| 0.5 | `masters` voucher_types: Contra and Payment `series_numbering` `observed`, `automatic`, `auto_retain` | PASS |
| 0.6 | the fallback (row 1761-S4's BK1): its folder holds one file, 24241 B, sha256 `bce8c06cc0dcf344bc4408e34e0c8374b9920518387552e916e44e0fc4c23986`, equal to that row's record | PASS |
| 0.7 | the seed, keyed: Bank Allocations sub-screen opened | **stop (FINDING above)** |
| 0.7 back | the voucher left unsaved; Gateway: No Vouchers Entered, no dialog; MARKS once (C8): M 214, V null, reused false | the book unchanged |
| C10 | client closed (exit code 0); Shut Company, no prompt; one tally.exe owning 9001; GET /status 200; no company loaded | PASS |

## Step 0.7, as keyed (evidence/notes.txt)

1. From the Gateway, Vouchers, then F6: Accounting Voucher Creation, Receipt No. 1, in the double-entry layout (To and By lines).
   - The header date read 1-Apr-25, Tuesday: the date the voucher opened with.
   - F2 (Change Voucher Date) showed `2-4-2025` in its field. `1-4-2025` was typed and Enter pressed; the header kept 1-Apr-25.
2. To BRIDGE Capital, Credit 1000 (its current balance then showed 51,000.00 Cr on the screen, unsaved).
3. By BRIDGE Bank B; the Debit amount 1,000.00 came up filled; Enter.
4. The Bank Allocations sub-screen opened (the FINDING above). Capture: shots/s0-seed-bank-alloc.png, 38441 B, sha256 `f3180038f7c6492f28b9e6e438b5cc12ec183f8929e70343e0a7437a990355d4`, kept on the lab PC. No key was pressed until the run lead's decision. The narration was not reached, and the voucher was never accepted.

**The route back (the run lead's decision (b), under C7 and C8):**
1. A screenshot of the sub-screen with its title, then Ctrl+Q once: back to Accounting Voucher Creation with both lines unsaved.
2. A screenshot with that title, then Ctrl+Q once: Tally went straight to the Gateway. No quit prompt appeared, so none was answered. Escape was never pressed.
3. The Gateway showed BRIDGE CASHPAY LAB as the one company, current date Tuesday, 1-Apr-2025, No Vouchers Entered, no dialog (shots/s07-after-quit.png, sha256 `e385753b301fd685e5715f472a6559350954519b837163278006d5c17e8e2670`).
4. GET /status: curl exit=0, status=200. Then MARKS once (answers-mcp/answers/005-s07-marks.json): M 214, V null, reused false, equal to M0 and V0.

So nothing was saved, and no restore was ordered.

## The book after the row

The masters only, as row 1761-S4 left it and as at step 0: M 214, V null. No backup was taken in this row, because the sheet takes BK1 at step 0.10, after the seed. The fallback (row 1761-S4's BK1) was checked but not used.

## Deviations

1. **answers-mcp/answers/000 is a mislabelled call.** The lab meant `tools/list` but sent it as a tool call to a tool named `tools/list`. The server refused it with `tool_not_found` (code -32602), and nothing reached Tally. answers-mcp/answers/001 is the `tools/list` itself.
2. **The seed's screen layout.** The voucher screen opened in the double-entry layout, so the sheet's "Account BRIDGE Bank B, Particulars BRIDGE Capital" was keyed as To BRIDGE Capital (credit) and By BRIDGE Bank B (debit): the same voucher, in the layout Tally showed. The mode was not changed.

## Not established

- Everything from step 0.8 on: the seed's marks and read, BK1, the parse, the build refusal, the batch, the post, the read-back and the restore.
- Whether a saved seed would move M.
- What the Bank Allocations screen would have saved had it been accepted. Nothing was accepted.

## Files

- **answers-mcp/:** every tool answer whole (company_number redacted where present), the calls sent, the client's log and `initialize`.
- **args/:** the call arguments.
- **data/:** the server's data folder, whole.
- **evidence/:**
  - notes.txt, one line per step, UTC;
  - fallback-check.txt;
  - build-hashes.txt;
  - the port listings cut to the port-9001 LISTENING lines.
- **statement.csv:** the row's copy.
- **scripts/:** the lab's helpers.
- **kept-local.txt and SCREENSHOTS.txt:** what stays on the lab PC, by size and sha256.
- **substitutions.txt:** what changed in pushed copies of the lab's own text.
