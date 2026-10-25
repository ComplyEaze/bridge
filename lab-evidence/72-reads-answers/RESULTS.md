# Results: the 0.7.0 reads on TallyPrime 7.2, part A (25 Oct 2026)

Windows lab PC, TallyPrime Release 7.2 (Latest), Silver Rental, `tally.exe` 1.1.7.2. One process owned port 9001 throughout, and GET /status returned 200 at the start and at the end. The item's files are at `lab-evidence/72-reads` @ 4b8adf04; `SHA256.txt` gave 19/19 OK.

Part A only, read-only:
- no import, no write, and no company created, renamed or restored;
- the only Tally keys pressed were Company > Shut and Select Company between groups.

Calls ran 14:46 to 14:52 IST.

## How it was sent
- **Sender:** one request at a time from one sender, the exact `curl.exe --data-binary` line of the lab's common procedure. It has no timeout, no retry and no cancel.
- **Hash check:** every request's sha256 was checked against `SHA256.txt` before it was sent.
- **Saved answers:** each answer was saved as received, BOM-less UTF-16LE, with its headers, size and sha256.
- **Count:** 32 sends, every one HTTP 200 with size equal to CONTENT-LENGTH (`evidence/send-log.txt`).
- **Marks requests:** the c1 marks template was filled with `@@COMPANY@@` and `@@GUID@@` only, using the GUID from that group's company list. For BRIDGE TIEOUT B the filled bytes equal the sheet's own `a1-5-flags-tieout-b` file (sha256 8f964258…).

## Proof and marks, per group
Before the first group, the company list read no company loaded. Before each group:
- the company list showed the named company alone, `EduMode` No;
- no non-BRIDGE company was loaded at any point;
- no dialog appeared.

| Group | Company (GUID) | Requests | Marks before = after (ALTVCHID / ALTMSTID) |
|---|---|---|---|
| g1 | BRIDGE TIEOUT B (d64eda55-da7c-4106-a171-3e3cb6cdd331) | a1-1 … a1-5, a3-2, a5-tieout-b | 3 / 218 = 3 / 218, identical bytes |
| g2 | BRIDGE TIEOUT C (824929d5-39e3-4ab7-a67d-a4efc503e24d) | a2-3, a5-tieout-c | 3 / 213 = 3 / 213, identical bytes |
| g3 | BRIDGE NEGSTOCK LAB 2 (551f8339-e023-4854-8d16-dac01f47bffc) | a2-1, a2-2 | 3 / 214 = 3 / 214, identical bytes |
| g4 | BRIDGE SHAPE LAB (3a6bd6e1-b835-4bff-89dd-8a6af138c346) | a2-4, a3-1, a4-1, a5-shape-lab | 112 / 292 = 112 / 292, identical bytes |
| g5 | BRIDGE PILOT LAB (6b43e498-430c-4d5c-bfef-d32e2ab93c85) | a5-pilot-lab | 68 / 245 = 68 / 245, identical bytes |

## A. Regression pass, 7.1 to 7.2 (`evidence/compare-71-72.tsv`)
**15 of 16 answers are byte-identical** to the 7.1 answers the sheet names: a1-1, a1-2, a1-3, a1-5, a2-1, a2-2, a2-3, a2-4, a3-1, a3-2, a4-1 and the four a5 reads. Each one therefore has the same root, element order, row count, number format, sign and empty form.

a2-3 (BRIDGE TIEOUT C negative stock) is the 46-byte empty envelope, as it was in 7.1. It counts as a pass because the company had been proved loaded first.

**One finding: a1-4 `List of Groups`, BRIDGE TIEOUT B.**
- What matches: the answer has the same size (54,280 bytes), the same 28 GROUP rows in the same order, and every element's value and attributes are equal.
- What differs: inside every GROUP row, the request's own computed field `BRIDGECOMPANYGUID` (from `<COMPUTE>`) now comes **before** Tally's native `PRIMARYGRPPARENT`.
  - 7.1 order: … `PARENTSTRUCTURE`, `PRIMARYGRPPARENT`, `BRIDGECOMPANYGUID`, `LANGUAGENAME.LIST`
  - 7.2 order: … `PARENTSTRUCTURE`, `BRIDGECOMPANYGUID`, `PRIMARYGRPPARENT`, `LANGUAGENAME.LIST`
- Sha256: 7.1 is `statement_groups_stock_book_live.utf16le.xml`, 7.2 is 493256b2… Both are kept; the 7.2 answer is in `answers/a1-4-groups-tieout-b.72.xml`.
- Why it matters: a parser that depends on the order of a GROUP's child elements would see a difference; one that reads by element name would not.

## B. Tool rows: NOT RUN
- **Why:** part B needs the built tool of the release-candidate build. The only build staged on the lab PC (bridge_mcp.exe 92a899a2…, master 2306d86d) has no `negative_stock` or `negative_ledgers` tool in its `tools/list` (27 tools), so B1 and B2 cannot run on it.
- **Not substituted:** B3 to B6 were not run on it either, to avoid mixing builds within part B.
- **What is needed:** downloading the release-candidate build waits for the owner's word.
- **Screen steps:** the steps for B1 and B2 were not done.

## Not run in this sitting
- The optional write item `lab-evidence/72-gst-status-row`. It runs only on the owner's word.
- The party mailing-details read, which the sheet itself excludes.

## Close
- BRIDGE PILOT LAB is left loaded alone at the Gateway of Tally, the lab's standing state.
- One `tally.exe` owns 9001, and GET /status returns 200.

## Kept on the lab PC (not committed)
- The other 31 answers with their headers. 15 of them equal committed 7.1 answers byte for byte.
- The filled marks requests.
- The 7.1 copies used for the comparison, taken from the repository.
