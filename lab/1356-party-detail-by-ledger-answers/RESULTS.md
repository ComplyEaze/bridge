# Results: #1356 party detail by ledger, 29 Oct 2026, through #2115's build at b85f9f99. On BRIDGE CORPUS DENSE the `outstandings` party detail for DN Party 001 to 30 June 2025, refused before the change, answered by the party's ledger: 61 vouchers `ledger_scoped`, the ledger matched exactly, all 61 bills `tied`. To 31 March 2026 (248 vouchers) it was refused: `native_outstandings_read_failed`, cause `request_deadline_exceeded`, after 31 requests answered and a 32nd held 20 s. On the amendment, to 31 July 2025 (82 vouchers, past one part at the planner's 42 a part; the trail does not show the parts) it answered `ledger_scoped` with all 82 bills `tied`, and Tally's own Ledger Outstandings screen agrees with the 30 June read in count and total. Reads only; the marks did not move

The sheet is `lab/1356-party-detail-by-ledger` at 805266060f8bff46b7f73b898ce92aed2146a7c3. `SHA256.txt` 3 of 3 equal.
- **BUILD-T:** #2115 at head b85f9f9925192595042661434bfc2c4f64f9afe7, Bridge CI run 40769047507, artifact `unsigned-bundle-smoke-windows-latest`.
  - `bridge-tally.mcpb` 15,357,398 B, sha256 380bfe9d213fa1395258d20eb69f85f01de718746665b67b14ff8f0979dd710f.
  - `bridge_mcp.exe` 31,828,992 B, sha256 1561c9f11f9a46e4af1072e5af34b292bf5fb965a0f6b0cf73f0728ec7037ebc, the same before and after.
  - Both equal the run's `mcpb-smoke.json`. The server reports 0.6.1.
- **Launch:** keys `BRIDGE_AGENT_DATA_DIR`, `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT`, `BRIDGE_TERMS_ACCEPTED` (posting off); launch file sha256 06f9c0f6409683ccada7f7746abffd16afb66d70a427a34be54e41094966adff.
- **Tally:** TallyPrime 7.2, Silver, `education_mode` false.
- **Company:** BRIDGE CORPUS DENSE, 100012, GUID d45bc1b0-e5e3-4261-b3b2-cce3915f42d3, `verified_tuple`, `books_from` 20250401, the only company loaded.
- **Marks:** ALTVCHID 29900 and ALTMSTID 328 at P1 and at D5, `reused` false.
- **Dialogs:** none.

## The calls (`outstandings`, party `DN Party 001`)
| call | as_of, detail | sent (UTC), seconds | answer |
|---|---|---|---|
| D1 | 20250630, `bill_trail` | 23:58:38, 53.5 | `detail.state` `bills_listed`; `window` 20250401 to 20250630, **`party_vouchers_read` 61**, no `company_vouchers_read`; **`window_scope.basis` `ledger_scoped`**, `vouchers_read` 61; `ledger_match` `exact`; **61 bills, every one `tied`** (all open) |
| D2 | 20250630, `unadjusted` | 23:59:43, 58.4 | `party_vouchers_read` 61, `ledger_scoped`, `exact`; `detail.state` `no_residual_row_for_party`, `residual` null, `on_account_sum` "0", 0 rows |
| D3 | 20260331, `bill_trail` | 00:00:52, 21.9 | **refused**: code `native_outstandings_read_failed`, cause `request_deadline_exceeded`, endpoint `http://127.0.0.1:9001`, "ComplyEaze Bridge refused this operation."; evidence `partial`. The refusal carries no `reads` or `window_scope`. |

D1's and D2's `window_scope.note`: "The company's vouchers for the window needed more reads than one call may spend, so only the vouchers Tally lists under the party's ledger were read. Compared with a whole-book read for one party over a year and one sales ledger on a day only, in windows where every voucher carried the ledger; a voucher Tally leaves out of its list for the ledger would not be seen."

D1's and D2's tool answers are marked `truncated`, because the 200,000-byte bound applies to the whole answer. The detail itself is whole (61 of 61 bills); the top-level `open_bills` list shows 422 of 7,432.

## D4, the egress log (counts only)
From each call's `request_trail`:

| call | requests sent | failed |
|---|---|---|
| D1 | 114 | 0 |
| D2 | 114 | 0 |
| D3 | 32 | 1 |

The failure is D3's 32nd request, a 770-byte post held 20,002 ms, outcome `request_deadline_exceeded`. The trail records each request's kind, bytes and outcome but no request fingerprint, so a split by request shape is not available from it.

**The finding:** to 30 June 2025 the new path answers as designed, the party read by its ledger and every bill tying. To 31 March 2026 the call fails on one Tally request held past ComplyEaze Bridge's 20-second limit. With 31 requests answered first, the census or the first date parts were read before it. Which step's request it was is **not established** by this row.

## Amendment of 29 Oct 2026: D3', D4', D5' and D6 (sheet at 6faf4eb3, the same build and launch)
**Result: the detail ran for 82 vouchers, past one part at the planner's figure of 42 a part.** D3' reaches the same path as D3, the party read by its ledger past one part, at a date whose bills read is about D1's size. It answered as the amendment expects.

| call | as_of, detail | sent (UTC), seconds | answer |
|---|---|---|---|
| D3' (011) | 20250731, `bill_trail` | 00:13:22, 47.6 | `bills_listed`; `party_vouchers_read` **82**, no `company_vouchers_read`; `window_scope.basis` **`ledger_scoped`**, `vouchers_read` 82; `ledger_match` `exact`; **82 bills, every one `tied`** |

- **P1 again** (008 to 010): the one company, `verified_tuple`, 29900 / 328.
- **D4'** (012): D3' sent 114 requests, failed 0. The trail does not state the number of date parts.
- **D5'** (013): 29900 / 328, equal.
- **References:** in D1's 61 bills and D3''s 82, the references step by 120 from DN-INV-000120, except once: DN-INV-001920 is followed by DN-INV-002160. **DN-INV-002040 is in neither.**
- **D6, Tally's Ledger Outstandings screen** for DN Party 001, 1-Apr-25 to 30-Jun-25:
  - **61 bills** (26 on the first screen and 35 more);
  - **no DN-INV-002040**: the list runs 24-Apr-25 DN-INV-001920, then 27-Apr-25 DN-INV-002160;
  - total pending **27,15,644.20 Dr**, equal to the sum of D1's 61 bill balances (−2,715,644.20).

  Tally's own report agrees with D1 in count, gap and total, so the missing 002040 is the book's own.

**Confidence:** one sitting, PARTIAL until repeated.

**Not covered:**
- the request bytes, since nothing captured the traffic between the build and Tally;
- a party name with any other character.

## Files
- `answers/`: every tool answer whole, 000 to 013 (008 onward the amendment's sitting), with UTC send times and seconds.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures by sha256; they stay on the lab PC.
