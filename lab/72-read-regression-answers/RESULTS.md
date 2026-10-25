# Results: 72-READS, the 0.6.1 read rows L1, L3 and L4 on TallyPrime 7.2 (25 Oct 2026)

Windows lab PC, TallyPrime Release 7.2 (Latest), Silver Rental (not Educational): `tally.exe` file version 1.1.7.2, one process owning port 9001, GET /status 200 at the start and at the end. The item's files are at `lab/72-read-regression-run-sheet` @ 726e3fb9 (`SHA256.txt` 11/11 OK). Read-only throughout: no Tally request other than Bridge's own reads, nothing posted, no key pressed in Tally except Company > Shut and Select Company between rows. Calls ran 13:45 to 14:27 IST. Counted writes: none.

## Build and route (a deviation, accepted before the rows ran)
- `bridge_mcp.exe` sha256 92a899a21f904f88cb2d13ce44b1417d5aecaf333caad5230de598bf54d89524 at the start and at the end (scheduled CI run 40077056130 on master 2306d86d, artifact 12553782422). Its `tools/list` reports serverInfo `bridge-tally` 0.6.0, the same label as the release, and 27 tools. Its `voucher_presence` schema carries the #1704 manual-numbering check text, which shows the staged build answered, not the release.
- The assistant session's own Bridge tools come from the installed 0.6.0 extension. They were not used for any row.
- Route: each call was one MCP stdio session with a lab client (`mcp-call.py`, sha256 dd6cb3f333137c8048fc933d379a02f92a50f943a06c692f48835a3da38d66ad, kept on the lab PC). The client started the staged exe, sent `initialize`, `notifications/initialized` and one `tools/call`, and saved the whole JSON-RPC response. Every Tally request went through Bridge's own dispatch.
- The environment was the same as the standing Mode B entry:

  | Variable | Value |
  |---|---|
  | `BRIDGE_TERMS_ACCEPTED` | true |
  | `BRIDGE_TALLY_HOST` | localhost |
  | `BRIDGE_TALLY_PORT` | 9001 |
  | `BRIDGE_AGENT_REDACTION` | none |
  | `BRIDGE_AGENT_ENABLE_IMPORT` | false |
  | `BRIDGE_AGENT_ENABLE_WRITES` | false |
  | `BRIDGE_AGENT_DATA_DIR` | the lab's separate, new, otherwise empty RC data folder |
- Preflight: every filled argument file has only keys from its tool's `inputSchema`, and every required key (`evidence/schema-check.txt`).
- Every one of the 20 results (`evidence/results-sha256.tsv`) matches a `response_prepared` egress line byte for byte. Re-serialising the saved response compactly, plus the newline, gives the egress `response_sha256` and `bytes_prepared`, and each line has its `stdio_write_completed` (`evidence/egress-match.tsv`).

## Companies (S0a; each `list_companies` showed exactly one company, `identity_state` verified_tuple)
With no company loaded, `tally_status` gave `loaded_companies` [], `product` not_observed, `education_mode` null, `license_tier` null; `list_companies` gave []. So each GUID below comes from a `list_companies` call with that company loaded alone.

| Company | GUID | books from |
|---|---|---|
| BRIDGE MANUAL NUMBERING LAB 1 (N1) | cbe97898-88e6-42ee-b48f-5e1c3246105c | 20260401 |
| BRIDGE MANUAL NUMBERING LAB 2 (N2) | c48f3906-7682-4828-9e87-2b660064b6f8 | 20260401 |
| BRIDGE SHAPE LAB | 3a6bd6e1-b835-4bff-89dd-8a6af138c346 | 20250401 |
| BRIDGE PILOT LAB | 6b43e498-430c-4d5c-bfef-d32e2ab93c85 | 20260401 |

## S0b: voucher types (`masters` kind voucher_types)
| | Expected (7.1) | Seen (7.2) | Egress response_sha256 |
|---|---|---|---|
| N1 Journal | Manual, duplicates prevented | `series_numbering` method manual, duplicates prevented, observed; 24 types | d9847592… |
| N2 Journal | Automatic, sub_method auto_retain | `series_numbering` method automatic, sub_method auto_retain, duplicates not_prevented, observed; 24 types | 4b32d216… |

