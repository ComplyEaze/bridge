# #683: the two currency reads that name a several-currency book's base, on TallyPrime 7.2

Not for merge. READS ONLY, on one company the lab already holds: BRIDGE FOREXGROUP LAB (synthetic, number 100090; its GUID is the one `lab/2071-forex-rerun-answers` recorded for it in `raw/m0.response`). `LAB-COMMON.md` in this folder (v3, a byte copy of the one in `lab/485-ledger-census-spans`) applies in full, with this company as THE COMPANY; where this sheet and it differ, the stricter holds.

**WHY.** The foreign-currency rerun of #2071 kept the book's ledger snapshot and Trial Balance as Tally wrote them on 7.2, but not the two reads Bridge makes first to name the book's base currency, so a test of the snapshot's and the Trial Balance's handling of a base-currency ledger with a foreign-currency balance has had to author the base. These two reads are the answer to that: the book's Currency masters, and the company's own `CURRENCYNAME`. Bridge identifies the base by comparing the second against the first, and classifies every ledger by comparing its own `CURRENCYNAME` with that base.

**THE REQUESTS** (`requests/`, all UTF-16LE with a byte-order mark, each the output of Bridge's own request builder for this company, unedited, sent with `charset=utf-16` as LAB-COMMON C3 says; `SHA256.txt` lists the hashes):

| File | What | Builder | Size bound |
|---|---|---|---|
| `marks` | the company marks (a byte copy of the one `lab/2071-forex-rerun` sent, with only the company name changed there) | | 8,192 bytes |
| `currency-masters` | the book's Currency masters: `NAME`, `MAILINGNAME`, `DECIMALPLACES` | `render_company_currency_request` | 8,192 bytes |
| `currency-masters-originalname` | the same with `ORIGINALNAME` also fetched | `render_company_currency_request_with_originalname` | 8,192 bytes |
| `company-base-currency` | every loaded company's `NAME`, `GUID` and `CURRENCYNAME` | `render_company_base_currency_request` | 8,192 bytes |

Both currency reads and the company read are the ones Bridge sends for a book with several Currency masters. The book holds two (`$` and the rupee master); nothing here writes, keys or restores anything.

**OWNER steps** are marked [OWNER]. None of the three currency reads has been sent to this book before: each is a NEW SHAPE on it (the same shapes were read on 7.1 only), so take a screenshot of the whole Tally window straight after each answer and before anything else is sent (C3).

## Before
1. Load THE COMPANY alone (C1) and run step 0 (C2). Record Tally's version, release, edition and licence (About screen and GET /status).
2. Check every file in this folder against `SHA256.txt` (hash and bytes). A difference = STOP, nothing is sent.
3. `python3 -I check-request-ids.py requests` in this folder. EXPECT `census: ok 4, skipped 0, FAIL 0`. Anything else = STOP.
4. [OWNER] Go-ahead.

## Steps
For every request record the UTC time before the send and after the answer.
1. GET /status. Then `marks`, named `m0`. EXPECT one COMPANY row, THE COMPANY's. Record ALTMSTID and ALTVCHID.
2. `currency-masters`, named `r1`. Screenshot at once. Then GET /status. EXPECT a collection of CURRENCY rows: the `$` master and the rupee master.
3. `currency-masters-originalname`, named `r2`. Screenshot at once. Then GET /status. EXPECT the same rows with `ORIGINALNAME`.
4. `company-base-currency`, named `r3`. Screenshot at once. Then GET /status. EXPECT exactly one COMPANY row (the company is loaded alone); more than one = STOP.
5. `marks`, named `m1`. EXPECT the same ALTMSTID and ALTVCHID as `m0` (every request here is a read); a difference = record it and STOP.
6. C10.

## RECORD (the lab judges only the STOP rules)
For every answer: the curl line's status and bytes, CONTENT-LENGTH and the response headers as received, the sha256, the send and answer times, and whether it parses. Keep each answer byte for byte (no re-encoding, no editor). Read each with an XML parser, never a text search (C4), and report:
- `r1`, `r2`: every CURRENCY row's `NAME`, `MAILINGNAME`, `DECIMALPLACES` and (`r2`) `ORIGINALNAME`, each exactly as sent, and for every `NAME` and `ORIGINALNAME` its length in characters and its code points in hexadecimal (the rupee master's name is the point of the sheet);
- `r3`: the COMPANY row's `NAME`, `GUID` and `CURRENCYNAME`, the last with its length and code points.
- `m0`, `m1`: ALTMSTID and ALTVCHID.
Answers go in files named after the step (`r1.response`, `r2.response`, `r3.response`, `m0.response`, `m1.response`) with their headers beside them. An answer with no rows is a result: keep it whole, with its size. No amount from the book is written in the report's text.

## COMPARED BY THE REQUESTER AFTER THE RUN
- The rupee master's `NAME` against the `CURRENCYNAME` of every ledger row of the ledger snapshot and of the Trial Balance that `lab/2071-forex-rerun-answers` kept (they showed the rupee symbol alone): equal is the result wanted, and it is the base the answers name.
- `r3`'s `CURRENCYNAME` against `r2`'s `ORIGINALNAME` and `NAME` of each master: which field the company names its base master by on 7.2 (on 7.1 it was `ORIGINALNAME`).
- Whether Bridge's own base identification (`identify_base`) names that master from `r2` and `r3`, run on the saved answers.

## The book
It does not change; every request is a read. The marks are taken at the start and the end and must be equal.
