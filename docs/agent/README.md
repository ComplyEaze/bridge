# ComplyEaze Bridge MCP

For ordinary Claude Desktop installation, start with the
[Download page](https://bridge.complyeaze.com/download.html), or use the
[fallback installation guide](./INSTALL.md).
The developer configuration below remains for supported client integrations.

`bridge_mcp` is Bridge's newline-delimited JSON-RPC 2.0 MCP server. It uses
Bridge's loopback-only Tally XML transport. Reads are enabled by default.
The MCPB extension also exposes voucher file preparation and bank-statement
parsing by default. Voucher posting (one Journal, Payment, Receipt or Contra) is
off by default because of the four limits under *Approved voucher posting* below;
the **Allow voucher posting (Journal, Payment, Receipt, Contra)** setting adds it, with
separate native approval for each new attempt. Command-line installations
retain explicit environment switches.

Build and run it with Rust 1.96:

```sh
rustup run 1.96.0 cargo run --manifest-path src-tauri/Cargo.toml --bin bridge_mcp
```

Every tool call refuses, in band, until the Terms of Use are accepted. Read them first
(https://bridge.complyeaze.com/terms); then set `BRIDGE_TERMS_ACCEPTED=true` (or `1`) to accept them. The extension asks for this
as its "I accept" setting. `initialize` and `tools/list` still answer without it. When it
is on, Bridge appends a line per terms version (version, time, source) to
`terms-acceptance.jsonl` in its data folder when it starts (two servers starting together
can each add one), and refuses if that file cannot be read or written. This is a local
record that the gate opened, not a security boundary and not proof of who accepted, when
they ticked the box, or which text they saw. A value of `true` set by hand names no
version, so it also opens the gate for a later terms version: re-read the Terms whenever
the version changes.

Configure it with `BRIDGE_TALLY_HOST` (default `localhost`),
`BRIDGE_TALLY_PORT` (default `9000`), `BRIDGE_AGENT_DATA_DIR` (Bridge's
platform application-data directory by default), `BRIDGE_AGENT_MAX_ROWS`
(default `500`), `BRIDGE_AGENT_MAX_BYTES` (default `200000`), and
`BRIDGE_AGENT_REDACTION` (`none`, `mask_parties`, or `drop_narration`). The
host is validated by `bridge-tally-transport`; non-loopback hosts are refused.
The resolved data path must be valid Unicode so saved artifact paths can be
returned exactly in JSON. Invalid platform encoding is refused before state
creation with `agent_data_dir_encoding_invalid`; no lossy path alias is used.

Before requesting financial data through an MCP client, the client may send the selected
Tally result to its AI provider, including company
identity, party or open-bill details, and amounts. An unset
`BRIDGE_AGENT_REDACTION` defaults to `none`; `mask_parties` shortens party and ledger
names and bank account numbers to their first two and last two characters (a name of
four characters or fewer becomes `…`; in `stock_summary` it also masks stock item names
and stock-group parents; GUIDs and Tally's reserved root stay plain) and `drop_narration`
drops narrations. Neither setting hides amounts, company names, dates, references, PAN,
GSTIN, IFSC, MSME or Udyam registration numbers or contact details, and the bank statement tool's `account_last4` (the last
four digits of the statement's account number) is sent under every setting. Set the
environment variable before launch when that better fits the workflow.

On Unix, new data directories use mode `0700`; an existing data directory
must belong to the current user and have that mode. Otherwise startup refuses
it without changing its permissions. Select a new dedicated leaf under a shared
parent rather than using the shared directory itself. Windows directories retain
their inherited ACLs; symlink and reparse-point leaves are refused.
Local journals, locks, staging files and delivery receipts must be regular files
with a single link, owned by the current user on Unix. Admission checks the
opened file before reading it or changing bytes or permissions. Existing files
that do not meet these requirements are refused; reconcile their storage before
continuing. New transaction directories are private from creation.

Claude Desktop example:

```json
{
  "mcpServers": {
    "bridge-tally": {
      "command": "/absolute/path/to/bridge_mcp",
      "env": {"BRIDGE_TERMS_ACCEPTED": "true", "BRIDGE_TALLY_HOST": "localhost", "BRIDGE_TALLY_PORT": "9000"}
    }
  }
}
```

Cursor uses the same server object in `.cursor/mcp.json`, with the same
`BRIDGE_TERMS_ACCEPTED` setting:

```json
{"mcpServers":{"bridge-tally":{"command":"/absolute/path/to/bridge_mcp","env":{"BRIDGE_TERMS_ACCEPTED":"true"}}}}
```

The ordinary default tools, in name order:

- `balance_sheet`
- `egress_log`
- `ledger_masters`
- `ledger_movement`
- `list_companies`
- `local_data_report`
- `masters`
- `outstandings`
- `profit_and_loss`
- `purchase_register`
- `read_evidence`
- `sales_register`
- `stock_summary`
- `tally_status`
- `trial_balance`
- `validate_masters`
- `verify_import`
- `voucher_presence`
- `voucher_schema`
- `vouchers`

`masters`, `stock_summary`, `profit_and_loss`, `balance_sheet`, `purchase_register` and
`local_data_report` were added in release 0.4.0; `sales_register` was added in release
0.4.2. `local_data_report` (also
`bridge_mcp --local-data-report [--show-paths]` on the command line) is a
read-only report of what Bridge keeps in its agent data folder: per class
(journal, import files, proofs, review records, approval notes, bank
statements, the egress log, lab files, lock files, other) the file count, bytes
and the age in days of the oldest file, symlinks it did not follow, sockets
and other special files it did not count, folders it could not list (named, not
read as empty), and the import journal's state: batches, batches sent or found
posted, how many of those are not settled (with no recorded response, or with a
response but a latest status that is not `posted_verified`: a post Tally
rejected stays not settled), batches with no recorded dispatch that were never
found posted (`no_dispatch_never_verified`: this includes a batch imported by
hand whose verification is incomplete, which may well be in Tally, so no
deletion may rest on it), and interrupted-write folders that Bridge must recover
before it builds or reads. A journal it could not read is reported as
`journal_unreadable` (could not be opened), `journal_read_failed` or
`journal_invalid` (refused twice, 100 ms apart), never as absent. It reads the
journal without the admission lock, names no file path (the command line adds the
folder paths only with `--show-paths`), and does not cover files outside this
folder (the desktop app's other settings, its mirror database and logs); it also
lists the per-user folder of dispatch lease locks (names, sizes and times only).
Its own call is logged in the egress log like any tool call. On the command
line the exit status is 0 for a complete report, 2 when the folder cannot be
read, 3 when the report is incomplete (journal not read, an entry or folder
that could not be read or listed, or a folder past the 100,000-entry listing cap:
the report's `incomplete_reason` says which, and the tool's evidence is then
`partial`), 1 when it could not be printed, and, for a malformed command line
only, 2 with a usage line. The journal and the
`imports/` folder are Bridge's memory of what it
already sent to Tally: archive the whole folder by moving it, never delete
them piecemeal. For a command-line
installation, `BRIDGE_AGENT_ENABLE_IMPORT=true` also exposes
`build_import_xml` and `parse_bank_statement`, which prepares local
bank-statement voucher proposals. `BRIDGE_AGENT_ENABLE_WRITES=true` enables
that import workflow and exposes `post_import` and `acknowledge_post_review`. The MCPB extension always
sets `BRIDGE_AGENT_ENABLE_IMPORT=true` and maps its **Allow voucher posting
(Journal, Payment, Receipt, Contra)** setting, off by default, to `BRIDGE_AGENT_ENABLE_WRITES`.
This is a source-configuration inventory, not a claim that an installed client
uses a particular setting or that a tool is qualified for every runtime. Each
call returns compact JSON with the
company identity where scoped, a read timestamp, request/response commitments,
byte count, completeness reason, and truncation state. A refused call returns
`result.error` with `code`, which names what failed, and `message`. Where a runtime
refusal has a typed, data-free reason, the error also carries `cause`, which names why
(for example `company_base_currency_undetermined` beside `party_ledger_master_read_failed`).
A read whose two paired halves differ, because the book changed while Bridge was reading it,
carries `native_report_pair_changed`. A voucher-window part that is not admitted
(`voucher_window_part_not_admitted`) names why, and a census disagreement also carries
`counts`, the rows the part `returned` against the rows the census `counted`.
A compliance ledger read (`ledger_masters fields=compliance`) whose company's master-alteration
mark puts the estimated response over Bridge's budget has its ledgers counted first with a
balance-free catalogue read (a stable pair, bound to the company by name and GUID) (#637, #668,
#679). A count that fits is read whole. A count that does not is read in parts: the catalogue's
parent groups are packed by ledger count into parts of at most 4,266 ledgers and 200 parents, and
each part is one filtered master and balance read, all inside the one extent bracket. Every ledger
the catalogue named must come back exactly once, in its own part, with the name and parent the
catalogue gave it; otherwise the whole call is refused and nothing partial is returned. A part
whose master comes back with a different ledger count than the catalogue holds under its parents
ends the read after that master (`parent_part_row_count_differs`): its balance, any later part and
the group read are never requested, but the master pair itself was sent, the complement's included.
The other causes are `parent_part_rows_missing`, `parent_part_row_outside_parents`,
`parent_part_row_not_in_catalogue`, `parent_part_row_differs_from_catalogue` and
`parent_part_row_repeated`. A part whose answer passes the response limit ends the read with
`parent_part_response_too_large`: Tally may have ignored its filter. Ledgers under a parent group
whose name cannot sit in a filter, such as one with a control character, are read as one extra
last part, the complement: a filter that excludes every named parent (`NOT (...)`, at most 200
parents per formula, all applied together), and the same coverage proof applies to it. A book
that cannot be split is refused before any master request: one parent group, or the ledgers
under unnameable parents, over 4,266 ledgers (`parent_over_budget`), more than 12 named parts
(`parent_partition_too_many_parts`), a complement filter over 256 KiB
(`parent_complement_over_budget`), a ledger with no parent (`ledger_without_parent`), every
ledger under an unnameable parent name (`parent_name_unsupported`, which carries
`unsupported_parent_ledgers`, the number of ledgers and no name) or a repeated ledger GUID
(`parent_partition_duplicate_ledger_identity`). These carry no `size` object. The extent bracket still checks the book afterwards, so a book that grows between the
count and the last read is refused, but only after those reads were sent. A mark above 22,857 has
its ledgers counted by AlterID span instead of by catalogue (#679): one read per slice of at most
4,000 AlterIDs over `(0, mark]`, each asking for the ledgers' GUIDs only and made once, not paired,
so a mark of 316,028 is 80 requests of about 0.15 s each. The census is bounded by the same extent
bracket as every other read. A slice holding more ledgers than its span (`ledger_span_slice_over_bound`),
a GUID seen twice (`ledger_span_duplicate_identity`), no ledger at all (`ledger_span_census_empty`:
an empty slice is the answer a closed or absent company gives too) or a slice past the response
limit (`ledger_span_slice_response_too_large`) refuses the call. The census's count then admits the
read like a catalogue's; a count that needs the catalogue to name its parents and whose catalogue
would pass the response limit is refused as `ledger_count_catalogue_too_large`, and two counts, or a
count and the ledgers the read returned, that differ are refused as `ledger_count_differs`. After the
slices and before the count is used, Bridge reads Tally's own count of the company's ledgers once
(`NUMLEDGERS` of the Company object, #938) and refuses the call as `ledger_count_company_differs` if it is
higher than the census's, or as `ledger_count_company_invalid` if that answer was damaged, named another
company or held something other than a plain number, or as `ledger_count_company_response_too_large` if that
answer was larger than the response limit (#1033); a count that is equal, lower or absent never admits
or sizes anything, and the
result of a counted read says which it was in `ledger_count_cross_check.status` (`matched`,
`company_count_lower` or `unavailable`, the last meaning the check did not run). Equality was measured
on three books only (one synthetic with its answer captured in the tree, two real books read separately
and recorded in #938), and the other direction (Tally's count below the census's) is covered by the count
against the rows the read returns, not by this check. A mark
above 400,000 is refused right after the opening extent with cause `ledger_catalogue_too_large` and a
`size` object (`master_alter_id`, `estimated_bytes`, `limit_bytes`, `limit_master_alter_id`): the
census reaches `limit_master_alter_id`, and the catalogue that would count the ledgers instead is
estimated past `limit_bytes`; a response past the transport's cap is cut off mid-read. The mark is an upper bound on ledgers, since every master raises it, so a
company with fewer ledgers may be refused. The limits are unverified against a live Tally beyond a
two-parent filter and the census slices measured on three books; `fields=basic` still reads any of
these books.
When Bridge got no response it could read, the `cause` names why and the error also carries
`endpoint`, the configured origin that was tried (#629). The causes are:
- `endpoint_invalid`: the configured endpoint failed validation. The `endpoint` field then appears only
  if a valid origin can still be formed from the configuration.
- `endpoint_unreachable`: the connection was not accepted.
- `request_failed` or `request_deadline_exceeded`: the request failed before any response, or its
  deadline passed.
- `http_status_failure`, `response_content_type_unsupported` or
  `response_content_encoding_unsupported`: the responder was rejected on its HTTP status or headers
  before any body was read.
- An `endpoint_…` runtime code: Bridge held the request back and sent nothing.

A wrong or reset port therefore reads as an endpoint problem, not as a Tally data problem. A
failed read without `endpoint` either received a response whose body then failed to read, decode,
parse or pass Bridge's checks, or hit a local limit or fault that does not involve the endpoint. A
withdrawn call is `request_cancelled`.

A refused argument that only needs sending again in the right form carries `expected`, a typed
field beside `remediation`: for `argument_invalid:from`, `argument_invalid:to` or
`argument_invalid:as_of`, `{"argument": <name>, "kind": "calendar_date", "formats": ["YYYYMMDD",
"YYYY-MM-DD"]}`, and for `company_guid_invalid`, `{"argument": "company_guid", "kind":
"company_guid", "from_tool": "list_companies"}`. Their guidance asks the assistant to work out a
relative date itself and state the dates it used, or to take the GUID from `list_companies` rather
than a company's name. Both are attached only to a refusal made before anything was read from
Tally: `company_guid_invalid` also comes back after a read when Tally itself lists a company whose
GUID is malformed, and then it carries neither. Other argument refusals carry no `expected`.

Like `remediation`, `expected`, `cause`, `counts`, `size` and `endpoint` are omitted when
`BRIDGE_AGENT_MAX_BYTES` is below 4,096, so that the code always fits. Before a tool response is written, Bridge appends a
`response_prepared` record to `agent-egress.jsonl`, including a unique `receipt_id`. It holds
hashes, counts and field paths, and one set of values: for a response that carries an error, its
`error` keeps the code, the cause when it is a code, and a voucher window's timings (requested
dates, request counts, per-part dates, rows, bytes, AlterID spans, whether each part was served,
milliseconds, and the kind of the request that failed; the last 64 parts listed and the rest
counted), taken by whitelist. For a JSON-RPC error, whose message is its code, it keeps that code. So a call that ended after the client had
timed out can be diagnosed without sending it again (bridge#799). The message and every other
field are dropped; no row content, name or narration is kept.
A call that sent requests to Tally also keeps a `request_trail` in that record (bridge#918): for
each of the last 32 sends (the rest counted, and so are the sends that failed), its place in the
call, its kind (`post` or `status`), the request's size, an outcome code (`answered`, the transport
error's code, or `send_abandoned` when a drop inside the call stopped it in flight: the read
queue's cancellation of a read, or the status probe's timeout), the response's size, and the
milliseconds the send held the endpoint's lock, so a failed read says which request failed. It
holds no request or response body, name or value, and no hash of either (a request names the
company, so a hash of it would let a reader who holds a guessed name confirm it). The request's
exact size remains, so it shows a company name's length to a reader who holds the request template.
A request that cannot be built (over the size cap) sends nothing and leaves no record. An
assistant's cancellation (`notifications/cancelled`) never drops a send in flight (a desktop
cancel can, through the read queue): a withdrawn call starts no further operation and
finishes the one it has started (a post before its intent stops after the request in flight). Such
a post is answered as cancelled, and that answer's receipt keeps the trail of its sends, with the
next read the withdrawal refused as `request_cancelled`. To match a failure with
`read_evidence`, use the place in the call and the receipt's time.
After `write_all` and `flush` succeed, it appends a `stdio_write_completed` record
with the same ID and response hash plus `bytes_written`. This confirms the local
stdio write, not consumption by the client. A missing completion leaves delivery
unconfirmed; a completion-record failure terminates the session before another
request runs. Receipt lines never contain voucher bodies. Redaction happens before a result reaches the client.
`fields_prepared` contains sorted, unique leaf paths from the final redacted,
byte-bounded `structuredContent`, including `company`, `evidence`, and `result`.
For example, `result.open_bills[].due_date` records the prepared field, never its
value. Arrays use `[]` without indices; null fields and empty collections remain
represented. Protocol refusals with no structured payload list no prepared fields.
`rows_prepared` and `bytes_prepared` describe the planned output, including its
newline. Tool-call notifications produce only a `notification_refused` record;
no response or write-completion record is produced. Existing untyped receipts
and their former `*_returned` fields remain readable historical records, but
cannot establish completed delivery. Consumers must join new records by
`receipt_id`; preparation alone is not a completed write.
All output object keys must remain server-defined; identifiers belong in values,
including when adding new grouped reports.
While a tool other than `post_import` runs, a `notifications/cancelled` naming it
stops the call before its next queued operation. An operation already started runs
to completion, including every request it makes (a paired read, its brackets and any
retries), because abandoning a request would not stop Tally; so a cancellation can
still be followed by the rest of that operation's requests. The call is answered with
`request_cancelled` and partial evidence, never with part of a read. Closing the
input is not a cancellation: the call in flight still completes. Requests other than
`ping` sent while a call runs are served after it, in order; a ping is answered at
once. Once eight requests are waiting, further input (including a ping or a
cancellation of the call) stays unread until the call ends. The lab write tools are
not cancellable.
`egress_log` reads only the final 256 KiB, in 64 KiB reverse-seek chunks, so a
larger receipt file still yields its bounded tail without loading the head.
`changed_since` is unavailable: it is omitted from tool discovery and direct calls
are refused as `changed_since_unqualified` before contacting Tally. Bounded change
enumeration and snapshot continuation have not been qualified. There is no operator
setting to enable this tool.

### Native Trial Balance

Use `trial_balance` with `company_guid`, `from` and `to` (YYYYMMDD or
YYYY-MM-DD) for native ledger totals without a voucher scan. For example, ask
for the selected company's Trial Balance from 1 April to 31 March. The runtime
requires freshly observed Licensed TallyPrime for this four-column report.
Education mode is refused before report dispatch until this complete request
has mode-specific live qualification. Dates before book start are refused.
The monetary scope also requires an INR base currency. On a book with several
Currency masters, the report covers the plain base-currency ledgers only and
names the ledgers it leaves out; its totals are not expected to balance.

Each opening, debit, credit and closing value is either
`{"state":"present","value":"-7000.00"}` or `{"state":"present_empty"}`.
Negative values are debits; positive values are credits. No empty value is
converted to zero. `totals` sums numeric observations across all returned ledgers
and carries an `empty_count` for each column. A fully observed nonzero opening
net is the difference in opening balances; Bridge does not add a balancing row.

`offset` and `limit` restrict output, not the source read. Each call captures a
fresh report; compare source evidence before combining separate pages. Native
Trial Balance uses Tally's TBAL fields, including the Profit & Loss treatment;
see [protocol semantics](../tally/TALLY_PROTOCOL_REFERENCE.md#56-native-trial-balance-fields).
It may include dormant ledgers hidden by Tally's screen. Paired response,
company, mode and extent checks detect observed changes, but do not prove an
atomic snapshot or voucher-level reconciliation. Keep the company quiet during
reads. Use `ledger_movement` with narrow dates when voucher detail is needed.

### Masters

Use `masters` with `company_guid` and one `kind`: `voucher_types`, `godowns`,
`units`, `stock_groups`, `cost_centres`, `cost_categories` or `groups`. It lists a company's masters of that
kind, for example a voucher type's numbering method (`automatic`, `manual` or
`default`, as Tally reports it) and its `active` and `optional` flags. Each row
carries `name`, `guid`, `master_id`, `alter_id` and `parent`; units add
`decimal_places` and `simple`; cost centres add `category` (null when none) and
cost categories `allocates_revenue`, `allocates_non_revenue` and `affects_stock`.
Cost centres and categories are returned whether or not the company's Cost
Centres setting is on: a book whose setting read No still returned its two
centres and two categories (one synthetic book), and a book whose setting read Yes
returned its three centres, one under another, with the same shape (a second synthetic
book), so a No setting is not "no centres", and an empty cost-centre list (one synthetic book with none defined
answered one) does not say whether the feature is off or none is defined; this
call does not return the setting. An empty cost-category list is refused
(`masters_cost_categories_empty`): the predefined Primary Cost Category always
exists. Under `mask_parties` the names, parents and categories of cost centres
and categories are masked, as godown and stock-group names are. How a voucher
was allocated to a centre is in the `vouchers` read, not here. A `groups` row carries `name`,
`parent` and `reserved_name` only, as the group snapshot returns them, and a
parent that is Tally's reserved root keeps its marker form, as in
`trial_balance`. Alias names are not returned.

The read runs inside the same company, mode and identity brackets as
`trial_balance`, with the book extent read before and after (they must be
equal) and the collection read twice and compared. Education mode is refused.
`offset` and `limit` restrict output, not the source read; a first page holds
its read in memory and a later page continues from it while the extent is
unchanged (`snapshot`, `snapshot_id`, `listing_snapshot_changed`), as
`trial_balance` does.

Godowns, units, stock groups, cost centres and cost categories are read whole only when the master-alteration
mark (`ALTMSTID`) times an assumed worst-case row for that kind fits 16 MB. The
mark counts the masters of every kind, so a book with few of this kind can be
refused. That admits a mark of at most 1,152 for godowns, 1,168 for units,
1,160 for stock groups, 1,037 for cost centres and 1,264 for cost categories; a larger book is refused before any collection request
as `masters_too_large`, with `size` (`master_alter_id`, `estimated_bytes`,
`limit_bytes`, and `limit_master_alter_id`, the largest mark this kind would be
read at). Retrying refuses again. Both stock-heavy client books measured, with
marks of about 100,000 and 300,000, refuse godowns, units and stock groups (cost
centres and categories were not tried on them); how common such
marks are across live books is unmeasured (protocol reference §12a.12). `voucher_types` and `groups` have no size check
before the read: voucher types keep the policy of Bridge's other voucher-type
read, and groups that of the group read `profit_and_loss` and `balance_sheet`
make.

The size rests on premises that are checked after the read, not assumed. For
every kind but `groups`, each row's length is checked against the assumed
worst-case row (`masters_row_exceeds_bound`), and the rows, their AlterIDs and
the response size (each collection is read twice, and the size check runs after
both reads) are checked against the mark and the admitted size: more rows
than the mark, an AlterID above it, a repeated AlterID or an oversize response
refuses the whole read as `masters_bound_premise_violated`, unless the closing
extent shows the book moved, which is reported instead
(`masters_extent_changed`). A response Bridge cannot read refuses at once with
a `masters_*` cause, without waiting for the closing extent. A `voucher_types`
answer with no rows refuses as `masters_voucher_types_empty`, because every
company has predefined voucher types; the other kinds may answer with none.

Evidence for the row shape: one synthetic book on one licensed TallyPrime 7.1
(`src-tauri/crates/bridge-tally-protocol/tests/fixtures/MASTERS_CAPTURE_PROVENANCE.md`).
`Default`, `Automatic` and `Manual` are the only numbering methods observed; any
other value is returned as `{"unrecognised": "<raw text>"}` rather than refused.
`default` is Tally's reported value, not evidence that a type numbers
automatically. The company's voucher-type count (`NUMVOUCHERTYPES`) did not
equal the rows returned on two books (35 vs 26, 33 vs 24), and on one book it
equalled the number-series count (inferred to count series, unmeasured), so the
rows are not checked against it. The completeness of the voucher-type list is
unverified: absence from it is not evidence that a voucher type is absent from
the book. The size bound rests on assumed limits (128
characters a name, four aliases a master) that no capture has measured; a row
that breaks them refuses the read as `masters_row_exceeds_bound`. No counts or
hints (the company's `NUM*` fields), stock items or writes are part of this tool.

Under `mask_parties`, godown and stock-group names and their `parent` values
are masked like a party name, because a job-work godown or a supplier-named
stock group can carry a party's name; Tally's reserved root as a parent is a
fixed marker and is left as it is. Voucher-type, unit and account-group names
are not masked: they are configuration labels, not counterparties.

### Stock Summary

Use `stock_summary` with `company_guid` and `as_of` (YYYYMMDD or YYYY-MM-DD) for
the closing stock value per stock item, with the total of those values checked
against Tally's own Stock Summary, and whether inventory is integrated with the
accounts. `as_of` must be a 31 March (a
financial-year end), the only date measured for stock, and not before the book's
start or after today. Any other date is refused as
`stock_summary_as_of_not_measured` before any request, and retrying the same date
refuses again. The only period measured is the period ending 31 March 2026
(FY 2025-26); other years' 31 March share its request shape but not its
measurement, so they are admitted but unmeasured. The period is the financial year containing `as_of`, from 1 April,
or the book's start if that is later.

A row is returned only when a check ran and held. The top-level `state` is one of
three values:

- `value_total_matched`: the items are returned. Their closing values add up to
  the sum of the top-level lines of Tally's own Stock Summary, and only that total
  was compared.
- `no_stock_items`: Tally's own stock item count is 0, the item list is empty and
  the Stock Summary is empty. `items` is an empty list.
- `not_established`: no item is returned (`items` is `null`). `reason` says why and
  `remediation` says what to do next:
  - `tally_stock_summary_differs`: the report has a total the items do not add up
    to.
  - `tally_stock_summary_shows_no_value`: the items carry a value and the report
    came back with no amount. An empty report is not told apart from one Tally did
    not render, so this is not called a contradiction.
  - `stock_values_not_comparable`: nothing could be compared (no item has a closing
    value and the report shows no amount or a total of zero, or the values add up
    to zero and the report shows no amount). Values that add up to zero against a
    report total of zero are a match.

A `not_established` result carries `unchecked_comparison` in place of `tie_out`:
its `state`, and what each side of the comparison that did not hold added up to
(`items_closing_values_added`, and `tally_stock_summary_lines_added` when the
report had a total). Nothing checked them: the closing-value total of a matched
read is the only thing `stock_summary` checks, and `checks` says so field by
field. They are for investigation only (`use` says so); neither is a stock value
or a total, and the items' side adds only the closing values present.

**Quantities are withheld.** Nothing checks a quantity yet, so none is returned.
`checks` says per field what is `checked`, `not_checked` or `withheld`: the
closing-value total is checked; each value on its own, the names, parents and base
units, whether the date was honoured and whether the item list is complete are
not; the closing quantity is withheld. A quantity Bridge could not read (a compound
unit, or a unit with a space in it) is counted in
`totals.closing_quantity_unread_count` and does not refuse the read.

Each item carries `name`, `guid`, `parent`, `base_unit` and `closing`; `closing`
holds `value` only (a plain signed decimal exactly as Tally sends it: the sign is
kept and never flipped). It is `null` where Tally sent none,
which is not zero, and is counted in `totals` (`empty_closing_value_count`); a
value Tally sent as `0.00` is a value. The opening quantity and value are read but
not returned, because their as-at date is unmeasured.

**The sign of a value.** Values keep the sign Tally sends, as in the Trial
Balance: **a negative value is a debit, which is stock held**, and Tally's own
Stock Summary screen shows it as a positive value. A positive value is what that
screen shows as negative, `(-)`. So ordinary closing stock is a negative number
here, and `value_sum` adds the values with those signs: stock held gives a
negative sum. `totals.value_sum_signs` is always `as_sent_negative_is_debit`
beside it, whether or not `value_sum` is null. This was measured on one synthetic
company on licensed TallyPrime 7.1 Silver against Tally's own screen (the protocol
reference, §12a.13). The committed captures come from a synthetic book most of
whose values are positive on the wire, that is, values Tally's screen shows as
negative.

`totals` also holds `item_count` and `value_sum`, which
is written at the scale of the values it adds and is `null` with `partial` true
whenever any item's closing value is empty. `inventory`
reports `integrated`, `inventory_on` and `batchwise` as `yes`, `no` or `unknown`,
and `basis` states only what Tally reported (`ISINTEGRATED` Yes, No or not sent),
that these are the stock items' closing values exactly as Tally sends them, that
how the books use them (as closing stock, or against a Stock-in-Hand ledger) is not
measured, and that the values of all the company's items (not only those on the
page) add up to the report's total, the only thing compared.

`tie_out` compares the sum of the items' closing values with the sum of the
top-level lines of Tally's own Stock Summary. It compares the grand total only, so
`value_total_matched` can stand beside `partial: true` when some items have no
closing value, and a match needs at least one value on the items' side and a total
on the report's.

An item valued at zero or with no value adds nothing to either total, so only
Tally's own stock item count vouches for it. That count followed the one delete
measured (one synthetic company, one sample: the protocol reference, §12a.13),
which is not proof of a complete list, so `checks.item_list_complete` is
`not_checked`. `item_count_cross_check` reports `rows`, `tally_count` and `status`
`matched`: items are returned only when the two are equal. A read whose rows differ
from Tally's count, either way, is refused as `stock_summary_item_count_differs`,
with both numbers under `counts` and the next call in `remediation`: with fewer
rows the list may be incomplete, and with more rows the count is not counting the
list Bridge read. A count
Tally did not give (missing, empty or not a number; never read as zero) refuses as
`stock_summary_read_failed` with cause `stock_item_count_unavailable`. A Stock
Summary answered without the report refuses with cause `stock_report_unknown`;
Bridge cannot tell why Tally did so. None of these three asks for a retry: every
source was read twice and the book's extent was the same before and after.

`items` (one to fifty GUIDs) filters the returned rows from the held read; a GUID
that is not found is listed under `items_not_found`. `totals`, `tie_out` and
`item_count_cross_check` always cover the whole book. A `not_established` result
or a refusal is not held, and it replaces any earlier read of the same date. On a
later page (offset > 0) without a `snapshot_id`, the call reads afresh when
nothing is held or the book moved; with one, it is refused
as `listing_snapshot_changed` (cause `snapshot_not_held`, or
`book_changed_since_first_page` when the book moved). A first page always reads
afresh.

The read runs inside the same company, mode and identity brackets as `masters`:
the company's inventory flags (a Company collection filtered to the company's GUID),
the stock items and Tally's Stock Summary are each read twice and compared, the book
extent is read before and after and must be equal, and Education mode is refused.
A book whose `ISINVENTORYON` is `No` is refused as `stock_not_enabled` before any
item is read; an absent flag is reported `unknown` and does not refuse. `offset`,
`limit`, `snapshot` and `snapshot_id` behave as in `masters`.

Small books only. The items are read whole only when the master-alteration mark
(`ALTMSTID`) times an assumed worst-case row (18,296 bytes) fits 16,000,000 bytes, which admits
a mark of at most 874. A larger book is refused before any item request as
`stock_summary_too_large`, with `size` (`master_alter_id`, `estimated_bytes`,
`limit_bytes` and `limit_master_alter_id`), and retrying refuses again. Typical stock-heavy client books refuse
today (the two measured had marks of about 100,000 and 300,000, protocol reference
§12a.12), until a counted read lands. After a read, the row count and the response
size are checked against the mark and the admitted size, and a breach refuses the
whole read as `stock_summary_bound_premise_violated`, unless the closing extent
shows the book moved (`stock_summary_extent_changed`). The stock-item request does
not fetch `ALTERID`, so the rows' AlterIDs are not checked as they are for `masters`.

Evidence: one synthetic book on one licensed TallyPrime 7.1
(`src-tauri/crates/bridge-tally-protocol/tests/fixtures/STOCK_CAPTURE_PROVENANCE.md`,
protocol reference §12a.13), and the tie once on a client book. The size bound
rests on the same assumed limits as `masters` (128 characters a name, four aliases)
and a fixed-size allowance for a row that one synthetic book has measured. No
quantity, no godown or batch split and no rates are returned. A book in which no
item has a closing value returns no item today; it waits for a capture of such a
book.

A company split by year, whose sibling companies share the GUID, is refused: the
company-flags read requires exactly one Company row for the GUID
(`company_flags_not_one_row`).

Under `mask_parties`, an item's `name` and `parent` are masked like a party name,
because stock-item and stock-group names are free text that can carry a customer's
or supplier's name; Tally's reserved root as a parent is a fixed marker and is left
as it is. `guid` is not masked and is the identity the `items` filter uses.

### Profit and Loss and Balance Sheet

`profit_and_loss` and `balance_sheet` take the same `company_guid`, `from` and
`to` as `trial_balance`, and read that Trial Balance under the same checks.
Inside the same bracket they also read the group tree and Tally's own Balance
Sheet for the window, and `profit_and_loss` reads Tally's own Profit and Loss
too (#692).

- **Lines.** Each ledger is classified by the reserved identity of its primary
  group, the last group in its chain. That identity survives renaming.
  - A P&L line is the window's debit plus credit movement.
  - A Balance Sheet line is the closing balance at `to`.
  - Signs follow the Trial Balance: a debit is negative, so a profit is positive.
  - Each line's `amount` sums the amounts Tally returned and counts the empty
    ones it left out.
  - `lines` is null while the tool's result (`net_result`, or the Balance
    Sheet's `carried`) is not established, so a derived line is never shown
    as the statement. `balance_sheet_gate` and `tie_out` then show how each
    of Tally's own lines compared.
- **When a result is established.** Only when all of these hold:
  - every ledger is classified; a ledger under a user-created primary group,
    or with an incomplete chain, is listed in `unclassified`, and blocks the
    results while it carries an amount;
  - no Stock-in-Hand ledger carries an amount, since closing stock is not
    derived from the Trial Balance;
  - Tally's own Balance Sheet for the window, read in the same bracket, ties
    line for line to the derived one (`balance_sheet_gate`). A line that
    differs, a Tally line with an amount nothing derived matches, or a derived
    line Tally does not show, refuses every result as
    `tally_balance_sheet_differs`, with those lines named;
  - the Profit & Loss A/c ledger is returned in the Trial Balance.
- **Reasons** a result is `not_established`: `unclassified_ledger_carries_an_amount`,
  `closing_stock_not_derivable_from_trial_balance`,
  `profit_and_loss_ledger_not_returned`, `tally_balance_sheet_differs`, and for
  gross and net `tally_profit_and_loss_differs`.
- **Top-level state.** `result.state` is `observed` only while this tool's
  result is established (`profit_and_loss`: both `gross_result` and
  `net_result`; `balance_sheet`: `carried`). Otherwise it is `not_established`
  and `result.reason` carries the same reason code as the nested result; the
  weaker result decides. It was always `observed` before (#984). No figure, gate
  or withheld line changes.
- **Limits.**
  - The gates are what catch what the Trial Balance cannot see, such as stock
    valued from stock items. No inventory book has been measured; one is
    expected to refuse.
  - A book with more than one currency master is refused before the Trial
    Balance is read (measured once on the lab's multi-currency book), so an
    unadjusted forex difference (#683) never reaches the gate.
  - A Tally line the derivation has no counterpart for, such as a heading with
    an amount or a difference in opening balances, refuses the results rather
    than being guessed at.
  - Tally's own statements carry no company identity; the company, mode and
    book-extent checks around the read are what bind them.
  - The gate has been measured over one full year on one book and one month on
    another. In that one-month window the book's one P&L ledger (sales) had a
    Trial Balance covering the window only, and the year's earlier result sat
    in the Profit & Loss A/c ledger's opening; the carried line includes both. A window spanning more
    than one financial year is unmeasured.
- **Gross and net** are the window's movement, which the Balance Sheet does
  not pin: stock held at `from` and gone by `to` could pass it. So
  `profit_and_loss` also reads Tally's own Profit and Loss and compares it in
  `tie_out`. Gross and net are refused as `tally_profit_and_loss_differs`
  unless it ties:
  - no line differs, and no derived line with an amount is missing from it;
  - no line of its with an amount is uncompared, except the `Cost of Sales :`
    heading, spelled exactly so, while its amount is exactly the derived
    Purchase Accounts plus Direct Expenses (the cost of sales without stock).
    That allowance was observed once, on one book. The heading is compared
    even when it reads zero or empty, as any line is: over a non-zero cost of
    sales it refuses, and over a zero one it ties (#1070).
  - An Opening or Closing Stock line refuses.

### Ledger-movement opening decision

`ledger_movement` reads the native ledger opening with `SVFROMDATE` set to the
requested `from` date, then combines it with voucher entries from the requested
window. It does not scan earlier voucher history. The ordinary ledger-master
export remains pinned to `BOOKSFROM`.

`docs/tally/TALLY_PROTOCOL_REFERENCE.md` §5.5 records account-dependent native
period semantics: observed balance-sheet ledgers carry balances into the period,
while observed nominal ledgers open at zero. The returned `balance_basis` is
`tally_period_opening_plus_direct_voucher_movement`. Calculated closing is a
period movement result, not Tally's balance-sheet `CLOSINGBALANCE` field. Bridge
uses the returned opening without guessing account classification from ledger names
or immediate parent groups. A missing opening keeps both opening and closing
unestablished, including at book start. Qualification covers the recorded account
groups and instances; it is not a claim of universal ledger-report parity.

`ledger_movement` reads the book's whole ledger catalogue twice, once before the
voucher window and once after it, whatever `ledger` names. Each catalogue read is
sent once: a read that outlived its deadline is not sent again, because the
gateway may still be building the abandoned response. A catalogue read that
outlives the deadline or passes the response cap refuses with
`ledger_movement_read_failed` and the cause `movement_catalogue_deadline_exceeded`
or `movement_catalogue_too_large`. The catalogue lists every ledger in the book, so
it does not shrink with the voucher window and narrowing `from` and `to` is not
known to help; the refusal's remediation says so. A
`ledger` that the first catalogue does not hold refuses as `ledger_not_found` right
after it, before any voucher is read.

A ledger name given to `ledger_movement`, `vouchers` (`ledger`) or the `outstandings` party detail
resolves only when it is spelled exactly as a ledger in the book, or when exactly one ledger differs
from it only in ASCII case and ASCII spaces and no other ledger differs from that one only in case or
whitespace (#1076; the case of a letter outside A-Z is not folded, reference §9.4f). A ledger's own name
(the first name in its `LANGUAGENAME.LIST`) can differ in case or symbols from the spelling its vouchers carry (26 of 4,017
ledgers in a separate census of 13 books, not reproducible from this repository); for `vouchers` and the
`outstandings` party detail either spelling is exact, a spelling that is two ledgers' is `ledger_ambiguous`,
and `ledger_match` shows the ledger's own name (with `ledger_row_spelling` when its vouchers spell it
differently), while the voucher filter and the trail still use the spelling the vouchers carry (#1085). What
the outstandings report carries for such a ledger is not measured, so a party detail that finds no bill or
no unallocated row for a ledger with two spellings carries `report_spelling: not_established`: it may mean
the report names the ledger differently. `ledger_movement` takes its names from
another report and is unchanged. Otherwise it
refuses as `ledger_not_found`, or as `ledger_ambiguous` when several ledgers differ from it only in case
or whitespace (such as a twin with a trailing line break, §9.4e). Every answer for a named ledger carries `ledger_match`:
the ledger read, `matched` (`exact` or `case_or_spacing`) and `similar_ledgers` (at most 25, with `similar_ledgers_total`; both left out under `mask_parties`), the other ledgers that
differ from an exact match only in case or whitespace. A name that only a looser reading reaches (a
dropped symbol or accent, words run together) is not read: the ledger it would reach is offered among
the candidates with the rule `lookup_key_equal`, listed first, and the user is asked. Both refusals can carry
`candidates`, from the catalogue
already read, so no request is added: each is `{name, rule}`, with no score, none marked best (the
order is by rule strength and then name, not by likelihood), and none is ever chosen for the caller. A candidate with the rule `shared_every_distinctive_token` holds every word the user typed, including words that more than a tenth of the ledgers in a book of 20 or more share (such as `input`), every typed word of under three characters (such as a GST rate like `5`) and every typed decimal (`2.5` is one word); `shared_token` holds some of them. That is a fact about the words, not a recommendation.
`candidates_listing` says what the list means: `listed`; `truncated` (more were found than fit, with the
full count in `candidates_total`, and `candidates_total_is_lower_bound` when that count is a floor;
`candidates_truncated` is true for it and for `withheld`); `withheld` (a whole family of ledgers
resembles the name and none stands out: counted, not listed); `none` (nothing resembles the name; it
does not mean the ledger is absent); `unavailable` (the search could not run, with `candidates_reason`
and no count: a book holding a name the binding rules refuse, such as a bidirectional-control or
zero-width character, or a name with too many identifiers, or a book past their bounds) or
`names_masked` (`mask_parties` hides the names; nothing is searched and no count is given, because a
count would answer "does a ledger start with this?" for every prefix a caller tries). A ledger named
by an embedded number is shown with the rule `identifier_match`, and the count is a floor
(`name_search_not_run`): the name search was not made, so it is not every ledger like the name, and
it is never used as the answer. The refusal's remediation tells the assistant to ask the user which
ledger they meant. The candidate fields that carry a list are attached only when `max_bytes` is at
least 16,384 (the list is framed twice and an error has no page to trim, so a list that does not fit
would cost the refusal its code), each list is cut to a sixteenth of the cap; the states that carry no
list (`none`, `withheld`, `unavailable`, `names_masked`) and the remediation need 4,096. A requested
ledger name that is not spelled exactly as a ledger in the book and carries `…` or `...` (Bridge
writes `…` only to shorten a masked name) is refused as `ledger_name_masked` whatever the setting is
now, because ignoring everything but its letters and digits would offer a ledger named `RARS` for
`Ra…rs`; under `mask_parties`, one that reads like the shortened form of another ledger's name (`Ra..rs`,
`Ra rs`) is refused too, whether or not it resolves. A ledger spelled exactly as asked is still reached.

The runtime retains its paired read, verified company and book-extent checks.
Native ledger openings, basic/compliance ledger balances, and native outstandings
require a freshly observed supported product and licence mode before and after
the reads. The operation then uses that mode's date-boundary profile: Education
accepts only its observed day-1/day-2/day-31 native boundaries, while Licensed
mode uses ordinary boundaries. An unsupported Education boundary returns the
operation-specific period refusal before its report is dispatched; Bridge never
rounds it. Release and licence tier remain observed facts, not blanket monetary
read exclusions. This shared runtime gate also affects native desktop consumers,
including those with an operator-supplied currency assertion. A prior status call
or cached profile does not grant admission.

The retained Education observations in protocol sections 5.3, 5.5 and 12a remain
valid within their recorded scope. Ordinary voucher reads retain their separate
literal-date and returned-row validation contract.
Every voucher, movement, presence and verification read observes the mode from the
`CompanyListV2` response of its own identity bracket, at no extra request. In Education
mode a read whose `SVFROMDATE` or `SVTODATE` is not on day 1, 2 or 31 is refused as
`window_part_boundary_unsupported_in_education` before it is sent, because Education
answers a read starting on another day with an empty collection rather than an error. A
divided window is checked whole before its first part, and a read whose closing bracket
reports Education is refused the same way, since either mode may have served it. Any
`EDUMODE` value other than `No` counts as Education even when the other capability fields
do not parse; a company list with no `EDUMODE` field keeps ordinary boundaries, and
`EDUMODE = Yes` itself has not yet been captured from a live Education instance. The end
side is held to the same rule without a live measurement in these shapes, so an Education
whole-month read ending on the 30th is refused.
A genuinely empty voucher response uses the same wider-window
corroboration as `vouchers` before zero movement can be reported. Cancelled and
optional rows establish response presence while contributing no accounting movement.

Capability profile version 4 adds the observed licence tier alongside the release.
Older serialized profiles remain readable with an unknown tier, but saved profile
reuse requires a fresh matching version-4 observation. Additive database migration
26 stores the observed tier as nullable `silver` or `gold`; historical snapshots,
including earlier version-4 rows, stay null. A tier change changes the reviewed
setup commitment. Historical commitments without a tier retain their exact bytes.

If discovery rejects company identity fields, `tally_status` reports the profile
refusal reason and partial evidence with the completed source commitments. A
valid empty collection remains distinguishable from invalid discovery.

### Purchase register (`purchase_register`)

Lists the Purchase and Debit Note vouchers of a date window that touch a ledger
under Duties & Taxes (#969), and says per entry what the books record. Nothing is
posted and nothing is inferred. It is a register of the books, not a GST return:
it does not decide input tax credit eligibility or blocked credit, matches
nothing against GSTR-2B or any portal, checks no GSTIN (`party_gstin` is returned
only when the voucher carries one), does not return `REFERENCEDATE` yet, does not
classify an item invoice's purchase as taxable, and never sums tax across heads
or vouchers.

- **Rows are selected by the ledger, not the voucher type.** A voucher is a
  candidate when one of its entries is on a ledger whose nearest predefined
  group is Duties & Taxes (by its `RESERVEDNAME`, so a renamed group or a user
  sub-group still counts). Purchase and Debit Note are the register. Every
  other voucher type that touches those ledgers (Sales, Journal, Payment) is
  listed apart in `other_voucher_types_touching_duties_taxes` with exact counts:
  whether such a voucher belongs in a return is the CA's call, not the tool's.
  A Purchase or Debit Note voucher with no entry on a Duties & Taxes ledger is
  counted in `purchase_vouchers_without_duties_taxes_entry`, not dropped. A cancelled
  voucher is listed there too, with `cancelled` true, whether or not it was taxed: the
  cancelled vouchers measured came back from Tally with no ledger entries (an empty entry
  list; protocol reference §11c.5 and §9.14), and one cancelled Purchase read this way on 1 October 2026
  (#1013). A cancelled voucher that keeps its entries is not measured. Rows are
  returned in `items` and paged by `offset` and `limit` like `vouchers` (each page re-reads the
  masters and the window, so rows can shift between pages); every ledger name in every list is
  masked when parties are masked, the same way `vouchers` masks it.
  A Debit Note can be a purchase return or a debit note issued to a customer, and a GST duty head
  does not say whether a ledger is input or output, so each row carries `party_group` (the party's
  predefined group, such as Sundry Creditors or Sundry Debtors) and the tool does not guess which it
  is. Reverse-charge journals, imports and input service distribution get no special treatment.
- **Tax comes only from the GST duty head on the ledger master.** An entry on a
  ledger with a recognised head is listed in `tax_in_books` as
  `{ledger, head, raw_head, amount}`. An entry on a Duties & Taxes ledger with
  no GST head is listed in `duties_taxes_entries_without_gst_head` and is never
  assigned one. Its `observation` says which case it is: `not_tax_ledger` is a
  ledger whose own tax type is not GST (usually TDS or another payable), and
  `absent` is a ledger with no head whose tax type is GST or was not reported,
  which may be a GST ledger whose head is missing (`tax_type` is returned).
  An entry whose head is not in the recognised vocabulary, or contradicts the
  ledger's tax type, is listed with its raw spelling in
  `duties_taxes_entries_with_unrecognised_head`. No name is ever matched and no
  amount is ever used to decide a head.
- **Amounts are as the books state them** (negative is a debit), never
  re-signed from the deemed-positive flag and never summed across heads. There
  is no direction field and nothing is called input credit.
- **Other fields.** `reference`, `party_gstin`, `is_invoice` and `post_dated`
  follow `vouchers` (absent means not observed). Cancelled, optional and
  post-dated vouchers are returned flagged, not excluded. `REFERENCEDATE` is not
  returned yet.
- **What `state` means.** The response `state` follows the rule `vouchers` and
  `voucher_presence` use (#985, #1031): a non-empty window is `complete` only when
  every voucher read was checked against a separate count of the window (a census,
  which ComplyEaze Bridge sends unless the book's voucher high-water mark alone
  proves it small, a few dozen vouchers), and an empty window when its
  corroboration read confirmed it. Otherwise it is `partial` with `reason`
  `nonempty_window_unqualified` (or the corroboration's own reason for an empty
  window), and the rows are still returned. Before #1031 the registers called a
  window nothing had counted `complete` on the company marks and the ledger
  masters alone. Those marks and masters reading the same before and after the
  window still decide whether the read stands at all (a drift refuses it). The one
  difference from `vouchers` is the reason `company_has_no_vouchers`, which
  `vouchers` returns (and records in its evidence) beside `complete` for an empty
  window on a book with no vouchers; a register returns `complete` with no reason
  in the result or the evidence. A row's `status`
  of `complete` is a different thing: every entry the voucher touches classified,
  and `state` does not change it.
- **Snapshot binding.** The ledger masters are read before the window (and the
  window is planned against their marks), the company's marks are read again
  after it, and the masters are read a second time and must classify exactly as
  the first read did, because whether a duty-head change, a re-parent or a
  delete moves the master mark is unmeasured. A difference refuses as
  `voucher_window_changed_during_read` or `ledger_snapshot_drifted` and releases
  no rows. A voucher that names a ledger the masters do not list refuses the
  same way; one the compliance read set aside for its currency refuses as
  `register_ledger_currency_excluded`.
- **Not measured.** A UI-typed purchase; item invoices whose purchase ledger sits
  in an inventory allocation (`taxable_entries` may be empty for them); books
  with several currencies; any GSTIN, `REFERENCEDATE`, or cancelled, optional or
  post-dated voucher in the captures the tests use. The captures are one
  synthetic lab book and one month.

### Sales register (`sales_register`)

The mirror of `purchase_register`: it lists the Sales and Credit Note vouchers
of a date window that touch a ledger under Duties & Taxes, and says per entry
what the books record. It is the same code with two things changed: the register's
voucher classes (Sales and Credit Note instead of Purchase and Debit Note) and
the group of the taxable ledgers (Sales Accounts instead of Purchase Accounts).
Everything else is shared: the reads, the company pin, the snapshot binding, the
states, the refusals, the paging, the masking, and the rule that tax comes only
from the GST duty head on the ledger master, never from a name or an amount.
Read the purchase register's section above for each of them. The response's
`profile` is `agent_sales_register_v1`; vouchers with no Duties & Taxes entry are
counted in `sales_vouchers_without_duties_taxes_entry`; each row carries
`party_group` and the tool does not decide whether a Credit Note is a sales
return or a credit note issued to a supplier. It decides no place of supply, tax
rate or return section, and matches nothing against any portal.

- **Measured.** `sales_register` was run against a live Tally on two synthetic
  companies (TallyPrime 7.1 Silver). On the first, once per day, for one taxed
  Sales item invoice and one untaxed one: the taxed sale came back as one row
  (voucher type Sales in the invoice view, the party as a debit entry, the sales
  ledger as its taxable entry, and two credit entries on tax ledgers whose
  masters carry the heads CGST and SGST/UTGST), and the untaxed one was counted
  under `sales_vouchers_without_duties_taxes_entry`. On the second, which has 44
  ledgers, for one Credit Note in voucher view booked on account: one row with
  its CGST and state-tax heads and its sales ledger as the taxable entry. The
  voucher windows of the first two invoices are committed, and the parsed
  windows have exactly those entries (a test); the requests are the ones the
  code sends (a test). The taxed invoice's day was read again on 2 Oct 2026
  by the build at commit 8c674d5e, with the book's own ledger masters, groups and
  company listings, and is replayed end to end from that recorded call (a test;
  the answer file carries that build's `coverage` wording). The untaxed
  invoice's day was read once by an earlier build; only its voucher window is
  committed, not its masters or the tool's answer, and a test with a Sales
  voucher whose tax entries are removed stands in for its list. The Credit Note day, and a Debit Note day through
  `purchase_register`, are also replayed end to end from their recorded calls
  (tests). One Sales accounting voucher (not an invoice) is also classified, in
  tests, against the ledger masters of the purchase register's lab book.
- **What `state` means.** The same rule as `purchase_register` (see its
  section): `complete` only when the window was counted (or an empty window its
  corroboration confirmed), otherwise `partial` with `nonempty_window_unqualified`.
  A row's `status` is separate.
- **A Credit Note keeps Tally's signs.** It is returned as a row with its signs
  reversed as Tally sends them: the tool neither nets nor flips, so a caller that
  sums tax over a window must add signed amounts. The measured Credit Note of
  1,000.00 with 90.00 CGST and 90.00 State Tax came back with the sales entry
  `-1000.00`, each tax entry `-90.00` and the party entry `1180.00`, where a Sales
  row has the sales and tax entries positive and the party entry negative.
- **The cost varies by book.** The same call sent 96 requests on a book with 8
  ledgers and one currency and 118 on one with 44 ledgers and two currencies (a
  voucher census and base-currency reads are added). The result does not report
  the cost.
- **There are two recognised state-side heads.** One is `state_tax` (raw `State Tax`)
  on one measured book and the other `sgst_utgst` (raw `SGST/UTGST`) on another. Both are
  recognised heads for the same side of the tax, so a caller must not look for
  one of them only.
- **Not shown by any run, and said so in the tool's text and in each
  response's `coverage`:** an invoice-view Credit Note; an inter-state (IGST)
  line; a cancelled or optional sales voucher; an unrecognised or missing duty
  head on a sale; more than one voucher in a window; paging; a company with a
  registration; a tax Tally computes itself (rate or HSN on the item); a sale
  typed on Tally's screen; accounting-invoice mode; a post-dated sale; a
  `REFERENCE` or a populated `PARTYGSTIN` on a sale; `REFERENCEDATE` (not
  returned); a ledger or voucher kept in a currency other than the book's base
  (the Credit Note run's book defines a second currency, but all of its ledgers
  are in the base).
- **A row of a kind no capture covers says so, where the row itself shows the
  kind.** It is returned, not withheld, with `not_measured_live` listing why:
  `invoice_view_credit_note`, `inter_state_line`, `sales_ledger_not_an_entry`
  (tax is present but no entry is on a Sales Accounts ledger: the sales ledger
  may sit in an inventory allocation), `cancelled`, `optional`, `post_dated`,
  `party_gstin_present`, `reference_present`. A row the captures cover has no
  such field, and the purchase register's rows never carry it. Some kinds a row
  cannot show, so they are never marked and are not vouched for: a sale typed on
  Tally's screen in voucher view, a tax Tally computed itself, a duty head no
  sales capture has (such as cess), an invoice of another shape than the one run
  (for example several goods lines), and a ledger or voucher kept in a currency
  other than the book's base; an unmarked row is
  not a measured one in those respects. A row is marked `inter_state_line` only
  when a tax entry's ledger master carries a recognised IGST head; an IGST
  ledger with no head, or an unrecognised head, is listed under the without-head
  or unrecognised list and the status is not complete.
  `sales_vouchers_without_duties_taxes_entry` lists such vouchers by identity
  only; the tool does not say why one carries no tax entry. A cancelled sale
  that Tally returns with no ledger entries is listed there too, with
  `cancelled` true; no cancelled sale has been read, so whether one keeps its
  entries is not measured (#1013). A Debit Note, even
  to a customer, is not a sales row: it is listed apart by identity and ledger
  names, with no amount.

### Pages of a `vouchers` window

`offset` and `limit` restrict the output, not the read: before this change every
page read the whole window again and applied the offset to the new read. A page
read that way could skip or repeat vouchers if the book changed between pages,
while each page said `complete`, and a long window cost its whole read once per
page. Now a `complete` window is held in memory (its rows after every check and
selector, unredacted; never written to disk) and a later page (`offset` above 0)
is served from it with one paired marks read in place of the window, while the
company's two marks (`ALTVCHID` and `ALTMSTID`) equal the ones read when the
window read began. Each screen action measured so far moved a mark (below), so a
change of that kind makes a later page read afresh, or refuse when it names its
snapshot, instead of serving the older read as current. Redaction and party
marking are applied to each page as it is served. The result carries `snapshot`
(`id`, `master_alter_id`, `voucher_alter_id`, `read_at`, `reused`); a served
page's `window` timings and `read_at` are the first page's, and its `evidence`
covers only the identity and marks reads it sent.

- **A named snapshot is loud.** A later page that passes the first page's
  `snapshot_id` is refused as `listing_snapshot_changed` (cause
  `book_changed_since_first_page`, or `snapshot_not_held` when the window has
  expired, was replaced, was evicted by the byte cap, or was dropped by a write
  through this server). `snapshot_id` is read on later pages only. Only a page
  whose `snapshot.reused` is true continues the earlier pages: a later page with
  `reused` false, or with no `snapshot`, is a fresh read, and its offsets may not
  continue them (a held window dropped by a write through this server, expired or
  evicted leaves no trace to flag).
- **An unnamed page says when the book moved.** Without the name, a page that
  cannot be served reads the window again, as before. If a held window of the
  same question was found and the book had moved on, the result carries
  `earlier_snapshot` (`id`, `cause` `book_changed_since_first_page`,
  `offsets_do_not_continue` true): the page is a correct read of the book as it
  is, but its offsets do not continue the earlier pages; start again from offset 0.
- **What is held.** Only a `complete` window; a `partial` one (an uncounted
  small book, a withheld foreign-currency voucher) is read again by each page.
  One window per company and question (dates, ledger, voucher-type selector,
  search, and listing or summary by grouping),
  for ten minutes after the read finished, within 64 MiB of its own, counted as
  the rows' JSON text, a proxy for memory (the ledger listings have another
  64 MiB); a window larger than that is not held and its
  result carries no `snapshot`. A write through this server drops the company's
  held windows. The desktop screen holds nothing. A later page is served only
  for the same question: the same dates (a date is the same question however it
  is written, `2026-08-01` or `20260801`), the same voucher-type selector, search and grouping and the
  `ledger` argument exactly as typed on the first page; a differently spelled
  `ledger` is a different question and reads the whole window again.
- **What a page cannot see.** A change that moves neither mark. The screen
  actions measured so far each moved a mark (§11c.5, one run each: a voucher
  delete moved `ALTVCHID` by 2, a cancel and a save with no change by 1, a
  regroup, an opening change and a ledger create or delete `ALTMSTID` by 1).
  Not shown or not established: the second `ALTVCHID` step seen on marking a
  voucher optional, whether a company feature or configuration change that
  alters export content moves a mark (enabling cost centres moved `ALTMSTID` in
  one PARTIAL run), a restored copy of the company with the same GUID and marks,
  and whether a Tally Gold remote user's save shows in the local `ALTVCHID` read
  at once. A write from the desktop app, another MCP process or Tally's screens
  never reaches this server's store; the marks read is then the only check.
- **What it costs.** A page served from a held window sends the identity read
  and one marks read in place of the whole window. Measured once on a synthetic
  book (a month of 2,542 vouchers, a release build): the first page took 68 s and
  sent 232 requests, and a later page naming its snapshot took 1.2 s and sent 10
  requests. Before this change each later page repeated the whole read.

### What a `vouchers` read cost (`window.read_cost`)

The census a window read pays follows the book's voucher mark, not the window:
on the largest book measured (mark about 1.03 million) one day is over a hundred
census reads and about 170 s (bridge#595). An assistant that reads a month day by day pays
that census once a day. When the window read took 20 s or more, or its census is
16 reads or more, the `window` of the first page of a result, and of a refusal that
carries one, has a `read_cost`. It says only what the call itself showed; it makes
no estimate for a larger or a smaller window.

- `ended` (`read` or `stopped`), `census_reads`, `vouchers_read` and
  `observed_seconds` (`marks`, `census`, `parts`, `total`) are what the window
  read did. They do not include the call's smaller reads (company check, ledger and
  type lists), and a window with no voucher is read once more, a day wider on each
  side, which is not counted either (`say` states that case).
- `floor_seconds` is **derived from the gate's rule**, not measured: the gate holds
  each request back until half a second after the previous one ended, so
  consecutive census reads are at least 0.5 s apart and a call that sent N of them
  took at least (N - 1) x 0.5 s in gaps. It is time between reads, not time spent
  sleeping (the gate sleeps only what is left of the half second after Bridge's own
  work). It is rounded down.
- `host_240` says how this window sat against the one measured host limit:
  `window_fits` with `vouchers_that_fitted` (what this window carried; it claims
  nothing about a larger or a smaller window, since fewer vouchers are not cheaper:
  a window with no voucher is read twice), `window_too_long` (this window did not
  fit; no number is given: the census follows the book's mark, not the window, so a
  shorter window saves only the time of its vouchers and how short is enough is not
  established), or `not_established` (no voucher was read, whatever the time: such a window is read
  twice, the second read wider and with its own census, which these figures do not
  include; or the read stopped). A call past 240 s on Claude Desktop is cancelled and
  its result never arrives, so `window_too_long` is seen on another host.
- A read that **stopped** (a refusal) states the floor and no verdict: a request
  that failed or hung is not what a window costs.
- `host_limits` names each host and its basis: Claude Desktop's chat app on macOS
  cancelled a silent 250 s call at 240 s in two runs on one build (calls between
  130 s and 240 s were not tried, and the call's other reads are not in the figures; and whether progress would extend the limit is not
  answered; protocol reference 11f); on Windows it is unmeasured; Claude Code
  completed a 150 s call under its defaults (CLI 2.1.285: one silent run and one with
  progress; Code tab 2.1.284: one silent run), and a 60 s per-server timeout set on a
  scratch project cut a 150 s call at exactly 60 s and was not reset by progress (CLI,
  one run per case; #703, 30 Sep; not yet in the protocol reference); an earlier 60 s
  abandon is unexplained.
- `say` is the same in a sentence, outcome first. When the window did not fit it
  also points to `trial_balance` for totals over a long period (windowed trial
  balances read no vouchers). It refuses nothing and changes no completeness rule.

The block is added only when the response can carry it: three times the smallest page
(one item) and the block, plus a kilobyte, must fit `max_bytes` (a result is carried
twice and its text copy is escaped). A page that fitted whole near the cap can lose
its last few rows to the block (it is trimmed like any field, with `truncated` and
`next_offset`): on a held window the rows left off come back on the next page, so
nothing is lost there; when the window is not held (a partial window, or the desktop
screen) the next page is a fresh read, and if the book changed meanwhile its offsets
may not continue this page's. A `summarise_by` page has buckets, not items, and is
judged as a whole page. When the block is left out the window says so
(`read_cost_left_out`). It is on a page read now: a later page served from a held
window carries the window timings and no `read_cost`, and a page read afresh is read
now. The desktop screen's voucher list shares the read and receives the
block too. `outstandings`, which also reports window timings, keeps its shape; the
other tools that read a window (`ledger_movement`, `verify_import`,
`voucher_presence`) do not report it yet; and a `vouchers` refusal raised after the
whole read carries no window.

### Search and summaries in `vouchers` (#1230)

Both work on the rows `vouchers` has already read and labelled; neither sends a
Tally request of its own, so each costs what the same `vouchers` call costs.

**Search.** `voucher_number`, `reference`, `narration_contains` and `amount` keep
the vouchers that satisfy every criterion given.

- `voucher_number` and `reference` are matched whole, ignoring ASCII case and the
  spaces around the term. Numbers repeat across voucher types, so a number can
  match several vouchers. A voucher with no reference never matches a reference.
- `narration_contains` is a phrase of at least three characters, matched ignoring
  letter case, accent composition (a composed and a decomposed accent meet) and
  runs of spaces or line breaks. Nothing else is folded: a different dash or
  apostrophe is a different character. Where narrations are withheld from the
  assistant (the `drop_narration` setting) the phrase is refused as
  `search_narration_redacted`, because answering whether a phrase occurs would
  let the assistant read the narrations back one probe at a time.
- `amount` is an unsigned decimal above zero. It is matched, by numeric value,
  against the absolute value of every ledger entry of a voucher, so it finds an
  invoice total and a tax line alike (checked live on an invoice-mode Purchase: 900.25 found its CGST and State Tax lines; an invoice total was not searched, nor an item invoice with stock lines). Each item carries `matched.amount_entries`,
  the positions in its `amounts` that equalled the amount.
- A blank or over-long term, a narration phrase under three characters and an
  amount that is not a plain positive decimal are refused before the window is read (the identity read has already happened)
  (`search_criterion_empty`, `search_criterion_too_long`,
  `search_narration_too_short`, `search_amount_invalid`).
- The search runs last, after the window is labelled and after the ledger and
  type selectors, so a zero from a `complete` window is a checked zero. A voucher
  withheld for a foreign-currency amount has no amounts to compare: an `amount`
  search keeps it (listed in `withheld_vouchers`, never as an item) and the result
  stays `partial`.
  Every other criterion is decided on the fields a withheld voucher does carry.
- A held window is reused only for the same search: the search is part of the
  question a held window answers. `voucher_types` counts (`included`, `in_scope`)
  are taken before the search, so with a search they do not add up to `total`.

**Summaries.** `summarise_by` (`ledger`, `month` or `voucher_type`) replaces
`items` with `buckets`, over the same window, selectors and search. `offset` and
`limit` page the buckets; a later page comes from the held window as a later page
of vouchers does. A summary holds its own window (the grouping is part of the
question), so a listing and a summary of the same window never replace or serve
each other.

- Each bucket has `group`, `vouchers` (an exact count; a voucher counts once in a
  bucket however many of its entries fall there), `debit` (zero or negative, as
  `ledger_movement` reports it), `credit`, `net` (debit plus credit) and
  `voucher_refs`: up to five vouchers by date, type, number and GUID, with
  `voucher_refs_complete` saying whether that is all of them. `vouchers` with the
  same arguments without `summarise_by`, narrowed to the bucket, lists the rest: for a month bucket narrow `from` and `to`; for a ledger bucket pass that ledger when the call carries none; for a type bucket pass `voucher_class` (a superset when a class has child types) or the type's GUID (a type name that is also a class is refused as `voucher_type_ambiguous`).
- A debit is an entry with a negative amount and a credit one with a positive
  amount, the rule `ledger_movement` uses (the same function); `ISDEEMEDPOSITIVE`
  decides only a zero amount, which adds nothing to either side but still counts
  its voucher. `ledger_movement` is expected to refuse the book of the live check (several Currency masters, #716; reasoned from the code, not run), so the live tie is to `trial_balance`; the two reads were not run side by side.
- A ledger bucket adds the entries on that ledger of the vouchers the window,
  selectors and search selected: with a `ledger`, a type or a search given, a
  counter-ledger's bucket holds only its entries on those vouchers, not that
  ledger's whole movement (use `ledger_movement` for that). A month or type bucket
  adds every entry of its vouchers (its debit and credit then equal in size), or, when
  `ledger` is given, only that ledger's entries; `entries_counted` says which
  (`all_entries` or `selected_ledger`).
- Every bucket has `position`, its place in the whole ordering. Under
  `mask_parties` several ledger labels can read alike (a short name masks to the
  same mark) and a masked label cannot be passed back as `ledger`; `position`
  keeps them apart and pages without a repeat. To list the vouchers of a month
  bucket, narrow `from` and `to` to that month.
- `totals` is the debit and credit the buckets add up to, `vouchers_summarised`
  the vouchers behind them. `excluded_from_buckets` counts the vouchers left out
  the way `ledger_movement` leaves them out: cancelled, optional, and vouchers with
  no accounting entry (a Stock Journal). They are counted, not hidden. Post-dated
  vouchers are summed: `post_dated_included` counts those Tally flagged Yes and
  `post_dated_flag_absent` those with no flag at all. The read asks for the flag and
  Tally asserted it on every voucher measured (reference 8.2c), so the second count is
  expected to be 0; only when it is not does a zero in the first prove nothing. A voucher type that does not post (a
  memorandum, a reversing journal, a sales or purchase order, a delivery or receipt
  note), if the book uses it and Tally exports it with ledger entries, is not told
  apart and is summed; the
  result's `basis` says so. `ledger_movement` has the same gap. The book of the live check used none of those types, so this is not seen live.
- A voucher whose entries do not sum to zero refuses the whole summary
  (`voucher_entries_unbalanced`): a bucket built from it could not be tied out.
- A voucher withheld for a foreign-currency amount is in no bucket; the result is
  `partial` and `coverage` says the totals are short by those vouchers.
- Buckets come by larger movement first (ledger, voucher type), buckets of equal
  movement in the order they are first counted in the window (never by name, which
  under `mask_parties` would show the alphabetical order of the real names), or
  in calendar order (month, `YYYY-MM`). A page also stops at a fifth of the response budget
  (the response carries it twice), with at least one bucket, and `truncated` says
  when more remain (the next `offset` is this `offset` plus the buckets returned); a page that still does not fit is refused
  `agent_response_too_large`, so lower `limit` or raise the budget. The egress
  receipt counts the buckets as the rows prepared.
- Ledger names in `group` are masked when parties are masked.

A summary over a `partial` window is only as complete as that window: `state` and `reason` say which, and `basis` does not repeat them.

**Checked once against a live Tally** (6 October 2026; TallyPrime 7.1 Silver; a debug build of master at
4c30f3f9f, which is not in a published build, with the response budget raised to 2,000,000 (the largest
result, 86 KB, fits the default); the synthetic company `BRIDGE SHAPE LAB`, 67 vouchers in
2025-04-01 to 2026-03-31 and 44 ledgers; the rows and the answers are committed as a fixture with a
provenance note, and the tests read them). Every ledger, month and voucher-type bucket equalled the
sums over the listed vouchers. The 30 ledger buckets equalled `trial_balance`'s period debit and credit
for the same year (the other 14 ledgers of the trial balance had no movement in it) and the totals
equalled its totals; the opening columns were not compared, and `trial_balance` prints `-28464.50`
where a bucket prints `-28464.5`, so compare them as numbers. Each search returned the vouchers the same
criterion selects from the listing: 9 with the reference, 4 numbered 7, 57 with a narration phrase, 1
holding an amount of 900.25 on its CGST and State Tax lines, and none for a number no voucher has, from
a window the read called `complete`. The vouchers left out of the buckets were a cancelled Purchase
that Tally exported with no entries, an optional Payment and a Stock Journal with no entries; the
voucher Tally flagged post-dated (a voucher dated after the read date could not be in this window)
was counted. A second page came from the held window in 10 small requests (4 status checks, 4
company-identity reads and 2 marks reads), with no voucher data. With `ledger` given, a month bucket
counted only that ledger's entries.

Cost, from the same run (one run, a debug build): a plain read of that year took 34 requests (12
status checks, 12 company-identity reads, 4 marks reads, the window's count as one paired read of 2
requests, and its two parts as a paired read each, 4 requests), about 7 s and about 6 MB of answers from
Tally. This book's mark (111) needed one count read; a large book needs many more. Every `summarise_by` or search call reads the window again at the
same cost (34 requests, 6 to 11 s). `ledger` adds 12 requests, four of them reads of the whole ledger
list, which grows with the ledger count. For a month or more on a large book, read ledger totals with
`trial_balance` instead: it has no month or voucher-type grouping and no search. The same year's
`trial_balance` took 34 requests and 2.5 s on this book. One day of vouchers took minutes on the largest
book measured (a voucher mark of about a million; see the cost note on window reads above, #595).

A bucket's `debit`, `credit` and `net`, and `totals`, are plain decimals with trailing zeros dropped
(`1000`, `-87900.5`); an item's `amounts` keep the two places Tally sent (`1000.00`).

Two entries (both `Round Off`, +0.50 and -0.50) carry a deemed-positive flag that disagrees with the sign
of their amount. The trial-balance tie is the same by either rule and does not tell them apart; each of
those two vouchers balances only by the sign, which is the rule the summary uses. No live entry had a
zero amount, so the `ISDEEMEDPOSITIVE` rule for a zero amount was not exercised.

Not measured: a memorandum or a reversing journal in the book; a voucher withheld for a foreign-currency
amount (none was withheld); `mask_parties` and withheld narrations; an item invoice with stock lines;
paging of a summary past one page; the `ledger` name resolution and its drift check; a book of many
thousands of vouchers and a window the read refuses; another TallyPrime edition or release; Windows; a
`reference` on a book other than this one (9 of its 67 vouchers carry one). A `voucher_class` summary was also run live and
is not kept in the fixture.

### Foreign-currency composites in `vouchers`

A foreign amount entered on a rupee ledger can be stored by Tally as a
composite, such as `-$ 100.00 @ I₹ 86/$  = -I₹ 8600.00` (#674).

- **`vouchers` withholds that voucher.** It passes every date, ledger and
  voucher-type check like any other, and is then listed in `withheld_vouchers`
  instead of `items`. The listing gives its GUID, date, type, number and cause,
  up to 100 vouchers, with an exact `withheld_total` that is the same on every
  page.
- **The result says so.** `state` is `partial` with `reason`
  `vouchers_withheld`, `total` counts `items` only (the buckets, under `summarise_by`), and `coverage` says what
  was left out.
- **No amount is read from a composite.** Anything that is neither a plain
  decimal nor an exact composite still refuses the whole window. So does a
  composite whose foreign and base amounts carry opposite signs, unless the
  foreign amount is zero: a voucher entry's two amounts share one sign.
- **Every other voucher reader still refuses such a window** (for example
  `voucher_presence`, `ledger_movement`, verify_import and the Bridge app's
  voucher screen), because each of them sums, matches or verifies amounts.

### Bill allocations in voucher reads

Each ledger entry's `bill_allocations` lists its typed allocations: `reference`
(`{"kind": "named", "name": ...}` or `{"kind": "on_account"}`), `bill_type` and
`amount`. Since #945 an allocation also carries the bill's own date and credit
period when Tally sends them (New Ref and Agst Ref allocations do):

- `bill_date` is the bill's date (`YYYYMMDD`), which for an Agst Ref is the
  original bill's date, not its voucher's. A malformed date refuses the read
  (`bill_allocation_date_invalid`).
- `credit_period` is `{"value": 30, "unit": "days"}` (units `days`, `weeks`,
  `months`). A text that is not one of those (including `10000 Days`, above the
  measured ceiling) is carried as `{"unit": "unrecognised", "text": ...}`, the
  text cut to 40 characters (characters, not bytes) with `"truncated": true`
  when it was cut. It is never read as a number of days and does not refuse the
  window.
- An element that is absent or empty is not observed: the key is omitted. It is
  never `""` or a zero period. The On Account allocations in the live captures
  carry no such element, or an empty one; if Tally sent a value it would be
  carried like any other.
- The unrecognised credit-period text is the only new free text from the book in
  the output. (The allocation's `reference.name`, the voucher's narration and
  party were already there.)
- Like every other scalar the parser reads, a repeated `BILLDATE` or
  `BILLCREDITPERIOD` inside one allocation, or a child element inside either,
  refuses the read as a protocol error (`agent_read_protocol_invalid`). So does a
  malformed `BILLDATE`. Both reach every reader that shares the voucher parser,
  not only `vouchers`: `changes` (where a refusal holds the checkpoint),
  `voucher_presence`, the empty-window corroboration read and the desktop voucher
  screen. Write-side verification is not affected, because its fetch names no
  allocation fields.
- An allocation is read only when it has a `BILLTYPE`. An untyped one is skipped
  if it is a placeholder (no name) and its `BILLDATE` is not validated; if it has a
  name, it refuses the read (`bill_allocation_field_missing`).

## Voucher-file preparation and verification

The MCPB extension exposes `verify_import` by default as a recovery tool (it
reads Tally, saves local proof files and writes nothing to Tally), and
`build_import_xml` by default because it always sets
`BRIDGE_AGENT_ENABLE_IMPORT`. A command-line installation keeps
`build_import_xml` behind `BRIDGE_AGENT_ENABLE_IMPORT=1`, or enables it with
Journal posting as described below. New file generation accepts `Journal`, `Payment`, `Receipt` and `Contra`, each
with freshly observed supported TallyPrime product and licence mode before and
after the build reads. Release and licence tier are returned as observed facts;
they do not independently refuse a file. `tally_status` reports the observed
release and licence tier; the optional status-page banner cannot supply these
facts. Every other voucher type is refused.

The four rest on different observations, and each build reports its own in
`live_evidence` rather than a single blanket claim:

- `Journal` — a licensed synthetic-lab file cycle and exact-file repeat import
  observed 2026-09-06; see [the assessment](ASSESSMENT-2026-09-06.md). The controlled
  repeat does not qualify recovery after an unknown outcome.
- `Payment`, `Receipt`, `Contra` — a licensed TallyPrime 7.1 Gold bank-statement
  import observed 2026-09-10; see
  [reference §9.13](../tally/TALLY_PROTOCOL_REFERENCE.md). These three are
  admitted with two or more entries (at least one debit and one credit, no
  ledger on both sides; more than two is bridge#466 and rests on narrower
  evidence: hand-built files of that shape were imported and read back
  over the gateway ([reference §9.3](../tally/TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md);
  a Contra only with a repeated ledger), and one Bridge-built three-entry Receipt was imported
  over the gateway and verified, but no multi-entry Payment or Contra has been, and none of the three, including that Receipt, through Tally's Import menu,
  so such a voucher reports `live_evidence` as `hand_built_gateway_readback` and
  its build result warns so) with no voucher number and no reference, and every leg on
  their money side must be a ledger whose live group
  ancestry reaches a reserved `Bank Accounts`, `Cash-in-Hand` or `Bank OD A/c`
  identity, while
  their counterparty side must be established as holding no money — money on
  both sides is a `Contra`, and an unresolvable group is refused too. A money group is admitted only where a captured ledger sits under
  it, so a ledger under `Bank OCC A/c`, which no capture carries, is
  refused on either side until one is captured. Bill-wise allocation is not supported: every party amount lands On
  Account. A build that names a ledger which keeps bills in Tally (`ISBILLWISEON` Yes)
  is refused as `bill_wise_party_unapproved` until each such party is approved with
  `on_account_approvals` (see the tool text), and the post and the queue refuse
  with `import_bill_wise_changed` a ledger that became bill-wise since.

Historical batch records remain readable. None of this qualifies every host,
licence mode, or manually imported file. In the MCPB extension an unnumbered
Journal, Payment, Receipt or Contra is eligible for native posting, one voucher
per approval (a voucher that carries a voucher number is refused); a saved batch of 2 to 50 posts in one import only in a source build
that turns that on.

1. Call `voucher_schema` and produce a payload matching its schema. Transaction
   IDs are client-supplied, unique within the batch, and retained in the local import ledger.
2. Call `validate_masters` with every ledger name. **`build_import_xml` admits
   `exact` only**, so replace the payload name for every row that is not
   `exact`, and never invent one:
   - `identifier` — the row is bound by a decisive identifier. Copy its
     `exact_live_spelling` into the payload verbatim; the import file carries
     whatever you send byte for byte. Folded names do not bind through this
     generic catalogue, even when exactly one candidate is found. Historical
     `normalized` records remain readable, but current validation does not
     produce them; revalidate against the current catalogue before selection.
   - `near_miss` — the row is **not** bound and Bridge chose nothing. Where
     `listing` is `withheld`, `candidates` is empty: there is no listed name to
     pick. This includes `master_binding_no_discriminating_candidate` and
     `master_binding_identifier_conflict`. Obtain a more complete source name
     for an indistinguishable name family; conflicting identifiers require
     correction of the source identity or explicit operator selection against
     the observed ledger list. Identifier-conflict recovery is independent of
     `listing`: a `truncated` result may show an outside name candidate while
     omitting the whole large identifier family, so choosing only among listed
     candidates is insufficient. Inspect the complete observed catalogue and
     correct or explicitly confirm the intended source identity; a fuller name
     alone does not settle conflicting identifiers. Where candidates are listed,
     each carries its comparison rule; even a single candidate still requires a
     decision. For every reason,
     render `candidate_count_is_lower_bound` as "at least N", never an exact total.
   - `missing` — no live ledger matched. Bridge never creates masters.
3. Call `build_import_xml` with the payload. It checks exact decimal balance,
   company date extent, live masters, and local journal integrity,
   repeats the full catalogue to reject intervening changes, then writes `<data_dir>/imports/<batch_id>.xml` and records an append-only
   `agent-import-ledger.jsonl` line. On a `Journal`, `voucher_number` and
   `reference` are optional: when a number is absent Tally applies the voucher
   type's own numbering configuration, and when supplied it is validated and
   sent so a Manual-type duplicate policy can reject it. On `Payment`,
   `Receipt` and `Contra` **both fields are refused** — neither element's fate
   has been observed on those types, and the bank's own reference belongs in
   the narration, which survives. A payload carrying one is rejected before any
   live read. A batch holds either Journals only, or `Payment`, `Receipt` and
   `Contra` vouchers only (those three may share a batch): a batch that mixes a
   Journal with any of them is refused as `voucher_type_shapes_mixed`, also before
   any live read, because the two are rendered in different shapes and no file
   mixing them has been imported (protocol reference §9.8 and §9.13). Split it
   into one batch of each kind (#1082). A batch that is not an amendment and holds a row another batch of
   the company already sent to Tally, or that a readback found posted, is
   refused here as `import_txn_already_posted` (described under approved voucher
   posting) and no file is written: a hand import of that file would post the
   row again. An amendment alters vouchers in place and adds none, so it is not
   checked.
   When the vouchers come from a `parse_bank_statement` proposals file, the build
   also checks the two ledgers the statement was parsed for: a `bank_ledger`
   under Cash-in-Hand is refused (`statement_bank_ledger_not_a_bank`), since a
   statement belongs to a bank account. The suspense ledger is only warned
   about, in the result's `statement_ledger_warnings` (each with a `code`):
   `suspense_ledger_outside_suspense_group` when its group is established and is
   not Suspense A/c (with the group reached), `suspense_ledger_group_not_established`
   when the ledger is in the book but its group does not lead to a reserved
   group, and `suspense_ledger_not_in_book`. A file that names neither ledger is
   not held to either rule.
   The refusal also applies when amending a batch that was built against a
   Cash-in-Hand bank ledger: the ledger must be changed first.
4. In Tally, with the intended company open, use **Gateway of Tally → Import →
   Vouchers** to import the file. Bridge does not dispatch this manual step.
   Alternatively, use the separately approved MCP voucher posting (or, for a
   Journal, the desktop posting) flow below instead of importing the file manually.
5. Call `verify_import` with the company GUID and batch ID. It reads the date
   window back, compares the exact signed ledger entries, reports missing or
   divergent rows and duplicates, writes `.proof.json` and `.proof.md`, and
   appends the verification status to the local import ledger. It compares the
   date, voucher type and entries; it does **not** compare `EFFECTIVEDATE` or
   `PARTYLEDGERNAME`, which `Payment`, `Receipt` and `Contra` files carry — see
   the limits noted in reference §9.13.

The file path is deliberately not a direct-posting path. Masters must already
exist and match exactly. File generation requires fresh supported product/mode
observations; the checks do not make a later manual import atomic with the earlier reads.
In Education mode, voucher dates must be on day 1, 2, or 31 — for every
voucher type, not only `Journal`. A different requested date is refused as
`education_voucher_date_unsupported`; Bridge does not move it.
Optional narration and reference must contain 1–2,000 Unicode characters when
supplied; omit them when unused. Control characters and the reserved attribution
marker are refused, including XML entity-encoded marker spellings. Voucher
numbers contain 1–32 characters and cannot contain controls or `$`. Lengths match
JSON Schema's character semantics; the 5 MB request-frame limit remains separate.
If verification would report any `not_found`, the supported TallyPrime product and
licence mode must have been observed before and after readback. Otherwise
`verification_mode_unqualified` withholds the absence verdict and leaves the
previous proof and status intact.
Positive historical readback remains available on an unqualified profile.
A failed profile probe remains a read failure.

Safety boundary: local loopback only, bounded responses, verified company tuple
selection, append-only receipts, and separately approved Journal, Payment, Receipt or Contra
dispatch. Unsupported:
Tally Cloud Access, every non-loopback Tally host, and change enumeration. A
`posted_verified` result is a readback comparison of the selected date window,
not live-Tally qualification or a claim that every Tally configuration or
licence mode has been qualified.

## Approved voucher posting

**Voucher posting is off by default in the MCPB extension** while four known
limits remain. Tally aims an import at a company by its name and cannot bind it to a company's GUID. Bridge's last request before the post checks that exactly one loaded company has the target's GUID and name, and that no other loaded company has the same name ignoring case and spacing; otherwise it refuses the post ([#607](https://github.com/ComplyEaze/bridge/pull/607)). A company renamed to, or loaded under, the target's name (or one differing only in case or spacing) in the moment after that check could still receive the voucher, if it has the voucher's ledgers. Bridge may flag afterwards that the loaded companies changed, but cannot always say where the voucher went, and cannot prevent it (accepted residual, [#574](https://github.com/ComplyEaze/bridge/issues/574)). A ledger renamed and replaced in that same moment means the post can land in the replacement ledger. Bridge marks the result as needing reconciliation when it sees that the ledger now resolves to a different master; a change that leaves the company's master mark unmoved, or is reverted before that check, is not seen, and a regroup in that moment is not detected ([#623](https://github.com/ComplyEaze/bridge/pull/623)). And Bridge has no tool to delete or undo a voucher it has posted, so a wrong post must be corrected by hand in Tally. It records the REMOTEID each post sends, but no delete tool exists yet ([#579](https://github.com/ComplyEaze/bridge/issues/579), [#582](https://github.com/ComplyEaze/bridge/pull/582)). And the approval window covers only ComplyEaze Bridge: another Tally connector in the same Claude Desktop that can change entries can do so without it.
The saved batch file is now checked byte for byte against the approved record
before posting ([#575](https://github.com/ComplyEaze/bridge/issues/575), fixed).
**Allow voucher posting (Journal, Payment, Receipt, Contra)** turns it on for
users who accept those risks. Existing
saved settings are respected, so an installation that saved the earlier default
may still have posting on; check the setting.
For command-line installation, set `BRIDGE_AGENT_ENABLE_WRITES=true`.
This enables `build_import_xml`, `parse_bank_statement`, `post_import` and
`acknowledge_post_review`, which asks the local user, in its own native dialog, to
record that they reviewed a post whose masters check found a changed ledger. That
record changes no verification status and nothing in Tally (#239).
`verify_import` remains available so an uncertain saved batch can be checked
after posting is turned off. `BRIDGE_AGENT_ENABLE_IMPORT=true` alone exposes
the manual file workflow and bank-statement proposal preparation, while
verification remains available without either switch.

`BRIDGE_AGENT_ENABLE_BATCH_POST=true`, together with
`BRIDGE_AGENT_ENABLE_WRITES=true`, lets `post_import` post a saved batch of 2 to
50 vouchers in one import, after one approval of the batch's summary: every
ledger's debit and credit totals, the money Receipts bring in and Payments take
out, and the standing cautions. It is off by default. It is a command-line
setting only, not in the MCPB extension, until a batch post through Bridge has
been proved on a live book. A batch is `posted_verified` only when Tally created
exactly that many vouchers, the readback verifies every one, and the company's
voucher mark moved by exactly that many. Otherwise it is
`reconciliation_required` (`batch_step_unconfirmed` when only the mark
was not confirmed). Review a doubted batch's vouchers in Tally and do not rebuild it;
`acknowledge_post_review` records that review, one doubt at a time, and changes
no verdict.

All three switches accept `true`/`false` or `1`/`0`; invalid values stop startup. No model-supplied argument can grant approval. Claude controls
its own tool-call permission prompts: Bridge cannot preselect **Always allow**
for the user. That client permission does not approve an accounting entry.

One native-approved Journal and restart reconciliation have been observed on
macOS against a synthetic Silver 7.1 instance. This remains a preview: Windows
interactive approval and native posting on Education have not been established.
Native posts on licensed 7.1 Gold were captured once, in one session on a
development build and one client book, with the approval step not recorded
([reference](../tally/TALLY_PROTOCOL_REFERENCE_VOUCHER_WRITES.md)). Also
observed on licensed 7.1 Gold is `verify_import`
returning `posted_verified` for Bridge-built Payment, Receipt and Contra files
sent over the gateway by a script rather than by this tool. That was verified
on one book, and partial on a second where larger reads failed (bridge#485); see
[reference §9.13](../tally/TALLY_PROTOCOL_REFERENCE.md).
Native posts of a Payment, a Receipt, a Contra and a three-entry Receipt have
been observed live on a synthetic Silver 7.1 company, each reading back
`posted_verified` (ADR 0004, amended 2026-09-23).

1. Validate the exact existing ledger names and build **one Journal, Payment,
   Receipt or Contra** using the file workflow above. Sales, purchases, tax,
   inventory and master creation remain unavailable. For a Payment, Receipt or
   Contra, `post_import` classifies every leg again from the ledgers' current
   parents and the group tree, before approval and again after approval inside
   the endpoint queue (before the final duplicate check and the post), and refuses with `import_bank_classification_changed` if any
   leg changed; nothing is sent. A ledger a bank cash answer named as cash in
   hand is checked again at the same two points and refused with
   `cash_ledger_not_cash_in_hand` (with `refused_ledgers`, as the build lists
   them) if it no longer reaches Cash-in-Hand: under Bank Accounts, its Contra
   would move the cash from bank to bank (#815). A batch built before ComplyEaze
   Bridge recorded those ledgers is refused with
   `import_batch_predates_cash_ledger_record`, before any Tally request; build it
   again. A batch built before ComplyEaze Bridge recorded which bill-wise ledgers
   a person approved is refused with `import_batch_predates_bill_wise_record`;
   first check in Tally whether its file was already imported by hand. A named
   ledger that is bill-wise now and was not approved at the build is refused with
   `import_bill_wise_changed`, from the same ledger-list read, before approval and
   again in the queue. Every post, of any type, is refused with
   `import_multi_currency_unsupported` if the company defines more than one
   currency: Bridge does not post into multi-currency books yet. This is checked
   before approval and again inside the queue. A Currency read that names no
   usable master refuses with `import_base_currency_undetermined`. Inside the
   queue, a change to the company's masters from just before the catalogue
   re-read to the last read before the post refuses with `post_masters_moved`
   (`post_masters_unconfirmed` if it cannot be checked); re-run the post. This
   sees only changes that move the company's master AlterID (`ALTMSTID`):
   measured for ledger renames and creates made through the gateway. A regroup,
   an edit made in Tally's own screens, and whether posting a voucher moves it
   are not yet measured. A queue catalogue re-read that does not parse as this
   company's catalogue refuses with `post_catalogue_unreadable`, whose `cause`
   names why, and nothing is sent; a repeated or unusable ledger name refuses
   again until it is corrected in Tally. A queue re-read of the group collection
   (for a Payment, Receipt or Contra) that does not parse refuses with
   `group_export_invalid`, with the same `cause` the read before approval names,
   and nothing is sent. Separately, the build records each ledger's GUID, and a
   post refuses any ledger now on another GUID (renamed and replaced, or deleted
   and recreated, since the build) with `import_masters_changed_since_build`,
   naming it. The name now means a different ledger: confirm the intended one
   (it may be under a new name) with `validate_masters` before building again.
   A batch built before this record existed is refused with
   `import_batch_predates_ledger_binding`, before any Tally request; build it
   again. The post reads the book before anything is sent and refuses a batch with
   `import_preexisting_identity` when any of its vouchers already matches a
   voucher in the book that it did not post (an earlier batch's twin, or one
   entered by hand). `error.preexisting_txn_ids` names those rows; nothing is
   sent and no attempt is recorded. Rows with the same date, type, ledgers, amounts and sides
   match the same voucher, so count the vouchers in Tally rather than reading
   every listed row as booked. Open the matching voucher and confirm it is a
   regular one (an optional or post-dated one is the user's call) and the same
   bank row, and leave the row out. If the voucher cannot be found, build the
   batch again so Bridge checks the book again (it refuses again if the voucher
   is there): the row is not entered by hand on a failed search, and if it is refused again the user decides; a row is never changed to get it past the check. Only a
   genuinely different transaction that shares the fingerprint of a voucher
   that was opened and confirmed is entered in Tally by hand. Build the other
   rows again so they post; a rebuilt batch can be refused again, naming rows
   the first answer did not list. Cut inline batches on whole days so same-day
   rows of one amount are not split across batches. The list and step are
   withheld on a very small response budget, and the same refusal inside the
   queue, after approval, carries no list. Any other read inside the queue that fails before the post is refused
   with `post_queue_read_failed`, with a `cause` where one is known; nothing is sent, and
   the post can be re-run. Checked under the admission lock as the attempt is
   about to be recorded, a batch no longer in the journal, already attempted,
   changed since approval, or whose REMOTEID the journal already records refuses
   with `import_batch_not_found`, `import_already_attempted`,
   `import_batch_changed` or `import_remote_id_reused`, and this post sends
   nothing. An approval withdrawn after this call took it, and before it is
   spent there, refuses the same way with `import_approval_revoked` (#791).
   Rebuild only when `attempt_recorded` is `false`, except after
   `import_txn_already_posted` (below), where a rebuilt row is refused again.
   A batch holding a row that another batch of the same company already sent to
   Tally, or that a readback found posted, refuses with
   `import_txn_already_posted` at build, before the approval dialog and again
   under the lock (#876). Two vouchers are the same row when they share a
   transaction id and either the id is the `st-YYYYMMDD-<16 hex>` form a
   bank-statement build derives (it survives a change of ledger) or their date
   and amounts agree. Do not rebuild the row (except as the exit for a repeated hand-typed id below says): verify the
   earlier batch, which the error's `blocking_batch_id` names (it can be absent
   when the journal was busy for the lookup; then verify the company's recent
   batches). For an overlapping statement, rebuild without the rows already
   posted. The refusal is never lifted by Bridge: if Tally rejected the earlier
   batch and the voucher is not in Tally, the user enters it in Tally.
   Limits: a
   batch imported by hand through Tally's Import menu counts only after
   `verify_import` has recorded `posted_verified` for the whole batch, so one
   whose readback is divergent or incomplete does not count; a
   batch that was sent and failed also blocks a rebuild of the same row; a
   rebuild that renames a hand-typed transaction id, or keeps it and changes
   the date or amounts, is not seen, and a hand-typed id in the
   `st-YYYYMMDD-<16 hex>` form is matched on the id alone; a different export
   of the same statement (other amount formatting or narration wrapping) derives
   different ids and is not seen; a hand-typed id reused for a genuinely
   different event is refused too. The next step is advice, not a control: the
   agent does not decide whether the refused row is the same transaction as the
   posted voucher or a second real one that shares its id, date and amounts. It
   asks the user to open the existing voucher in Tally, compare, and say which.
   If it is the same and its ledger or narration is wrong, the posted voucher is
   corrected in Tally (or amended, for a batch imported by hand); if it is a
   second real transaction, it is rebuilt under a new id. Bridge does not check
   that answer (for a `posted_verified` voucher `verify_import` returns no date,
   amounts, ledgers or narration), and nothing binds it to the rebuild. A statement row that is re-entered inline
   under any other id, including a `st-` id with a suffix, is a hand-typed id
   and is not seen; two Bridge installs on one company keep separate
   journals. The check at build is a point in time: a batch posted
   natively after this one was built sees it only as built, so a file already
   written can still be imported by hand after that post, and two hand imports
   of one file are not seen at all. A build that is an amendment is not checked,
   and a row that no earlier batch holds by id is not seen. A proposals file is
   built whole, so an overlapping statement is rebuilt without the posted rows
   only by parsing it again with a narrower `from` and `to`; those are whole
   days, so a day that holds both a posted row and an unposted one is left out
   whole and its unposted rows are entered in Tally.
2. Call `post_import` with the original `company_guid` and `batch_id`.
3. Review the native dialog's company, endpoint, date, numbering, reference,
   narration, every debit/credit entry, and totals; for a bank voucher, also the
   side that must be bank or cash. A batch's dialog shows the same company and
   endpoint, and summarises the vouchers: their count, types and date range,
   each ledger's totals, and the overall totals. It does not show any voucher's
   own date, amounts, entries, narration or reference: equal ledger totals do
   not prove each voucher is right, so check those before building the batch.
   Choose **Post voucher** (for a batch,
   **Post N vouchers**) on macOS or **Yes** on Windows to permit this attempt. **Cancel** or Escape
   declines on macOS; Return may leave the dialog open. Windows defaults to
   **No**. Long or directionally ambiguous previews are refused; use the
   manual file workflow instead. A desktop session is required.
4. Bridge refreshes company identity, product/mode, date admission and exact
   masters, checks the batch is absent, then records a durable dispatch intent
   before one POST through the existing serial Tally queue. It saves response
   commitments/counters and performs mandatory accounting readback. Only a
   clean create response together with matching readback confirms the first
   posting as `posted_verified`. Unless the company's master AlterID is proven
   unmoved across the POST, Bridge re-reads the ledgers and reports it in
   `masters_after_post`. If an approved ledger no longer resolves to its
   approved GUID (`posted_under_changed_masters`), or that check cannot be
   completed (`masters_after_post_unconfirmed`), the voucher is in Tally but the
   result is `reconciliation_required`: review it in Tally, correct it there if
   needed, and do not rebuild the event. Bridge records the check with the
   batch. A changed ledger stays reported on every later `verify_import`, even
   after the voucher is corrected in Tally; a check that could not finish is
   finished by the next `verify_import` that finds the voucher. If Bridge cannot
   record the check, the post is refused before anything is sent
   (`post_masters_record_unavailable`).

Keep the selected company free of other imports and ledger changes while posting,
and leave Tally's product/licence mode unchanged. Bridge serializes its own writers;
its separate checks cannot lock out Tally UI edits or other importers. Concurrent
external changes are outside this preview's validated posting workflow.

Cancel, client disconnect, or the two-minute approval timeout ends the pending
approval. (The tool call itself returns "approval pending" after about 40 seconds
and the dialog stays open (for up to the two minutes); calling `post_import`
again with the same batch waits on that same dialog.) If dispatch has already begun, cancellation cannot undo Tally's
work. A timeout, crash, malformed response or incomplete readback requires
`verify_import` on the **same original batch**. Once dispatch intent exists,
`post_import` only reconciles and never resends, including after process restart.
Each post sends a fresh `REMOTEID`, and one that any recorded dispatch intent
already carries is refused as `import_remote_id_reused` before any Tally request
(protocol reference §9.3: a resend can undo a person's cancel or delete).
If another process holds import admission, the call returns `import_admission_busy`
without waiting for that process or scheduling a later post. Reconcile any
recorded attempt before requesting another action. Confirmation requires all seven
result counters to be explicitly observed: `CREATED`, `ALTERED`, `DELETED`,
`IGNORED`, `ERRORS`, `CANCELLED`, and `EXCEPTIONS`. Missing counters remain
incomplete evidence even when voucher readback matches. Older saved responses
without these presence records remain readable but cannot establish a clean
response. Keep the original history for investigation; never guess missing
presence records or resend a Journal to obtain a new receipt.
An intent may exist even if the request never reached Tally: this is deliberately
an unknown outcome, not permission to build a replacement voucher. The saved
response metadata helps distinguish clean counters from readback alone.

A native post writes the narration as given, print-ready, with no `[BRIDGE:…]`
tag. A file built for a person to import by hand keeps the tag, because Bridge
never sees that import.

The posted vouchers are identified by their place in the post's own range of
Tally AlterIDs (protocol reference §9.15). The dispatch intent records the
company's voucher mark from the last read before the POST, and the readback
binds each voucher to the Tally GUID its POST created. Binding needs a clean
response (`CREATED` equal to the voucher count, and `LASTVCHID`), the mark after
the POST moved by exactly `CREATED`, and the vouchers in that range in the order
sent with the content sent. If the mark after the POST could not be read, the
range the clean response implies is used instead. A post whose response was lost
is never bound. The binding, or a refusal of it, is recorded once with the
batch: a refusal is final, while a read that failed is not, and the next
`verify_import` tries again. A bound voucher is then verified by its GUID
(`"marker": "post_span_binding"`).

The readback reports, each as `reconciliation_required`: a bound voucher the
window no longer holds as `bound_not_in_window`, never `not_found`; and, for a
company whose voucher mark reads below the mark the post left, every voucher it
no longer holds as `book_rolled_back`, never `not_found`, with nothing bound in
it. Such a book was rolled back (a backup restored, or another copy put in its
place). The check sees a rollback only while the mark reads below the post's: a
book keyed past it again after a restore is not seen as rolled back, and if
Tally then gave new vouchers the MasterIDs the post's vouchers held (unmeasured),
a bound voucher names another voucher, which reads `posted_divergent`, or a false
`posted_verified` when its content is the same (bridge#1050). Bridge still records those rows as posted, so a rebuilt batch holding
them is refused as `import_txn_already_posted`; re-entering them in that book is
the person's decision. The result's `post_span_binding` names the binding's
`state`: `bound`, `refused` with its `code`, `unsettled` with its `code` (for
example `binding_effective_date_not_observed`, when the read left out a
Payment, Receipt or Contra's effective date: never refused for it, and decided
again by the next verification),
`not_bound`, `book_rolled_back` or `not_applicable`.

A native post that sent no tag is never attributed by one: a row carrying its
batch's tag is a hand import of the batch's file, matched by content only. The
binding compares narration byte for byte. A narration holding the one sequence
the agent readers are known to rewrite (a literal U+FFFD followed by `#`, digits
and `;`) is refused when the batch is built (`voucher_text_invalid`), and
`post_import` refuses a batch saved before that check in the same way, before
any request; it is still admitted for review and reconciliation. Other text,
such as Devanagari or the rupee sign, is admitted, and whether it reads back
byte for byte is not yet measured: a narration that reads back changed refuses
that post's binding for good.

A voucher of an untagged native post that was not bound (its binding refused or
its response lost), and that its content no longer finds (for example after an
edit in Tally), is `sent_not_attributed`, never `not_found`. In the post's own
readback only, when its own answer from Tally reported every counter, created
none of the vouchers sent and reported one exception for each, with nothing else
counted, and none of them is found, they are `tally_reported_not_created`
instead (bridge#1108). For a batch this also needs the company's voucher mark
read on both sides of the post and unmoved. The person is told to check that
each is not in Tally and enter it there by hand, not through Tally's Import
menu. A later `verify_import` never reads the post's answer, since someone may
have entered a voucher by hand and edited it since: it reads
`sent_not_attributed`. A partly created batch is never read as not created: a
count does not say which voucher Tally rejected, and two vouchers of one batch
with the same content defeat matching by content. A binding refusal is final: an
edit to one voucher of a batch in Tally before the binding is made
(a deferred bind, or a later `verify_import`) refuses it for the whole batch,
whose vouchers are then matched by content only. Such a batch stays
`reconciliation_required`: the person checks its vouchers in Tally, and
`acknowledge_post_review` does not apply to it, because it records a review only
of a doubt beside vouchers that read back verified (closing such a batch inside
Bridge is bridge#1039). `voucher_presence` cannot identify a native post's
vouchers, because they carry no marker: one edited or re-dated in Tally can read
`absent` there. Check a natively posted batch with `verify_import`, which finds
its vouchers by the GUIDs its post created once its binding is made (and
otherwise reports them as never absent), before posting any of them again.

If the last read before the POST does not yield the company's voucher mark, the
post is refused as `post_mark_unrecorded` before its dispatch intent is recorded
and before anything is sent,
and the approval is withdrawn, so the next call asks again. Known limits:
identical vouchers in one batch are bound by position alone, since they are
identical in content and their own order cannot be observed (the request order of
vouchers that can be told apart was measured in two raw runs); the local journal is the trust root for the bindings (a lost journal
leaves the batch unknown, `import_batch_not_found`; in an edited journal, a
bound voucher's GUID, MasterID and content are still read against the book,
but not whether this post created it); and whether a write from another Gold
user's process can share or skip the mark Bridge reads is unmeasured. Open
follow-ups: the hand-import file still carries the tag (bridge#1037); re-posting
the rows of a rolled-back batch needs the person's approval (bridge#1038);
closing a batch whose binding was refused (bridge#1039); and detecting a restore
keyed past the post's mark (bridge#1050).

Posting binds the saved batch to its loopback endpoint and full company tuple.
Legacy batches without that endpoint binding remain readable/verifiable but
cannot be posted. Only a uniquely selectable loaded company is admitted.
Existing batch files and proofs keep their formats; new journal fields are
optional on read, and no database migration or background queue is introduced.
Disabling the switch and restarting the connector removes posting from tool
availability without deleting reconciliation evidence.
**Keep this connector version for recovery.** The journal reader refuses a
dispatch or status record, or a voucher, carrying a field it does not know; a
saved batch's own record can carry fields an older connector does not read. So
after a native post attempted with 0.4.2 or later, a connector older than 0.4.2
refuses the whole journal, including reconciliation of batches it
wrote itself. Since bridge#579, each native dispatch intent records the
REMOTEID it sent, which 0.2.0 and earlier do not know. From the first post attempted
with 0.4.2 or later, the dispatch intent also records the pre-POST voucher mark and
the journal a binding record, which a connector older than 0.4.2 refuses: do not
downgrade after posting with it. A downgrade before that first post leaves the
journal readable, and versions 0.3.0 to 0.4.2 then refuse to post a batch this
version built (`import_batch_predates_ledger_binding`, nothing posted): they
cannot make the cash-in-hand and bill-wise checks it was built with. Their
message for that refusal says "Build the batch again"; it was written for
batches older than they are, and for a batch this version built it is the wrong
step. Reinstall this version (or a newer one) and post the batch from it. What
an older version builds and posts itself, the same batch built again included,
has neither check. 0.2.0 and earlier have no such refusal: do not run them over a
data folder this version has built in.

This is a bounded first posting slice, not blanket host/licence qualification.
A ledger mapper is unnecessary for exact existing names: `validate_masters`
returns exact matches and bounded near matches. Resolve ambiguity with the
user rather than silently creating or choosing a ledger.

## Review and post from the ComplyEaze Bridge app

1. Build one Journal with `build_import_xml` as above. Keep its original XML
   file and local batch history on the same computer. The desktop app and MCP
   use the same default data directory; a custom `BRIDGE_AGENT_DATA_DIR` must
   be the same for both processes.
2. In Bridge, set the same Tally host and HTTP port used to build the file.
   Open **Review Journal file** from the overview and choose the original XML.
   Selection is local: it does not contact Tally. Bridge accepts only bytes
   matching a single saved, admitted batch and its original private file.
3. Review the company, date, reference, narration, ledger entries and totals.
   Choose **Post Journal**, then review and approve the independent native
   dialog (its button reads **Post voucher**). The app uses the same validation, dispatch and readback service as
   MCP. Changing app connection settings cannot redirect an open review.
4. If an attempt is already recorded or its outcome is uncertain, use
   **Reconcile original batch**. This action only reads and cannot open an
   approval dialog or send an import. Confirmation needs both the original
   clean response and matching readback. Keep the original batch when recovery
   is inconclusive; do not rebuild it as a retry.

Bridge prevents Journal posting during a Core Accounting snapshot, including
snapshots in another updated Bridge process using the same operating-system
account and Tally port. Wait for the snapshot to finish or cancel it before
posting. If a snapshot start or resume result is uncertain, use local evidence
to restore monitoring or cancel the active run; connection settings stay locked
until that uncertainty is resolved.

A snapshot does not itself mean a Journal was attempted. Bridge can still
check the saved local history during a snapshot; an active posting process
keeps that result uncertain until its attempt can be observed safely.

An XML file alone is not portable posting authorization. Files generated
elsewhere, edited files, legacy unbound batches and unsupported voucher types
are refused. This first desktop flow has no Journal editor or arbitrary XML
importer. It adds no persisted format beyond the shared posting service.

## Evidence-shaped outputs

Every tool response carries the same outer evidence envelope. This is an
illustrative response shape from the synthetic simulator test; it is not a
live-Tally compatibility claim:

```json
{
  "company": {"name":"BRIDGE SYNTHETIC BOOK","guid":"00000000-0000-4000-8000-000000000001","identity_state":"verified_tuple"},
  "read_at":"2026-09-04T00:00:00.000Z",
  "evidence":{"request_sha256":"…","response_sha256":"…","bytes":123,"state":"complete"},
  "truncated":false,
  "result":{"companies":[{"name":"BRIDGE SYNTHETIC BOOK","guid":"00000000-0000-4000-8000-000000000001"}]}
}
```

For Tally reads, request commitments hash the transmitted request body (UTF-16LE
for XML; empty for the status GET), and response commitments hash encoded response
bodies. Multiple sources combine their commitments in read order.

A `request_sha256` or `response_sha256` is therefore the hash of one request or
response only where exactly one source was read. Wherever evidence covers more
than one source, which includes the top-level `evidence` of most tools and named
sub-evidence such as `verify_import`'s `mode_opening`, the field is a combined
commitment: `sha256("<left>:<right>")` over the two lowercase hex digests, folded
left to right in the order the tool's code combines them. No request on the wire
has that hash. To check one against captured wire bytes, hash each captured body
and fold the digests in that same order; probes and paired reads interleave, so
the order is the code's, not the wire's. Not every request has its own digest in
the result: on a lab capture of one `verify_import`, the company-mark and census
requests were folded into commitments without being reported separately (#726).
The runtime's own combination passes a side through unchanged when the other
side's two digests are both empty, so runtime evidence that one source alone fed
carries that request's own hash; the tool-level combination always hashes both
sides. The result does not say which case produced a given value, so a digest
that matches no captured request should be treated as a combination, and one
that matches a captured request as that request.
`evidence.bytes`
counts committed response bodies, including both accepted bodies of a paired read;
it excludes auxiliary health and identity guards and is not total network traffic.
Status commits its status and company-discovery responses. Scoped agent reads use
a single attempt. Company discovery can retry transient failures; its commitments
describe the terminal attempt, excluding earlier attempts. Read failures retain
source observations already returned to the connector, including when parsing or
window validation fails. Runtime-internal requests that fail without returning
source evidence are not fabricated; zero retained bytes does not establish that
no HTTP request was attempted. Local-only tools and refusals without retained
source observations carry local evidence.

`outstandings` returns the runtime's paired native result. Its `result.as_of` (YYYYMMDD) is always the date read as of, in every state: the caller's `as_of`, or this computer's date when it was left out. `tally_status.today` is that date. A complete read has
billed totals explicitly scoped to open bills, four overdue-age buckets, an
`unaged` bucket for future-due or unobserved ages, top parties,
open bills, and unallocated counts and directional totals; a refused runtime read instead has `state: "partial"` and its
exact `partial_reason`. A Bills report row whose dates Bridge cannot read refuses the whole
read (leaving a bill out would change the totals) with its `cause` (a typed code
for the rule that failed), a `bill_row` (`report`, `receivable` or `payable`, and
the 1-based `row` in the order Tally sent them: never the bill's party, reference
or date) and a next step. A refusal that is not about one row (an amount, the
shape of the report, the book window) has a `cause` and no `bill_row`. A due date
printed with a four-digit year of 2100 or later is read as written; no other form
is added (protocol reference section 12a.3, one observation). `ledger_movement` returns literal-window voucher
movement with exact decimal `opening`, `debit`, `credit`, `closing`, parent,
and `vouchers_touching`. `ledger_masters` accepts `fields: "compliance"` to
return the paired party-master GSTIN/PAN/MSME/bank/IFSC/email/phone/state and
address observations; `mask_parties` redacts the ledger name before it leaves
the server. Each `ledger_masters` row's `opening_balance` is the opening at the
start of the company's books, and `opening_balance_as_of` names that date (the
admitted `BOOKSFROM` the request pins). On a book holding several years it is
not the current year's opening: for a period's opening, use `trial_balance` or
`ledger_movement` with that period's `from`.

The unavailable `changed_since` implementation must not be used as
change-enumeration evidence; its retained internal response states that
deletion detection is unsupported.

### What each tool's evidence covers

What a tool's top-level `evidence.request_sha256` and `response_sha256` cover,
read from the code tool by tool (#726). It covers the read tools, `verify_import`
and the tools that read nothing from Tally; it does not cover
`build_import_xml`, `post_import` or `acknowledge_post_review`, which also send
requests. Each list is in fold order. A step
marked "(if …)" is folded only when that holds. Every step after the first is
joined with the tool-level combination, so it is hashed even when one side is
one request. The building blocks:

- **Company read.** The company-list request every company-scoped tool makes
  first, after the probe for `verify_import` (`Server::companies` in
  `src-tauri/src/agent_company.rs`). It is sent
  twice as a paired read and reported as that one request's own hash. Only the
  last attempt of a retried read is reported.
- **Scoped read.** One admitted read (`Server::post_read` in
  `src-tauri/src/agent.rs`), reported as that one request's own hash.
- **Probe.** The status GET (an empty body) folded with the company-discovery
  POST by the runtime combination (`probe_with_wire_evidence` in
  `src-tauri/src/tally/connection.rs`). If the GET failed, it is the POST
  alone. If the newer company list did not parse, the legacy one is folded in
  third.
- **Runtime read.** A read built inside the runtime and folded with the
  runtime combination: the opening probe, the read's own requests in the order
  listed, then the closing probe. The `outstandings` read returns early as
  partial with the opening probe only when `as_of` is before the books begin or
  its snapshot period is partial.
- **Window read.** A bounded voucher window (`read_voucher_window_timed` in
  `src-tauri/src/agent_voucher_window.rs`): the company's voucher marks (when
  not already known), the census spans (when the marks alone do not bound the
  window), the data parts, then closing marks (only when the window was
  divided). Each is folded with the tool-level combination.
- **Extent check.** On a later page of a listing, the two book-extent requests
  that decide whether the held read can be reused, folded with the runtime
  combination (`continued_listing` in `src-tauri/src/agent_ledgers.rs`).
- **Corroboration.** The window read again, one day wider on each side, with
  the marks already known, plus a scoped marks read if that window is empty
  too (`corroborate_empty_voucher_read` in `src-tauri/src/agent_vouchers.rs`).

These requests are sent but never digested anywhere in a result:
- the identity-bracket company-list requests around each scoped and runtime read;
- the status checks between and after a paired read's two sends;
- the book-extent reads inside a runtime read;
- the earlier attempts of a retried read;
- a `verify_import` masters-check catalogue read that fails, which is sent and
  then dropped.

Per tool:

- `list_companies`: the company read alone. This is one request's hash, not a
  combination (`src-tauri/src/agent_company.rs`).
- `tally_status`: the probe alone (`Server::status`,
  `src-tauri/src/agent_company.rs`).
- `masters`:
  1. the company read;
  2. the extent check (if a later page);
  3. (if not served from a held read) a runtime read of the requested kind's
     collection, paired (`fetch_masters_with_extent` in
     `src-tauri/src/tally/runtime_masters.rs`).
- `ledger_masters`:
  1. the company read;
  2. the extent check (if a later page);
  3. (if not served from a held read) a runtime read
     (`src-tauri/src/agent_ledgers.rs`, `src-tauri/src/tally/runtime.rs`).
  - With `fields=basic` the runtime read is the currency masters, the ledger
    export, and (if a `group` filter is given) the group collection. It is
    sent once, as `ledger_movement`'s catalogue is: a read that outlived its
    deadline is not sent again, and the call refuses as `ledger_export_invalid`
    with the cause `request_deadline_exceeded`.
  - With `fields=compliance` it is the base-currency read, folded with a
    source read (`fetch_agent_party_ledger_masters_with_evidence`). The source
    read is the opening probe, then one commitment over the master, balance
    and group reads, then the reads that counted the ledgers first, if any,
    then the closing probe. Those counting reads are the census slices and the
    ledger-count cross-check (when the read was admitted census-first), and
    the catalogue read (when it was admitted count-first, or census-first with
    a read split by parent group).
  - That commitment's request digest hashes the joined request hashes
    (`fetch_party_ledger_master_source` in `src-tauri/src/tally/connection.rs`).
    Its response digest hashes `"<master>:<balance>:<group>"`
    (`party_ledger_master_source_evidence` in `src-tauri/src/tally/runtime.rs`).
    The master and balance sides are themselves aggregates when the read was
    split by parent group; the group side is one paired read.
  - The count requests are sent before the reads they admit but folded after
    the commitment. None of the individual master, balance or group requests
    is reported on its own.
- `validate_masters`: the company read, then one scoped read of the ledger
  catalogue (`src-tauri/src/agent_import.rs`). Its `catalogue_evidence_sha256`
  hashes the parsed catalogue, not a request or a response.
- `vouchers` (`src-tauri/src/agent_vouchers.rs`):
  1. the company read;
  2. the ledger catalogue (if `ledger`);
  3. the window read, which always reads the marks;
  4. the corroboration (if no rows);
  5. the ledger catalogue again (if `ledger`);
  6. the voucher-type catalogue (if a named type selected nothing).

  A later page of a window held from an earlier read (#485) is served from it
  while the company's marks are unchanged. That page reports the company read
  and one scoped read of the marks, nothing else (`serve_voucher_page`). If
  the marks moved and no `snapshot_id` was named, the page reads afresh, and
  that marks read is folded second, before step 2.
- `voucher_presence` (`src-tauri/src/agent_presence.rs`):
  1. the company read;
  2. the ledger catalogue;
  3. the window read;
  4. the corroboration (if empty);
  5. the ledger catalogue again.

  Its `catalogue_evidence_sha256` hashes parsed names, not a request.
- `ledger_movement` (`src-tauri/src/agent_movement.rs`):
  1. the company read;
  2. a runtime ledger read (currency masters, ledger export);
  3. the opening window's marks and census;
  4. its data parts, with the corroboration folded in when empty;
  5. its closing marks (if divided);
  6. the runtime ledger read again;
  7. the replay window's data parts, with the corroboration folded in when
     empty (no marks read: it reuses the first window's);
  8. the replay window's closing marks (if divided).

  The corroboration is sent after the closing marks but folded before them.
- `trial_balance`:
  1. the company read;
  2. the extent check (if a later page);
  3. (if not served from a held read) a runtime read
     (`fetch_trial_balance_sources` in
     `src-tauri/src/tally/runtime_trial_balance.rs`): the currency masters,
     (if the book has several currency masters) the currency-name reads that
     find the base, then the trial balance.
- `balance_sheet` and `profit_and_loss`: the company read, then a runtime read
  (`fetch_statements`, same file): the currency masters, the trial balance,
  the group collection and the Balance Sheet. `profit_and_loss` adds the
  Profit and Loss statement.
- `outstandings` (`src-tauri/src/agent_outstandings.rs`):
  1. the company read;
  2. the base-currency read;
  3. a runtime read of receivable bills, the group collection, payable bills
     and the ledger collection (`fetch_outstandings_native_with_currency` in
     `src-tauri/src/tally/runtime.rs`);
  4. (if `party` with `detail` on a complete read) the ledger
     catalogue and a window read (`outstandings_detail_within` in
     `src-tauri/src/agent_bill_trail.rs`).
- `stock_summary`:
  1. the company read;
  2. the extent check (if a later page);
  3. (if not served from a held read) a runtime read of the company's
     inventory flags, the stock items and Tally's Stock Summary
     (`src-tauri/src/tally/runtime_stock_summary.rs`).
- `purchase_register` and `sales_register`, which send the same requests
  (`register` in `src-tauri/src/agent_register.rs`):
  1. the company read;
  2. a masters read, as `ledger_masters` with `fields=compliance`;
  3. the window read, with the marks already known from that masters read;
  4. a scoped read of the closing marks;
  5. the masters read again;
  6. the corroboration (if no rows).
- `verify_import` (`verify_import_with_dispatch` in
  `src-tauri/src/agent_import.rs`):
  1. the probe;
  2. the company read;
  3. the window read's marks and census, data parts and closing marks;
  4. the corroborating window's data parts and closing marks (no marks read:
     it replays the first window's);
  5. a scoped read of the company's marks (if the batch was posted natively
     and its voucher mark before the post was recorded, `current_voucher_mark`);
  6. the closing probe (if any voucher was not found);
  7. the ledger catalogue (if the saved masters check runs and the read
     succeeds; when it fails, nothing is folded and the check reports
     `check_unavailable`).

  `evidence.mode_opening` is the opening probe and `mode_closing` the closing
  one (or null). `company` is the company read. `voucher_read` is the first
  window's data parts only, and `voucher_read_corroboration` the second's.
  No named key covers the marks or census requests, which is what the lab
  capture in #726 showed. A later page read with `proof_sha256` sends no
  request: its request digest hashes
  `verify_import_page:<batch_id>:<offset>` and its response digest hashes
  the saved proof.
- `changed_since` is refused before any request, so it carries only the
  refusal's local evidence.
- `egress_log`, `local_data_report`, `parse_bank_statement`, `read_evidence`
  and `voucher_schema` read nothing from Tally. Their request digest hashes the tool's name, and
  their response digest hashes what they return (for `voucher_schema`, the
  name again).

A refusal made before any request reports a request digest hashing
`<tool>:<sha256 of the arguments>` and a response digest hashing the refusal
code. A refusal after some requests keeps what was folded up to that point.
A later page of `masters`, `ledger_masters`, `trial_balance` or `stock_summary`
served from a held read reports only the company read and the extent check,
not the first page's reads. A served `vouchers` page reports the company read
and its marks read instead (see `vouchers`).

## The plain headline

A result may carry a top-level `headline` beside `result`, in words and built only from the typed
state the tool already has (never from the result's text): `lead` names the company (in quotes), the
exact period (`1 Apr 2026 to 2 Sep 2026`, never `01/04/2026`) and the state, and `rows` says which rows
this response lists. A read with any gap is `Partial` and its lead names every gap, with counts; a
read with none says it covered every ledger. The type that decides this cannot build a whole read
beside a gap. The headline sorts ahead of `result` in the serialized form (the keys of a response
are in alphabetical order), so it is read before the figures. When a byte cap trims the page of rows
the headline lists, the `rows` sentence is restated from the rows that are left, and `page` (`offset`,
`shown`, `total`) keeps the numbers it is made from; a headline that cannot be restated loses its
`rows` sentence rather than keeping a stale one. A partial read names every gap with its counts, and
the result names up to 20 ledgers of each kind that were left out. The codes stay in `result`. `profit_and_loss` and `balance_sheet` carry one too, with no `rows`:
when every result of the statement is established, the lead says so and what the derived lines passed
the comparison with (Tally's own Balance Sheet, and its own Profit and Loss when a profit and loss read it
as well); when any result is not established, the lead starts "Not established" and names each result
with its own state, the reason in words (the reasons are a closed list, so a new one is a compile error
until it has words) and, for a difference, how many lines did not tie (a Tally line that differs, a Tally line carrying an
amount that nothing derived was compared with, a derived line Tally has no counterpart for, and for a
profit and loss the Cost of Sales heading when it is off the derived cost of sales), says when the
derived lines are withheld, and gives one next step for each reason. So far `trial_balance`,
`profit_and_loss` and `balance_sheet` carry one; the other read tools and the refusals follow.

## Protocol and migration notes

The server negotiates MCP `2025-06-18` or `2024-11-05`, returning a supported
version when a client proposes a newer one. Initialization must precede tool
requests. Incoming frames are limited to 5 MB. All responses obey the configured
byte cap, including control replies, the JSON-RPC wrapper, and newline. A tool
catalogue that cannot fit returns `agent_response_too_large`; the session remains
usable. Text content contains the same
serialized, redacted JSON as `structuredContent` for older clients. The `initialize` result also
carries `instructions`, a short text for the client to show its model: start with `list_companies`,
use the one open company only when the user named no client (or exactly one open company matches the
name they gave), otherwise ask which and offer the list, state the company, dates and ledger used in
the first line, anything partial, withheld, not established or not checked ahead of the figures, ask
before a read that scans vouchers over more than one month unless the user gave the dates or the
financial year, and always before an outstandings party detail, which reads from the start of the books,
that what is read goes to the AI provider, and what to do with a refusal (relay it, take only a
different read, narrower dates or one repeat of the same read that it names, otherwise ask the user).
Its closing sentences follow the posting settings: with posting on, ask before preparing or posting
anything and never choose a ledger for a voucher; with import only, this connection cannot post, only
prepare a local import file the user imports themselves, and the assistant asks before preparing
anything and never chooses a ledger; with neither, it cannot prepare or post vouchers, and still never
chooses a ledger. Both of the latter tell the assistant to say so and never say anything was or will
be posted from the chat. It is left out when
`max_bytes` is below 4,096 or the request id is over 256 bytes, so that a client asking for tiny
responses still gets its handshake. The company rule is also in `list_companies`' own description, so
a client that does not pass the instructions on keeps it; the other sentences are not repeated there.

`tally_status.education_mode` is a boolean: `true` for observed Education mode,
`false` for observed Licensed mode, and `null` when mode is unobserved. Product
identity uses the observed gateway capability; an optional status-page banner
cannot override it. Clients of the earlier string-valued field must update.

Company selectors accept native hyphenated UUID spelling, case-insensitively.
Malformed selectors refuse before network reads. A malformed observed GUID cannot
construct a verified identity; discovery reports `identity_state: "invalid_guid"`
instead of `verified_tuple`.
Nonempty `BOOKSFROM` must also parse as a valid Tally date before scoped access;
discovery reports `invalid_books_from` for a malformed value.
Observed company numbers must contain 1–16 ASCII digits, using the same rule as
desktop selection. Discovery labels malformed values `invalid_company_number`;
scoped access refuses them before any company report is read.

Port zero and ports above 65535 are rejected at startup.
Unknown arguments, wrong selector types, and invalid enums are rejected before
any Tally request. Checkpoint numeric strings are no longer accepted at the tool
boundary. `changed_since` is unavailable; existing clients must stop calling it.

`validate_masters` accepts 1–100 nonblank names, each at most 1024 characters.
Near-miss suggestions are limited to 25 names and 8192 UTF-8 bytes per requested
name; `candidate_count`, `candidate_count_is_lower_bound` and
`candidates_truncated` preserve ambiguity and count precision. A true lower-bound
flag means "at least N" even when no candidates are listed. Import
planning allows 1000 vouchers but at most 100 distinct ledger names per batch.
Repeated uses of a ledger do not consume additional distinct-name slots.
Voucher-type and ledger selectors share the 1024-character bound; ledger
lookup keys are computed once before scanning live names.

Voucher `offset`, `limit`, ledger and voucher-type selectors apply after the
complete source window is read and validated. They do not page Tally's work.
The fixed profile fetches named voucher fields and three ledger-entry fields;
it does not expand `ALLLEDGERENTRIES.*`. Each source response is subject to the
transport's 32 MiB limit and 20-second deadline. A failed source read releases
no complete page or movement total. These client limits do not bound Tally's
server-side generation cost. Dense-window throughput and automatic source
partitioning are unqualified; start with a narrow date window and do not treat
a small output limit as a source-volume safeguard. Even a single day can be too
large. The connector does not automatically retry or subdivide such a failure.

Byte-limited pages retain forward progress or return `agent_response_too_large`;
they never advertise the same offset after removing every row. Outstandings
trims both collections to a shared page width because they share an input offset.
Active vouchers without observed accounting entries are refused before movement
filtering; cancelled and optional vouchers remain excluded from movement totals.
Movement corroborates the complete opening-ledger snapshot after voucher reads
and rejects unknown entry names before selecting a ledger. Caller-specified
opening dates require a freshly observed supported product/mode and a valid native
boundary before and after the read; a prior status call or cached profile does not
grant admission.

Top-party ranking uses `gross_exposure`, with billed and unallocated receivable
and payable fields kept separate. `totals.scope` is `open_bills_only`.
`open_bills_total` counts every open bill in the requested direction (the bills `totals` and
`ageing_buckets` cover; on a partial read, the base-currency ledgers' bills only, beside those figures) and `open_bills_shown` counts the bills on the page returned. A page cut by
the response size keeps `limit` unchanged and restates `open_bills_shown`, so a page shorter than the
total is read from `open_bills_shown` and `next_offset`, never from `limit`.
`unallocated.totals` contains `receivable`, `payable`, `gross_unallocated` and
`by_composition` (the same gross split by composition, below).
The previous ambiguous `outstanding_total` and `unallocated.amount` fields have
been removed. Gross exposure is not net money due.

Each `unallocated.parties[]` row says what the ledger data can and cannot say about
its amount, without reading vouchers (#945). `ledger_bill_wise` is the ledger's own
`ISBILLWISEON`, written from the `composition` that carries it so the two cannot
disagree. `opening_balance` is the ledger's own opening as of the start of the books
(not the current year's), shown and never interpreted; it is absent when Tally sent
an empty element, which is unknown and not zero. It keeps Tally's sign (a debit
opening is negative) while `amount` is a magnitude and `direction` says which side.
`composition` is `not_bill_wise_ledger` when the ledger's bill-wise flag is off (it
keeps no bills; seen on one ledger that never had bills, while a flag switched off
after bills existed is unmeasured, and the flag is read as of the read, not of
`as_of`) and `bill_wise_ledger_components_not_separated` for what is left on a
bill-wise ledger after its named bills: on-account entries, an opening balance not
allocated to a reference, notes with no reference and anything else all land there
and are not told apart, because the bills reports carry none of them. No unallocated
figure is labelled on-account. Advances and credit or debit notes kept as their own
bills are in the bills, not here.

`unallocated.totals.by_composition` splits the gross by composition, receivable and
payable apart, over every party in the requested direction before paging, so its
parts add up to the totals. A row saved without a composition (older saved data)
counts under `composition_not_observed`, which appears only when such a row exists.
Each row is about 100 bytes wider than before, so under a byte cap a page can now
hold fewer rows and `next_offset` can move; no figure changes.

`outstandings` also takes `party` (a ledger name) with `detail`, which adds a `detail` object for
that one party, tied to the same as-of (#945). Without them nothing changes, and passing `detail`
is the request to read the company's vouchers from the start of the books (below). `detail:
bill_trail` (optionally with one `reference`) lists every allocation of each of the party's bills
in vouchers that are neither cancelled nor optional (those are skipped, as Tally's own reports
skip them), oldest first, each with its own bill date, and gives each bill a state: `tied` (the
signed allocations equal Tally's own balance for that bill, or zero for a bill the report no
longer lists), `trail_does_not_tie` (both numbers shown) or `bill_identity_ambiguous` (more than
one native row or bill date for one reference, or one native row dated differently from the
allocations; nothing is merged, and the native dates are shown). Naming a `reference` that Tally's
bills reports list starts the window at the earliest date they list for it, so allocations dated
earlier are not read (a reference they do not list is read from the start of the books): a
reference that carries two bill dates over the whole history, and is ambiguous there, can tie
when named. The detail's own `state` is `bills_listed`, or, for an empty list,
`not_bill_wise_ledger` (the ledger snapshot says the ledger keeps no bills) or
`no_named_bill_for_party` (no voucher read allocates a named bill on the ledger and Tally lists
none for it; a ledger that keeps no bills, one that is not a party's and a party with no bills are
not told apart).

`detail: unadjusted` lists the party's on-account, advance and pending credit or debit note
allocations and compares the on-account sum with the party's unallocated amount. `tied` there
means the two figures are equal, not that the composition is proven: components that net to zero
are not seen. `residual_not_explained_by_vouchers` gives the difference and whether it equals the
ledger's opening balance, as a fact and not a label. `no_residual_row_for_party`, with `residual`
null, means Tally's ledger snapshot lists no unallocated amount for the ledger (a zero residual, a
ledger that is not a party's and a name that matched no row are not told apart), so nothing is
tied; `not_bill_wise_ledger` lists no rows. Its rows carry `row_amounts: as_allocated`: each
amount is the allocation as it was made, never net of what later allocations adjusted against its
reference, so an advance shows what was received, not what is left. What is still open on an
advance's or a note's reference is Tally's own `native_balance` beside the row, from its bills
reports at the same as-of. It is null in two different cases, told apart only by `native_rows`: 0
means the reports do not list the reference, and 2 or more means they list it more than once, so
no balance is chosen. No captured book shows how Tally records a later adjustment of an
advance, so no netting is built on one. Either kind is `window_returned_no_vouchers`, with nothing
tied or listed, when the voucher read returned no voucher: unlike `vouchers`, the detail does not
corroborate an empty read.

The detail reads the whole company's vouchers from the books' beginning (or from the earliest
date Tally lists for the named bill, when it lists one) to `as_of` and keeps the entries on the party's own ledger,
so its cost is that of a `vouchers` read over the same span. That cost is unmeasured on a large
book, and any refusal of that read fails the whole `outstandings` call: one foreign-currency
composite voucher anywhere in the window fails it (`voucher_amount_invalid`, or
`bill_allocation_amount_invalid` when the composite is on an allocation). The read is bounded as
every window read is. It may send at most 128 data requests (each sent twice, as every read is, with its
census and the company marks besides), and before anything is measured a data request holds at
most 42 vouchers, so a window of more than 5,376 of the company's vouchers is always
refused, and a smaller one may be when its parts measure heavier. The refusal comes before any
data request when the census shows it (the census stops as soon as it has counted more vouchers
than the allowed requests can hold), and otherwise when a measured part does, as
`trail_window_too_large` (name a `reference` that `open_bills` lists for the party; the read then
starts at that bill's date), `named_bill_window_too_large` (a reference was named already, so
nothing narrows it further) or `unadjusted_window_too_large` (nothing narrows it, so it is not
available for that party on that book). Either refusal carries `reads.needed_at_least` against
`reads.allowed`, and `window` lists any part already read. It needs a complete read
(`detail_requires_a_complete_read`, with the read's own `partial_reason` beside it), and refuses
rather than cuts an answer of more than 500 allocations: `trail_too_large` for a bill trail (name
a `reference`) and `unadjusted_detail_too_large` for the unadjusted detail (nothing narrows it, so
the detail is not available for that party; its vouchers can still be read with `vouchers` and
`ledger`, untied). That limit counts allocations, not bills, so bills Tally lists that no voucher
allocates to (opening bills) are not counted by it, and a party with very many of them can meet
the response size refusal (`agent_response_too_large`) instead. A reference neither the vouchers
nor Tally's report know for the party is refused (`bill_reference_not_found`). The detail ignores
`direction`, `top`, `offset` and `limit`; `window.company_vouchers_read` counts the company's
vouchers in the window, not the party's. A named bill's window never starts before the books.
The vouchers and the bills reports are two reads whose extents are not compared: a voucher posted
between them usually shows as `trail_does_not_tie`, but two changes that compensate, or
allocations that net to zero, can still read `tied`. Measured on one synthetic book only
(reference 12a.14); an `as_of` earlier than the last voucher and post-dated vouchers were not
measured.

`receivable` and `payable` follow the sign of each bill's balance, as Tally's own
Bills Receivable and Bills Payable reports scope them, not the type of party, and
those reports carry no bill type. A customer's advance, or a credit note raised to
a customer, appears under `payable`; a supplier's advance, or a debit note raised
to a supplier, appears under `receivable`. That holds for an advance or a note
kept as its own bill: an on-account advance goes to `unallocated` instead, and a
credit note set against an open invoice reduces that invoice. It was measured on one
synthetic book (TallyPrime Silver 7.1). An open bill's `kind` is therefore a
direction, not "owed by a customer" or "owed to a supplier": with a 50,000
supplier bill, a 20,000 customer advance and a 10,000 credit note to a customer,
`payable` reads 80,000 and only 50,000 of it is owed to a supplier. The direction
of an `unallocated` amount is the sign of the party's net unallocated balance, so
an on-account receipt and an on-account payment on one party net into one figure.
For one party, `detail: unadjusted` separates advances, pending credit and debit notes and
on-account amounts by their voucher's bill type (below). The book-wide `open_bills.kind` and
`unallocated` figures do not; what is still to be measured is in #1356, the follow-up to #945.

A fingerprint match without a retained transaction marker is
`matching_content_observed`, with attribution unestablished; it is not counted
as `posted_verified`. A batch voucher cancelled in Tally stays `posted_not_effective` and
carries `effective_copies_observed`: the effective, unmarked vouchers no batch voucher took that
carry its content (a `count`, up to five entries, `attribution: "not_established"`, `0` when
there is none), with `counts.cancelled_with_effective_copy`. It reports a possible re-entry and
attributes nothing: identical content can be a genuine second transaction. Verification entry differences are structured objects;
duplicate metadata uses `fingerprint_sha256`. Each observed voucher can satisfy
at most one expected transaction. Exact numeric comparison tolerates equivalent decimal spellings
without changing the generated file or its stored hash.

New files use `identity_scheme: "batch_v1"`. Each voucher's wire `REMOTEID` and
narration marker share a UUID derived from the generated batch ID and caller's
`bridge_txn_id`. The caller ID remains the local transaction label; it is not
sent directly as Tally's upsert key. Reused labels in independent batches therefore
have different wire identities, so rebuilding after losing the batch journal
creates a new identity and does not deduplicate the business event. That is the
file a person imports; a native post sends its own fresh `REMOTEID` and no
narration marker, and is bound by its own span (Approved voucher posting).

**An unknown outcome requires reconciliation for every voucher type, which writes nothing to Tally.**
Preserve the original batch and saved file, then call `verify_import`. Do not
re-import or rebuild the same business event, including a `Journal`. The
controlled repeat observation returned `CREATED=0, ALTERED=1` and left one
voucher; it did not qualify a resend after a lost response, restart or an
intervening change. Each measured `Payment`, `Receipt` and `Contra` bank file
was imported once. Neither observation authorizes another write to discover
what happened to the first one.
Historical records without an identity scheme retain their original raw-label
interpretation. Unknown schemes are refused. Narration markers support readback
attribution; they are not authenticated provenance.

If file publication fails after its recovery marker is created, the partial
response retains `result.batch_id`. This does not mean a complete XML file or
import-ledger row exists. Preserve the recovery journal and any staged bytes;
reconcile them before continuing the blocked import workflow.

If a build persists a file but its response exceeds the framing budget or its
preparation receipt fails, the recovery JSON-RPC error contains
`error.data.batch_id`. Retain it and use `verify_import` or inspect the local import
ledger; do not blindly rebuild or import another batch. If stdout itself fails,
the recovery ID may not reach the client; the generated XML and import ledger
remain available for local recovery. Proof JSON,
Markdown, and ledger status are published under one admission lock. Handled
publication failures restore the prior proof pair and ledger state. Builds create
the journal first, write and sync staged XML, then expose the importable filename. An interrupted
publication or failed rollback leaves a recovery journal and blocks further import
admission until the local files and ledger are reconciled. Preserve the journal,
its backups, and generated XML; do not delete it merely to retry. This is explicit
recovery after a partial file transaction, not a power-loss atomicity guarantee.

Prepared receipts count the bounded master-validation and loaded-company rows. Unknown tool
names are represented by `unknown` and `tool_name_sha256`; company IDs are
canonical UUIDs. Failed receipt appends restore the previous file length.
An incomplete log or failed rollback stops the session; a persisted build still
returns its recovery batch ID before termination. Reads withheld by the result
byte cap retain partial source commitments in the in-process evidence store.
`vouchers`, `voucher_presence`, `purchase_register` and `sales_register` label a
window by one rule (#985, #1031): `complete`
only when its rows were admitted voucher for voucher against the census that
sized the read (protocol reference §11c.3), or it was empty and corroborated;
otherwise `partial` with reason `nonempty_window_unqualified`. A book whose
voucher high-water mark alone proves it small (a few dozen vouchers) sends no
census, so its nonempty windows are `partial`. Voucher selectors are applied
after the window is labelled, so a nonempty counted source with no matching
ledger returns a complete empty selection, and an uncounted one a partial one. Amounts
must parse as exact decimals, polarity flags must be `Yes` or `No`, and dates
must be valid calendar dates before ordinary voucher rows are released. Ledger
selectors require matching catalogues before and after the voucher read, unique
names, and catalogue membership for every observed entry. Movement
metadata counts all source vouchers, including non-posting rows excluded from
balances. Ordinary voucher rows expose boolean `cancelled` and `optional` fields;
non-posting amounts are not presented without that state. Movement repeats the
voucher source after its final opening snapshot and refuses changes to either
source before calculating balances. This establishes stability across repeated
observations, not an atomic Tally snapshot.
Each active voucher must also have nonempty entries whose signed amounts sum
exactly to zero before movement arithmetic or ledger selection. An unequal
projection returns `voucher_entries_unbalanced`, retaining source evidence.
This necessary check cannot detect an omitted subset that itself balances.

Evidence-history and egress-log reads use the smallest of the requested limit,
`BRIDGE_AGENT_MAX_ROWS`, and the 256-record ceiling. Omitted history sets
`truncated`; egress-log reads also report omissions from the bounded tail scan. Missing terminal newlines, invalid
UTF-8, or invalid JSON in retained complete rows return `egress_log_incomplete`.

Verification captures the batch journal generation before readback and checks it
under the exclusive publication lock. If another process published the same batch
in the meantime, `import_verification_conflict_retry` preserves the newer proof
and status; repeat verification to obtain a fresh observation. Unrelated batches
do not conflict, and no file lock is held during network reads.

Verification rejects malformed accounting fields before matching. Distinct,
fully attributed expected vouchers may have identical accounting contents; each
observed identity can satisfy only one expected transaction, and unexpected
duplicate postings remain blocking. A stable observed identity claiming multiple
expected transaction tags returns `import_verification_tag_ambiguous` before
matching, so transaction order cannot choose an attribution.

Verification appends a compact status record bound to the batch ID and original
file hash, rather than duplicating vouchers and narration. The reader accepts
existing full batch records and their historical updates. Output row limits do
not determine verification-source completeness; the collection read is uncapped
by that setting and its identity set is independently corroborated.

Before publishing a new import file, the builder reads the exact min/max-date
verification window with the same fixed projection used by `verify_import`.
Both native responses must agree, and all rows must pass company, accounting
and window admission, including the GUID/AlterID pair required by verification. A timeout, oversized response or invalid source prevents
file and batch publication while retaining completed source evidence.
`verification_preflight` records the observed rows, paired source bytes and
response commitment. It proves that the current window was readable; the new
import or later changes can still make subsequent verification exceed the
transport limits. It is not a future-capacity reservation.

Migration 26 adds a nullable, constrained capability-tier column without
backfilling observations or deleting historical rows. Existing immutable-snapshot
triggers remain active. Local migration tests preserve historical columns and
review hashes and exercise the previous named-column insertion shape. An older
executable has not been tested against the migrated database; older binaries
cannot reproduce new tier-bound review commitments and require a fresh review.
Keep the additive column on rollback; no downgrade SQL is required.

Older binaries cannot read the new compact status records. Preserve the data directory, import ledger, and proofs; use the
new binary for the import workflow or disable imports after a binary downgrade.
Do not truncate the ledger to force downgrade compatibility. A binary rollback
does not undo a separately imported Tally voucher. Existing files and transaction
IDs remain local recovery evidence. Downgrading to a binary predating the
financial-read profile gate restores broader monetary admission and is not a
recommended way to access an unqualified endpoint.

Journal admission streams every record and checks even unrelated compact-record
hash bindings. Builds retain no historical voucher payloads; verification retains
only its requested batch. Each record is limited to 32 MiB. Memory still grows
with distinct batch IDs and hashes, and scan time grows with total journal bytes;
there is no automatic truncation. Every nonempty record must end with a newline;
a complete JSON object at an unterminated tail is refused before a later append
can concatenate objects. Preserve the file for explicit recovery instead of
truncating history or silently repairing it. Reserved narration markers begin exactly with
`[BRIDGE:`. Unrelated text such as `[BRIDGE CLUB]` is ordinary narration, while
malformed or multiple reserved markers still refuse verification.