Both companies are as found, so every L3 step ran. The type-level `numbering_method` reads `{"unrecognised":"None"}` on 2 types of N1 (Journal among them) and 3 of N2, and `default` on the rest. The result's own limitations describe both values, so this is not a 7.2 difference.

## L1a (#1741): BRIDGE SHAPE LAB, group Purchase Accounts, compliance with GST details. PASS
| Expected (7.1) | Seen (7.2) |
|---|---|
| `ITC Test Ledger` (if present) has ONE entry: 20250401, Taxable, "Specify Details Here", `ineligible_itc` true | Present (4 ledgers under Purchase Accounts, immediate scope). One entry: `applicable_from` 20250401, `taxability` Taxable, `source` "Specify Details Here", `ineligible_itc` true, `other_content_skipped` false. One state row, `�#4; Any`: CGST 9, SGST/UTGST 9, IGST 18 (raw " 9", " 9", " 18"); Cess and State Cess rate kind absent |
| `itc_eligible` nowhere | 0 occurrences in the whole result; `ineligible_itc` 4 |
| six limitation lines, including "Do not read true as eligible" and "A Yes on any other ledger says nothing either way" | 6 lines; both phrases present, verbatim |

SHAPE LAB was not restored. Egress response_sha256 a07c7900….

## L1b (#1741): BRIDGE PILOT LAB, no group filter. PASS
The result holds 18 ledgers: the capture's 14 plus 4 extras. It is complete: `truncated` false, no next page, and `itc_eligible` occurs 0 times. The sheet's table writes six names without the "BRIDGE " prefix (Output CGST XML, Round Off, SGST 2.5%, SGST 9%, Supplier RJ, Walk-in). The book names them "BRIDGE Output CGST XML" and so on, and they were compared under those names.

| Ledger | Expected (7.1) | Seen (7.2) |
|---|---|---|
| BRIDGE CGST 2.5% | 20260401, As per Company/Group, taxability absent, ineligible_itc true, no rate at all | same: `taxability` null, `ineligible_itc` true; every rate in its state row is kind absent |
| BRIDGE Goods 18% | 20260401, Specify Details Here, Taxable, true; IGST raw " 18", decimal "18" | same (CGST/SGST " 9"/"9") |
| BRIDGE Purchase Svc 5% | 20260401, Specify Details Here, Taxable, null; CGST raw " 2.50", decimal "2.50" | same: `ineligible_itc` null |
| BRIDGE Svc 998313 5% | 20260401, Specify Details Here, Taxable, true | same |
| BRIDGE Svc 998314 5% | 20260401, Specify Details Here, Taxable, true | same |
| BRIDGE CGST 9%, BRIDGE Output CGST XML, BRIDGE Round Off, BRIDGE SGST 2.5%, BRIDGE SGST 9%, BRIDGE Supplier RJ, BRIDGE Walk-in, Cash, Profit & Loss A/c | no GST details entry | each: observation `entries`, 0 entries |

Extras (names and counts only): BRIDGE Svc 4D 5%, BRIDGE Svc 998315 5%, BRIDGE Svc INH 5%, BRIDGE Svc INH2 5%, one entry each. The six limitation lines are present. Egress response_sha256 d76ee2ba….

## L3 (#1704): voucher_presence and the book's numbering. PASS (a, b, c, d)
Ledgers: N2 holds 6 ledgers (`ledger_masters` basic): Cash, Lab Bank, Lab Charges, Lab Seed Income, Profit & Loss A/c and Suspense. `<N2 LEDGER A>` = Lab Charges and `<N2 LEDGER B>` = Suspense. N1 holds the same six, so Lab Charges and Suspense are present there.

Request counts come from each call's egress `request_trail` (`sent`). The trail lists only the last 32 requests; `omitted` gives the rest (`evidence/request-trails.txt`).

