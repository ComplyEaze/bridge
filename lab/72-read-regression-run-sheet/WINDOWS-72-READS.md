# Windows lab item 72-READS: the 0.6.1 read rows L1, L3 and L4 as the TallyPrime 7.2 regression (25 Oct 2026). READ-ONLY, synthetic BRIDGE companies only.

Not for merge. Everything earlier was measured on TallyPrime 7.1; the lab is now on 7.2 (Silver renewed; tally.exe 1.1.7.2; no migration prompt; backup BK-72-0 taken by the lab). This item re-runs three read rows on the staged build and compares each result with the 7.1 expectation below. A difference is a 7.2 FINDING (keep the whole result), not a failure of the build. Nothing is written to Tally; nothing is posted; no key is pressed in Tally except the lab's own company-load acts between rows.

## Build (already staged by the lab)
Scheduled Bridge CI run 40077056130 on master 2306d86df, artifact `unsigned-bundle-smoke-windows-latest` id 12553782422 (zip sha256 74ae2daa851397b71cb59e80373d9478a2da72c74398108e33e34ea83e56eb1e), `bridge_mcp.exe` sha256 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524. It holds #1794 (`ineligible_itc`), #1704 (voucher_presence numbering check) and #1787 (the refusal cause); checked by ancestry. Mode B, the standing entry, data folder `C:\bridge-lab\rc\data` ONLY (this build records a restore it sees in the import journal; older releases refuse that journal). Terms ticked by the owner; posting may stay OFF (these rows read only). Redaction none.

