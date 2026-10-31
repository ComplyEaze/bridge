# Tally XML gateway protocol reference — reads and date semantics

> This is a section of the canonical [Tally XML gateway protocol reference](./TALLY_PROTOCOL_REFERENCE.md). Read its confidence-marker convention and legacy-link note before relying on a finding.

---

## 5. Date and period semantics — the most dangerous area

### 5.1 `SVFROMDATE` / `SVTODATE` do not filter collection membership

**VERIFIED.** They select which *period* Tally loads, not which rows match. A collection with
no date variables returns only the current display period. Two different narrow windows can
return byte-identical result sets.

### 5.2 A `<FILTERS>` predicate does bound correctly

**VERIFIED.** With `<SYSTEM TYPE="Formulae">` comparing `$Date` against `##SVFromDate` /
`##SVToDate`, the response contains only in-window rows. Measured: unfiltered returned 2
vouchers; the same collection with a wide filter returned 150 across two financial years;
a two-day filter returned exactly 6.

### 5.3 Rejected period boundaries widen silently — **THE KEY TRAP**

**VERIFIED, Education mode.** `SVTODATE` is honoured only when its day-of-month is **1, 2 or
31**. Any other day is silently ignored and the period expands to the entire book. The rule is
literally the set `{1, 2, 31}`, not “month end”: both `20260630` and `20260228` were refused.
Under Education, observed requests for days 15 and 22 were silently aged to the book end.

Twenty-three Education-mode data points, no exceptions:

| Day-of-month | Behaviour |
| --- | --- |
| 1, 2, 31 | honoured — data bounded to request |
| 15, 28, 29, 30 | rejected — period widens to whole book |

**VERIFIED, limited licensed observation (2026-08-22).** On one machine, one TallyPrime
Silver licence tier, one Tally build, and one Bridge Ageing Lab book, arbitrary `SVTODATE`
days were honoured:

| Requested day | Returned span as-of | Result |
| --- | --- | --- |
| 07 | 2026-08-07 | honoured |
| 15 | 2026-06-15 | honoured |
| 22 | 2026-07-22 | honoured |
| 23 | 2026-08-23 | honoured |

This settles #115 item 1: licensed arbitrary-boundary support is no longer unresolved for
that observed TallyPrime Silver profile. It does **not** establish the same result for another
machine, Tally build, licence tier, company, or configuration.

**The failure is invisible from the response.** When the period is silently widened:

- **without** a `<FILTERS>` clause you receive far **too many** rows;
- **with** a `<FILTERS>` predicate bound to `##SVFromDate` / `##SVToDate`, you receive
  **zero** rows, because the refused date variable does not resolve and the predicate excludes
  everything. This observation does not describe a predicate using literal date bounds.

Both return `STATUS=1`. Neither reports an error. A zero-row response is indistinguishable
from a genuinely empty period without corroboration.

**VERIFIED, limited literal-predicate counter-observation (2026-08-21).** Retained request
and response bytes from TallyPrime 7.1 Education, synthetic `WR2 Unicode Lab`, distinguish
that variable-dependent failure from a literal-date predicate. Both requests sent
`SVFROMDATE=20260701` and the refused `SVTODATE=20260830`:

| Predicate bounds | Actual voucher rows | Returned voucher dates | Response bytes | Response SHA-256 |
| --- | --- | --- | --- | --- |
| `$Date >= ##SVFromDate AND $Date <= ##SVToDate` | 0 | none | 3,022 | `49d1cf0c7cf56220fbaa7e2f583a99835f6af74dda7ad807ad54a34277bbff7e` |
| `$Date >= $$Date:"20260701" AND $Date <= $$Date:"20260831"` | 3 | all `20260801` | 9,974 | `544dd8facaffa54263ded46db4f18a75cb940450ca49aacce7f0014b22703b98` |

Both responses carried `STATUS=1`. Rows and dates were counted from complete XML elements,
not substring matches. The literal predicate does not depend on the refused date variable;
this is direct counter-evidence to treating every filtered voucher collection as the zero-row
failure above. Ordinary agent voucher reads and import readback use literal bounds and validate
returned row dates. This differs from a native opening/balance report with no returned span,
whose monetary period still requires current-mode admission.

**Measurement limits:** one Education instance and one synthetic company. The literal request
used `DATE,VOUCHERNUMBER`; the variable request used the broader accounting projection. Its
literal upper bound was `20260831`, one day after the refused static-variable upper bound;
the three observed vouchers fall inside both windows. This was not an otherwise-identical
full-projection comparison, does not qualify every arbitrary-date window or mode transition,
and does not establish that repeating an empty response proves absence. A regression test
protects the literal predicates in both agent renderers; it is not additional live evidence.

**Import verification admission policy:** persisting any `not_found` verdict
requires a fresh observed TallyPrime product and recognised mode before and
after the voucher reads. Release and licence tier are returned facts, not an
independent refusal. An unqualified product or mode must leave the prior proof
and batch status unchanged. Positive historical readback remains available.
Education Journal construction retains its day-1/day-2/day-31 date refusal;
verification uses literal voucher bounds and returned-row validation. On
licensed 7.1 Gold, `verify_import` has returned `posted_verified` for
Bridge-built Payment, Receipt and Contra batches sent over the gateway (§9.13's
2026-09-25 correction: VERIFIED on one book, PARTIAL on a second). This is not a
claim that native posting or another licensed release has completed live
qualification, nor that Education was observed to reject a literal day-15
predicate.

**Bridge native-outstandings policy.** The paired ledger snapshot is the only exact money
discriminator available after a zero-row Bills response: a book with no named bills and no
party-ledger residual has no balance whose as-of can be misattributed, but any non-zero
`CLOSINGBALANCE - sum(BILLCL)` residual means the requested date is unconfirmed. Bridge must
withhold that report with `native_outstandings_as_of_unconfirmed_without_bill_references`; it
must not call the period honoured merely because no named bill supplied a counter.
Likewise, no bill rows, empty `BILLOVERDUE` counters, and zero-only counters identify no
effective date: a substituted period can produce the same absence or zeros. Bridge withholds
those reports under `native_outstandings_as_of_unconfirmed_without_effective_date_evidence`;
this is a fail-closed policy boundary, not a claim that a particular Tally release always
serializes future-due counters as zero or empty.

**Required discipline, regardless of licence mode:**

1. Compare the returned date span against the requested span on every read. If the span
   exceeds the request, the period was not honoured.
2. Never treat a zero-row window as empty without re-querying a strictly wider window.
3. Never emit a deletion tombstone from an uncorroborated empty result.

**PARTIAL:** one observation does not fit the model — a request with `SVFROMDATE` =
`SVTODATE` = a valid day-1 date returned 75 rows spanning a different year. The `SVTODATE`
rule is well-supported but is not a complete model of period resolution.

### 5.4 Company period fields

**VERIFIED.** `BOOKSFROM` and `STARTINGFROM` give the true data extent. The "current period"
shown on Tally's About screen is a **display** period and does not bound what a filtered
collection can reach.

### 5.5 `OPENINGBALANCE` follows `SVFROMDATE`, not just the ledger master

**VERIFIED — live 2026-08-21, Aarav.** In a native `List of Ledgers` export,
`OPENINGBALANCE` is the opening of the loaded `SVFROMDATE` period, inclusive of the ledger
master's own opening balance. It is not permanently the master opening, and it is not a
date-less/current-display value.

Two ledgers discriminate the candidate meanings:

| Ledger | Master opening | FY24 movement | `SVFROMDATE=20250401` opening | Check |
| --- | ---: | ---: | ---: | --- |
| HDFC Current | 350000.00 | -598044.20 | -248044.20 | `350000.00 + -598044.20 = -248044.20` |
| Petty | 25000.00 | -33960.00 | -8960.00 | `25000.00 + -33960.00 = -8960.00` |

At `FROM=BOOKSFROM`, only 3 of 88 Aarav ledgers were non-zero; the same date-less request had
59 of 88 non-zero. That separates the book-start/master-opening result from the currently loaded
display period. Bridge therefore pins the native master export to `BOOKSFROM..LASTVOUCHERDATE`
and treats its period as an accounting input, not a cosmetic request variable.

**CORRECTION — live 2026-09-06, licensed synthetic lab.** Period opening is
account-dependent; the two cash/bank observations above do not establish a
running-balance rule for every ledger. Three Sales vouchers on 2026-08-01
contributed `306.06`, but that Sales ledger returned native opening `0.00` for
both 2026-08-02 and 2026-09-01. A debtor carried `-102.02` into September, and
Cash carried a 2026-09-01 posting of `-12.50` into 2026-09-02. Paired native
responses and MCP results agreed. These observations corroborate the earlier
account-nature distinction: observed balance-sheet ledgers carry prior balances;
observed nominal ledgers open at zero for the selected period.

