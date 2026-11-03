# Results: row 683-B, the three registers through a build of #2323 on the several-currency lab book, behind the keeping relay, 4 Nov 2026

**All three registers answered as expected, and each register call's 140 requests equal the replayed call's sequence.** For `sales_register`, `purchase_register` and `gst_by_counter_ledger` alike:
- `isError` false, `state` `complete`, no `withheld_total`;
- `vouchers_observed` 11, `ledger_masters_observed` 8.

Sales has 3 vouchers without a duties entry, purchase 2, and gst_by_counter_ledger has 11 vouchers without a GST entry; `total` is 0 for the two registers. By the sheet's Decides: **#2323 holds through a build on this book**, for one book, one day and one currency. The earlier build refused all three with `bill_allocation_amount_invalid` (`lab/683-register-answers`). No `import_refused`, no `reply_lost`, no prompt, and the marks after the row are byte-identical to step 0's. [verified]

Sheet: `lab/683-build-row-sheet` at 3ea8147c, folder `lab/683-build-row/`. `SHA256.txt` hashes to dabe1b1bdae9…, and all 7 files it lists are equal. `relay.py`, `check-relay.py` and the two args files are byte-equal to the 683-register row's, and `marks.utf16le.xml` to that row's marks request. [verified]

Build: merge-queue run 41717087257, `merge_group`, head a4b4adffef21e5058e91f9fbd3c642092b746c9a, success. Artifact `unsigned-bundle-smoke-windows-latest`, id 13270783263, zip sha256 f8a6388b2271696a0fc4ae1d2cfacf03909325f65859bcbe96129c9b23fe43e4, equal to the sheet and the API digest. Each of these equals the bundle's smoke file, before and after the row (`evidence/exe-before.txt`, `exe-after.txt`):
- `bridge-tally.mcpb` 780dab593492d8e4e0ba6904e8d0e5dba2ecb9aa8dda15a31694aca34b8c0993;
- `bridge_mcp.exe` c1bc36a46d158432f7b19fc3ba73a07d8f799f1f33fe0b9a4fee2eb39c2f6876;
- `pdfium.dll` 79d4676b656cfb1abcea88f9ade3b4b0826c5200382db5f4ec72a636c598c118.

The server reported `bridge-tally` 0.7.0. [verified]

Client: `scripts/client.py` of cc28c8f9, sha256 85752d4a4467baf158d726d0e3685193114d7e1fee610e33376a63950ca77498. Launch: `BRIDGE_TERMS_ACCEPTED=true`, host 127.0.0.1, port 9002, one new empty data folder, and no write, batch-post or redaction setting (`run/launch-env.json`; the client logs `BRIDGE_AGENT_ENABLE_WRITES present: False`). One server process for every call, exit code 0. [verified]

Marks sender: the 683-register row's pushed `raw.py` (sha256 4ca5215d…9534, as the sheet names) with its one folder line, `L = '<lab folder>/'`, set to this row's folder. That line had been replaced when that row was staged. Nothing else differs. [verified]

## Steps
| step | as written, or what differed |
|---|---|
| 0 | BRIDGE EINVOICE LAB (left loaded by the row before) shut by Company > Shut, with no question. BRIDGE FOREXGROUP LAB loaded alone by its name filter, with no prompt. Gateway: period 1-Apr-25 to 31-Mar-26, current date Tuesday, 1-Apr-2025. PC's date: 4-Nov-2026 (02:57 IST). `GET /status` 200. m0: one COMPANY, this GUID, **V 11, M 214**. One tally.exe, no second one. |
| 1 | `check-relay.py`: 13 PASS, exit 0. |
| 2 | Relay on 9002 into a new empty folder; client started; `tools/list` sent first (no exchange). |
| 3 | `list_companies`: this company, `verified_tuple`. |
| 4 | `vouchers`: 11 items, `complete`, 8 with a `foreign` entry. |
| 5 to 7 | As written; answers below. |
| 8 | Client stopped (server exit 0), relay stopped. m8: **V 11, M 214**, byte-identical to m0. No dialog on Tally's screen after the row. |