## Rules
Section 3 of the 0.6.0 sheet (reviews wr30 rc-sheet WINDOWS-RC-0.6.0.md): one request at a time, none repeated or cancelled, the Gateway of Tally with no dialog before every call, every result saved WHOLE as JSON with the `response_prepared` egress line, a company loaded ALONE (Shut Company between rows), STOP on an unexpected dialog. EDUCATION STOP: any sign of Education mode ends the sitting before the next call. No raw request files exist in this item: every call is a Bridge tool call with arguments from `args\`, so there are no request IDs to preflight; the preflight is `SHA256.txt` (a different hash is a STOP) and checking each argument file against the tool's schema in `tools/list` (the lab pastes that schema once; a refusal `argument_invalid` is a typo in the filling, not a finding).
Filling the placeholders: replace each `<...>` with the value the step names, keep every other byte. Save the filled file beside the result.

## Step S0 (once): the 7.2 baseline
S0a. Record `tally.exe` version, Tally's product/edition/mode from the screen, `tally_status` (whole), and `list_companies` (whole; note each company's GUID, name, company number).
S0b. For N1 (BRIDGE MANUAL NUMBERING LAB 1) and N2 (BRIDGE MANUAL NUMBERING LAB 2), each loaded alone: `masters` with `args\s0-masters-voucher-types.json` (kind voucher_types). EXPECT (7.1): N2 Journal reads Automatic (`series_numbering` automatic with `sub_method` auto_retain); N1 Journal reads Manual with duplicates prevented. If a company is not as found (types changed), say so and skip the L3 steps that depend on it.

## L1 (#1741): the renamed key, read through Bridge, BEFORE any restore of SHAPE LAB
L1a. BRIDGE SHAPE LAB loaded alone. `ledger_masters` with `args\l1a-ledger-masters-shape-lab.json` (group Purchase Accounts, fields compliance, include_gst_details). EXPECT (7.1 sitting of 15 Oct, ledger left with its setting at No): if `ITC Test Ledger` is still in the book, its row carries ONE entry in `compliance.gst_details.entries`: `applicable_from` 20250401, `taxability` Taxable, `source` "Specify Details Here", `ineligible_itc` **true** (the wire Yes of the keyed No); the key `itc_eligible` appears NOWHERE in the result; `result.limitations` holds six lines including "Do not read true as eligible" and "A Yes on any other ledger says nothing either way". If the ledger is gone (the book was restored), record "ledger gone" and go on. Do NOT restore SHAPE LAB in this item.
L1b. BRIDGE PILOT LAB loaded alone. `ledger_masters` with `args\l1b-ledger-masters-pilot-lab.json` (no group filter). Compare the 14 ledgers the committed capture names, by name, with this 7.1 expectation (entries as sent; `ineligible_itc` true for Yes, false for No, null when the element is absent; nothing defaulted):
| ledger | entry applicable_from | source | taxability | ineligible_itc | other |
|---|---|---|---|---|---|
| BRIDGE CGST 2.5% | 20260401 | As per Company/Group | (absent) | true | the entry carries no rate at all (rate kind absent) |
| BRIDGE Goods 18% | 20260401 | Specify Details Here | Taxable | true | IGST rate raw " 18", decimal "18" |
| BRIDGE Purchase Svc 5% | 20260401 | Specify Details Here | Taxable | null | CGST rate raw " 2.50", decimal "2.50" |
| BRIDGE Svc 998313 5% | 20260401 | Specify Details Here | Taxable | true | |
| BRIDGE Svc 998314 5% | 20260401 | Specify Details Here | Taxable | true | |
| BRIDGE CGST 9%, Output CGST XML, Round Off, SGST 2.5%, SGST 9%, Supplier RJ, Walk-in, Cash, Profit & Loss A/c | none | | | | no GST details entry (observation `entries` empty or `not_observed` as 7.1 showed) |
The book now holds more than the 14 (four extras were added after the capture): do not compare the extras; list their names and counts only. Any other difference, in a field name, a value, an added field, or a read refusal, is a 7.2 finding.
PASS: the key is `ineligible_itc` everywhere, the values equal the table, no `itc_eligible`, the six limitation lines present. 7.2 differences: keep the whole result and the egress line.

## L3 (#1704): voucher_presence checks a type declared manual against the book's numbering
Each call ONE at a time; record `result` whole and the number of Tally requests the call sent (egress). Choose `<N2 LEDGER A>` and `<N2 LEDGER B>` as any two ledgers of N2 (read them first with `ledger_masters` basic); N1's `Lab Charges` and `Suspense` are the ledgers its earlier runs used (if absent, name two of N1's).
L3a. N2 loaded alone: `args\l3a-presence-n2-declared-manual.json`. EXPECT (7.1 unit tests, never run live): refused as a whole, `presence_numbering_method_differs_from_book`, `candidates` naming Journal with what the book says (automatic); NO voucher window request in the egress (only the company-mark and numbering reads).
L3b. N1 loaded alone: `args\l3b-presence-n1-manual-numbered.json`. EXPECT: NOT refused; the result reports the numbering was read (`numbering_read` other than `not_read`) and shows the book's setting for Journal (Manual, duplicates prevented); about 20 more Tally requests than L3c (record both counts).
L3c. N1: `args\l3c-presence-n1-declared-automatic.json`. EXPECT: `numbering_read` is `not_read`; no numbering request; the earlier (smaller) request count.
L3d. N2: `args\l3d-presence-n2-type-not-held.json`. EXPECT: refused `presence_numbering_method_differs_from_book` naming `Lab No Such Type`.
PASS: a, b, c, d as EXPECT. FAIL/FINDING: a, d answered with presence values or any voucher request sent; b refused; a refusal code other than the one named (record it: `presence_numbering_unreadable` or `masters_*` would be a 7.2 read problem).

## L4 (#1787): a refused read names the cause when the company is not loaded
Load one BRIDGE company (say PILOT LAB) alone. Take the GUID of ANOTHER BRIDGE company that exists on the PC but is NOT loaded (from S0a). `ledger_masters` with `args\l4-ledger-masters-not-loaded.json`. EXPECT (unit-level only until now): a refusal whose cause names the company identity problem, not a generic read failure: `company_identity_absent_or_ambiguous` for a company that is not loaded or listed twice (record the exact `code` and `cause`; `company_identity_presentation_collision` is the renamed/collision case and would be a finding to explain here). Nothing is read from the other company. If only one BRIDGE company can be loaded and none can be unloaded, say so; do not rename or duplicate a company to provoke the other causes.
PASS: the cause is one of the company-identity causes. FAIL: an operation code such as `ledger_masters_read_failed` alone.

## Hand-back
One table per row: expected (7.1) vs seen (7.2), the code or values, request counts (L3), the egress `response_prepared` sha256 of each result, the exe hash at the start and the end, `tally.exe` version. Whole results saved on the PC (list by sha256); only the table leaves. Counted writes: none. Not covered: the Windows Store extension rows (L02/L03), posting, anything with a key in Tally.