For a period movement report, use the observed native opening at the requested
`from`, then apply only direct voucher entries inside the literal window. Do not
add pre-window nominal activity to that opening. The resulting calculated
closing is a period movement figure, not the balance-sheet `CLOSINGBALANCE`
method. This is bounded evidence for the recorded ledgers and host, not universal
report parity. Empty opening values remain unknown, never zero.

**Compatibility boundary.** Education mode can silently refuse a non-01/02/31 `BOOKSFROM` and
load its display period instead. The master response does not carry a returned date span, so this
path cannot apply the voucher reader's I12 span comparison. Before dispatch, Bridge instead uses
a freshly observed endpoint `DateBoundaryProfile`: verified Education evidence rejects unsupported
boundaries; observed licensed mode permits ordinary calendar dates under the limited evidence
above. Native opening, compliance-ledger, and native-outstandings reads require a recognized
current mode and a matching closing mode observation. A missing or stale cached probe cannot
admit these date-dependent balances. Unknown mode is refused. Do not replace this with a global
day-of-month rule or imply that one licensed observation qualifies every installation.

### 5.6 Native Trial Balance fields

**VERIFIED within captured synthetic scope — 2026-08-21 and 2026-09-08.**
For ledger-wise native Trial Balance, request `TBALOPENING`, `DEBITTOTALS`,
`CREDITTOTALS` and `TBALCLOSING` from `List of Ledgers` with both period
boundaries admitted under the observed mode. `CLOSINGBALANCE` is not a substitute:
the captured Profit & Loss row for 1 June–31 July 2026 has native Trial Balance
closing `7000.00`, while the balance-sheet method yields `11027.00`.

The captured native response retains empty amount elements. They are distinct
from an explicit numeric `0.00`; absent required fields are invalid. Signed
amounts use negative debit and positive credit. Opening columns need not net to
zero: the captured opening corpus has an opening net of `-49833.50`. Report that
difference separately; do not manufacture a balancing ledger. Numeric column
sums must disclose any empty observations excluded from arithmetic.

**A Closing Stock ledger's closing is not opening plus movements (verified once on each of
two synthetic TallyPrime 7.2 Silver books, 28 October 2026).** A ledger under Stock-in-Hand
whose closing balance was keyed on Tally's screen keeps its opening as `TBALCLOSING`, and the
change shows only in `DEBITTOTALS`: on a book not integrated with inventory, opening -1000.00,
debit -400.00, credit empty, closing -1000.00 (keyed closing 1,400.00); on an integrated book,
the same figures (Tally offers the closing-balance field on both). Tally's own Trial Balance
screen shows the ledger as an Opening Stock line. On the non-integrated book its Profit and Loss
and Balance Sheet used the keyed 1,400.00; on the integrated book they valued the stock from the
stock item (1,200.00, 12 Nos) (that book's report figures are from screenshots kept on the lab
machine, not a committed answer). Bridge returns the row as sent and adds `movements_differ_by`
(closing minus opening, debit and credit, exactly, in Tally's sign, at the amounts' own number of
decimal places) when that is not zero, counting an empty amount as zero, as the statements' row
check does. That check is the same code: `profit_and_loss` and `balance_sheet` refuse a read
(`statement_trial_balance_row_inconsistent`) where it differs. Every row of the committed Trial
Balance captures adds up (known lab 6 rows, opening-year 8, dense month 123, year 7, stock book 9,
several-currency 10). The raw XML of the two lab rows is not committed (the lab kept the tool
answers), so the shape is pinned on the answers' values and on committed captures edited in one
element. Other readers of `TBALCLOSING`, named and not changed here: `bridge-tax-audit`
`book.rs` (`closing_paise` is read from `TBALCLOSING` and `movement_paise` is closing minus
opening, so such a ledger shows no movement there; `financial_statements.rs` recomputes the
closing stock from opening and movements and counts such ledgers as stale, and its text says the
quirk occurs only on books not integrated with inventory, which the integrated book above
contradicts; `stock.rs` `books_close_tb` still takes the raw closing) and the desktop Trial
Balance workbook.

The export includes dormant ledger masters that Tally's rendered report may
omit. Stable paired bytes and bracketed company/mode/book-extent observations
are bounded source evidence, not an atomic snapshot or a completeness proof.
The committed `native/trial_balance_*` fixtures retain complete decoded responses
and provenance. Earlier Education observations established the opening and
closing Trial Balance fields, but did not establish the gross debit and credit
fields used by this four-column request. This runtime therefore admits the
complete native report only on observed licensed TallyPrime. The 8 September
runtime checks exercised six-, eight- and eleven-ledger synthetic companies;
they do not qualify Education mode, other currencies, account types or
production books.

**An empty amount is not a zero, measured again (2026-10-01, one synthetic
dense book, TallyPrime 7.1 Silver).** For one ledger that carried no debit in any
window, the native Trial Balance returned an explicit `0.00` debit for a one-day
and a one-week window and an empty debit element for the one-month window, which
the agent tool reports as `present_empty`. `ledger_movement` reported a debit of
`0` for all three. A caller that compares the two must treat the empty state
explicitly and never read it as zero.

**Cost against `ledger_movement` (same book and ledger, one run each).** The
native Trial Balance took 26 sends and 1 to 2 seconds for each of the three
windows. `ledger_movement` took 90, 126 and 186 sends and about 7, 14 and 30
seconds for windows holding about 80, 570 and 2,460 vouchers of that ledger: its
cost grows with the window, because each part is read twice for the snapshot
comparison, while the Trial Balance does not. The two agreed on the ledger's
opening, debit, credit and closing in all three windows. That book has a
one-sided ledger and, as far as these reads show, no optional, cancelled or
post-dated vouchers, so agreement is not shown for a book that has them.

#### Parent follow-up query contract

The retained-capture parent query is an exact row selector, not a qualified
financial group balance. It compares the parsed `PARENT` observation as-is:
returned text is case-preserving and case-sensitive, and a returned empty
string is distinct from `NotObserved` (a `PARENT` element omitted by that
response). It does not infer `Primary`, normalize a hierarchy, or turn either
state into the other.

The captured native fixtures include Tally's invalid numeric control reference
form `&#4; Primary`; XML sanitization preserves it as a replacement-marker
prefixed returned value. That marker remains an exact selectable value and is
not renamed to `Primary`. The null and empty parent cases in the report-level
tests are synthetic mutations that prove this distinction; they are not claims
that those cases were observed in the captured native Trial Balance responses.
The query only summarizes rows already present in the retained capture and
reports numeric totals with their empty-amount counts.

**Local discovery contract.** Parent discovery also uses only the opaque handle
of that retained capture. It returns at most 100 distinct observations, exact
row counts for those observations, and an overflow indicator. The backend
retains only a bounded set of candidates while scanning borrowed capture rows;
it does not build a second index of every distinct parent in the webview.
Discovery runs away from the UI thread and never acquires Tally data.

An empty search preserves source order without creating per-row search strings.
A nonempty search ranks literal raw/display-label matches first, then exact
matches using Unicode lowercase conversion, then lowercase substring matches.
Source order breaks ties. This folding is for discovery only; the subsequent row selector
still receives the original exact observation. Missing and empty fields use
explicit field-state labels, while returned group strings use a `Group:` prefix
in both the selector and report table. Large-capture resource tests are derived
synthetic stress cases, not additional live-Tally or completeness evidence.

---

## 6. Crashes and rendering traps

### 6.1 `$$` functions and identifiers containing spaces

**VERIFIED — this terminates the Tally process.**

> **RULE: no `$$` function may reference an identifier containing spaces.**

Tally's formula parser stops at the first space and fails, and the failure is not handled
gracefully. Spaces are tolerated elsewhere — `<REPEAT>` and `<FILTERS>` accept spaced names —
so the rule is specific to `$$` function arguments. Name collections without spaces anyway.

Full diagnostics are retained in `.bridge-live/` and deliberately not reproduced here.

### 6.2 Custom reports need display geometry

**PARTIAL.** A custom report with no form/part display attributes returns an empty
`<ENVELOPE></ENVELOPE>` — no error, no rows. Adding `<TITLE>` on the report, `<HEIGHT>` and
`<WIDTH>` on the form, and `<SCROLLED>`/`<COMMONBORDERS>` on the part restored full output.

**Which attribute is individually necessary was not isolated** — several were changed at
once. `<PLAINXML>Yes</PLAINXML>` was tested and is *not* the cause.

Largely moot if you use collection exports (§2.1).

### 6.3 A custom-report FIELD without `TYPE` renders money for display

