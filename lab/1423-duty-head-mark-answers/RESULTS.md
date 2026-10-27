# Results: #1423 capture 1. A GST duty-head change on Tally's screen moves the master mark by 1, each time, on TallyPrime 7.2, 28 Oct 2026

The sheet is `lab/1423-duty-head-mark/README.md` at b4427246463104711164f44e254c13fb1112e544 (SHA256.txt 3 of 3 equal).

## Verdict
**PASS (the row ran).** Every call was answered whole and kept. Each change was keyed as written. The restore brought the book back to M0/V0/T0 with no probe, and every ledger equals m0.

**The finding.** Each of D1 to D4 moved `ALTMSTID` by exactly **+1**. That covers the two tax-type changes (D1, D2) and the two type-of-duty changes (D3, D4). Creating the ledger also moved it by +1. In every change the probe's duty head, as `ledger_masters` reports it, followed the screen. The other 45 ledgers stayed identical throughout.

**Control 2a (a save with no change) was not run.** The lab order for this sitting skips it, because a no-change save is not what #1423 measures. So this row does not tell whether the mark moves on any save of the ledger or only on the duty-head change. A change keyed on the screen always saves, so the mark catches a duty-head change between two reads either way. This holds for a ledger with no vouchers, on 7.2.

## Setting
- TallyPrime 7.2, Silver, licensed. One tally.exe, listening on port 9001, at step 0 and at the close; GET /status returned 200 both times.
- BRIDGE SHAPE LAB, company number 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346, loaded alone. Gateway: 1-Apr-26 to 31-Mar-27, current date 20-Apr-2026.
- Built tool of master 609776ae (`bridge_mcp.exe` sha256 4c9bace38a82f77471d9b30d88e20ac92d3472a187b744cfeaf15537edf2df44 before the first call and after the last; bridge-tally 0.6.0). It ran over stdio in one launch with a new empty data folder and terms true; writes, import, max bytes and redaction were unset. The server's error stream stayed empty.
- MARKS: `ledger_masters` with `fields` `compliance`, `limit` 500, first page. Every read returned all items in one page (`truncated` false, items = total).
- BK0: `TDBK1800_100021.001`, 102,253 bytes, sha256 04b8878671104f5360056e48397e818635bac6dc383d24163bdb86681ea3d143. It stays on the lab PC.

## Marks and the probe's duty head
| read | after | M | ΔM | V | T | `LAB Duty Probe` `gst_duty_head` | `tax_type` |
|---|---|---|---|---|---|---|---|
| m0 | step 0 | 292 | | 112 | 45 | (absent) | |
| mb | BK0 | 292 | 0 | 112 | 45 | (absent) | |
| m1 | C, create (GST, CGST) | 293 | **+1** | 112 | 46 | `{recognized, head cgst, raw "CGST"}` | GST |
| m2 | D1, tax type CGST → SGST/UTGST | 294 | **+1** | 112 | 46 | `{recognized, head sgst_utgst, raw "SGST/UTGST"}` | GST |
| m3 | D2, SGST/UTGST → CGST | 295 | **+1** | 112 | 46 | `{recognized, head cgst, raw "CGST"}` | GST |
| m4 | D3, type of duty GST → Others | 296 | **+1** | 112 | 46 | `{not_tax_ledger, tax_type "Others"}` | Others |
| m5 | D4, Others → GST, tax type CGST | 297 | **+1** | 112 | 46 | `{recognized, head cgst, raw "CGST"}` | GST |
| m5b | nothing (repeat of m5) | 297 | 0 | 112 | 46 | as m5 | GST |
| m6 | restore of BK0 | 292 | | 112 | 45 | (absent) | |

- **Row by row** (`comparisons.txt`):
  - m0 = mb, m1 = m3 = m5, m5 = m5b, and m6 = m0.
  - Each of m1→m2, m2→m3, m3→m4 and m4→m5 differs in exactly one row, the probe, and only in its `compliance` block.
  - m1 adds only the probe.
  - V and T never moved except T for the probe.
- **D3:** Tally dropped the head. `ledger_masters` answered `not_tax_ledger` with `tax_type` `Others`, the first of the two outcomes the sheet allowed.
- **Spelling of the State head.** After D1 the probe reads `head sgst_utgst`, `raw "SGST/UTGST"`, as the 7.2 screen names it. The book's older `State Tax` ledger, untouched, reads `head state_tax`, `raw "State Tax"`.
- The four duty ledgers (`CGST`, `IGST`, `State Tax`, `Cess`) read the same in every MARKS: `cgst`, `igst`, `state_tax`, `cess`, all `recognized`.

## What the screen offered (TallyPrime 7.2, Ledger Creation and Alteration, under Duties & Taxes)
- **Type of Duty/Tax:** GST, Others. The screen's default on creation was Others.
- **Tax type** (shown only with GST): IGST, CGST, SGST/UTGST, Cess. The sheet's "Central Tax" and "State Tax" are CGST and SGST/UTGST on this screen.
- **With GST:** Percentage of calculation (0 %), and Rounding method (Not Applicable, Downward Rounding, Normal Rounding, Upward Rounding; Not Applicable kept).
- **With Others:** the Tax type and Rounding method fields disappear. "Include in Assessable Value calculation" appears (Not Applicable, GST; Not Applicable kept).
- **Unchanged fields:** Mailing Details and Banking Details (Provide bank details: No) and PAN/IT No. were left blank or No each time, as offered.

## Not measured
- The items the sheet lists: a duty ledger already used in vouchers, a change through the gateway, the GST rate and valuation-type fields, a cess ledger, TDS/TCS/VAT heads, 7.1, Gold, and whether `purchase_register` refuses when the change lands between its two reads.
- Also not measured: control 2a (skipped by the lab order).

## Files
- `tool-answers/`: every tool answer of the session, whole (`list_companies` at step 0 and after the restore, and every MARKS).
- `client-log.txt`: each call with its saved sha256.
- `comparisons.txt`: the row-by-row comparisons by parser.
- `notes.txt`: one line per step, in UTC, with the screens.
- `SCREENSHOTS.txt`: the screen captures by name, bytes and sha256. The captures, BK0 and the data-folder listings stay on the lab PC.