No prompt of any kind appeared: no shut or load question, no second tally.exe, and no Windows prompt on the unsigned `bridge_mcp.exe`. [verified]

## What count.py shows beyond the expected
- **`vouchers` exchanges:** its 44 include 2 that count.py marks "unknown". They are the plain-entry window read (exchanges 22 and 24, request sha256 986017edb475d126…). Its 388,956-byte answer has the same sha256 as the 683-register row's same read (6608d39d02d7337f…). count.py's letter table lists only the register legs, so that read has no letter. [verified]
- **`gst_by_counter_ledger`'s `total`:** prints None because that answer has no `total` key. Its count is `vouchers_without_gst_entry` 11, as expected. [verified]

## Other Bridge processes
The four Bridge connector servers of the desktop client were left running, and the row did not close them: PIDs 384 and 6668 (started 30 Oct 2026), 26480 and 26180 (started 2 Nov 2026), parent the desktop client. All four are unchanged before and after the row (`evidence/bridge-procs-before.txt`, `-after.txt`). No call was made to them from this sitting, and their calls were not measured. [verified as recorded]

## Kept local
The relay exchanges, the tool answers and the screenshots stay on the lab PC, listed by sha256 and bytes in `kept-local.txt`. `relay.jsonl` (request lines, sizes and sha256 only) and the two raw marks answers are here. Neither marks answer holds the company number (checked in UTF-8 and UTF-16LE).

## The book now
BRIDGE FOREXGROUP LAB, V 11, M 214, unchanged, loaded alone at the Gateway. One tally.exe (PID 1200) on 9001. Relay and client stopped.

**Confidence:** one book, one day, one currency, one run, 7.2 Silver: PARTIAL.

## `count.py run relay`, whole output

```
relay: 468 exchanges, import_refused 0, reply_lost 0, connections 1
000-tools.json: tool list, isError None, 0.019 s
  exchanges 0 (POST 0, other 0), unknown requests 0
001-list_companies.json: tool list_companies, isError False, 0.147 s, state None, reason None, total None, withheld_total absent, vouchers_observed absent, ledger_masters_observed absent
  exchanges 4 (POST 2, other 2), unknown requests 0
002-vouchers.json: tool vouchers, isError False, 3.058 s, state complete, reason None, total 11, withheld_total absent, vouchers_observed absent, ledger_masters_observed absent, items 11, with a foreign entry 8
  exchanges 44 (POST 26, other 18), unknown requests 2
003-sales_register.json: tool sales_register, isError False, 6.577 s, state complete, reason None, total 0, withheld_total absent, vouchers_observed 11, ledger_masters_observed 8, sales_vouchers_without_duties_taxes_entry.total 3
  exchanges 140 (POST 78, other 62), unknown requests 0
  sequence equals the replayed register call (140 legs): yes
  letters: esesebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbseseensnseewswseebsbscscsCsCsksksbsbseemsmseebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbsese
004-purchase_register.json: tool purchase_register, isError False, 7.308 s, state complete, reason None, total 0, withheld_total absent, vouchers_observed 11, ledger_masters_observed 8, purchase_vouchers_without_duties_taxes_entry.total 2
  exchanges 140 (POST 78, other 62), unknown requests 0
  sequence equals the replayed register call (140 legs): yes
  letters: esesebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbseseensnseewswseebsbscscsCsCsksksbsbseemsmseebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbsese
005-gst_by_counter_ledger.json: tool gst_by_counter_ledger, isError False, 6.572 s, state complete, reason None, total None, withheld_total absent, vouchers_observed 11, ledger_masters_observed 8, vouchers_without_gst_entry 11
  exchanges 140 (POST 78, other 62), unknown requests 0
  sequence equals the replayed register call (140 legs): yes
  letters: esesebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbseseensnseewswseebsbscscsCsCsksksbsbseemsmseebsbscscsCsCsksksbsbseseebsbsLsLsBsBsgsgsbsbsese
```
