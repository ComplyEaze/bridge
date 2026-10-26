# Lab run sheet: one whole day of vouchers, read the way the transfer pairing view will read it (#1392). SYNTHETIC COMPANY ONLY. Not for merge.

About 25 minutes. Reads only: thirteen requests on one book. ComplyEaze Bridge writes nothing and nothing in Tally is changed.

**WHAT IT DECIDES:** the `transfers` view may say `not_found` for a skipped bank-statement row only when it has read every voucher of that date in Tally (a whole day) and found no Contra between the two banks that could carry it; otherwise the row is `tally_unchecked`. The type that says "this day was read whole" can only be built from a read that completed. No such day read has been sent: the committed answer of this book (`lab/transfer-1392-requests`, `answers-1392-h/h-contra.response`) is one five-day `vouchers` request without its marks read or census. This sheet captures, for single days, every request the day read sends, so the read is built and replayed on real answers (AGENTS.md P1) before any answer depends on it.

**THE READ BEING SPECIFIED (for review with this sheet):**
- **No new request shape.** One day is the existing voucher window read for a one-day window (`read_entry_window_rows`, entry-wildcard shape, counting small books): the company marks (`company_high_water_read`), the census for that day (`render_agent_voucher_census`), and the day's vouchers (`render_agent_vouchers_in_span`), on the serial path. The request carries the company name and the two dates and nothing else. The two bank ledgers are **not** in the request: the vouchers read is company-wide and the ledgers are matched in ComplyEaze Bridge against the session's ledger catalogue, so one day read serves every pair of banks.
- **Which builder it becomes:** a day reader in the pairing view that calls `read_entry_window_rows` with `from = to = the day` and admits the result as a whole day only when the window read says complete, nothing was withheld, and the window is exactly that day. Anything else (partial, too large, refused, another endpoint, a day over the cap) is not a whole day, and its rows stay `tally_unchecked`.
- **Day cap:** the days of open skipped rows only, the newest first, adjacent days merged into one window: at most 4 windows and 31 days per call. A day past the cap is not read and its rows are `tally_unchecked`. About 3 requests per day on a book this small (marks, census, vouchers), so at most about 12 more requests per call; the time per day is measured below.
- **What goes to the AI provider:** nothing from the day read itself. The view returns, per open row, its state label and, where a Tally voucher pairs with it or conflicts with it, that voucher's GUID, voucher number and date. It does not return the day's other vouchers, their amounts or their narrations.

**EXPECT (an expectation, not a fact):** marks `ALTVCHID 5` before and after; the four day censuses together list the five Contras keyed on 12 Oct, each on its own date, and the 6 April census lists none; the spanned and unspanned census of a day are byte-identical; each day's vouchers answer holds the same Contras as the census, with the same bank allocations as `h-contra.response`. Report what happened, not what is expected.

**REQUEST FILES** (each the product's own rendering, unedited: UTF-16LE with a byte-order mark, no final line break; check every one against `SHA256.txt` with `Get-FileHash -Algorithm SHA256` before the first send, and STOP on any mismatch). For each day `DD` in `03`, `04`, `05`, `06` (April 2026):

| file | what it is |
|---|---|
| `m.utf16le.xml` | company marks (`Bridge Agent Company High Water`), BRIDGE TRANSFER LAB |
| `dDD-c1.utf16le.xml` | census of that one day, no AlterID filter (what master sends today) |
| `dDD-c2.utf16le.xml` | the same census with `$AlterID > 0 AND $AlterID <= 5` (what #899 would send) |
| `dDD-v.utf16le.xml` | that day's vouchers, every ledger entry (`Bridge Agent Vouchers`) |

**RULES:**
- Company BRIDGE TRANSFER LAB only (the five Contras of 50,000.00 keyed on 3 to 5 April 2026 between `TLab Bank A` and `TLab Bank B`). Load only it. Port 9001 only. Never touch a client company. If it is not on the lab PC, STOP and report; do not re-key it.
- Before the first send: ComplyEaze Bridge's `tally_status` shows a licensed release (not Education) and exactly that company loaded. Record the release and edition.
- One request at a time; never abandon one; no `--max-time`. STOP on a Tally dialog you did not expect, a second company loaded, Education mode, or 10 minutes without an answer.
- Send with exactly: `curl.exe -sS --data-binary "@<file>" -H "Content-Type: text/xml; charset=utf-16" -o "evidence\<name>.response" -D "evidence\<name>.headers" -w "%{http_code} %{size_download} %{time_total}\n" http://localhost:9001/` and record the exit code, the `-w` line and the ISO time of each send in `evidence\log.txt`. A non-zero exit or a non-200 answer is a transport failure: STOP, and report it as a failure, never as "no vouchers".
- Evidence goes in `C:\bridge-lab\1392-day\evidence\`; zip it and report its SHA256. Do not trim or re-save any response.

**STEPS**
1. Load the book alone; `tally_status` as above.
2. **Marks before.** Send `m.utf16le.xml` (name `m1`). Report `ALTVCHID` and `ALTMSTID` exactly as sent. **If `ALTVCHID` is not 5, STOP**: report both marks; the spanned census files are rendered for 5 only.
3. **Each day, in order 03, 04, 05, 06:** send `dDD-c1` (name `dDD-c1`), then `dDD-c2` (name `dDD-c2`), then `dDD-v` (name `dDD-v`).
4. **Marks after.** Send `m.utf16le.xml` again (name `m2`). If either mark differs from step 2, the book changed during the reads: report it; the answers are void.
5. **Compare.** Per day, whether `dDD-c1.response` and `dDD-c2.response` are byte-identical (`fc /b`, or equal SHA256); if not, the first differing offset and both sizes.
6. **Count by structure, not by text.** For every census and vouchers response, decode it as UTF-16LE, parse it as XML (for example PowerShell `[xml]`) and count the nodes named exactly `VOUCHER`; list each voucher's `GUID`, `ALTERID`, `DATE` and `VOUCHERTYPENAME`. A substring count of `<VOUCHER` also matches `<VOUCHERNUMBER>` and is not accepted. For each vouchers response, also list per voucher each ledger entry's `LEDGERNAME` and `AMOUNT`.
7. **Time.** Report each send's `time_total` from the log; this is the cost per day the cap rests on.

**PASS / FAIL**
- PASS: all thirteen answers received in full (HTTP 200, complete XML), the marks equal before and after, and steps 5 to 7 reported. The sheet passes whatever the answers hold, including a day whose census and vouchers disagree: that is also an answer.
- FAIL: a transport failure, a dialog, a changed mark, or a count made by substring.

**REPORT, one message:** the release and edition; the marks of steps 2 and 4; per response its size, SHA256, `VOUCHER` count and the listed fields; the result of step 5; the times of step 7; any STOP; the evidence zip SHA256. Commit the responses, headers and log unedited to this branch under `lab/1392-whole-day-read/answers/`, with their SHA256 list, for the fixtures and their provenance.
A command that errors is reported as an error, never as "absent" or "unchanged".
