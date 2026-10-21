# LR4 (#1602 part a): what Tally sends for a stock item's opening quantity, value and rate. READ-ONLY.
WHY: the tax-audit engine reads a `stock_items` part with NAME, GUID, PARENT, BASEUNITS, OPENINGBALANCE and OPENINGVALUE, and decides from it whether opening stock is known, zero or unknown (#1602 part a). That part has only ever come from a generated fixture and has never been captured from a real Tally. These four reads give its real shape: an item with an opening value, items with none, and a book with no items at all.
DERIVATION (P1/P6): each file is the committed request `lab-evidence/tonight-read-answers/requests/R05-stock-items-tieout-c.xml` of lab/tonight-read-answers-2026-10-21 (f2692282; 1,190 B, sha256 6237ae51f4bffc9ba4b1ff348b6a0cbfbbe9cf5cc12f5be4e398bfdda118472a). Only two things change: the FETCH line and the SVCURRENTCOMPANY. FETCH `NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST` -> `NAME, PARENT, GUID, BASEUNITS, OPENINGBALANCE, OPENINGVALUE, OPENINGRATE`. Header ID = COLLECTION NAME = `Bridge Master Stock Items` (unchanged). UTF-16LE with BOM, as the base.
FILES (sha256, bytes): see MANIFEST.sha256.
1. LR4-stock-opening-tieout-c.xml  BRIDGE TIEOUT C (1 item, the book with an opening stock)
2. LR4-stock-opening-shape-lab.xml BRIDGE SHAPE LAB (11 items incl. "Zero Stock Item")
3. LR4-stock-opening-tieout-b.xml  BRIDGE TIEOUT B (3 items)
4. LR4-stock-opening-pilot-lab.xml BRIDGE PILOT LAB (0 stock items per R01: the empty-list shape)
STEPS: one request at a time, plain HTTP POST as tonight's R-reads; the company must be loaded (a company not loaded is skipped and recorded as skipped, never as empty). Save each answer AS RECEIVED with its headers, bytes and sha256. A failed request is recorded as failed.
EXPECT (record, do not predict): per item, whether OPENINGBALANCE / OPENINGVALUE / OPENINGRATE are absent, present-empty, or present with text, and the exact text. For file 4: the exact envelope (an empty COLLECTION, no COLLECTION, or an empty ENVELOPE).
PASS = all four answers saved with the item counts equal to R01's NUMSTOCKITEMS (1, 11, 3, 0) (or the difference recorded). FAIL = HTTP error, a dialog, or a count that differs without explanation.
SCREEN CHECK (only if the licence allows a minute): Tally's Stock Item alteration screen for "Lab Gadget" (TIEOUT C) and "Zero Stock Item" (SHAPE LAB) with their opening quantity/rate/value; one screenshot each.
STOP: any dialog before a call.
Evidence: a new branch lab/1602a-stock-opening-answers with MANIFEST (name, bytes, sha256, request, company); send the SHA to the requester before pushing.
