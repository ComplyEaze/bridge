# Results: #1761 live row, a cash deposit from the cash box against the cash book, 3 Nov 2026

**Every EXPECT holds. The build projects Cash's lowest day-end at 200 on 25 April and saves the batch; 400.00 is refused at -100 on 25 April; a deposit on 5 April is refused on the book's own -300 that day; a Payment added after the build stops the post as `cash_projection_stale`, with no window and no voucher; once it is deleted the same batch reaches the window and posts `posted_verified`. Tally's screen then shows the Contra and Cash at 200.00 Dr.**

Build: head 1a364052 on fix/1761-cash-projection, `bridge_mcp.exe` sha256 90000d448c551070c98a8400353cc31330a0da795e0eb290b7882f5a83286f40, equal to the package's smoke file, before and after. The branch tip after it adds an ack file only: checked, one file changed. Posting on. BRIDGE CASHDAY LAB as row s1 left it, TallyPrime 7.2 Silver, licensed, loaded alone.

The build ran through a loopback relay on 9002 that passes every byte and saved each request and answer. No request was changed; no answer was held.

## The statements
The row asks for a statement any format `parse_bank_statement` reads, with one credit row narrated CASH DEPOSIT. `parse_bank_statement` reads PDF only, and `parse_bank_statement_csv` proposes nothing yet, so each step has a one-row, one-page PDF in the Union Bank shape, written by the repository's fixture generator as it stood at 4ebb9ea9 (removed from the repository since), imported unchanged. Every name, digit and amount is invented.

The row's narration is `BY CASH`, not CASH DEPOSIT: that is the printed wording the Union Bank profile names as the party `CASH DEPOSIT`, and the answers give `printed_as` CASH DEPOSIT, `movement` deposit. [verified]

Opening 12,000.00, with the closing balance the deposit puts on it. `suspense_ledger` was given as `Suspense`, which this book does not have; the build warns `suspense_ledger_not_in_book` and says no voucher uses it. [verified]

Two calls, `002` and `003`, were refused `statement_password_file_permissions`. Both are the lab's own set-up: the file inherited wider access. Access set as on row 1628, the next call read. Neither reached Tally.

## Steps
| step | call | `isError` | answer | seconds |
|---|---|---|---|---|
| 1 | marks, Trial Balance screen, `list_companies` | false | V 11, M 214 | 0.25 |
| 2 | build, 100.00, 12 April | false | batch saved, `cash_projection` below | 10.20 |
| 3 | build, 400.00, 12 April | false | `state` refused, `cash_projection_negative`, Cash, 20250425, -100 | 6.96 |
| 4 | build, 100.00, 5 April | false | `state` refused, `cash_book_already_negative`, Cash, 20250405, -300 | 5.55 |
| 5 | Payment by direct XML, `post_import` | true | `cash_projection_stale`, `attempt_recorded` false | 7.16 |
| 6 | Payment deleted, `post_import` | false | window, Yes, `approval` approved | 26.90, 15.5 of them the window |
| 6 | `post_import` again | false | `posted_verified`, created 1 | 16.49 |
| 7 | marks, Trial Balance screen | | V 15, M 214 | |

The refused builds answer `isError` false with `state` refused; the stale post answers `isError` true. All as saved. [verified]

## Step 2
`cash_projection`: from 20250412 to 20260331, `lowest_day_end_after_deposits` Cash, 20250425, 200. The saved batch's line in `agent/agent-import-ledger.jsonl` holds `cash_projection` with `ledgers` [Cash], status built. The file holds one Contra, 12 April, BRIDGE Bank A Dr, Cash Cr, 100.00. Nothing was posted: the marks moved only with the Payment until the post, and the relay holds one Import, `req-0569`, in step 6. [verified]

**The Trial Balance with `from` off the books' first day tied.** `relay/req-0054.bin` asks 20250412 to 20260331; `relay/ans-0054.bin` gives Cash opening -1200.00, credits 900.00, closing -300.00, debit negative, so 1,200.00 Dr on 12 April and 300.00 Dr at the end, as the book's table has it. The ledger voucher read in `relay/ans-0072.bin` covers the same days. `0056` and `0074` are the second of each paired read, byte-equal. `cash_projection_not_tied` did not fire. [verified]

## Step 5
Payment created by direct XML, 13 April, BRIDGE Office Expenses Dr, Cash Cr, 50.00: created 1, set-up only. The post took 7.16 s and answered `cash_projection_stale`. No window was seen in 45 s (`box/s5-box.txt`). The marks are equal before and after the post, V 12. The relay shows no Import in that call. [verified]

## Step 6
Payment deleted on the Day Book screen, 13 April, whose list is then empty. V 14 after the delete. The same batch reached the window (`box/s6-box.txt`): one Contra, 12-Apr-2025, Dr 100.00 BRIDGE Bank A, Cr 100.00 Cash. Yes clicked. The first call returns the approval, and the second posts, as the tool's next step says.

On screen after the post: Contra No. 4, 12-Apr-25, By BRIDGE Bank A 100.00, To Cash 100.00, current balances 12,100.00 Dr and 1,100.00 Dr. `alter_id_delta` 1, created 1, errors 0. [verified]

The sent voucher differs from the saved file in two ways: a fresh REMOTEID, and no `[BRIDGE:…]` tag in its narration. A comment in `agent_import_post.rs` at this head names both as the native post's design. The book-facing fields are equal. [verified]

## Step 7
Trial Balance screen, ledger-wise, 1 to 30 April:

| ledger | before, Dr | before, Cr | after, Dr | after, Cr |
|---|---|---|---|---|
| BRIDGE Bank A | 12,000.00 | | 12,100.00 | |
| BRIDGE Capital | | 11,000.00 | | 11,000.00 |
| BRIDGE Customer A | | 5,000.00 | | 5,000.00 |
| BRIDGE Office Expenses | 3,400.00 | | 3,400.00 | |
| BRIDGE Petty Cash | 300.00 | | 300.00 | |
| Cash | 300.00 | | 200.00 | |
| total | 16,000.00 | 16,000.00 | 16,000.00 | 16,000.00 |

[verified]

## Requests
600 relay rows, none dropped. Each falls in `list_companies`, a build or a post, and none outside. The relay stamps whole seconds; no row carries the second of any parse. One Import. `evidence/check.txt` holds the counts. [verified]

## Left behind
BRIDGE CASHDAY LAB now holds the Contra, V 15, M 214: no restore was asked for, so backup BK2 of row s1 is before it. Company shut, no company loaded, one tally.exe on 9001.

**Confidence:** one synthetic book, one cash ledger, one sitting: PARTIAL. Two or more cash ledgers in one batch are not exercised here.