**VERIFIED 2026-09-26, licensed TallyPrime 7.1 Silver** (`education_mode=false`). One synthetic company, one run of each request. **Confidence: PARTIAL**: one release, one book, and `$ClosingBalance` only.

Two custom reports were sent over the same ledgers and the same full-year window. They differ in exactly one element, the `<TYPE>Amount</TYPE>` on one FIELD. Each report carried two amount FIELDs per ledger:

```xml
<FIELD NAME="BridgeProbeClosingRaw"><SET>$ClosingBalance</SET><TYPE>Amount</TYPE><XMLTAG>"CLOSINGRAW"</XMLTAG></FIELD>
<FIELD NAME="BridgeProbeClosingFmt"><SET>$ClosingBalance</SET><TYPE>Amount</TYPE><FORMAT>"No Symbol, No Comma"</FORMAT><XMLTAG>"CLOSINGFMT"</XMLTAG></FIELD>
```

The second request removed only the first FIELD's `<TYPE>Amount</TYPE>`. A `List of Ledgers` collection read of the same window gave the signed reference values.

| Ledger `CLOSINGBALANCE` (collection) | FIELD with `TYPE Amount` | FIELD without `TYPE` | `TYPE Amount` + `FORMAT "No Symbol, No Comma"` |
| --- | --- | --- | --- |
| `-4250.00` (2 ledgers) | `-4250.00` | **`4,250.00`** | `-4250.00` |
| `4250.00` | `4250.00` | `4,250.00` | `4250.00` |
| empty (4 ledgers) | empty | empty | empty |

**Without `TYPE`, the FIELD returns a display string.** The minus sign is dropped and Indian digit grouping is added, so a debit balance reads as the same magnitude as a credit. With `<TYPE>Amount</TYPE>`, with or without that `FORMAT`, the value came back signed and ungrouped, byte-equal to the collection's. Neither report raised a dialog, and each took under 0.6 s.

**Rule.** Read money through a collection `FETCH` (§2.1, §3). A custom report that must carry an amount through a FIELD declares `<TYPE>Amount</TYPE>` on it. A FIELD without it returns display text, which must never be parsed as an amount. **Not measured:** `$TBalOpening` / `$TBalClosing`, other amount methods, Gold, Education, and other releases.

Captured requests as sent (UTF-16LE with BOM) and their responses, SHA-256:
- collection: request `c27358c32393f57d…`, response `71d52c899fb15330…`;
- report with `TYPE`: request `333404b08c98d51c…`, response `078d40c5cb23c677…`;
- report without `TYPE`: request `5083c1ba411fce41…`, response `b268769604d005e1…`.

---

## 7. Balances

**CORRECTION — 2026-08-24, licensed TallyPrime Silver 7.1.** The earlier claim that a
`Ledger` collection's `ClosingBalance` was a lifetime figure that ignored the requested window
was wrong. Against `BRIDGE CORPUS DENSE` (29,930 vouchers, 2025-04-01 through 2026-03-31),
`SVTODATE=20250630` returned `-2715644.20` and `SVTODATE=20260331` returned
`-11815459.60`; 122 of 123 ledgers differed, with only empty `Cash` unchanged. The result
held for both `List of Ledgers` with `ISMODIFY="Yes"` (the production shape) and plain
`<TYPE>Ledger</TYPE>` with `ISMODIFY="No"`, so `ISMODIFY` is not the discriminator.

`ClosingBalance` is therefore **as-of scoped** and must be read with the same admitted
`SVFROMDATE`/`SVTODATE` window as the bill report it is reconciled against. The prior four
windows all enclosed every transaction in the observed book; identical balances were the
expected result of that accidental test shape, not evidence that the window was ignored. This
is the same window-mismatch trap corrected on #177.

**PARTIAL.** At certain `SVTODATE` values the field renders as **empty** rather than a
number. An empty `TYPE="Amount"` is not zero — coercing it silently produces a wrong balance.
Fail closed or quarantine. Cause not established.

---

## 8. `FETCH` semantics

**VERIFIED.**

- Dotted ledger-entry sub-paths resolve **one level deep**: `ALLLEDGERENTRIES.LEDGERNAME`,
  `ALLLEDGERENTRIES.AMOUNT`, `ALLLEDGERENTRIES.ISDEEMEDPOSITIVE` all work. The similarly
  curated `BILLALLOCATIONS.NAME/.BILLTYPE/.AMOUNT` shape is **not trustworthy for
  outstandings**: it returned empty names and misreported real `New Ref` / `Agst Ref`
  allocations as `On Account`. Guide §2.4a records the A/B proof and the one allowed
  wildcard exception. **§8.2a below measures a second, narrower loss on an instance
  where §2.4a's corruption does not reproduce, and the cheaper fetch that avoids it.**
- **Two levels do not.** `ALLLEDGERENTRIES.RATEDETAILS.GSTRATE` returns zero elements, as do
  `ALLLEDGERENTRIES.RATEDETAILS.*` and `ALLLEDGERENTRIES.RATEDETAILS`. The data exists —
  the same window under `ALLLEDGERENTRIES.*` yields 56 `GSTRATE` elements.
  `GSTRATE` sits at `VOUCHER > ALLLEDGERENTRIES.LIST > RATEDETAILS.LIST > GSTRATE`.
- **Wildcard cost:** `ALLLEDGERENTRIES.*` measured **19,658 B/voucher** versus **3,142 B**
  curated — **6.3×**. The wildcard pulls audit-entry lists, interest collections and other
  sub-lists.
- **Bill-allocation polarity is contextual, not an amount-sign invariant.** The retained named
  `New Ref` / `Agst Ref` wildcard capture has negative amounts under
  `ISDEEMEDPOSITIVE=No` and positive amounts under `Yes`; the live `On Account` journal probes
  have negative amounts under `Yes` and positive amounts under `No`. Outstandings therefore
  uses the exact bill-allocation amount sign and does not validate it against ledger polarity.
- **Bounded outstandings parity is verified on both Education SKUs.** The identical
  `20260401..20260401` wildcard request completed as a byte-stable paired read on port 9000
  (Edit Log 7.0 EDU: 94,464 encoded bytes, 118/106 ms) and port 9001 (standard 7.1 EDU:
  99,813 encoded bytes, 110/102 ms). Both parsed exactly 7 vouchers and 12 `On Account`
  allocations and computed 600 receivable / 600 payable. The wire serialization differs by
  SKU, but the Unit A canonical result does not.

**Practical consequence:** GST rate data is currently reachable only via the wildcard. Most
voucher types (payment, receipt, contra, journal) carry no GST lines, so the wildcard cost
can be confined to sales and purchase types.

**Fields confirmed available on a voucher:** `GUID`, `MASTERID`, `ALTERID`, `REMOTEID`,
`VCHKEY`, `DATE`, `VOUCHERTYPENAME`, `VOUCHERNUMBER`, `NARRATION`, `PARTYLEDGERNAME`,
`PARTYGSTIN`, `ISCANCELLED`, `ISDELETED`, `ISDELETEDVCHRETAINED`, `ASORIGINAL`,
`ISDEEMEDPOSITIVE`, `ALLLEDGERENTRIES.LIST`, `BILLALLOCATIONS.LIST`,
`ALLINVENTORYENTRIES.LIST`, `RATEDETAILS.LIST`, `AUDITENTRIES.LIST`, `OLDAUDITENTRYIDS.LIST`.

### 8.1 Ledger-master field availability — **VERIFIED 2026-08-28; field-presence only**

A sequential read-only `List of Ledgers` collection probe against a loaded TallyPrime demo
book requested `NAME`, stable identifiers, `PARENT`, `OPENINGBALANCE`, and the candidate
party/master fields. The response reported success, contained 89 ledger rows and 91,746
encoded bytes, and has recorded SHA-256 `8dd316f8f5fb70c82514bcdec9f1f7e79f876286a7fbc7879633c6714adfb3d0`.
No names, identifiers, or values from the response are retained here.

- `PARTYGSTIN` was returned on 40 rows. Its presence is source evidence, not a reason to
  manufacture a value on rows where it is absent.
- An unfiltered object export showed every candidate tag on all 88 ledger objects, empty
  in this corpus: `INCOMETAXNUMBER`, `NAMEONPAN`, `LEDPINCODE`, `LEDGSTPINCODE`,
  `MSMEREGNUMBER`, `LEDUDYAMREGNUMBER`, `BANKACCHOLDERNAME`, `BANKDETAILS`, `IFSCODE`,
  `EMAIL`, `LEDGERPHONE`, `STATENAME`, and `LEDADDRESS.LIST`. A collection `FETCH` emits
  only values that are set, so absent output is not evidence that a field is unavailable.
  Bridge requests these verified tags but must not manufacture a value when the master is empty.
