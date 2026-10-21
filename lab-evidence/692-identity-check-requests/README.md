# Request: do the three stock identities hold on one approved book that has opening stock carried from a prior year? READS ONLY

Why: the identities between Tally's own Profit and Loss, Balance Sheet and the ledger collection were measured on two hand-keyed synthetic books with one item each. One approved book with opening stock carried from the previous year shows whether they hold on something real. Only verdict lines come back.

## Rules
- A book the lab names to the requester, with the requester's go after the size read. Never BRIDGE REGISTER LAB. Reads only: no write of any kind, no import, no Bridge tool posting. One request at a time.
- Answers stay on the lab computer (or in the folder the requester names), are never committed and never pasted into any message, and are deleted when the cycle ends.
- What comes back: the lines printed by `identities.py` for each window, the book's counts from the first read, and the yes/no items listed at step 5. No ledger, item or party name and no amount.
- Request files are UTF-16LE with a BOM exactly as here (`shasum -c MANIFEST.sha256`). The only edits allowed are the placeholders `@@COMPANY@@`, `@@GUID@@`, `@@FROM@@`, `@@TO@@`.
- STOP (stop and report in words, without figures): a prompt or error these steps do not describe, a mark that moves between a group's first and last read, a flag that contradicts step 2.

## Steps
1. The lab names the book and the last completed financial year to the requester.
2. SIZE FIRST: send `requests/t-flags-TEMPLATE.xml` (fill `@@COMPANY@@`, `@@GUID@@`). One row comes back with ISINTEGRATED, ISINVENTORYON, NUMLEDGERS, NUMSTOCKITEMS, ALTVCHID and ALTMSTID. Report only those counts and the two flags to the requester, who says go or stop. ISINTEGRATED must read Yes.
3. Whole financial year (`@@FROM@@` the 1 April, `@@TO@@` the 31 March), in this order, the step 2 request sent again before and after the group (the marks must be equal): t1 Profit and Loss, t2 Balance Sheet, t3 ledger collection, t4 group tree, t5 Negative Stock, t6 Stock Summary.
4. The same year's 1 April to 30 September for t1, t2, t3 and t6, marks around.
5. Run `python3 identities.py PL BS LEDGERS GROUPS` for each window on the answers as received (UTF-16LE), and return the printed lines. If Python 3 is not on the lab computer, say so and the requester names a folder for the four answers of each window; the check is run there and the files are deleted. Also report in words: whether t5 is an empty envelope or lists items (the count only), whether the t6 closing value equals the Profit and Loss closing-stock line (yes or no), and whether Tally's own Balance Sheet screen shows a "Difference in opening balances" line (yes or no).
6. Delete the raw answers when the cycle ends and say so.