| Step | Expected (7.1 unit tests) | Seen (7.2) | Tally requests | Egress response_sha256 |
|---|---|---|---|---|
| L3a N2, Journal declared manual | refused `presence_numbering_method_differs_from_book`; candidates name Journal as automatic; no voucher window request | refused, that code. One candidate: Journal, declared manual, `agrees` false, book automatic / auto_retain / not_prevented, observed | 30 sent, 0 failed (18 posts, 12 status) | 5bb7407c… |
| L3b N1, manual, number "1" | not refused; `numbering_read` other than not_read; Journal Manual, duplicates prevented; about 20 more requests than L3c | not refused. `numbering_read` state `read`, `read_before_window` true, `master_mark` 215; Journal declared manual, `agrees` true, book manual / prevented. Item `present` on basis `manual_voucher_number`, book key …-00000003, with differences date 20260715 vs proposed 20260724 and amount 10 vs 16. Window 20260701–20260731 read complete, 3 vouchers | 54 sent, 0 failed (22 omitted from the trail) | a69203ea… |
| L3c N1, declared automatic | `numbering_read` not_read; no numbering request; the smaller count | `numbering_read` {state `not_read`, reason `no_type_declared_manual`}. Item `possibly_present`, reason `party_not_supplied`, 0 candidates. Window complete, 3 vouchers, `unmatched_book_vouchers` 1 | 34 sent, 0 failed. L3b sent 20 more | cdf8338c… |
| L3d N2, `Lab No Such Type` declared manual | refused, that code, naming `Lab No Such Type` | refused, that code. One candidate: `Lab No Such Type`, declared manual, `agrees` false, book state `not_in_book` | 30 sent, 0 failed (18 posts, 12 status) | c7b6c78b… |

What the trails show, by request and response size:
- **L3a and L3d (refused)** sent the same posts as each other: 2524→4052 bytes ×10, 2000→9724 ×2, 1002→3420 ×2, 1116→81970 ×2 and 1214→45180 ×2.
- **L3c (declared automatic)** sent no 1116→~81970 pair. That pair therefore looks like the numbering read, and it appears in L3a, L3b and L3d.
- **L3b and L3c (not refused)** both sent 1462→10230 ×2 and 1766→101014 ×2. These look like the voucher window reads, and the refused L3a and L3d sent neither.

These labels are inferred from sizes, not from the request bodies.

## L4 (#1787): a refused read when the company is not loaded. A 7.2 FINDING to explain
N2 was loaded alone, and the call used N1's GUID: N1 exists on this PC but was not loaded.

| Expected (unit level) | Seen (7.2) |
|---|---|
| a company-identity refusal, `company_identity_absent_or_ambiguous`, with a `cause`; not an operation code alone | refused, `result.error.code` **`company_identity_not_found`**, message "ComplyEaze Bridge refused this operation.", with no `cause` and no `remediation` field. `evidence.reason_code` company_identity_not_found, state partial |

- The code is a company-identity code, not `ledger_masters_read_failed`. But it is neither `company_identity_absent_or_ambiguous` nor `company_identity_presentation_collision`, and the refusal carries no separate cause.
- Nothing was read from N1: the call sent 4 requests (2 posts of 2524→4052 bytes, which is the identity read, plus 2 status) and returned 0 rows.
- Egress response_sha256 bb6e9c32….

## Close
- N2 was shut, leaving Tally at Select Company with no company loaded, the state found at the start.
- One `tally.exe` owns 9001, and GET /status returns 200.
- The exe hash is unchanged at the end.
- SHAPE LAB was not restored. No company other than the four above was loaded.

## In this commit
- `args/`: the eleven filled argument files: the eight templates (the S0b one filled once for N1 and once for N2) plus the two `ledger_masters` basic reads used to choose the L3 ledgers. Each is filled with the placeholders replaced and every other byte kept.
- `evidence/`:
  - `results-sha256.tsv`: every saved result, with its size and sha256.
  - `egress-match.tsv`: each result against its `response_prepared` line (receipt id, time, sha256, bytes).
  - `request-trails.txt`: the L3 and L4 request trails, summarised by size.
  - `schema-check.txt`: the argument preflight.

## Kept on the lab PC (not committed)
- The 20 whole results and the `tools/list` answer.
- The egress log.
- The client script.
- The step 0 and close outputs.