- This is an observation of this request/profile, not a claim that every Tally build or
  configuration lacks those fields. Product release/version was not observed in this probe.

The raw response is deliberately not committed: a wire capture carrying party identifiers
does not meet this repository's public-fixture privacy boundary. The hash, byte count, and
field-presence observation above preserve the reproducible evidence boundary without copying
client-like data into source control.

### 8.2 `List of Groups` can return a request-computed company GUID — **VERIFIED 2026-08-31; single captured profile only**

A read-only native `List of Groups` collection request against the synthetic
`Aarav Trading Company Demo` on licensed TallyPrime 7.1 requested
`NAME, PARENT, GUID, MASTERID, ALTERID, RESERVEDNAME` and added:

```xml
<COMPUTE>BRIDGECOMPANYGUID:$GUID:Company:##SVCurrentCompany</COMPUTE>
```

The healthy, 27,140-byte `STATUS=1` response had 28 Group rows. All 28 carried
the selected company's computed GUID; the retained, privacy-screened fixture
hash is `bb2c20f7d9e11634f9cf1f6429f655dc31d50b60fca72c71a6ce981c47db099c`.
The request and capture were bracketed by healthy `/status` checks. The raw
fixture contains no GSTIN, PAN, contact, address, email, phone, website, or
PIN-code fields and intentionally retains Tally's invalid numeric references.

This establishes only the exact request, licensed 7.1 instance, and synthetic
company profile described here. It does **not** establish that other releases,
modes, or Group shapes emit the field; Bridge must continue to fail closed when
the response lacks or mismatches the selected company GUID.

**Release 7.2 sends the computed field earlier in each row — PARTIAL; one book, one release, one send.** The
same request on licensed TallyPrime 7.2 Silver (25 Oct 2026) against `BRIDGE TIEOUT B` returned a
response of the same size (54,280 bytes) with the same 28 Group rows in the same order and every value
equal to the 7.1 capture of that book. Inside each row the computed `BRIDGECOMPANYGUID` comes before
Tally's `PRIMARYGRPPARENT`; on 7.1 it came after it. The reorder was seen in one send and was not repeated. The fixture is
`statement_groups_stock_book_72_live.utf16le.xml` (sha256 `493256b2…`; provenance in
`STATEMENT_CAPTURE_PROVENANCE.md`). The group readers take a row's children by name, so both
captures give the same records; only the per-row source hash, taken over the row's bytes, differs
(`tests/groups_release_72.rs`). A reader that depended on the order of a row's children would not be
safe across the two releases. Not measured: another book on 7.2, Gold, another release.

### 8.2a Curated `BILLALLOCATIONS` drops the `On Account` type — **VERIFIED 2026-09-11; single instance**

**Scope: TallyPrime 7.1 Silver, licensed, `education_mode: false`, one company, 144 allocations.**
This does **not** reproduce §2.4a's corruption and does **not** supersede it.

Three FETCH shapes, same instance, same window, same filter:

| Fetch | `New Ref` | `Agst Ref` | `On Account` | `BILLTYPE` absent | Bytes |
| --- | ---: | ---: | ---: | ---: | ---: |
| `BILLALLOCATIONS.{NAME,BILLTYPE,AMOUNT}` (curated) | 31 | 1 | **0** | 112 | 150,512 |
| `ALLLEDGERENTRIES.BILLALLOCATIONS.*` | 31 | 1 | **6** | 106 | 168,051 |
| `ALLLEDGERENTRIES.*` (wildcard) | 31 | 1 | **6** | 106 | 1,103,107 |

Named allocations: 32 in all three. So here the curated path does **not** collapse `New Ref` or
`Agst Ref` — all 32 survive intact. What it does is silently omit `BILLTYPE` on the six
`On Account` allocations, which then arrive as **amount-only containers**.

**Why that is worse than a missing field.** An amount-only container is indistinguishable from a
ledger entry that has no allocation at all, and Tally emits such placeholders legitimately. A
reader cannot tell "unattributed money against this entry" from "nothing here" — so the correct
handling of a placeholder, ignoring it, deletes a real allocation and reports nothing. A boundary
that *rejects* untyped rows fails loudly; one that *skips* them, which is right for genuine
placeholders, turns this into silent loss. Both boundaries are defensible alone.

**`ALLLEDGERENTRIES.BILLALLOCATIONS.*` recovers what the entry wildcard recovers**, at **1.12×**
the curated payload against **7.3×**, introducing no element type the parser did not already
receive — the allocation's children are the same set either way.

**But Bridge's agent reads use `ALLLEDGERENTRIES.*` anyway, deliberately.** The narrower shape is
measured equivalent *here* and untested on the instance §2.4a describes, which is the one where
curated allocation paths misreport `New Ref`/`Agst Ref` as `On Account`. The asymmetry decides it:
if the narrow shape is wrong there, a reader silently receives incorrect bill types on compliance
data; if the wide shape costs too much, that is loud, measurable and fixable. An unverified
narrowing is not worth a payload saving when the failure mode is silently-wrong evidence.

Use the narrower shape only where the payload genuinely binds and the instance is known good.
A read that **discards** allocations should fetch neither — Bridge's `ledger_movement` profile
omits them entirely rather than paying for data its result type drops.

**What is still unknown.** Whether `BILLALLOCATIONS.*` also cures §2.4a's `New Ref`/`Agst Ref`
corruption **on the affected instance** is untested — nobody in reach has that book. Until someone
runs this three-way A/B there, §2.4a's rule stands for outstandings specifically: bill-level
outstandings needs `ALLLEDGERENTRIES.*`. This section says only that the allocation wildcard is
strictly more faithful than the curated triple and strictly cheaper than the entry wildcard.

**How the loss was nearly missed**, because the method generalises: a first A/B bucketed
allocations with no `BILLTYPE` as "(none)" on both sides and compared the buckets. The wildcard's
typed `On Account` rows sat inside the curated side's "(none)" pile, the tallies matched exactly,
and the conclusion published was "identical". A comparison whose categories can absorb the
difference cannot detect the difference — count the absent case as its own bucket.

### 8.2b `RESERVEDNAME` is a group's rename-proof identity, and `NAME` is not — **VERIFIED 2026-08-20**

**Why this matters:** any rule of the form "ledgers under Sundry Debtors are parties" or
"ledgers under Bank Accounts hold money" is written against a name a user is free to change.

Three observations on the §8.2 collection, over two synthetic companies (28 and 29 groups) on
TallyPrime 7.1:

1. **Every predefined group carries a non-empty `RESERVEDNAME`, and a user-created group
   carries an empty one.** The empty value is Tally's own positive signal that the row is
   user-created — it is not a missing field, and a reader that never requested the attribute
   at all must be kept distinct from both.
2. **A predefined group can be renamed over XML and `RESERVEDNAME` survives it.** An
   `Import Data` / `All Masters` `<GROUP ACTION="Alter">` carrying a `NAME.LIST` renamed the
   predefined `Suspense A/c` — `ALTERED=1`, group count unchanged, and the readback returned
   `<GROUP NAME="WR5 Renamed Suspense" RESERVEDNAME="Suspense A/c">`. The book was restored
   afterwards and the group set verified identical to the committed fixture. Renaming
   predefined groups is not exotic in books migrated from other software.
3. **`RESERVENAME` is a different field with the opposite meaning.** In the same readback,
   `RESERVEDNAME` held the original predefined identity and `RESERVENAME` held the *current*
   name. They differ by one letter; reaching for the wrong one silently restores the bug.

A group also exposes `PARENTSTRUCTURE` — its whole ancestry chain, separated by raw `U+0003` —
but **ledgers do not**: fetched explicitly against all 88 ledgers of one company it returned
zero occurrences. So a ledger-to-group ancestry walk climbs one `PARENT` hop at a time through
the Group collection; `PARENTSTRUCTURE` is a shortcut for the group tree only. A top-level
group's `PARENT` is the control-marked reserved root of §1.1, not the word `Primary`.

> **RULE: classify a group by `RESERVEDNAME`; treat an empty one as "user-created, keep
> climbing"; treat an absent one as no evidence at all.** §9.13's cash/bank gate is built on
> exactly this.

---

### 8.2c `REFERENCE`, `ISPOSTDATED`, `ISINVOICE`, `PARTYGSTIN` on the voucher `FETCH` — **VERIFIED presence 2026-09-18; `PARTYGSTIN` population UNVERIFIED**

**Scope: TallyPrime 7.1 Silver, licensed, `education_mode: false`, synthetic company
`BRIDGE SHAPE LAB`, twelve one-month voucher windows (2025-04 through 2026-03), 67 vouchers
total.** Requests and responses retained with request/response SHA-256 in the capture manifest.

