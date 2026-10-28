# Results: #2062 cancel capture, 28 Oct 2026. On TallyPrime 7.2 the candidate cancel request answered `ALTERED` 1, every other counter 0 and no line error: the answer §9.14 observed. The request is the native Create with `ACTION="Cancel"`, 5 bytes different. The Journal read back with the same GUID and master_id, `cancelled` true and no amounts, and its alter_id went from 113 to 114. The book was restored from BK-CX

The sheet is `lab/2062-cancel-capture` at 22ef7ff6403624181bbe2e44e4c01f4e8c7ab279 (row `C1-cancel.md`).
- **Sheet checks:** `SHA256.txt` 6 of 6 equal; the two requests differ in 5 bytes.
- **Company:** BRIDGE SHAPE LAB, 100021, GUID 3a6bd6e1-b835-4bff-89dd-8a6af138c346. Current date 20-Apr-2026 throughout.
- **Build:** master 609776ae (`bridge_mcp.exe` sha256 4c9bace3...df44, the same at start and end; `bridge-tally` 0.6.0). One launch; writes unset.
- **Tally:** TallyPrime 7.2, Silver Rental, Licensed.
- **BK-CX:** `TDBK1800_100021.001`, 102,253 B, sha256 5b96360f43d1f64c418d20a9342e3a8ffcb1b2466e08ce2cab97f73c44ab394c.
- **MARKS deviation:** `ledger_masters` basic is refused on this book (`company_several_currency_masters`). Every MARKS read is `ledger_masters` with `fields=compliance`, `reused` false.

## Marks
| point | M | V |
|---|---|---|
| M0 / V0, and Mb / Vb after BK-CX | 292 | 112 |
| M1 / V1 after the two ledgers; M2 / V2 before the create | 294 | 112 |
| M3 / V3 after the create; M4 / V4 before the cancel | 294 | 113 |
| M5 / V5 after the cancel | 294 | 114 |
| after the restore | 292 | 112 |

- **The two ledgers:** M1 − Mb = 2.
- **The create:** V3 − V2 = 1.
- **The cancel:** V5 − V4 = 1, with M5 − M4 = 0.

## The two requests (each sent once, by curl)
| file | sha256 | answer |
|---|---|---|
| `requests/w1-create.xml` (1,798 B) | ba93bc46... | 200, 540 B, sha256 5eda5c4d...: CREATED 1, ALTERED 0, DELETED 0, LASTVCHID 86, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0 |
| `requests/w2-cancel.xml` (1,798 B) | 7e2967fe... | 200, 540 B, sha256 7f322210...: CREATED 0, **ALTERED 1**, DELETED 0, LASTVCHID 86, LASTMID 0, COMBINED 0, IGNORED 0, ERRORS 0, CANCELLED 0, EXCEPTIONS 0 |

Neither answer had a LINEERROR, and no dialog appeared after either send.

## Voucher rows for 15 June 2026 (`vouchers`)
| point | rows |
|---|---|
| step 0.4 | none |
| step 2.4 | Journal No. 1, CX-1; guid ...-00000056; master_id 86; alter_id 113; cancelled false; optional false; SYN CX Expense -10.00 (Dr), SYN CX Party 10.00 (Cr) |
| step 4.1 | one row; guid ...-00000056; master_id 86; alter_id 114; **cancelled true**; **amounts []**; narration CX-1; optional false |
| step 5.2 (after the restore) | none |

Every §9.14 expectation held:
- one row, with the same GUID and master_id;
- `cancelled` true;
- no amounts;
- alter_id above A1.

**Confidence:** one cancel, PARTIAL until repeated.

**How the ledgers were made:** keyed on Tally's screen with the offered defaults. The party ledger has Registration type Regular with no GSTIN, and bill-by-bill set to No.

**Not covered:**
- a delete;
- other voucher types;
- a voucher posted by `post_import`;
- a batch of vouchers;
- Gold;
- a later Create with the same REMOTEID.

## Files
- `requests/`: the two request files as sent.
- `raw/`: both import answers as received, with headers.
- `answers/`: tool answers 000 to 014.
- `send-log.txt`.
- `client-log.txt`.
- `notes.txt` (UTC).
- `SCREENSHOTS.txt`: the captures and BK-CX by sha256; they stay on the lab PC.