Adding `REFERENCE,ISPOSTDATED,ISINVOICE,PARTYGSTIN` to the end of the voucher `FETCH` list (after
`ALLLEDGERENTRIES.*`) returned cleanly on every window; none of the four altered the shape of any
other field or the response's `STATUS`. Per field, counted exactly across all 67 vouchers:

- **`REFERENCE`** (`TYPE="String"`, the same shape `NARRATION` uses): populated on 9 of 67 rows,
  every one carrying the literal value `SHAPELAB-MANUAL-1`; empty
  (`<REFERENCE TYPE="String"></REFERENCE>`) on the remaining 58. Both shapes are structurally
  identical to `NARRATION`'s already-handled empty/populated cases.
- **`ISPOSTDATED`** (`TYPE="Logical"`): `No` on 66 rows, `Yes` on 1 (the 2025-06 window). This is
  live corroboration — not just the wire-request change a prior commit made — that this
  release/licence does assert the tag when it is requested; it does not establish that every
  release ever asserts it, which is exactly the case the parser's optional-field handling (empty
  or absent means "not observed", never `false`) defends against.
- **`ISINVOICE`**: unlike the other three logicals here, Tally emits this **without** a
  `TYPE="Logical"` attribute on all 67 rows — always exactly `<ISINVOICE>No</ISINVOICE>` or
  `<ISINVOICE>Yes</ISINVOICE>` (16 `Yes`, 51 `No`), never with a `TYPE` attribute at all. Bridge's
  scalar admission matches on tag name only and never inspects attributes, so the missing `TYPE`
  does not change how it is read.
- **`PARTYGSTIN`** (`TYPE="String"`): the tag is present and well-formed on all 67 rows, but
  **empty on every one** — this synthetic company's party ledgers carry no GSTIN. The capture
  proves the tag round-trips through this FETCH list; it does not establish what a populated
  value looks like on the wire. Treat presence and the empty case as **VERIFIED**, a populated
  value as **UNVERIFIED**.

No fixture in this repository was captured with this FETCH list before this date. `REFERENCE`,
`ISINVOICE` and `PARTYGSTIN` are proven at the parser layer by fault-injecting these exact
observed shapes (empty and `SHAPELAB-MANUAL-1`/`Yes`/`No`) into an existing captured voucher
fixture, following §8.2a's own convention for a shape not yet exercised in the committed corpus.

### 8.2d A ledger's `CURRENCYNAME` is the NAME of the Currency master it is kept in — **VERIFIED 2026-09-23; TallyPrime 7.1, synthetic and client-derived books**

**Scope:** TallyPrime 7.1 Silver, licensed, `education_mode: false`. `CURRENCYNAME` appended to the
`FETCH` of the production `List of Ledgers` snapshot. Two books are captured and committed:
- `BRIDGE CORPUS FOREX` (INR base plus a `$` master), as `ledgers_currency_forex_live`;
- `Bridge Billwise Lab` (one master), as `ledgers_currency_single_live`.

Three client-derived lab copies were read the same way, counts only (see "Every ledger carries the
field" below).

**The field names a master by its NAME, not its ORIGINALNAME.**
- On FOREX, the `$` ledgers carry `$` and the rupee ledgers carry `I₹`, which is the rupee master's
  NAME. Its ORIGINALNAME is `₹`.
- On Billwise every ledger carries `Rs.`, where NAME and ORIGINALNAME are both `Rs.`.
- The base master's NAME therefore differs by book: `I₹`, `₹` or `Rs.` have been seen. A comparison
  must use the NAME read from the same book, never a constant.

**The company's own `CURRENCYNAME` is the base master's ORIGINALNAME, not its NAME.** Measured on
a `Company` collection export with `CURRENCYNAME` in its `FETCH` (2026-09-22; it lists every
loaded company, so it is not committed): `₹` on FOREX, `Rs.` on Billwise. On FOREX that differs
from the `I₹` its ledgers carry. The company field identifies the base master (bridge#551); ledgers
are compared with that master's NAME.

**Every ledger carries the field.** No ledger row in any book read lacked `CURRENCYNAME` or had it
empty:
- FOREX: 10 of 10;
- Billwise: 13 of 13;
- the client-derived copies: 109 of 109 across three books, each with one master.

On each single-master book every value equals that master's NAME.

**Adding the field changes nothing else.** On FOREX the response was otherwise byte-identical to
the same request without it. The same holds for the native Trial Balance request
(`TBALOPENING, DEBITTOTALS, CREDITTOTALS, TBALCLOSING` plus `CURRENCYNAME`): measured on FOREX and
Billwise, not committed.

**Why it matters.** A foreign-currency ledger's bills arrive from the Bills reports as plain
amounts, and a zero foreign balance closes as a plain `0.00` (`FOREX_LEDGER_CAPTURE_PROVENANCE.md`).
Only this field tells such a ledger from a base one.

**Not measured:**
- whether a book with a single Currency master can hold a ledger in another currency. It is inferred
  not to, because Tally assigns a ledger its currency from the masters;
- a base-currency ledger, on a book with several masters, holding a bill entered in another
  currency: whether its `BILLCL` or its closing balance shows that currency. The bills on FOREX's `$`
  ledgers are plain decimals, so the bill alone may not; a composite closing balance on a base ledger
  is refused (bridge#551);
- releases before 7.1.

---

### 8.2e A voucher type's class comes from Tally's class functions, not its display name — **VERIFIED 2026-09-24; TallyPrime 7.1, one synthetic book**

**Scope:** TallyPrime 7.1 Silver, licensed, `education_mode: false`. One synthetic book (bridge#625).
- Its reserved Purchase type was renamed to `PURCHASE A/C`, and its reserved Payment type to
  `PAYMENT A/C`.
- A child type `Purchase Local` was created under the renamed Purchase type.
- A user type named exactly `Purchase` was created under Attendance.
- One child type was created under each of the seven other classes, and one each under
  Reversing Journal and Memorandum.
- Three purchase vouchers were posted: two under the renamed type, one under the child.

**What is committed.** Only the class-resolving voucher read's response is committed, as
`native-vouchers-renamed-purchase-class`. It establishes, at row level, Purchase alone (three
rows). Everything else below was measured on 2026-09-24 as one request or write at a time with
the response inspected, but those captures are not committed.

**A renamed reserved type keeps its identity.** Measured, captures not committed.
- It keeps its `RESERVEDNAME` and GUID, and stays self-parented under its new `NAME`.
- A child's `PARENT` is the parent's new name.
- A voucher carries its type only by display name (`VOUCHERTYPENAME` and `@VCHTYPE`). That name
  follows a rename at once.
- A rename moves the company's `ALTMSTID` and the type's `ALTERID`, but no voucher `ALTERID` and
  not `ALTVCHID`.
- Moving a type with vouchers under another class by gateway `Alter` is refused in-band: "Cannot
  change Type of Voucher!".

**Class, per voucher row, in the same response.** Each is a COMPUTE on the voucher collection.

`$GUID:VoucherType:$VoucherTypeName` (committed):
- It returns the row's type GUID, company-prefixed like other masters.

`$ReservedName:VoucherType:$VoucherTypeName` (committed):
- It returns the type's own `RESERVEDNAME`.
- That is empty for a child type, so on its own it misses children.

`$$Is<Class>:<type name>` for Sales, Purchase, Payment, Receipt, Contra, Journal, Debit Note and
Credit Note. Row-level `Yes` is committed for Purchase; the rest was measured, not committed.
- **Reserved types.** Each function answers `Yes` on its own class's reserved type, including
  Payment under its renamed name. The other seven functions answer `No` on that type.
- **Types of no class.** Every function answers `No` on the Attendance type named `Purchase`.
- **Children.** Every child type answers `Yes` to exactly its parent's class and `No` to the other
  seven, for all eight classes. The Payment child sits under the renamed `PAYMENT A/C`.
- **Other reserved types.** On the 16 other reserved types (Attendance, Delivery Note, Job Work In
  and Out Order, Material In and Out, Memorandum, Payroll, Physical Stock, Purchase Order, Receipt
  Note, Rejections In and Out, Reversing Journal, Sales Order, Stock Journal), all eight functions
  answer `No`. So do children created under Reversing Journal and Memorandum.
- The values carry `TYPE="Logical"`.

**A type name is resolved ignoring case.** `$GUID:VoucherType:"purchase local"` and
`"Purchase Local"` return the same GUID, and `$$IsPurchase:"purchase local"` answers `Yes`. This was
measured for ASCII letters only; non-ASCII case folding is unmeasured.

**Two traps.**
- `$$Is<Class>` on a name that no type carries returns `No`, not an error. So it cannot prove a type
  exists, and the GUID COMPUTE is what does that.
- An unknown `$$` function omits its element from every row, rather than returning `No`. A reader
  must treat a missing element as a refusal.

**Telling an unknown name from an empty one** (committed, 2026-09-25, same book, bridge#664). The
native `List of VoucherTypes` collection (the sync connector's request) returned every one of the
book's 35 types in 45,776 bytes: the reserved types, the renamed ones under their new names, the
user types and the children, each with its company-prefixed GUID. It is committed as
`native-voucher-types-reads-lab`. Because the class functions answer `No` for a name no type
carries (the first trap above), `vouchers` reads this list once when a `voucher_type` name selected
no row, and refuses a name no type carries (ASCII case ignored) as `unknown_voucher_type`.

**Not measured:**
- grandchild types (a child of a child);
- whether a voucher type copied into the book from another company keeps a foreign GUID prefix.
  Bridge refuses a row whose type GUID is not the company's (`voucher_type_unresolved`), so such a
  book would fail loudly on a type-filtered read, not answer wrongly;
- releases before 7.1.

---

### 8.2f `REFERENCEDATE` on the voucher `FETCH` — **VERIFIED 2026-10-07; TallyPrime 7.1, one synthetic book**

**Scope: TallyPrime 7.1 Silver, licensed, `education_mode: false`, synthetic company `BRIDGE SHAPE LAB`, one
read of its year (67 vouchers, 134 data rows with the paired read) and one read of one day (one voucher).**
Requests and responses retained with sha256 in the capture's provenance file
(`vouchers-reference-date.PROVENANCE.md` in the agent fixtures).

Adding `REFERENCEDATE` to the end of the voucher `FETCH` list (after `PARTYGSTIN`, §8.2c) returned the element
on every voucher:

- **Shape.** `<REFERENCEDATE TYPE="Date">…</REFERENCEDATE>` on all 134 data rows of the year read and on the day read's
  row. Empty (`<REFERENCEDATE TYPE="Date"></REFERENCEDATE>`) on 132 rows (66 vouchers); populated with `YYYYMMDD`
  on two rows (one Purchase whose reference date is its own voucher date and which has a `REFERENCE`) and on the
  day read's row (a reference date five days before its voucher date).
- **One variable.** With the word removed, the data requests are byte-equal to the reads without it; with the
  `REFERENCEDATE` elements and the company counter block (`CMPINFO`) removed, the data responses are byte-equal to
  the reads without it. The word adds 50 characters a voucher (100 bytes on the wire as UTF-16), about 0.24% of the year's response.
- **Not shown.** A reference date on another voucher type, a voucher with a `REFERENCE` and no reference date,
  another release, another book. `reference_date` is returned as Tally sent it (`YYYYMMDD`); a value that is not
  a date refuses the read (`voucher_reference_date_invalid`).

### 8.2g Cost-centre allocations on a voucher's ledger entry — **VERIFIED 2026-10-14; TallyPrime 7.1, one synthetic book**

**Scope: TallyPrime 7.1 Silver, licensed, `education_mode: false`, synthetic company `BRIDGE SHAPE LAB` (Cost Centres setting No), one read of one day (5 Jun 2025): one Journal with two ledger entries.** Request and response
retained with sha256 in `tests/fixtures/VOUCHER_COST_CENTRE_CAPTURE_PROVENANCE.md`; the request is the one the vouchers read
sends (`ALLLEDGERENTRIES.*` in the `FETCH` list, §8.2c).

- **Shape.** An entry that carries allocations holds one `CATEGORYALLOCATIONS.LIST` with `CATEGORY` (`Business Line`),
  `ISDEEMEDPOSITIVE`, and one `COSTCENTREALLOCATIONS.LIST` per centre. Each centre carries `NAME` and a signed `AMOUNT`
  (`Trading` -600.00 and `Assembly` -400.00, which add up to the entry's -1000.00), `PAYHEADSORTORDER` and
  `EMPLOYEESORTORDER` (0, 0) and empty `ACTUALQTY` and `BILLEDQTY`.
- **An entry without allocations has no `CATEGORYALLOCATIONS.LIST` element at all** in this read (the other entry of the
  voucher; both entries carry a `BILLALLOCATIONS.LIST`, the first one empty). No empty container was observed in this read. Two captured import-verification reads (whose fetch does not ask for
  allocation fields) hold an empty container on an entry; the parser reports that as present and not observed.
- **What the tool returns.** An entry with allocations gains `cost_centre_allocations`, a list of
  `{category, cost_centre, amount}` as sent (names trimmed, the amount as sent); an entry without them has no such key. An allocation list that cannot be read
  whole (a centre with no name or no amount, an amount that is not a decimal, a centre named twice in one category, a
  category named twice in one entry, a category without a name or without centres, a field sent twice, text that cannot
  be decoded, an allocation list at a depth the reader does not take, or an empty container, which is reported as
  present and not observed, never as no allocations) is not shown in part: the entry carries `cost_centre_allocations_unreadable`
  with a reason and the rest of the response is unchanged. An empty amount is never read as zero.
- **Second read, 27 Oct 2026: TallyPrime 7.2 Silver, synthetic company `BRIDGE COST KEY LAB` (Cost Centres setting
  Yes), two reads of one day each, of vouchers keyed on Tally's screen.** Request and response retained with sha256 in
  `tests/fixtures/VOUCHER_COST_CENTRE_KEYED_CAPTURE_PROVENANCE.md`. The shape is the one above.
  - A Payment's expense entry holds one category with two centres, one of them under the other in the cost-centre
    masters: an allocation names only the centre, not its parent.
  - A Journal's expense entry holds two `CATEGORYALLOCATIONS.LIST`, one per category, and **each category's centres add
    up to the whole entry's amount**. A category is a separate view of the same amount, not a share of it.
  - The bank entry of each voucher carries no `CATEGORYALLOCATIONS.LIST`.
  - The `vouchers` tool returned each allocation as sent. Its `summarise_by: cost_centre` relies on this: it refuses
    (`summary_cost_centre_allocation_mismatch`) an entry where one category's centres do not add up to the entry, a
    shape not observed.
- **Not shown.** Quantities; a foreign-currency amount; an item invoice (its sales or purchase ledger's entry carries
  the goods line, §12a.13, and no captured item invoice carries allocations; one beside the goods line is read, one inside it
  is flagged `cost_centre_allocation_shape_unrecognised`, by test only); voucher types other than Journal and Payment;
  another release or book; an entry whose allocations under one category do not add up to its amount (the vouchers read
  does not check this; the summary refuses it).

### 8.2h Bank allocations on a voucher's ledger entry — **VERIFIED 2026-10-28; TallyPrime 7.2, two synthetic books**

**Scope: TallyPrime 7.2 Silver, licensed, `education_mode: false`, synthetic company `BRIDGE BANKREF LAB` (28 Oct 2026; a recapture of a 12 Oct run on 7.1 whose references the privacy scan could not carry, which is not committed), one read of five days (1 to 5 Apr 2026): five Contras of 50,000.00 between two bank ledgers, keyed in double entry on Tally's screen.** Request and response
retained with sha256 in `tests/fixtures/VOUCHER_CONTRA_HAND_KEYED_CAPTURE_PROVENANCE.md`; the request is the one the vouchers read
sends (`ALLLEDGERENTRIES.*` in the `FETCH` list, §8.2c).

- **Where a hand-keyed transfer reference sits.** The voucher-level `REFERENCE` is empty on all five. A reference typed in the narration is only narration text. A reference typed in the bank allocation sits in `INSTRUMENTNUMBER` of the `BANKALLOCATIONS.LIST` on **each** of the voucher's two entries (voucher 2: `222222222222`, type `Same Bank Transfer`; voucher 5: a cheque number `123456`, type `Cheque` on one entry and `Cheque/DD` on the other). `TRANSACTIONID` was empty in every case.
- **Every bank entry carries an allocation, with or without a number.** Vouchers 1, 3 and 4 carry one with a `TRANSACTIONTYPE` (`Cheque` or `Cheque/DD`) and an empty `<INSTRUMENTNUMBER/>`; the allocation's `INSTRUMENTDATE` equals the voucher date.
- **The reconciliation fields, with nothing reconciled.** On all ten allocations `BANKERSDATE` was empty (`<BANKERSDATE/>`), `STATUS` was `No`, `BANKRECONSTATUS` (`TYPE="String"`) was `Available Only in Books` and `PAYMENTMODE` was `Transacted`. Counted with an XML parser over the capture. No entry of this book had been reconciled; a reconciled entry is in the next bullet, from another book.
- **A reconciled entry (VERIFIED 2026-10-27; one run, TallyPrime 7.2 Silver, synthetic company `BRIDGE TRANSFER LAB`, five Contras of 50,000.00 between two bank ledgers).** Contras 1 and 2 were reconciled on screen on the second bank ledger with bank date 6 April 2026, and the same raw request read again. On exactly that entry of each of the two vouchers (`ALLLEDGERENTRIES.LIST[2]/BANKALLOCATIONS.LIST`) and nowhere else: `BANKERSDATE` went from empty to `20260406`, `BANKMANUALSTATUS` from empty to `Reconciled` and `BANKRECONSTATUS` from `Available Only in Books` to `Reconciled`; `STATUS` stayed `No`. Each voucher's `ALTERID` moved (1 to 6, 2 to 7) and the company's voucher mark from 5 to 7; the master mark did not move. The first bank ledger's entries of the same two vouchers and the other three Contras were byte-identical. The request and both responses are retained with their hashes on lab branch `lab/1231-bank-reconciliation-answers` at `bb6531ca`, not in this tree: before, 211,552 bytes, sha256 `262e30039b6879b0d9bd37cf00c1000eada923dd1344947049a9f1a49b6197de`; after, 211,698 bytes, sha256 `b9ac75ffa3218444277eeae7d6e8821d759180806bf75f390e004b2e5647cb57`. Blanking the bank date in Set Status to clear a manual reconciliation was refused by Tally (the bank date cannot be earlier than the reconciliation beginning date), so a reconciled and then cleared entry was not read. What the tool returns for such an entry follows from its five fields: the bank date, and a `status` of `No`; the tool has not been run against it.
- **Elsewhere in the repository's captures.** The other committed voucher answers carry 365 `BANKALLOCATIONS.LIST` elements, every one with no children (one more, populated, is a project-authored fixture and is not evidence).
- **What the tool returns.** Only when `vouchers` is called with `bank_allocations: true`: an entry with one or more allocations that have children gains `bank_allocations`, at most eight, each with `bank_date` (`BANKERSDATE`), `instrument_number`, `instrument_date`, `transaction_type` and `status`, every one `absent`, `empty` or a trimmed `value`; a date that is not `YYYYMMDD` is not shown. No reconciliation verdict is drawn, and neither `BANKRECONSTATUS` nor `BANKMANUALSTATUS` is read. A list that cannot be read whole (a field sent twice in one allocation, more than eight, an allocation list inside another list of the entry, an element inside a field that is read, an undecodable text, a date that is not a date) is not shown in part: the entry carries `bank_allocations_unreadable` with a reason and the rest of the response is unchanged. Without the argument, and in the registers, neither key is returned.
- **Not shown.** A reconciled entry read through the tool, and a reconciled entry that was then cleared; other transaction types (NEFT, RTGS, IMPS) and whether any keeps a UTR in `TRANSACTIONID`; a cash deposit or withdrawal; Payment, Receipt and Journal vouchers; Gold and other releases; several currencies. That the number on one entry equals the number on the other is observed here for two vouchers and is not enforced by the tool.

### 8.2i Goods lines on a voucher's ledger entry — **VERIFIED 2026-10-14; TallyPrime 7.1, two synthetic books**

**Scope: TallyPrime 7.1 Silver, licensed, `education_mode: false`; synthetic companies `BRIDGE SHAPE LAB` and `BRIDGE STOCK LAB`; item invoices that were imported or keyed for the lab books.** The request is the one the vouchers read sends (`ALLLEDGERENTRIES.*` in the `FETCH` list, §8.2c). Captures, each retained with its sha256: `voucher_goods_lines_two_godowns_shape_lab_live.utf16le.xml` (one day, 24 Apr 2025: a Payment and a Purchase; `tests/fixtures/GOODS_LINES_CAPTURE_PROVENANCE.md`), `sales-day/register_window_sales_day_live.utf16le.xml` and `register_window_taxed_sales_day_live.utf16le.xml` and `stock-lab-day/stock_lab_taxed_day_voucher_window_response.utf16le.xml` (one Sales line each), and `agent/vouchers-reference-date-year-rows.utf16le.xml` (one Purchase line).

- **Where a goods line sits.** On an invoice-mode voucher (`ISINVOICE` `Yes`) each goods line is an `INVENTORYALLOCATIONS.LIST` directly under the `ALLLEDGERENTRIES.LIST` of the sales or purchase ledger it posts to, with `STOCKITEMNAME`, `RATE`, `AMOUNT`, `ACTUALQTY` and `BILLEDQTY` as direct children, and its godown and batch split as `BATCHALLOCATIONS.LIST` children of the line (`GODOWNNAME`, `BATCHNAME`, `AMOUNT`, `ACTUALQTY`, `BILLEDQTY`, `BATCHRATE`). Many other children are present and empty (GST overrides, `RATEDETAILS.LIST` rows, empty allocation lists).
- **Text, not numbers.** A quantity is a number and a unit, usually with a leading space (` 666.667 Kgs`, ` 1 U.`; a unit can end in a dot); a rate is an amount per unit (`30.00/Kgs`, `100.00/U.`). Amounts are plain signed decimals: the Purchase line `-20000.00` with `ISDEEMEDPOSITIVE` `Yes`, a Sales line `100.00` with `No`, each of the same sign as its ledger entry.
- **A split over two godowns.** The Purchase of 24 Apr 2025 carries one line, Hydrochloric Acid 33pc, 666.667 Kgs at 30.00/Kgs, -20000.00, with two batch rows: Main Location, batch H-2504-A, 300.000 Kgs, -9000.00; Factory Floor, batch H-2504-B, 366.667 Kgs, -11000.00. The batch amounts add up to the line amount; they do on every captured line.
- **The same shape on TallyPrime 7.2.** The same book's vouchers from 1 Apr to 30 Jun 2025, read with the same request on TallyPrime 7.2 on 25 Oct 2026 (lab branch `lab/698-gst-ledgers-read-answers`, `a10-vouchers.xml`, 2,504,288 bytes, not committed here), hold 62 vouchers: 14 invoice-mode vouchers (12 Purchase, 2 Sales) carry one goods line each in this shape, one of them the line above; the Credit Note, the Debit Note and 8 Purchases are accounting-mode and carry none, nor do the other 38 (12 Payments, 7 Receipts, 4 Contras, 14 Journals and the Stock Journal). Counted with an XML parser.
- **Where it is not.** That book's Stock Journal answered an `ALLLEDGERENTRIES.LIST` with no children in the 7.2 read above (#388; that answer is not committed, and no committed capture holds a Stock Journal), so a Stock Journal's stock is not in this read; neither is a Delivery Note's, Receipt Note's, Physical Stock's or Manufacturing Journal's (not captured). An `ALLINVENTORYENTRIES.*` fetch returns the same Purchase as one `ALLINVENTORYENTRIES.LIST` at the voucher level with the same two batch rows (lab branch `lab/missing-reads-answers`, `ans-a08.xml`); no shipped read sends that fetch.
- **What the tool returns.** Only when `vouchers` is called with `goods_lines: true`: an entry with goods lines gains `goods_lines`, each with `stock_item` and with `actual_quantity`, `billed_quantity`, `rate` and `amount` as `absent`, `empty` or a trimmed `value`, and `batches` (`godown`, `batch`, both quantities, `amount`) when the line has batch rows. A line whose batch amounts do not add up to its amount, whose amount or a batch amount is not a decimal, or that names no item, a field sent twice, an element inside a field that is read, a line or batch list at another depth, more than 1,000 lines or 100 batch rows: the entry carries `goods_lines_unreadable` with a reason and no partial list, and the rest of the response is unchanged. Without the argument, and in the registers, neither key is returned.
- **Not shown.** A Credit or Debit Note with items; discounts; additional cost; a bill of materials; a foreign-currency line; a voucher typed on Tally's screen; a compound unit; Gold, Education and releases other than 7.1 and (one book) 7.2.

### 8.3 GST duty head — the vocabulary is irregular and `TAXTYPE` qualifies it — **VERIFIED 2026-09-12; single instance**

**Scope: TallyPrime 7.1 Silver, licensed, one company, 28 ledger masters (2026-09-12) and 36 (2026-09-30).** Captured from
`List of Ledgers` with `FETCH … TAXTYPE, GSTDUTYHEAD`, retained as
`tests/fixtures/agent/native-ledger-masters-duty-heads.utf16le.xml`. A second capture of the same
book (2026-09-30, 36 ledger masters, after one probe ledger and seven more duty ledgers were
created by import) is retained as
`tests/fixtures/agent/native-ledger-masters-sgst-utgst.utf16le.xml`, with its JSON sidecar naming
the binary, the relay and the times.

**The measured vocabulary, verbatim on the wire:**

| `GSTDUTYHEAD` | meaning |
| --- | --- |
| `CGST` | central tax |
| `IGST` | integrated tax |
| `State Tax` | state tax — **NOT** `SGST` |
| `SGST/UTGST` | state or union-territory tax; its relation to `State Tax` is not established |
| `UT Tax` | union-territory tax |
| `Cess` | cess |

`SGST` never appears. Two spellings that name a state-side head have been measured, `State Tax`
and `SGST/UTGST`, which is why the set is enumerated rather than pattern-matched, and why an
unrecognised spelling is surfaced with its raw value instead of being normalised into a neighbour.
They are two heads (`state_tax` and `sgst_utgst` in the tool's JSON): no capture shows they are the
same head, and folding one into the other would hide which spelling a book uses.

**`SGST/UTGST` was measured on 2026-09-30 by writing it.** A raw `ACTION="Create"` master import of
one ledger under Duties & Taxes with `TAXTYPE` `GST` and `GSTDUTYHEAD` literally `SGST/UTGST` was
answered `CREATED=1 ERRORS=0` (not retained here). The read-back is
what this repository retains: the second capture returns `SGST/UTGST` verbatim with `TAXTYPE` `GST`
on two ledgers. The value was not among the spellings tried on 2026-09-12 (`SGST` and six others,
all silently dropped; not retained here). Not established: whether
Tally's own screens offer that spelling, or which books carry it.

**`TAXTYPE` qualifies the head and the two can contradict.** Four states, and all four are
distinguishable only because both fields are read:

| `TAXTYPE` | `GSTDUTYHEAD` | classification |
| --- | --- | --- |
| `GST`, or not observed | one of the six | recognised |
| `GST`, or not observed | anything else, non-empty | unrecognised, raw value retained |
| observed, non-`GST` (e.g. `Others`) | absent or empty | not a tax ledger |
| observed, non-`GST` | non-empty | **contradictory — neither is asserted** |

The last row is a response contradicting itself. Classifying head-first recognises it and never
consults `TAXTYPE`, which releases the contradiction as valid compliance data. Only an **observed**
non-`GST` tax type contradicts: an absent or empty `TAXTYPE` is not evidence that the ledger is
non-GST, and treating it as such would refuse real GST ledgers on any version that omits the field.

**Absence has two wire shapes and they mean the same thing.** This instance **omits**
`GSTDUTYHEAD` entirely for non-GST ledgers — 0 self-closing elements across 28 masters — while a
committed capture elsewhere in the repository carries `<GSTDUTYHEAD/>`. A reader that treats only
one shape as absent classifies ordinary ledgers wrongly on the other.

**Nested markup is refused, not flattened.** A scalar reader that counts depth and concatenates
child text turns `<GSTDUTYHEAD><VALUE>CGST</VALUE></GSTDUTYHEAD>` into a recognised `CGST`. Both
fields here are read with a scalar reader that rejects any child element, because an unexpected
response shape must fail at the boundary rather than become compliance data.

**The head is settable at CREATE and silently not settable at ALTER.** Measured both ways. An
`ACTION="Create"` master import carrying `TAXTYPE` and `GSTDUTYHEAD` returns `CREATED=2 ALTERED=0
ERRORS=0` and the values read back set. An `ACTION="Alter"` against an existing ledger returns
`ALTERED=1 ERRORS=0` — a success — and the field stays empty. An earlier note of ours recorded only
the second and concluded the head "cannot be set by import", which was an alter-time observation
written as an import-time rule.

**Not established:** whether these six spellings hold across Tally versions or localisations. The
captures are one instance. An unrecognised value is therefore surfaced, never guessed.

### 8.4 A party's dated mailing details are in `LEDMAILINGDETAILS.LIST`, which the `FETCH` must name — **VERIFIED 2026-10-12; TallyPrime 7.1, one synthetic book**

**Scope: TallyPrime 7.1 Silver, licensed, one synthetic company, one `List of Ledgers` read of eight ledgers.** The
request and the answer are retained, byte for byte, as
`tests/fixtures/agent/native-party-masters-mailing-details.request.utf16le.xml` and
`tests/fixtures/agent/native-party-masters-mailing-details.utf16le.xml` (sizes and SHA-256 in
`native-party-masters-mailing-details.PROVENANCE.md`). The request is the production party-master request (§8.1's
fields and `LEDGSTREGDETAILS.LIST`) with `LEDMAILINGDETAILS.LIST` added at the end of its `FETCH`.

- **The address is in the dated list, not in the flat elements. VERIFIED.** For the party that holds an address, the
  answer's `LEDADDRESS.LIST`, `LEDPINCODE` and `LEDGSTPINCODE` are empty (`LEDADDRESS.LIST` is a whitespace-only
  placeholder) and `STATENAME` is absent, although the request names all four. The address is in that row's
  `LEDMAILINGDETAILS.LIST`: an `ADDRESS.LIST` of two `ADDRESS` lines, then `APPLICABLEFROM` (`YYYYMMDD`), `PINCODE`,
  `MAILINGNAME`, `STATE` and `COUNTRY`. An address line repeats inside `ADDRESS.LIST`; no other child repeated.
- **Absence is a whitespace-only placeholder, in every row. VERIFIED.** The other seven ledgers (two more parties and five
  that are not parties) carry `<LEDMAILINGDETAILS.LIST>` holding only whitespace, so a ledger with no address is an
  element with no children, not a missing element.
- **A `FETCH` that does not name the list does not get it. VERIFIED on this book.** The earlier read of the same book, whose
  request differed from this one only by not naming `LEDMAILINGDETAILS.LIST` (1,744 bytes against 1,792), returned the
  same eight ledgers with every other element the same and no mailing list on any row. Its answer is retained as
  `tests/fixtures/agent/native-party-masters-without-mailing-details.utf16le.xml`; its request is not retained (it is the
  request above without `, LEDMAILINGDETAILS.LIST`), and its size and hash are recorded in the provenance note, not
  shown by a file.
- **The element makes every row longer. VERIFIED on this book.** The answer is 20,896 bytes against 19,094 for the read
  that did not name it. By ledger row (UTF-16 bytes, counted by structure): the one party that holds an address grew
  from 3,010 to 3,958 (its mailing element is 934 bytes), and each of the other seven rows by 122 (an empty element is
  108 bytes). A second entry on that party was then measured (TallyPrime 7.2, 2026-10-29, `lab/1490-mailing-captures-answers`): the answer grew from 20,896 to 21,840 bytes, so the second entry added 944 bytes and the party's row is 4,902; the eight ledgers average 2,612 bytes a ledger with one entry (20,896 / 8) and 2,730 with two (21,840 / 8). Bridge's per-ledger budget for a read that names the list is
  5,000 bytes (above that largest row; it was 4,700 before the second entry was measured) against 3,750 for one that does not (UNVERIFIED: a party with three entries, longer address lines or a book of many populated parties has not been read). `ledger_masters` sends it only when asked (`include_mailing_details`, refused with `include_gst_details` because the request naming both has not been captured).
- **An answer past the response cap is refused at its head, owes no drain, and the standard read after it answers
  whole. PARTIAL: TallyPrime 7.2 Silver, 2026-10-30, one synthetic book, one sitting
  (`lab/2214-mailing-cut-drain-answers`).** Parties holding many dated mailing entries were added to the book step by
  step. The mailing master answer grew by about 940 bytes an entry and answered whole at every step up to about 32,000
  entries (about 30 MB), never reaching the 20 s limit. At about 40,000 entries (an answer of about 37.6 MB) Tally
  declared the answer's length in its head, so Bridge refused it there, about 5 s into the post, as past its 32 MiB cap
  (`ResponseTooLarge`, declared by the peer), with no drain owed. A standard read sent about half a second later
  answered whole, every row equal to the read at rest and the marks unchanged, in three of three tries. A mailing read
  cut at the 20 s limit, and the drain after one, were not reached on this book. After either failure the party/ledger
  master export reads the book once more with the standard request (#2214).
- **Not shown (by the 7.1 capture).** A second mailing entry with a later date (so which entry applies on a date, and in what order Tally
  sends them, was not measured there: Bridge sorts by `APPLICABLEFROM` and reads the one in force on a date), an entry with no
  `APPLICABLEFROM`, another release, Gold, and any answer larger than eight ledgers. The 7.2 read above has two entries on one
  party, sent oldest first; three or more entries, an entry with no date and a book of many populated parties are not measured.

Bridge reads the list into a dated history (`compliance.mailing_details` of `ledger_masters` with `include_mailing_details`, and the mailing columns of
the party/ledger master workbook) and leaves the flat fields as returned. A response that carries no such element is
reported as not observed, which is not the same as an empty history.

