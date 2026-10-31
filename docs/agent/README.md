# ComplyEaze Bridge MCP

For ordinary Claude Desktop installation, start with the
[Download page](https://bridge.complyeaze.com/download.html), or use the
[fallback installation guide](./INSTALL.md).
The developer configuration below remains for supported client integrations.

`bridge_mcp` is Bridge's newline-delimited JSON-RPC 2.0 MCP server. It uses
Bridge's loopback-only Tally XML transport. Reads are enabled by default.
The MCPB extension also exposes voucher file preparation and bank-statement
parsing by default. Voucher posting (one Journal, Payment, Receipt or Contra, or
a saved batch of 2 to 50) is off by default because of the limits under
*Approved voucher posting* below; the **Allow voucher posting, one voucher or a
batch of up to 50** setting adds it, with separate native approval for each new
attempt. Command-line installations
retain explicit environment switches.

Build and run it with Rust 1.96:

```sh
rustup run 1.96.0 cargo run --manifest-path src-tauri/Cargo.toml --bin bridge_mcp
```

Every tool call refuses, in band, until the Terms of Use are accepted. Read them first
(https://bridge.complyeaze.com/terms.html); then set `BRIDGE_TERMS_ACCEPTED=true` (or `1`) to accept them. The extension asks for this
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
`BRIDGE_AGENT_REDACTION` (`none`, `mask_parties`, or `drop_narration`).
`BRIDGE_AGENT_MAX_BYTES` bounds each tool's answer. Control replies (to
`initialize`, `ping` and `tools/list`), which the build fixes and which carry no
book data, are held to 1,000,000 bytes, or to it if it is larger. The host is validated by `bridge-tally-transport`; non-loopback hosts are refused.
The resolved data path must be valid Unicode so saved artifact paths can be
returned exactly in JSON. Invalid platform encoding is refused before state
creation with `agent_data_dir_encoding_invalid`; no lossy path alias is used.

Before requesting financial data through an MCP client, the client may send the selected
Tally result to its AI provider, including company
identity, party or open-bill details, and amounts. An unset
`BRIDGE_AGENT_REDACTION` defaults to `none`; `mask_parties` shortens party and ledger
names, bank account numbers and the address lines of a ledger's details to their first two and last two characters (a name of
four characters or fewer becomes `…`; in `stock_summary` it also masks stock item names
and stock-group parents; GUIDs and Tally's reserved root stay plain) and `drop_narration`
drops narrations. Neither setting hides amounts, company names, dates, references, PAN,
GSTIN, IFSC, MSME or Udyam registration numbers, a stock item's HSN codes (an HSN description is masked under `mask_parties`) or contact details, and the bank statement tool's `account_last4` (the last
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
- `cash_flow`
- `company_features`
- `egress_log`
- `gst_by_counter_ledger`
- `gst_status`
- `gst_uncertain_vouchers`
- `ledger_masters`
- `ledger_movement`
- `list_companies`
- `local_data_report`
- `masters`
- `month_register`
- `negative_ledgers`
- `negative_stock`
- `outstandings`
- `profit_and_loss`
- `purchase_register`
- `read_evidence`
- `sales_register`
- `statement_tie_out`
- `stock_summary`
- `tally_status`
- `trial_balance`
- `validate_masters`
- `verify_import`
- `voucher_presence`
- `voucher_read_cost`
- `voucher_schema`
- `vouchers`

`masters`, `stock_summary`, `profit_and_loss`, `balance_sheet`, `purchase_register` and
`local_data_report` were added in release 0.4.0; `sales_register` was added in release
0.4.2. `cash_flow` was added in release 0.5.0. `company_features`, `gst_status`, `month_register`, `statement_tie_out` and `voucher_read_cost` were added in
release 0.6.0. `audit_test_run`, `gst_uncertain_vouchers`, `negative_ledgers` and `negative_stock` were added in release 0.6.1; `audit_test_run` is
not a default tool: it is listed only when its setting is on (below). `gst_by_counter_ledger` was added in release 0.7.0. `local_data_report` (also
`bridge_mcp --local-data-report [--show-paths]` on the command line) is a
read-only report of what Bridge keeps in its agent data folder: per class
(journal, import files, proofs, review records, approval notes, bank
statements, the egress log, lab files, lock files, other) the file count, bytes
and the age in days of the oldest file, symlinks it did not follow, sockets
and other special files it did not count, folders it could not list (named, not
read as empty), and the import journal's state: batches, batches sent or found
posted, how many of those are not settled (with no recorded response, with a
response but a latest status that is not `posted_verified`, or covered by a
recorded restore whatever their status: a post Tally
rejected stays not settled; `not_settled_binding_refused` counts the native
posts whose binding to their own vouchers was refused, all of which stay not
settled for good, a post Tally rejected in part or whole among them;
`not_settled_reviewed_not_attributed` counts the native posts matched by content
only, their binding refused or no answer from Tally recorded, that a person
recorded checking in Tally (`acknowledge_post_review` with `doubt` set to
`content_only`), still not settled and never verified; `not_settled_restored`
counts the native posts a recorded restore covers (bridge#1050), not settled
whatever their latest status reads; they never read `posted_verified` again,
and nothing in ComplyEaze Bridge clears that count yet, so they stay there, in
no other count, with their rows held: check them in Tally;
none of these overlaps
`not_settled_no_response`, `not_settled_binding_refused` or `not_settled_not_verified`;
`settled_by_person` counts the native posts Tally answered as creating none of their vouchers that a person
settled (`acknowledge_post_review` with `doubt` set to `not_created`), which are in none of the `not_settled`
counts),
batches with no recorded dispatch that were never found posted
(`no_dispatch_never_verified`: this includes a batch imported by hand whose verification is incomplete, which may well be in Tally, so no
deletion may rest on it), and interrupted-write folders that Bridge must recover
before it builds or reads. A journal it could not read is reported as
`journal_unreadable` (could not be opened), `journal_read_failed` or
`journal_invalid` (refused twice, 100 ms apart), never as absent. It reads the
journal without the admission lock, names no file path (the command line adds the
folder paths, where their files are, the journal's location and any package
copy of the folder only with `--show-paths`), and does not cover files outside this
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
bank-statement voucher proposals, and `parse_bank_statement_csv`, which checks the shape of one bank's CSV
statement export and proposes nothing yet (#1320; below). `BRIDGE_AGENT_ENABLE_WRITES=true` enables
that import workflow and exposes `post_import` and `acknowledge_post_review`. The MCPB extension always
sets `BRIDGE_AGENT_ENABLE_IMPORT=true` and maps its **Allow voucher posting, one
voucher or a batch of up to 50** setting (`enable_posting_2026_10`), off by
default, to `BRIDGE_AGENT_ENABLE_WRITES`. `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true` lists `audit_test_run`;
the extension maps its **Allow tax-audit test runs (audit_test_run)** setting
(`enable_tax_audit_2026_10`), off by default, to it. Off, the tool is absent from `tools/list` and a
call naming it is refused as `audit_tool_disabled` before any request. The setting takes
`true`/`false` or `1`/`0`; empty, or the extension's unresolved placeholder, reads as off, and any other
value stops startup.
This is a source-configuration inventory, not a claim that an installed client
uses a particular setting or that a tool is qualified for every runtime. Each
call returns compact JSON with the
company identity where scoped, a read timestamp, request/response commitments,
byte count, completeness reason, and truncation state. A refused call returns
`result.error` with `code`, which names what failed, and `message` (`agent_response_too_large`
puts `error` directly in `structuredContent`, with no `result`). Where a runtime
refusal has a typed, data-free reason, the error also carries `cause`, which names why
(for example `company_base_currency_undetermined` beside `party_ledger_master_read_failed`).
A read whose two paired halves differ, because the book changed while Bridge was reading it,
carries `native_report_pair_changed` or a cause specific to that read (for example
`masters_collection_changed` or `party_ledger_balance_changed`). A voucher-window part that is not admitted
(`voucher_window_part_not_admitted`) names why, and a census disagreement also carries
`counts`, the rows the part `returned` against the rows the census `counted`.
A compliance ledger read (`ledger_masters fields=compliance`) whose company's master-alteration
mark puts the estimated response over Bridge's budget has its ledgers counted first with a
balance-free catalogue read (a stable pair, bound to the company by name and GUID) (#637, #668,
#679). A count that fits is read whole. A count that does not is read in parts: the catalogue's
parent groups are packed by ledger count into parts of at most 4,266 ledgers (3,200 with `include_mailing_details`, 2,000 with `include_gst_details`) and 200 parents, and
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
under unnameable parents, over 4,266 ledgers (3,200 with `include_mailing_details`, 2,000 with `include_gst_details`; `parent_over_budget`), more than 12 named parts
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
`ledger_masters fields=compliance` takes an argument, `include_mailing_details` (a JSON boolean, default false), which
adds each ledger's dated mailing details (`compliance.mailing_details`, #1490). With any other `fields`, a true value
is refused before any read as `ledger_masters_mailing_details_requires_compliance`; together with `include_gst_details`
it is refused as `ledger_masters_mailing_details_with_gst_details_not_captured`, because the request that names both has
not been captured from a real Tally; a value that is not a boolean is `argument_invalid:include_mailing_details`.
Without it (absent or false) no request names the mailing details, the read is the production request, sized at
3,750 bytes a ledger, and no row has a `mailing_details` key. With it, the master read names every ledger's
`LEDMAILINGDETAILS.LIST` and each row's `compliance.mailing_details` has `observation` `entries` (each dated entry, oldest
first, with `applicable_from`, `address_lines`, `pin_code`, `state`, `country`, `mailing_name` (present only when Tally
sent one) and `other_content_skipped`, true when the entry held content that was not read, so its address lines may be
incomplete; an empty list is a ledger with no mailing details), `not_observed` (the response held no such list) or
`unreadable` (a `defect`, for that ledger only). For a date, use the entry in force on it, the latest on or before it.
The flat `compliance.address`, `pin_code`, `gst_pin_code` and `state` fields are Tally's older elements: TallyPrime 7.1
returned them empty or absent for a party that held an address in the one synthetic book measured, so read the address
from `mailing_details`; other books may fill the flat fields. `mask_parties` shortens the entry's `mailing_name` and each
of its address lines (and the flat `address`) like a party name, to their first two and last two characters, so a house
number or a town at either end of a line can stay readable; the mailing entry's PIN code, state and country reach the
assistant's provider under every redaction mode (the flat `pin_code`, `gst_pin_code` and `state` already go on every
compliance read, unchanged); for a party that is an individual, such as a proprietor, the flat PIN code and state sent
on every compliance read, and the mailing entry's PIN code, state and country when asked for, can point to the
locality of a home address. The read is admitted at 5,000 bytes a ledger instead of 3,750, above the largest row
measured (4,902 bytes: a party with two short dated entries; UNVERIFIED for a populated book, and a party with three entries
or longer address lines is larger), so a read holds at most 3,200 ledgers
(the 16 MB budget over the constant, not measured); a book that fits a standard read may come back in more parts, or be
refused as `parent_over_budget`, never truncated. A held listing is served only to a page that asks for the same mode, so
repeat `include_mailing_details` on later pages. Measured on one synthetic book (TallyPrime 7.1, and 7.2 for a second dated entry on one party, oldest first); three or more entries,
an entry with no date and other releases are not measured. The party/ledger master workbook, which the desktop app writes to a
local file, names the mailing details only where the book's master-alteration mark is at most 3,200 (the
whole read fits their bound) and otherwise sends the standard request, as before #1490, so a book that exported
before is never refused for the larger estimate; its header then says that no ledger's mailing details were
observed and the status column reads not observed. Where it reads them it shows only the entry in force on its
Balances-as-of date.
`ledger_masters fields=compliance` takes one more argument, `include_gst_details` (a JSON boolean, default
false). With any other `fields`, a true value is refused before any read as `ledger_masters_gst_details_requires_compliance`,
and a value that is not a boolean is `argument_invalid:include_gst_details`. Without it (absent or false) every
request, the request commitment, the sizing and the result are what they were, and no row has a `gst_details`
key. With it, the master read of each request (the whole book or each part by parent group) also names every
ledger's `GSTDETAILS.LIST`, and each row's `compliance.gst_details` holds Tally's ledger-level GST details as
sent: `observation` is `not_observed`, `entries` (each dated entry with its taxability, source, input-credit
flag and state rows, whose rate rows carry a `rate` that is `absent`, `empty`, `zero` or `value` with `raw` and
`decimal`, never merged) or `unreadable` (a `defect`, for that ledger only). HSN/SAC and type of supply are
not returned because no capture shows them, the place of supply is in the registration history
(`compliance.gst_registrations`), and slab rates are flagged, not read. The result's `limitations` say this
and that the shape was measured on one 14-ledger lab book of TallyPrime 7.1 Silver and has not yet been run
through this tool on a live Tally. The read is admitted at 8,000 bytes a ledger instead of 3,750, an
UNVERIFIED margin over the one capture (a ledger with an empty list 2,138 to 2,704 bytes; one dated entry
5,248 to 5,394; UTF-16 bytes of the ledger's own element), not a measured ratio: a read, whole or one part, holds at most 2,000 ledgers (4,266 without
the flag; both figures are the 16 MB budget over the per-ledger constant, not measured), so a book that fits one
read today may come back in more parts, or be refused as the existing `parent_over_budget`, never truncated.
A held listing is served only to a page that asks for the same mode, so repeat `include_gst_details` on later
pages as well as `as_of`: a later page without it reads afresh and returns rows without `gst_details`
(`snapshot.reused` false), and only a call that names `snapshot_id` is refused.
When Bridge got no response it could read, the `cause` names why and the error also carries
`endpoint`, the configured origin that was tried (#629). The causes are:
- `endpoint_invalid`: the configured endpoint failed validation. The `endpoint` field then appears only
  if a valid origin can still be formed from the configuration. `bridge_mcp` checks the host
  at startup and stops with `host_setting_invalid` (a bad port stops with `port_setting_invalid`), so its tool
  calls do not meet this one.
- `endpoint_unreachable`: the connection was not accepted.
- `request_failed` or `request_deadline_exceeded`: the request failed before any response, or its
  deadline passed (also while the body was being read). On a read whose code would be the
  generic `agent_runtime_read_failed`, `request_deadline_exceeded` is the `code` itself.
- `http_status_failure`, `response_content_type_unsupported` or
  `response_content_encoding_unsupported`: the responder was rejected on its HTTP status or headers
  before any body was read.
- An `endpoint_…` runtime code: Bridge held the request back and sent nothing. So do
  `tally_endpoint_busy` (with `retry_after_s`) and `tally_endpoint_lock_unavailable`, which
  come back as the `code` itself, and `tally_endpoint_draining` (with `retry_after_s`), when an
  earlier post was cut for time and Tally has not yet answered `/status` three times in a row
  since (#2142); a draining refusal may have sent `/status` checks, but not the request.

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
GUID is malformed, and then it carries neither. Other argument refusals carry no `expected`. A date that fits the pattern but is not a real day,
such as `2026-02-30`, is `invalid_date`; it gets the same date guidance in `remediation`.

A `post_import` refusal does not carry `remediation`. When its journal shows that nothing was sent
(`attempt_recorded` is `false`), it carries the same next step as `error.next_step` instead, when
one exists: its code's, or else its cause's, under the same 4,096-byte rule. A step the post sets itself, for a busy
port or a row another batch posted, is kept. A refusal after a send, or when a send could not be ruled
out, is given no such step: it is reconciled with `verify_import`, never retried (#1573).

Like `remediation`, `expected`, `cause`, `counts`, `size`, `endpoint`, `ledger`, `bill_row`,
`partial_reason` (and `partial_reasons`), `reads`, `unsupported_parent_ledgers`, `candidates_listing`,
`candidates_reason`, `candidates_total`, `candidates_total_is_lower_bound`, `candidates_truncated`,
`candidates`, `requested` and `window` are omitted when
`BRIDGE_AGENT_MAX_BYTES` is below 4,096, so that the code always fits. A refusal's list of
candidate ledgers needs at least 16,384, so it can be absent above 4,096 too. The two
voucher-numbering refusals (`import_voucher_numbering_not_automatic` and
`import_voucher_numbering_not_retain`) need at least 8,192 for their `next_step` and, on
`post_import`, for the message that names the refused voucher types: from 4,096 up to 8,192 a
refused `build_import_xml` still returns `state`, `reason` and `refused_types`, and a refused
`post_import` its `error.code` and `error.refused_types` with the general message; under about
2,500 bytes even those do not fit, and the answer is cut down as any tool's is. The tests hold a
refused post naming three voucher types and a refused build naming one within 8,192 bytes with
the next step and within 4,096 without it, and an answer naming four voucher types within the
server's caps at 8,192; the sizes themselves are not printed here, since the texts change. Before a tool response is written, ComplyEaze Bridge appends a
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
A ledger whose closing is not its opening plus its debit and credit totals (an empty amount
counted as zero) carries `movements_differ_by` (the exact difference, closing minus the three, in
Tally's sign), the first line says how many do not add up, and a limitation explains. The row is
as Tally sent it and nothing is corrected. Measured on a Closing Stock ledger whose closing
balance was keyed on Tally's screen: Tally kept the opening as its closing and showed the change
only in the debit total. On the book not integrated with inventory Tally's Profit and Loss and
Balance Sheet used the keyed figure; on the integrated book they valued the stock from the stock
item, so the ledger's `closing` is not necessarily the figure those statements use.

`offset` and `limit` restrict output, not the source read. A first page captures a
fresh report and holds it in memory; a later page continues from it while the book's extent is
unchanged (`snapshot`, `snapshot_id`, `listing_snapshot_changed`), as `masters` does, so compare source
evidence before combining pages from separate calls. Native
Trial Balance uses Tally's TBAL fields, including the Profit & Loss treatment;
see [protocol semantics](../tally/TALLY_PROTOCOL_REFERENCE.md#56-native-trial-balance-fields).
It may include dormant ledgers hidden by Tally's screen. Paired response,
company, mode and extent checks detect observed changes, but do not prove an
atomic snapshot or voucher-level reconciliation. Keep the company quiet during
reads. Use `ledger_movement` with narrow dates when voucher detail is needed.

### Masters

Use `masters` with `company_guid` and one `kind`: `voucher_types`, `godowns`,
`units`, `stock_groups`, `stock_items`, `cost_centres`, `cost_categories` or `groups`. It lists a company's masters of that
kind, for example a voucher type's numbering method (`automatic`, `manual` or
`default`, as Tally reports it) and its `active` and `optional` flags. Each row
carries `name`, `guid`, `master_id`, `alter_id` and `parent`; units add
`decimal_places` and `simple`; cost centres add `category` (always present: a centre without one is refused as `masters_row_field_invalid:category`) and
cost categories `allocates_revenue`, `allocates_non_revenue` and `affects_stock`;
stock items add `hsn` and `gst` (below).
Cost centres and categories are returned whether or not the company's Cost
Centres setting is on: a book whose setting read No still returned its two
centres and two categories (one synthetic book), and a book whose setting read Yes
returned its three centres, one under another, with the same shape (a third synthetic
book, captured in a separate sitting), so a No setting is not "no centres", and an empty cost-centre list (one synthetic book with no cost centre defined
answered an empty list) does not say whether the feature is off or none is defined; this
call does not return the setting. An empty cost-category list is refused
(`masters_cost_categories_empty`) on the expectation that the predefined Primary Cost
Category always exists: it was present in the two books whose categories were captured (one
with the setting at No, one at Yes), and the categories of a book with no cost centre defined were not captured. The two
cost collections must carry the `MSTDEPTYPE` Tally printed on every captured answer
(32 for centres, 16 for categories), or the read is refused as
`masters_collection_type_unexpected`, so an empty answer that was not resolved to the
type asked for is not read as "none defined". Under `mask_parties` the names of cost centres and categories, a cost centre's parent
and a cost centre's category are masked, as godown, stock-group and stock-item names are. How a voucher's entries were allocated to cost centres is read with the voucher reads
("Cost-centre allocations in voucher reads" below), not by this tool.

A `stock_items` row carries the item's own dated `hsn` and `gst` lists, each as Tally
sent it, and `inherited_from_group_or_company: not_read`. Each list is an `observation`:
`entries` (sorted by `applicable_from`, every entry as sent; an HSN entry has its
`hsn_code` as exact text, never trimmed, cut or read as a number, a `description`
Tally fills from its own list and a `source`; a GST entry has `taxability`, `source`,
`ineligible_itc` (Tally's `GSTINELIGIBLEITC` as sent: true for Yes, false for No, null when absent, never filled in) and its state rows, each rate
being `absent`, `empty`, `zero` or a `value` with its `raw` text and `decimal`, never
merged), `unreadable` with its `defect` (an undated entry, an invalid date, a repeated
field, two different entries on one date, a rate that is not a plain decimal, a credit
flag other than Yes or No, a repeated state or duty-head row, or content only of a kind
not modelled, such as text, a reference or CDATA directly in the list; that list of that item only, and the read and the row stand) or
`not_observed` (the row had no such list). Nothing chooses the code or the rate for
a date. An empty list, or `not_observed`, is not a statement that the item has no HSN
code or no GST rate: after a code was keyed on a stock group, the group's four other items still carried empty
lists (one synthetic book, one release); a GST rate or an HSN code on the company, and whether an
item that defers to a coded group inherits its code, were not measured; the group's and the
company's setting is not read. Tally adds an entry of its own (source `As per Company/Stock Group`,
no code, no rate, dated the book's first day: 1 April 2025 on the one book measured, no other book's
date measured, so recognise it by its source) when an item is first saved. `state_name` and
`valuation_type` carry the reserved-character marker (U+FFFD `#4;`) where Tally sends
`Any` and `Not Applicable`. On one item `GSTINELIGIBLEITC` was Yes on the entry whose screen
row read `Eligible for Input Tax Credit: No` and absent on the entry whose screen row read Yes,
which fits an ineligibility flag (PARTIAL: one book, two rows, screen against wire), so a stock
item row names it `ineligible_itc`, as `ledger_masters` does. What an
absent element means for the tax is not known: null is neither eligible nor ineligible. On an entry whose source is `As per Company/Stock Group` (the entry Tally adds itself on a first save; a person can also choose that source) a flag Tally sent is withheld: `ineligible_itc` is null and `ineligible_itc_withheld` is `tally_default_entry`, because the Yes seen on such an entry was never compared with the screen (both screen readings are of rows a person edited or keyed, and a row's origin and its flag cannot be told apart on one item); a flag Tally did not send stays null with no marker, and an entry with any other source is sent as it is. A Tally `ERROR` or `LINEERROR`
anywhere in a list, markup inside a value, or a list cut off refuses the whole read
(`masters_tally_reported_failure`, `masters_scalar_not_text_only`,
`masters_xml_invalid_escape`, `masters_row_unterminated`). The collection must carry
`MSTDEPTYPE` 512 (a choice made like the cost collections', not a measurement: no
zero-row stock-item answer was captured). Under `mask_parties` item names and parents are
masked, and so is an HSN description, which Tally fills from its tariff list and a person can edit;
HSN codes and rates are not names and are returned as sent.
Measured on one synthetic book of TallyPrime 7.1 Silver (PARTIAL); a state row other than
`Any`, slab rates, another release, a larger book and a zero-row stock-item answer are not measured (protocol
reference §12a.22). A `groups` row carries `name`,
`parent` and `reserved_name` only, as the group snapshot returns them, and a
parent that is Tally's reserved root is written `&#4; Primary`, the text Tally
sends, as in `trial_balance` (a group a user named Primary is written plain). Alias names are not returned.

A `voucher_types` row also carries `series_numbering`, the numbering of the
type's number series, which `numbering_method` is not (that is the type-level
value, which read `None` for types set to Automatic and to Manual on the
screen). It is read from a second paired read of the voucher-type export with
the series-level fields added, inside the same brackets and between the first
collection and the closing extent, and is joined to each row by GUID with the
name as a second equality. It is `{"observation":"observed","method":...,"duplicates":...}`
(`method` is `automatic`, `manual`, `automatic_manual_override` or
`{"unrecognised": "<raw text>"}`; `duplicates` is `prevented` or `not_prevented`),
`{"observation":"series_count","count":n}` where a type has no series or
several (nothing is read from it), or `{"observation":"field_invalid","code":...}`.
Under `"method":"automatic"`, and under no other method, it also carries
`sub_method`, the series' own `NUMBERINGSUBMETHOD`: `auto_retain` (the screen's
"Retain Original Voucher No."), `auto_renumber` ("Renumber Vouchers"), `absent`
where the series printed none, or `{"unrecognised": "<raw text>"}` for any other
text, a blank one included. A Manual series printed `Auto Retain` as well on
both books read, which is not the numbering in force there, so it is not
returned for one.
A type found in one answer only (`masters_numbering_names_differ`), or a GUID
and name that disagree between the two answers
(`masters_numbering_identity_differs`), refuses the whole read; so does the
numbering collection changing between its two reads
(`masters_numbering_collection_changed`) and a numbering answer whose row count
or size breaks the mark's premise (`masters_bound_premise_violated`). A
numbering answer that does not parse, a missing or repeated GUID among the
causes, refuses at once with a `masters_*` cause, and a moved book is reported
as `masters_extent_changed` in place of a join or premise refusal. Measured on one synthetic
book (TallyPrime 7.1 Silver, Windows): 24 voucher types, each with one series
named `Default`, of which three (Journal Automatic, Payment Manual with
duplicates prevented, Receipt Manual with duplicates not prevented) were
checked against Tally's screen. Automatic (Manual Override) through this tool
(it is mapped from the series text measured on a synthetic book on 13 October
2026), more than one series, Multi-user Auto, a single series not named
`Default`, a voucher type created new and other releases are not measured, and
an `unrecognised` method is never `manual`. The integrated read, the two
requests sent as one tool call, was run live on 14 October 2026 on two
synthetic books (TallyPrime 7.1 Silver, Windows; 24 voucher types each, each
with exactly one series, whose name is not read): it was not refused, and on
the six types whose numbering was set or checked on Tally's screen it returned
that numbering (Manual with duplicates prevented, Manual with duplicates not
prevented, Automatic); the other types, left at a new company's defaults and
not checked on screen, read Automatic. `sub_method` was measured on a
second synthetic book (TallyPrime 7.1 Silver, Windows, 13 October 2026) and one
voucher type, Receipt: it read `Auto Retain`, then `Auto Renumber` after the
type's screen was set to "Renumber Vouchers", then `Auto Retain` after it was
set back. With the type on "Renumber Vouchers", numbers already in the book
changed: at the switch, when a voucher with an earlier date was entered by
hand, and when a voucher with an earlier date was sent by import without a
number, where the import's answer read CREATED 1, ALTERED 0 and no voucher's
AlterID moved; a voucher dated after every other changed nothing. Later runs
on that book (protocol reference §9.8) found that a file of two Receipts
without numbers was numbered by date and changed four numbers, that a
Receipt carrying a number read back the number of its place in date order,
and that a Journal imported without a number and dated between two others
was numbered by its date and moved the later one. A file of more than two
vouchers, a mixed file, a Journal that carries a number or is dated after
every other, Payment and Contra, Gold, other releases and the same through
ComplyEaze Bridge's own post are not measured. Neither is any other
wording, or a series that prints no sub-method: on such a book every Automatic
type would read `absent` or `unrecognised`.

The read runs inside the same company, mode and identity brackets as
`trial_balance`, with the book extent read before and after (they must be
equal) and the collection read twice and compared. Education mode is refused.
`offset` and `limit` restrict output, not the source read; a first page holds
its read in memory and a later page continues from it while the extent is
unchanged (`snapshot`, `snapshot_id`, `listing_snapshot_changed`), as
`trial_balance` does.

Godowns, units, stock groups, stock items, cost centres and cost categories are read whole only when the master-alteration
mark (`ALTMSTID`) times an assumed worst-case row for that kind fits 16 MB. The
mark counts the masters of every kind, so a book with few of this kind can be
refused. That admits a mark of at most 1,152 for godowns, 1,168 for units,
1,160 for stock groups, 489 for stock items (their row is allowed for several dated entries: a row is refused past 16,344 UTF-16 units, about seven further GST entries of the measured size, against 4,373 measured, whatever its state rows; a limit of 489 refuses, by arithmetic, both client books measured, not tried on them), 1,037 for cost centres and 1,264 for cost categories; a larger book is refused before any collection request
as `masters_too_large`, with `size` (`master_alter_id`, `estimated_bytes`,
`limit_bytes`, and `limit_master_alter_id`, the largest mark this kind would be
read at). Retrying refuses again. Both stock-heavy client books measured, with
marks of about 100,000 and 300,000, refuse godowns, units and stock groups, and by the same arithmetic stock items (cost
centres, categories and stock items were not tried on them); how common such
marks are across live books is unmeasured (protocol reference §12a.12). `voucher_types` and `groups` have no size check
before the read: voucher types keep the policy of Bridge's other voucher-type
read (their second, numbering read is likewise sent with no size check), and groups that of the group read `profit_and_loss` and `balance_sheet`
make.

The size rests on premises that are checked after the read, not assumed. For
every kind but `groups`, each row's length is checked against the assumed
worst-case row (`masters_row_exceeds_bound`), and the rows, their AlterIDs and
the response size (each collection is read twice, and the size check runs after
both reads) are checked against the mark and the admitted size: more rows
than the mark, an AlterID above it, a repeated AlterID or an oversize response
refuses the whole read as `masters_bound_premise_violated`, unless the closing
extent shows the book moved, which is reported instead
(`masters_extent_changed`). The voucher types' numbering answer is held to
the same kind of checks (more rows than the mark or an oversize response).
A response Bridge cannot read refuses at once with
a `masters_*` cause (for `groups`, the group reader's, such as `group_xml_malformed`),
without waiting for the closing extent. A `voucher_types`
answer with no rows refuses as `masters_voucher_types_empty`, because every
company has predefined voucher types, and a `cost_categories` answer with no rows as
`masters_cost_categories_empty`; the other kinds may answer with none.

Evidence for the row shape: one synthetic book on one licensed TallyPrime 7.1 (cost centres: three synthetic books, cost
categories: two; the third book's sitting recorded TallyPrime 7.1, licence Silver, Education mode off in its own status read, which is not committed; see the provenance file) (`src-tauri/crates/bridge-tally-protocol/tests/fixtures/MASTERS_CAPTURE_PROVENANCE.md`).
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
hints (the company's `NUM*` fields), stock-item quantities or values (use `stock_summary`) or writes are part of this tool.

Under `mask_parties`, godown, stock-group, stock-item, cost-centre and cost-category names, their `parent`
values and a cost centre's `category` are masked like a party name, because a job-work
godown, a supplier-named stock group or stock item or a customer-named cost centre can carry a party's name; Tally's reserved root as a parent is shown as
`&#4; Primary` and is not masked. Voucher-type, unit and account-group names
are not masked: they are configuration labels, not counterparties.

### Cash Flow

Use `cash_flow` with `company_guid`, `from` and `to` for Tally's own Cash Flow: the
month-wise movement of the cash and bank ledgers. It is not a cash flow
statement under AS 3. The window must be whole months: `from` the 1st of a month,
`to` the last day of a month, at most twelve months, so that each row Tally prints
(a month name, with no year) can be placed in its year. Otherwise the call is
refused before any trial balance or report request, after the status and company reads (`cash_flow_window_not_month_start`,
`cash_flow_window_not_month_end`, `cash_flow_window_too_many_months`; a window starting
before the book is `trial_balance_before_books`). A reversed window is refused first, as
`invalid_date_range`, before any request. Education mode (`trial_balance_education_unqualified`) and a book with several currency
masters (`company_base_currency_undetermined`) are refused before any Cash Flow request, as
for the statements.

The months are returned only once the **net total** of the whole window has been
compared with the trial balance read for the same window and found equal: the
months' closing amounts added up, against the debit and credit totals of every
ledger under Cash-in-Hand or Bank Accounts (a ledger under a group a user made inside
one is counted by the group tree and was not measured). Both sides must carry an amount.
The top-level `state` is `observed` when the months are returned and
`not_established` otherwise, with `reason`:

- `cash_flow_differs_from_trial_balance`: the two figures differ, or only one side
  carries an amount (an empty amount is not a zero: that side shows as `null`).
  `months` is `null`; `net_total` carries both figures for investigation only (`use` says so),
  and a count of ledgers with movement whose group could not be resolved, which is
  where to look. Neither figure is the cash movement.
- `cash_flow_money_group_unmeasured`: a ledger under Bank OD A/c or Bank OCC A/c
  has movement in the window. Tally's Cash Flow was seen counting one such ledger
  (debit only, one book), which the tie does not count, so the refusal stands in
  place of a difference; it is covered by tests and has not been seen against Tally,
  and a Bank OD credit and a Bank OCC ledger were not measured. `months` is `null`.
- `cash_flow_nothing_to_compare`: nothing could be tied. Either neither the cash and
  bank ledgers of the trial balance nor Tally's Cash Flow carry an amount for the
  window (an empty amount is not a zero), or both sides net to zero, which would be
  equal under any sign convention and any meaning of the columns. `months` is
  `null`. Months that net to zero over no cash or bank ledger at all are
  `cash_flow_differs_from_trial_balance`, not a tie.

`unmeasured_in_this_answer` lists, in the result itself, what this answer holds that
was never compared with Tally's own Cash Flow screen: `credit_amount_present` (a non-zero
credit column), `positive_closing` (a closing on the credit side) and
`window_runs_from_march_into_april` (a window across a financial year of the usual
April-to-March kind). The first two were seen tied to the trial balance in live runs on one
synthetic book (a credit in two windows, a positive closing in one of them) and are still named; the third was never run. The list is empty only for
debit-only months in one year, and when it is not empty the lead says to treat each month's
figure as unverified and compare it with Tally's own Cash Flow. A bank ledger placed under
another group, a liability one for example, is not counted as cash or bank: if Tally
counts it the net total differs and the result is `not_established`.

`checks` says per field what was `checked`, `differs` (compared, and the figures
disagree), `not_checked` or `withheld`: the net total is checked; the split into months is Tally's own and is not checked (a total
can tie while one month is wrong); Tally's debit and credit columns are read but
not returned: on the two books with credits each column differed from the debit and credit
totals of the ledgers Tally counts as cash and bank (on one of them including a Bank OD A/c
ledger) by one common amount in size (the ledgers' columns larger in both) while the net tied, and what that amount is has not been established. A month
Tally printed with no amounts is returned with an empty `closing`, which is not zero and does not
say the month had no entries (whether entries that cancel print an empty closing is not measured).
A negative amount is a debit: cash and bank grew.

An unknown report name, a reported failure, an empty answer and rows for other
months than the window holds are each refused, classified by the structure of the
answer: on licensed 7.1 an unknown name is answered in band (`STATUS` 0 with a
`LINEERROR`) and is refused as `cash_flow_tally_reported_failure`; only a bare
`RESPONSE` (an earlier build) is `cash_flow_report_unknown`; the others are
`cash_flow_empty_envelope` and `cash_flow_months_unexpected`. A change between the
two paired Cash Flow reads is `native_cash_flow_changed`.

The read failures just above reach the caller as an `isError` result whose `error.code` is
`cash_flow_read_failed` and whose `error.cause` is the code named (the window codes
are the `error.code` itself). The three `not_established` reasons are not errors:
each is the `result.reason` of a result that is returned.

The tool was run against Tally on two synthetic books on licensed TallyPrime 7.1 (2026-10-07,
one call at a time through a recording relay): on five windows the net total tied to the trial
balance and the months were returned (three windows with debits only; two with a credit amount,
one of them with a positive closing as well), and a quiet window with no cash or bank amount was
answered as `cash_flow_nothing_to_compare`. An independent recomputation from the raw bytes
agreed with the tool on all six. Tally's own answers were captured on four synthetic books in all: on
every captured month with an amount the closing is the debit plus the credit, and a year of empty
months on a large book was answered at once. Not measured: a ledger under Bank OD A/c or Bank OCC
A/c in the tool's own run (Tally was seen counting a Bank OD ledger, so the refusal above stands in
place of a difference; it is covered by tests only), what Tally's debit and credit columns each
include (each differed from the totals of the ledgers Tally counts as cash and bank by one common amount in size (the ledgers' columns larger in both) while the net tied; on one book that set includes a Bank OD A/c ledger), a contra, a window
ending in February, a window crossing a financial year, optional or post-dated vouchers, a book with
several currencies, and a large book with cash activity.

### Negative Ledgers (`negative_ledgers`)

Use `negative_ledgers` with `company_guid`, `from` and `to` for Tally's own Negative
Ledgers list: the ledgers on that list, each as `ledger`, `side` (`debit` or `credit`) and `amount` (an exact decimal
with no sign). The list is Tally's own and is returned as it is, in Tally's order. The
rule by which Tally lists a ledger is not established and is not derived here: on three
books each listed ledger's balance sat on the side opposite its group, and on the first
book measured a creditor with a debit balance and a debtor with a credit balance were
listed and a ledger under Suspense A/c with a credit balance was not.

It is **not compared with any other report**: `checks` says `list_complete` and
`amounts_against_trial_balance` are `not_checked`, so a ledger Tally did not list is not
shown to be absent, and a listed amount is not shown to equal the ledger's closing
balance in the trial balance (reported, not kept: on the first book both did; on two more books the one ledger listed on each did too). An empty list is
returned with `state` `not_established` and `reason`
`negative_ledgers_empty_answer_is_not_a_finding`: Tally's empty answer cannot be told from
a report that was not rendered, so it never says that no ledger is negative. A non-empty
list is `observed`. `mask_parties` masks each `ledger` name.

The report was captured for one whole financial year (1 April to 31 March) on one
synthetic company, and read for a whole year on two more synthetic books: any other window
is named in `unmeasured_in_this_answer` as `window_other_than_a_whole_financial_year`.
What GST, stock, several currencies, or optional or post-dated vouchers do to the list was
not measured: one of the two further books holds GST ledgers and
one optional voucher, and neither book's settings were read. A window starting before the book is
`negative_ledgers_before_books`, a reversed window `invalid_date_range`, and Education mode
`negative_ledgers_education_unqualified`. A refused answer fails the call as `negative_ledgers_read_failed`, with the specific code below in `error.cause`; `negative_ledgers_before_books`, `negative_ledgers_education_unqualified` and `negative_ledgers_period_not_honoured` are the error's own code. The answer is refused, never reinterpreted, when
a listed ledger carries both columns, neither, a zero, or the other sign than the one
measured (a debit as a negative debit amount, a credit as a positive credit amount:
`negative_ledgers_balance_unexpected`), when a ledger is named twice
(`negative_ledgers_ledger_repeated`), when Tally reports a failure
(`negative_ledgers_tally_reported_failure`) or does not know the report
(`negative_ledgers_report_unknown`), when an amount is not a plain decimal
(`negative_ledgers_amount_invalid`), or when the report or the book changed between the paired
reads (`native_negative_ledgers_changed`), or the book's extent differed before and after the read (`native_ledger_extent_changed`). A window the read could not build a period for is `negative_ledgers_period_not_honoured`. The report has no
size check and its cost on a large book is not known; the report is one paired read (two requests), never retried. Measured on one
synthetic TallyPrime 7.1 Silver book (the captures are in the protocol crate's fixtures);
this tool has not yet been run against Tally. Two further synthetic books were read with the same report request (raw requests to Tally, not through this tool) for one whole financial year each,
with Tally's trial balance of the same year read beside the report: each listed one ledger (an expense ledger and a bank ledger, each
with a credit balance), its amount equalled the ledger's closing balance in the trial balance, and no other ledger of either
trial balance read on the opposite side to its group. No empty answer, no list of many ledgers, no other group and no shorter
period was seen on those books.

### Company settings and currency symbol (`company_features`)

`company_features` takes `company_guid` and nothing else. It reads the company's own record in Tally
twice, with the book extent checked before and after, and returns three settings that its company record holds
today (cost centres, GST and batch-wise stock) and the currency symbol. It returns no ledger, voucher, amount or narration. A company in Education mode is
refused (`company_features_education_unqualified`), as for the stock summary.

The result holds `settings` with `cost_centres`, `gst` and `batch_wise`. Each is `{"value": ..., "evidence": ...}`
where `value` is `yes`, `no` or `not_reported`, and `evidence` says what it rests on:
`compared_with_tally_screen_on_synthetic_books` (each of the three settings equalled Tally's F11 screen on three synthetic books, not on this company; a setting changed inside a sitting has not been measured). `not_reported` means Tally sent no such element, and its `evidence` is `not_reported_by_tally`; it is never
read as `no`, and the result's `state` is then `partial`. An element that is present but empty, or says
anything but `Yes` or `No`, refuses the read instead (`company_features_setting_invalid:cost_centres`,
`:gst` or `:batch_wise`).

`base_currency` is `{"state": "reported", "symbol": ..., "kind": "symbol_not_iso_code"}` or
`{"state": "not_reported"}`. It is the symbol Tally holds (the rupee sign on the books captured). It is not an
ISO code and it does not say whether foreign currencies are used: `ISISOCURRENCYAPPLICABLE` read No on a
book that has currency masters, so it is not read.

The headline leads with the three settings as "on", "off" or "not sent by Tally" and says they are the
settings the company record holds today, not what the books contain or what they were during a year: a cost-centre allocation
can be stored while the setting reads No (protocol reference §12a.17). Tally's other 14 feature flags are
fetched in the same request and are not returned, because each has only ever read one value on the books
captured. Nothing this tool returns changes what another tool reads or refuses; `stock_summary` reads its own
inventory, integrated and batch-wise flags from its own request.

The company is bound by the GUID filter and by the row's own name, company number and books-from date,
which must equal the verified company's; a row for another company, or not exactly one row, refuses the
read. These arrive as `company_features_read_failed` with a `cause`: `company_features_company_mismatch`
and `company_features_not_one_row` (the GUID in the filter is the verified company's, in its parsed
spelling, so it cannot fail there), and also `company_features_response_invalid` (the answer is not the
collection shape), `company_features_currency_invalid` (a symbol over 16 characters, or with a
control character, a bidirectional override or isolate, a zero-width or other invisible format character, or a line or paragraph separator), `company_features_tally_reported_failure`, and the stability codes
`company_features_changed` and `company_features_extent_changed`. Measured on three synthetic books of one
TallyPrime 7.1 Silver (protocol reference §12a.18); the tool was run against those three books and equalled the F11 screen on all nine values.

### Month-wise Sales and Purchase Registers (`month_register`)

`month_register` takes `company_guid`, `kind` (`sales` or `purchase`) and a window of whole months,
`from` the 1st of a month and `to` the last day of a month, at most twelve months, not starting before
the book does. It reads Tally's own Sales Register or Purchase Register by report name, twice, with the
book extent checked before and after and the company and mode checked around the read, and returns
`months`: one row per month, `month` as `YYYY-MM`, and Tally's three columns `debit`, `credit` and
`closing`, each `{"state": "present", "value": ...}` or `{"state": "empty"}` exactly as Tally printed
it. An empty amount is not zero.

Nothing ties the rows to the vouchers, to the trial balance or to `sales_register` and
`purchase_register`; `checks` says so. On the one synthetic book read (BRIDGE PILOT LAB, one financial
year, 13 Oct 2026) the Sales Register's credit was the month's sales invoices, tax included, neither
cancelled nor optional, and its closing was printed for August and September only, as a running total,
with the other months empty; the Purchase Register's debit was the month's purchases as a negative amount,
and its closing was printed for August and, unchanged, for September, a month with no purchase. That
reading is not applied. Equality with Tally's own register screens has not been measured, and `checks`
says `against_register_screen: not_checked`. The set of months is not left unchecked: an answer whose
months are not the window's is refused (`month_register_months_unexpected`). A window that is not whole
months is refused after the company and mode reads and before the register is asked for
(`month_register_window_not_month_start`, `_not_month_end`, `_reversed`, `_too_many_months`,
`_unsupported_boundary`), as is one that starts before the books (`month_register_before_books`) and
Education mode (`month_register_education_unqualified`); an answer Tally marks as a failure, an
empty answer and a book that moved during the read fail the call (`month_register_read_failed` with the
cause). The Journal Register prints a count of vouchers, not amounts, and is not read; the Receipt and
Payment Registers have not been captured.

### Negative Stock (`negative_stock`)

Use `negative_stock` with `company_guid`, `from` and `to` for Tally's own Negative Stock list: the
stock items on that list, each as `item`, `quantity`, `rate` and `value`, in Tally's order. `quantity` is
`{"state": "read", "amount": ..., "unit": ...}` with the sign Tally sent, or `{"state": "empty"}` or
`{"state": "unread"}` (text that is not `<number> <unit>`); `rate` and `value` are each
`{"state": "present", "value": ...}` or `{"state": "empty"}` exactly as Tally printed them, and an empty one
is not zero. A `value`, when present, keeps the sign Tally's export carries, as in the Stock Summary and
the trial balance: a debit is negative and a credit is positive, and the screen shows a credit as `(-)`
and a debit unsigned. On the one book measured, the item at -3 Nos and a rate of 120.00 was exported
with the value `360.00` (a credit) on TallyPrime 7.1 and on 7.2, and the 7.2 screen showed `(-)360.00`
(the screen was not read on 7.1); the other two short items captured had no value, and the sign of a
value can change as an item trades (protocol reference §12a.13 and §12a.16). The list is Tally's own and no verdict is added: being listed does not make an item negative
(on one book Tally also listed items of positive quantity whose closing value was a credit), so read each
quantity. The rule by which Tally lists an item is not established and is not derived here.

It is **not compared with any other report**: `checks` says `list_complete` and
`quantities_against_stock_summary` are `not_checked`, so an item Tally did not list is not shown to have a
quantity of zero or more. A non-empty list is `observed`. An empty answer (Tally answers an empty
envelope) is returned with `state` `not_established` and `reason`
`negative_stock_empty_answer_is_not_a_finding`: it cannot be told from a report that was not rendered, so it
never says that no item is negative. `mask_parties` masks each `item` name.

Measured on synthetic TallyPrime 7.1 Silver books (protocol reference §12a.16): a whole financial year listed
one item at `-3 Nos` with its rate and value; the same book to 31 May listed it at `-5 Nos`, the quantity as at
the window's end, with the rate and value present and empty; a book with stock items and none short answered
an empty envelope. A window that does not start on 1 April, or that runs past the following 31 March, is
named in `unmeasured_in_this_answer` as `window_other_than_from_1_april_within_one_financial_year`. A
window starting before the book is `negative_stock_before_books`, a reversed window
`invalid_date_range`, Education mode `negative_stock_education_unqualified`, and a window the read could
not build a period for `negative_stock_period_not_honoured`.

Small books only. The report has no size check and its cost on a large book is not known, so the read is
admitted the way `stock_summary` is: the master-alteration mark (`ALTMSTID`) times the same assumed worst-case row
(18,296 bytes) must fit 16,000,000 bytes, which admits a mark of at most 874. This ceiling is a stated guess, not a
measurement of this report, and the mark counts masters of every kind. A larger book is refused before any
report request as `negative_stock_too_large`, with `size` (`master_alter_id`, `estimated_bytes`,
`limit_bytes` and `limit_master_alter_id`), and retrying refuses again. After a read, the row count and the
response size are checked against the mark and the admitted size, and a breach refuses the whole read as
`negative_stock_bound_premise_violated`, with the reason (`stock_rows_exceed_master_mark` or
`stock_response_over_admitted_bytes`) in `error.cause`, unless the closing extent shows the book moved
(`negative_stock_extent_changed`). The report is one paired read (two requests), never retried. Any other
refused answer fails the call as `negative_stock_read_failed` with the specific code in `error.cause`
(for example `negative_stock_item_duplicated`, `negative_stock_amount_invalid`,
`negative_stock_tally_reported_failure`, `negative_stock_report_unknown`), as does a report that changed between
the paired reads (`native_negative_stock_changed`). This tool has not yet been run against Tally.

### GSTR-1 status of one voucher type (`gst_status`)

`gst_status` takes `company_guid`, `from`, `to` (YYYYMMDD or YYYY-MM-DD) and `voucher_type_name`, and
optionally `limit` and `offset`. It reads Tally's own GSTR-1 status flags on the vouchers of that one voucher
type in the window, and returns for each voucher its `voucher_number`, `date`, `voucher_type`, `master_id` and
`alter_id`, a `gst_status` and `accepted_as_it_stands`. It returns no amount, party, ledger or narration.
`voucher_type_name` is the exact name of ONE voucher type in the book (take it from `masters`); a type of the
same class under another name is not read, and a name that holds a double quote, a backslash, `&`, `<`, `>`, `'` or a control
character, or a space at either end, is refused before any read (`gst_status_type_name_invalid`).

`gst_status` is `included`, `uncertain`, `excluded`, `not_in_return`, `cancelled`, `optional` or `post_dated`. `cancelled`,
`optional` and `post_dated` are Tally's own `ISCANCELLED`, `ISOPTIONAL` and `ISPOSTDATED` flags read Yes, decided before the three status flags and
never counted as `included`, `uncertain`, `excluded` or `not_in_return`: on the lab's book a cancelled Sales voucher read the included flag Yes,
an optional one read all three No, and a post-dated one read the included flag Yes (one voucher each). `post_dated` is
the flag, not a date still ahead: the lab's post-dated voucher was marked so by hand (Ctrl+T) and dated the day Tally's
current date (the date set with F2, not the calendar's) had been set to.
Whether Tally lists a cancelled voucher anywhere in the return, or a post-dated one while Tally's current date is before
the voucher's date or once it has reached it, is not measured, and ComplyEaze Bridge states neither: look at the return
in Tally. Such a voucher's item also holds `status_flags`, what Tally's three status flags read on it (`included`,
`uncertain`, `excluded` or `not_in_return`); it is returned so that nothing read is hidden, it is not counted, and it does
not say that the return lists the voucher. A voucher that reads more than one of cancelled, optional and post-dated Yes is unread. `not_in_return` means Tally's three flags
all read No. On the lab's Receipts that means Tally does not list the voucher in the return (read with the request before it
fetched the cancelled, optional and post-dated flags; the request as it is now is measured on Sales vouchers only); it is not measured for other types, and the lab's keyed Purchase invoices read `uncertain`. `accepted_as_it_stands` is `true` when a person
accepted the voucher as it stands (Accept As Is) and `false` otherwise; `included` counts such a voucher too, so it is not
'clean', and it is included by a person's choice, not by its figures. A status is Tally's own flag and not a judgement of ComplyEaze Bridge: it does not say the
voucher is right or wrong, or that a return is ready. The reason for a status (the one measured is a tax figure that did not match Tally's own)
is on Tally's screen only; no field carries it, so none is returned. Tally keeps the
status a voucher was given when it was saved (one voucher, cause unverified). A total of 0 may mean that the type name matched nothing. Each page is a new read,
so a voucher saved between two pages can shift the offsets. The result holds `counts` (`included`, `uncertain`,
`excluded`, `not_in_return`, `cancelled`, `optional`, `post_dated`, `unread`, `accepted_as_it_stands`) over the whole window, `total`, and the page of
`items`; page with `limit` and `offset` (`next_offset` when more remain).

A voucher whose flags, or whose cancelled, optional or post-dated flag, are absent, repeated, nested in another element or
spelled another way keeps its row with
`{"unread": code}` (`gst_status_not_reported` or `gst_status_unreadable`) in place of the value and is never read as
`included`; the result's `state` is then `partial` (`gst_status_voucher_unread`). So does a voucher whose `ISDELETED` or
`ISDELETEDVCHRETAINED` flag reads Yes (`gst_status_voucher_deleted`, which its acceptance flag carries too, so it is not counted as accepted): Tally returns
the two flags without being asked, No on every voucher the lab read, a Yes was never measured, and what
`ISDELETEDVCHRETAINED` means rests on its name. On the lab's book, where no Edit Log was seen to be on, one deleted
voucher left the answer altogether. Both flags absent is allowed; one absent beside the other is unread. An answer in which a voucher
cannot be told apart from another (a missing or repeated `MASTERID`, or a missing number, date, type or
`ALTERID`) is refused (`gst_status_read_voucher_unidentified`, `gst_status_read_voucher_repeated`), as is a
voucher dated outside the window (`window_not_honoured`) or of another type name
(`gst_status_type_not_honoured`).

Limits: at most 93 days (`gst_status_window_too_long`), and a book whose voucher mark is above 25,000 is
refused (`gst_status_book_too_large`), because the selection is evaluated on every voucher of the book. That
bound is provisional: the read has been timed only on a book of about thirty vouchers. Measured on one
synthetic book of TallyPrime 7.1 Silver, for the unregistered buyer of its Sales invoices (protocol reference
§9.17 holds the Sales flags and Accept As Is; the by-name selection and the narrow field list are in the fixtures' PROVENANCE.md); a registered buyer, other releases and Gold are not measured, and `excluded` has not been observed.

### Tax-audit test run (`audit_test_run`)

Off by default: the tool is listed only when **Allow tax-audit test runs (audit_test_run)**
(`BRIDGE_AGENT_ENABLE_TAX_AUDIT`) is on, and refused as `audit_tool_disabled` otherwise.

Use `audit_test_run` with `company_guid`, `from`, `to` (YYYYMMDD or YYYY-MM-DD), `test` and the
two group lists `cash_groups` and `bank_groups` to run one test of ComplyEaze Bridge's tax-audit
engine on the company's own books. Today `test` can only be `cash_44ab`, the share of cash in the
books' receipts and payments against the s.44AB(a) limit. A test joins the list only with a real-book
parity record at a pinned reference commit; the tool's schema lists only those tests, so any other is refused
as `argument_invalid:test` before any request.
The rules the engine holds are those of assessment year 2026-27, so `from` and `to` must lie inside
1 April 2025 to 31 March 2026; any other period is refused as `audit_period_outside_ay_2026_27` before a
request, and a part-year inside it is read and judged against the whole year's thresholds.

The group lists are required and have no default: which groups hold cash and which hold bank accounts
is for the user to say (`masters` with `kind=groups` lists the book's groups), and whether an overdraft
group counts as a bank is a judgement for the user's accountant. A group left out is not counted, and
a group named in both lists is refused as `audit_groups_overlap`.

The books are one read made for the call: the company's marks, the company object, the groups, the
ledgers, the trial balance for the period, the voucher types and every voucher of the period (in as many
parts as the period needs, at most 128, after a census that counts the period's vouchers, sent for a
book small enough for one request too), one request at a time, each between two checks that the same
single company is loaded. The marks are read again at the end; a window the census did not count, such as
a book that has never held a voucher, is refused as `audit_window_not_counted`. The engine then checks the read as a whole (the company, the period, the marks, that the
voucher windows cover the period without a gap, that every count agrees) and refuses it by rule code if
it does not hold. Nothing is written to disk, nothing is written to Tally, and the read is not kept.

What comes back is `result.state`: `completed`, or `completed_with_failed_book_checks` when at least one
book check failed; `result.test`; `result.figures`, each with its `id`,
`value` (an integer, or null when the figure has no value, such as a share of nothing), `unit` (`paise`,
`bp`, `count` or `days`) and the test's fixed `definition_text`;
`result.findings`, each with its `id`, `confidence` (`computed`, `indicative`, `needs_document` or
`judgement_required`), `clauses`, the `facts` (a name and the figure id it rests on), and the test's
fixed `title_text`, `limits_text` and `ask_client_text`; the test's `population_note`; and
`book_checks`: the checks evaluated, and the code and count of each that failed. A failed book check
means the books disagree with themselves somewhere (for example the vouchers do not add up to the trial
balance for a ledger), and a figure may then leave something out: a ledger without a master or without a
chain of groups to a primary group (`MAP-0`, `MAP-1`) is in no group, so it counts as neither cash nor
bank. Do not rely on the figures until the failed check is explained. No ledger, group, party or voucher name is returned, and the engine's evidence
references and the subject and detail of a failed check are left out because they name them. A result
past 64 figures or 32 findings is refused as `audit_result_too_large`, never cut short.

Everything this tool returns, amounts included, goes to the AI provider like every other read, and no
redaction setting hides it, so the tool is refused as `audit_redaction_unsupported` while
`mask_parties` or `drop_narration` is on.

Refusals: `audit_part_deadline_exceeded` (a part ran past the 20 seconds a request is given; the same
request would be sent again and Tally may still be building the abandoned answer, so do not retry as it
stands), `audit_part_drain_required` (a previous request was abandoned, in this call or an earlier one,
and Tally may still be building its answer: nothing was sent. ComplyEaze Bridge does not yet probe Tally to
clear that, so the refusal stays until ComplyEaze Bridge is restarted; look at Tally first), `C6-moved`
and `voucher_window_changed_during_read` (someone changed the book during the read: it may be tried
again), `audit_window_divided_by_alterid` (a day too heavy for one request: the read is refused, because
the engine's windows must tile the period by date; refused before any voucher is read when the count
shows it, over 42 vouchers on the first day that holds any or over 85 on a later day), `BIND-GROUP-UNKNOWN` (a group named in `cash_groups`
or `bank_groups` is not in the book: check the spelling against `masters` with `kind=groups`), the
engine's other rule codes for a read it does not admit (`C1-...` to `C11-...`, `CFG-...`), and the
tool's own: `audit_test_refused` (the test refused the book without a rule code), `audit_book_refused`,
`audit_part_unparsable`, `audit_window_marks_missing`, `audit_window_not_counted`,
`audit_window_row_count_differs`, `audit_groups_overlap` and
`audit_result_not_projectable` (the engine returned something this tool does not project). The call has
no overall deadline: it is up to 128 parts, each between two checks, so a client that gives up early
leaves the request it was waiting for abandoned.

Measured on the engine's synthetic book replayed through the simulator. The engine's `cash_44ab` passed
a real-book parity check on 21 Oct 2026 at reference pin 6168aeaa (crate tree 53938eb8), on one book
whose cash payments were zero, so its payments side was checked only at zero. The tool itself
has not yet run against a live Tally: the lab comparison comes first.

### GST booked per counter ledger (`gst_by_counter_ledger`)

`gst_by_counter_ledger` takes `company_guid`, `from` and `to` (YYYYMMDD or YYYY-MM-DD). It makes the reads
`purchase_register` makes, in the same order and with the same drift checks and refusals, and for every voucher with an
entry on a ledger under Duties & Taxes whose master carries a recognised GST duty head it decides one thing:

- **attributed** to the voucher's one ledger under Purchase Accounts, Direct Expenses, Indirect Expenses or Fixed
  Assets, when the voucher is a
  Purchase, Debit Note, Journal or Payment with a ledger under Sundry Creditors on it, has exactly one such counter
  ledger (a round-off ledger under Indirect Expenses counts as one), its ledger entries sum to zero, all its GST entries
  are on one side, and every other entry is on the party, cash, bank or a ledger typed TDS or TCS. A duty head does not
  say input or output, so a sale, a credit note or a tax payment with no supplier on it is never attributed; or
- **set aside** under the first reason that applies: `class_not_admitted` (sales, credit notes, memoranda and any
  other class), `head_conflict`, `unrecognised_head`, `head_absent`, `duty_ledger_without_gst_type` (a Duties & Taxes
  ledger typed neither GST nor TDS or TCS, such as the `Others` an imported ledger reads by default),
  `unresolved_group` (a ledger whose group does not resolve, which may be a GST ledger), `sign_and_flag_disagree`, `entries_do_not_balance` (a ledger sits where the read does
  not look, such as an item invoice of a shape not measured whose purchase ledger is in an inventory allocation), `sales_ledger_present`,
  `gst_on_both_sides` (reverse-charge and set-off journals), `other_ledgers_present` (any other group, such as Current
  Assets or Stock-in-Hand), `no_creditor_party` (such as a GST challan payment),
  `no_counter_ledger` or `several_counter_ledgers`.

`counter_ledgers` gives, per ledger, its `ledger_guid` (Tally's GUID of the ledger), its predefined `group`, its `parent` (masked like a ledger name when it is a group
the book named), `vouchers`, `debit_side_tax` (negative, as
the books state it) and `credit_side_tax` (positive) per head, never netted. `set_aside` gives, per reason, the same
totals and at most 100 vouchers listed. Cancelled, optional and post-dated vouchers are counted and not totalled; a
window that is not complete is refused as `gst_by_ledger_window_partial`. Every total is folded from one decision per
voucher, so a voucher's tax is counted once. Nothing is apportioned, and GST booked to a ledger outside Duties & Taxes
is not seen. It is the books side a CA starts from for expense-head-wise input tax credit (GSTR-9C Table 14); it
decides no eligibility, availment, input or output side, and no return row.

For a GSTR-9C Table 14 working: a counter ledger's `debit_side_tax` is the GST booked against it and its
`credit_side_tax` the GST booked back (a debit note or a return), both as the books state them and never netted here;
the CA nets them for the expense head that ledger is mapped to. Credit-side tax can also be output tax, for an asset or
goods returned or sold to a supplier: the tool does not tell the two apart. `set_aside` holds every voucher with GST
the tool could not place, of every kind: input tax on a voucher with two counter ledgers or an untyped duty ledger, but
also output tax, set-offs and tax payments. The CA places them voucher by voucher from the vouchers listed, so a
working built from `counter_ledgers` alone misses at least the input tax among them. Cancelled, optional and
post-dated vouchers are counted, not totalled, and GST booked outside Duties & Taxes is not seen. A ledger's value for
the year is read separately, from `trial_balance` for the same dates, where the same GUID is the ledger's `guid` (a
masked name is not a key). That value covers every voucher on the ledger, set-aside ones included, so it is never
divided into this tax as a rate. The mapping of ledgers to the table's heads, eligibility, what was availed, and the
GSTR-9 figure the table is compared with are the CA's.

Measured on one synthetic book of TallyPrime 7.2 Silver: fourteen vouchers typed on Tally's screen, read a day at a time and, by the
build before Direct Expenses and Fixed Assets were admitted, as one month of 64 vouchers that the window read in two
parts: purchases on Indirect Expenses, Direct Expenses and Fixed
Assets ledgers and on two expense ledgers at once, two item invoices whose purchase ledger was among their entries
(one with freight), journals with input IGST, with a TDS ledger typed Others, a set-off and a reversal with no
supplier, a debit note, two tax payments (one also paying interest) and a credit note with output tax.
A registered book was read once, by an earlier build that did not yet admit Direct Expenses or Fixed Assets ledgers or name the ledger GUID
(the same synthetic book after its company and one supplier were registered and December's vouchers keyed, TallyPrime 7.2 Silver): none of
December's six vouchers was set aside, and a supplier invoice entered twice (the same reference, date and bill) was totalled twice with nothing
flagged, so the tool does not detect a duplicate invoice. A year of 70 vouchers was read whole in the same run.
Not measured: vouchers created by import, Gold and other releases, a registered book on this build, other item-invoice shapes, more than 64 vouchers
in a window.

### Uncertain vouchers of one voucher type (`gst_uncertain_vouchers`)

`gst_uncertain_vouchers` takes `company_guid`, `from`, `to` (YYYYMMDD or YYYY-MM-DD) and `voucher_type_name`, with
the same checks and refusals as `gst_status` (an exact type name, at most 93 days, a book whose voucher mark is at most
25,000) and no `limit` or `offset`. It reads the status flags `gst_status` reads and adds each voucher's party and
amount, and lists two things in one `items` list, each item carrying a `list`: `uncertain` (Tally's uncertain flag reads
Yes once cancelled, optional and post-dated are decided first) and `overridden` (the acceptance flag, Accept As Is,
reads Yes, with the `gst_status` the voucher reads now; a voucher that reads uncertain or excluded after it was
accepted is the row to look at, and one voucher can be on both lists; that nothing but Accept As Is sets the flag to
Yes is not measured, and the before-capture already held five vouchers with the flag Yes, an Accept As Is on Tally's screen is recorded for two of them, `BP/26-27/0005` and `BP/26-27/0016`, and the origin of the other three is not recorded). An item holds `voucher_number`, `date`,
`party` (the party ledger), `amount` (the invoice total as Tally wrote it, without its sign, an exact decimal) with
`party_side` (`debit` for a negative amount, `credit` for a positive one), `alter_id`, `master_id` and `gst_status`.
A party or an amount that is empty, absent, repeated, zero or not a plain decimal (a foreign-currency amount) is
`{"unread": code}`, never a name or a zero. `counts` are those of `gst_status` over the whole window, except that `unread` also counts a listed voucher whose party or amount was not read. The result
also holds `pre_filing_line`, the line a CA reads before filing, with the number of vouchers Tally marks Uncertain
and the window's dates, naming the voucher type read (the type's class is not checked, so the line does not call it a Sales type), and `filed_period_note`,
which says the list may differ from a filed return and to check against it. The line is withheld (`{"withheld": ...}`) and the note is left out, and the
result is `partial` (`gst_uncertain_voucher_unread`), when a voucher's flags or acceptance flag were not read or a
listed voucher's party or amount was not, so the number in the line is never short. The line alone is withheld
(`{"withheld": ...}`), the result staying `complete` with its counts, when no voucher was read in the window
(`gst_uncertain_no_vouchers_in_window`, as a mistyped type name gives) or none is marked Uncertain
(`gst_uncertain_none_marked_uncertain`), because a line of 0 would read as clearance. A cancelled voucher reads an
empty party and amount and is not listed as uncertain; one whose acceptance flag reads Yes is listed under
`overridden`, and its empty party and amount then make the result `partial` and withhold the line.

Two reads are made. The first is `gst_status`'s own read, which counts the window's vouchers; a window above
1,000 is refused with `gst_uncertain_window_too_large` (the count is in the refusal's `counts`), before the second
read, which asks for the party and the amount and so makes Tally send each voucher's ledger entries as well
(about 7.5 KB a voucher on the lab's book). That bound is provisional. Every status is read from the second read
alone. A second read that returns fewer vouchers than the first counted is refused (`gst_uncertain_wide_read_short`, with both numbers in `counts`), never reported as a smaller window. A count of rows above this server's row limit is refused (`gst_uncertain_rows_over_limit`). The answer is not paged, so one over this server's response byte cap is refused whole (`agent_response_too_large`, no counts): ask for a shorter window. The party is
the party ledger (a cash sale shows the cash ledger); no GSTIN is fetched. The amount is the invoice total on the
party's entry, tax and round-off included, not the taxable value of the GSTR-1 tables. `mask_parties` masks the
party names of this tool's answer and not its amounts, and the answer carries both to the assistant, which
`gst_status` does not.

The reason for uncertainty is not read, and bills missing an HSN/SAC are not split out from the others: they need
re-saving with the code, not Accept As Is. To confirm an acceptance after someone accepts vouchers in Tally, run the
tool again and compare by `master_id`: the acceptance is confirmed only if an earlier run listed the voucher uncertain
and this run lists it overridden with `gst_status` included and the same amount. That shows only that the acceptance
flag was set and the voucher reads included with the same amount, not that the voucher is correct or complete. Nothing
is kept between runs. Measured on one synthetic
book of TallyPrime 7.1 Silver, once before and once after one Accept As Is, which changed that voucher alone, for the
unregistered buyer of its Sales invoices. Not measured: item invoices, a registered buyer, credit notes or other
Sales-class types, foreign currency, the amount of a post-dated voucher, an Accept As Is of several vouchers at once,
Gold and other releases.

### Stock Summary

Use `stock_summary` with `company_guid` and `as_of` (YYYYMMDD or YYYY-MM-DD) for
the closing stock value per stock item, with the total of those values checked
against Tally's own Stock Summary, and whether inventory is integrated with the
accounts. `as_of` must be a month's last day, the shape of every period end
measured for stock, and not before the book's start or after today. Any other
date is refused as `stock_summary_as_of_not_measured` before any request, and
retrying the same date refuses again. Month ends inside the year were measured on
two synthetic TallyPrime 7.2 books (a voucher dated on the end date was counted
on a 15 October read, not at a month end); February's end, a month end with a
voucher on it, several vouchers or a Stock Journal or purchase on the end date, 7.1 at such a date and a company whose year does not start on 1 April
were not. A month end before the book's start or after today is refused as
`stock_summary_as_of_before_books` or `stock_summary_as_of_in_future`, after the
status, company and extent reads. The period runs from 1 April of the year
containing `as_of` (the company's own financial-year start is not read), or the
book's start if that is later, to `as_of`.

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

**A quantity under the root is returned only where Tally agrees with it; one
inside a stock group is the item list's own figure.** `checks` says per field
what is `checked`, `checked_where_agreed` or `not_checked`: the closing-value
total is checked; the closing quantity only where it is `agreed`; each value on
its own, the names, parents and base units, whether the date was honoured and
whether the item list is complete are not.

An item's `closing.quantity` is an object with a `state`. It is `agreed`, with
`amount` and `unit`, only where the item sits directly under the root and Tally's
own plain Stock Summary has exactly one line with the item's name, and that line
shows the same quantity, the same unit and the same amount (two empty amounts
agree).

It is `inside_stock_group`, with `item_list_amount` and `item_list_unit`, when
the item's own parent is a stock group. The plain report has a line only for what
sits directly under the root, so a line of the same name is read as a group's,
and the quantity is the item's own row in the stock item list, which nothing
compared. The keys differ from `agreed`'s on purpose: never read this figure as
an agreed one. On one synthetic book (TallyPrime 7.2, groups three deep beside an
item under the root, two units, one negative quantity), each such quantity
equalled Tally's Stock Summary screen with every level expanded. Neither the
value total nor the item count checks a quantity, and an item with an empty or
zero value adds nothing to the value total, so nothing in a read vouches for a
nested quantity.

Every other state says why no quantity is returned:

- `none_sent`: Tally sent no closing quantity, which is not zero.
- `unread`: the quantity is a compound unit, or has a unit with a space in it. It
  is also counted in `totals.closing_quantity_unread_count` and does not refuse
  the read.
- `parent_not_resolved`: the item's parent reads `Primary` without Tally's root
  marker (ASCII case and outer spaces ignored), or carries a marker anywhere
  else. It may be a group a person named Primary or a root spelling Bridge does
  not know, so it is neither compared with the report nor taken for a group.
- `report_has_no_line`: the item sits directly under the root and no line of the
  report carries its name: it has nothing to show, or the report names it
  differently (an alias, a spacing difference, a case difference). It is also
  given when Tally sent no parent for the item, whatever a line shows.
- `report_name_not_unique`: more than one line carries the item's name.
- `report_differs`: a line carries the item's name and shows another quantity,
  another unit, no quantity, or another amount.

The report does not say whether a line is a stock group or an item, so a line is
tied to an item by its name, quantity, unit and amount and by the item sitting
directly under the root. A quantity below
zero is returned like any other: on one synthetic book, a sale of an item that held
none left a stock line of `-5 Nos` with no value, in the item rows and in the
report alike (protocol reference §12a.13).

Each item carries `name`, `guid`, `parent`, `base_unit` and `closing`; `closing`
holds `value` (a plain signed decimal exactly as Tally sends it: the sign is
kept and never flipped) and `quantity`. The value is `null` where Tally sent none,
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
`limit` and `snapshot_id` behave as in `masters`, and the result reports `snapshot`.

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
godown or batch split and no rates are returned, and a quantity only where the
report agrees (above). A book in which no
item has a closing value returns no item today; it waits for a capture of such a
book.

A company split by year, whose sibling companies share the GUID, is refused: the
company-flags read requires exactly one Company row for the GUID
(`company_flags_not_one_row`).

Under `mask_parties`, an item's `name` and `parent` are masked like a party name,
because stock-item and stock-group names are free text that can carry a customer's
or supplier's name; Tally's reserved root as a parent is shown as `&#4; Primary` and is
not masked. `guid` is not masked and is the identity the `items` filter uses.

### Profit and Loss and Balance Sheet

`profit_and_loss` and `balance_sheet` take the same `company_guid`, `from` and
`to` as `trial_balance`, and read that Trial Balance under the same checks.
Inside the same bracket they also read the group tree and Tally's own Balance
Sheet for the window, and both read Tally's own Profit and Loss too (#692).

- **Lines.** Each ledger is classified by the reserved identity of its primary
  group, the last group in its chain. That identity survives renaming.
  - A P&L line is the window's debit plus credit movement.
  - A Balance Sheet line is the closing balance at `to`.
  - Signs follow the Trial Balance: a debit is negative, so a profit is positive.
  - Each line's `amount` sums the amounts Tally returned and counts the empty
    ones it left out.
  - `lines` is null while the tool's result (`net_result`, or the Balance
    Sheet's `carried`) is not established, so a derived line is never shown
    as the statement. `balance_sheet_gate` and, where it is not null (on a Balance
    Sheet only when a closing stock was considered), `tie_out` then show how each
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
    line with an amount that Tally does not show, refuses every result as
    `tally_balance_sheet_differs`, with those lines named;
  - Tally's own Balance Sheet lines sum to zero (each line's amount is the column it
    carries; a debit is negative). On the one book measured (TallyPrime 7.2, §12a.21),
    Tally's screen showed a gap in the book's opening balances as `Difference in
    opening balances`, and the export had no line for it. When the lines do not sum to
    zero the result carries `tally_balance_sheet_sides` (`credits` and `debits`, as
    magnitudes), the headline says both totals, and no result is established:
    `tally_balance_sheet_does_not_balance` when no other reason applies, the other
    reason when one does;
  - the Profit & Loss A/c ledger is returned in the Trial Balance.
- **Reasons** a result is `not_established`: `unclassified_ledger_carries_an_amount`,
  `closing_stock_not_derivable_from_trial_balance`,
  `profit_and_loss_ledger_not_returned`, `tally_balance_sheet_differs`,
  `tally_balance_sheet_does_not_balance`, and for gross and net
  `tally_profit_and_loss_differs`.
- **Top-level state.** `result.state` is `observed` only while this tool's
  result is established (`profit_and_loss`: both `gross_result` and
  `net_result`; `balance_sheet`: `carried`). Otherwise it is `not_established`
  and `result.reason` carries the same reason code as the nested result; the
  weaker result decides. It was always `observed` before (#984). No figure, gate
  or withheld line changes.
- **Limits.**
  - The gates are what catch what the Trial Balance cannot see, such as stock
    valued from stock items. One integrated inventory book has been measured
    (see Tally's own closing stock below); every other book with stock items
    is expected to refuse.
  - A book with more than one currency master is refused before the Trial
    Balance is read (measured once on the lab's multi-currency book), so an
    unadjusted forex difference (#683) never reaches the gate.
  - A Tally line the derivation has no counterpart for, such as a heading with
    an amount, refuses the results rather than being guessed at. On the one
    book measured the export did not carry Tally's `Difference in opening
    balances` line (§12a.21), which is why its own lines must sum to zero, and
    the derived lines too before the closing stock is used. A balanced average-cost
    book whose rounded lines sum to a paisa off zero has not been measured.
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
  - An Opening or Closing Stock line that carries an amount refuses, except
    the one case below.
- **Tally's own closing stock (#692).** On an integrated inventory book,
  Tally's Profit and Loss carries its closing stock as a `Less: Closing Stock`
  line (protocol reference §12a.21; the report's shape is §12a.11). Both tools read that Profit
  and Loss, and use the figure only when all of these hold; otherwise the
  result is the one a book without the figure would get, and `closing_stock`
  says why it was declined:
  - the line is spelled exactly so, appears once, carries a debit in its sub
    column alone, the `Opening Stock` line is shown empty (opening stock is
    not measured) and the `Cost of Sales :` heading appears once;
  - the company's own flags (`ISINTEGRATED` and `ISINVENTORYON`, read only
    when the Profit and Loss offers the figure) are both `Yes`;
  - no Stock-in-Hand ledger carries an amount;
  - the derived Balance Sheet lines sum to zero (a Trial Balance that does not
    balance leaves Tally's `Difference in opening balances` out of this book's
    Balance Sheet export, so the statement would not balance);
  - that one figure closes every gap at once: Current Assets (compared with
    the derived group plus the figure), the Balance Sheet's Profit & Loss A/c
    line and the trading result (moved by the figure), and the `Cost of Sales :`
    heading (the derived Purchase Accounts plus Direct Expenses, less the
    figure). `Add: Purchase Accounts`, Tally's name for the purchase line on
    such a book, then ties to Purchase Accounts.
  `closing_stock` is `{"state": "applied", "value", "source":
  "tally_profit_and_loss_less_closing_stock"}`, `{"state": "declined", "why",
  "lines"}` (`why` is `shape_not_as_seen`, `not_a_debit`,
  `opening_stock_not_shown_empty`, `book_not_shown_integrated`,
  `inventory_flags_not_read`, `balance_sheet_does_not_balance` or
  `does_not_close_every_gap`), or null when no closing stock was considered:
  the Profit and Loss offered none, or the statement was refused for another
  reason first (an unclassified ledger, a Stock-in-Hand balance, no Profit &
  Loss A/c ledger). The derived `lines` leave the figure out (the Trial Balance
  holds no closing stock), so they do not add up to the result: the figure is
  the separate line `closing_stock`. A flags read that fails is a declined
  figure (`inventory_flags_not_read`), not a failed statement; so is a
  `balance_sheet` whose Profit and Loss cannot be read (no closing stock is
  considered and the sheet is the one it always was). The figure is Tally's, not derived or taken from
  `stock_summary`, and the derived Current Assets line keeps the Trial
  Balance's sum. Measured on one synthetic book (three items, TallyPrime 7.1 Silver): opening stock, a costing
  method, a non-integrated book, a window spanning two years and more than one
  item or purchase rate are not.

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
`EDUMODE = Yes` itself was observed on a live Education instance on 22 September 2026 (with `SILVER = Yes` and `GOLD = No`). The end
side is held to the same rule without a live measurement in these shapes, so an Education
whole-month read ending on the 30th is refused.
A genuinely empty voucher response uses the same wider-window
corroboration as `vouchers` before zero movement can be reported; when the
corroboration cannot confirm it, the refusal's code is its reason
(`empty_uncorroborated` or `empty_window_wider_read_nonempty`, explained under
`vouchers` in the protocol and migration notes). Cancelled and
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
only when the voucher carries one), returns `reference_date` only when the voucher
carries one (`YYYYMMDD`; measured on one synthetic book, on one Purchase), does not
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
  `{ledger, head, raw_head, amount}` (each entry also carries what the vouchers read carries on it, among it
  `cost_centre_allocations`, with the cost-centre and category names masked as `vouchers` masks them). An entry on a Duties & Taxes ledger with
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
  post-dated vouchers are returned flagged, not excluded. `reference_date` (the
  voucher's `REFERENCEDATE`, `YYYYMMDD`) is present only where Tally sent one; one synthetic book
  was read (protocol reference 8.2f), a sale was not.
- **What `state` means.** The response `state` follows the rule `vouchers` and
  `voucher_presence` use (#985, #1031): a non-empty window is `complete` only when
  every voucher read was checked against a separate count of the window (a census,
  which ComplyEaze Bridge sends for every book that has held a voucher, however
  few), and an empty window only when its corroboration read confirmed it: the
  read widened by a day on each side was also empty and the company has no
  vouchers. Otherwise it is `partial` with `reason`
  `nonempty_window_unqualified`, or, for an empty window, the corroboration's own
  reason: `empty_uncorroborated` (the widened read was also empty, but the company
  has vouchers) or `empty_window_wider_read_nonempty` (the widened read found rows
  on the neighbouring days, which cannot count the window), and the rows are still
  returned. Before #1031 the registers called a
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
  with several currencies; any GSTIN, a `REFERENCEDATE` on a sale, or cancelled, optional or
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
  corroboration confirmed), otherwise `partial` with `nonempty_window_unqualified`
  (an empty window it could not confirm carries `empty_uncorroborated` or
  `empty_window_wider_read_nonempty`, as there).
  A row's `status` is separate.
- **A Credit Note keeps Tally's signs.** It is returned as a row with its signs
  reversed as Tally sends them: the tool neither nets nor flips, so a caller that
  sums tax over a window must add signed amounts. The measured Credit Note of
  1,000.00 with 90.00 CGST and 90.00 State Tax came back with the sales entry
  `-1000.00`, each tax entry `-90.00` and the party entry `1180.00`, where a Sales
  row has the sales and tax entries positive and the party entry negative.
- **The cost varies by book.** A call sends 102 requests on a book with 8
  ledgers and one currency (96 were recorded before a small book was counted, plus
  the six the count adds; not measured since) and 118 on one with 44 ledgers and two
  currencies (base-currency reads are added). The result does not report the cost.
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
  `REFERENCE` or a populated `PARTYGSTIN` on a sale; a `REFERENCEDATE` on a
  sale; a ledger or voucher kept in a currency other than the book's base
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
covers only the identity and marks reads it sent. `voucher_alter_id` is null
when Tally's answer carried no voucher mark, which it omits for a company that
has never held a voucher, as `ledger_masters` reports it; a number, 0 included,
is the mark Tally sent.

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
- **What is held.** Only a `complete` window; a `partial` one (a withheld
  foreign-currency voucher) is read again by each page.
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
- **What a page cannot see.** A change that moves neither mark. Each screen
  action measured on TallyPrime 7.1 moved a mark (§11c.5, one run each: a voucher
  delete moved `ALTVCHID` by 2, a cancel and a save with no change by 1, a
  regroup, an opening change and a ledger create or delete `ALTMSTID` by 1).
  One run on TallyPrime 7.2 did not (PARTIAL, reported by the lab, §11c.5): opening
  Tally's GST Annual Computation report showed a prompt that it would update the
  GST status of 20 vouchers; answered Yes, neither mark moved, and no read of the
  status was made before and after. If the update did change a status, a
  held page of a read that carries a voucher's GST status (`gst_status`,
  `gst_uncertain_vouchers`, and any page with GST fields) could be stale on 7.2 with
  both marks unchanged. Open until a lab row runs: whether a read through the
  gateway can cause that update, and whether 7.1 does the same.
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
  twice, the second read wider (with its own census on a large book; a small book's
  wider read sends none), which these figures do not include; or the read stopped). A call past 240 s on Claude Desktop is cancelled and
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
other tools that read a window (`verify_import`, `voucher_presence`) do not report it
yet; `ledger_movement` does (below); and a `vouchers` refusal raised after the whole
read carries no window.

`ledger_movement` reads its voucher window twice, a planned read and a replay of the
same parts that proves nothing moved, so its `result` carries the block (`read_cost`
or `read_cost_left_out`, beside `ledgers`) when the two reads together were slow
enough, with `window_reads: 2` and the same rules: the vouchers are the window's,
counted once; `observed_seconds` add both reads; `floor_seconds` is each read's own
gaps between consecutive census reads (the replay sends none, so it is the planned
read's); the 240 s verdict is of the two voucher reads together, so a window whose planned
read fitted can read `window_too_long` once the replay is added. The figures cover
the two voucher reads alone: not the two ledger catalogue reads, the company check,
or the wider read an empty window gets in each of the two reads (each pays its own
census when the book is large; the verdict is then `not_established`). A
`ledger_movement` refusal states no block yet. The result of a quick call is
unchanged.

### What one windowed call will cost, before it runs (`voucher_read_cost`, #1465)

`voucher_read_cost` takes `company_guid` and nothing else: no window, because the figure has none.
The census a window read pays follows the book's voucher mark, not the window, so a narrower window
does not lower it and N one-day calls pay it N times. The tool reads the company's marks once (the
paired, identity-bracketed read every windowed call opens with: the identity read, then one marks read;
no census and no voucher request) and says, for `vouchers`, `ledger_movement` and `verify_import`
each, the most census reads one call of it will send on this book (a read can be refused before its census
ends).

- `voucher_mark` is the mark that read saw. `per_call` has one entry per tool: `state: counted` with
  `census_reads` and `floor_seconds`, or `state: refused_by_allowance` with `code:
  voucher_window_book_too_large` and `limit_census_reads` (a mark above 2,097,152, the refusal the read
  itself makes, from the same function; no floor is given for a read that sends no census).
- `census_reads` is **derived** from the mark and the read's own rules (`census_read_count`, which the
  read also calls), never a second copy of them. `vouchers` counts a small book (#1314), so it sends at
  least one census read on any book that has held a voucher; `ledger_movement` and `verify_import` send
  none while the mark times their default voucher size fits one request (a mark of 170 or less). A
  book that has never held a voucher (no mark) sends none, and only then is the count 0.
- `floor_seconds` is **derived from the gate's rule**, as `read_cost.floor_seconds` is, and from the
  same function: (N - 1) x 0.5 s, rounded down. It is the least time spent on the gaps between census
  reads of a call that is not refused earlier. It leaves out every data part, and the recorded runs took
  more than twice their floor. There is no estimate and no field for one: a computed figure was cut from
  #1289, and a floor is never presented as one.
- `measured` is **constant text**: whole calls the protocol reference records (§11c.5), each with its
  census reads, data parts, seconds (the whole call, not the census alone), book, host, date, confidence and
  `not_a_bound: true`. Bridge computes nothing from them. The book's
  mark is rounded (it is not a lab company).
- `host_limits` is the block `read_cost` states, unchanged.
- If the marks cannot be read, the call fails with the code that read gives (for example
  `master_checkpoint_not_observed`) and states no figure; a missing figure is never 0.
- It is as of its own read of the mark. The call that follows reads the mark again. It refuses
  nothing; `vouchers` and the other two read as before.

Not covered: the data parts (they need the census), the wider read an empty window gets in `vouchers` and in
`ledger_movement` (it pays its own census on a large book), the second window read of `ledger_movement`
(it sends no census),
`outstandings` and `voucher_presence`. Windows hosts are not measured.

### Search and summaries in `vouchers` (#1230)

Search, and the `ledger`, `month` and `voucher_type` summaries, work on the rows `vouchers`
has already read and labelled and send no Tally request of their own, so each costs what the
same `vouchers` call costs. The `group` and `primary_group` summaries also read the ledger
list and the group list (see below).

**Search.** `voucher_number`, `reference`, `narration_contains`, `amount` and
`suspense_tagged` keep the vouchers that satisfy every criterion given.

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
- `suspense_tagged: true` (only `true`, and only together with `ledger`, the
  suspense ledger) keeps the vouchers of that ledger whose narration ends in a
  suspense tag ComplyEaze Bridge writes: `Bridge: purpose not confirmed;
  reclassify` (a cash line answered "don't know") or `UNIDENTIFIED - reallocate
  from <ledger>` (a party no mapping names; the whole tail, the tag's own words
  and the ledger, is compared as the build compared it, ignoring case, spacing and
  hyphens, and the ledger must be one of the voucher's own; `Bridge: purpose not confirmed; reclassify` is matched exactly). One
  closing ` [BRIDGE:...]` marker set off by whitespace, as a file imported by hand
  carries, and trailing whitespace are ignored; a second marker, a marker with no
  space before it, or text after the tag means the tag is not at the end and the
  voucher is not listed. The tag must name one of the voucher's own ledgers, not
  necessarily the `ledger` you gave. Cancelled and optional vouchers are listed
  too; each item carries its flags. Each item says which tag in
  `matched.suspense_tag` (`purpose_not_confirmed` or `unidentified`). The tag
  classifies; it does not define what is in suspense: a voucher on the ledger
  whose narration was edited after the tag is not listed, and the same text
  typed by hand matches, so a listing is not proof that ComplyEaze Bridge wrote
  the voucher. Read the ledger without the filter too, and treat every voucher
  left over as still in suspense. Refused as `search_suspense_tagged_needs_ledger`
  without `ledger`, and `search_narration_redacted` where narrations are withheld.
  A value other than `true` never reaches the search: the argument check of a
  tool call refuses it as `argument_invalid:suspense_tagged`
  (`search_suspense_tagged_invalid` is the search's own second guard). Not measured: a
  tagged narration read back from a real Tally (no capture holds one), including
  a hand import and a narration near the 2000-character limit.
- A term of spaces only, a narration phrase under three characters and an
  amount that is not a plain positive decimal are refused before the window is read (the identity read has already happened)
  (`search_criterion_empty`, `search_narration_too_short`,
  `search_amount_invalid`). An empty term, or one over 256 characters, is refused
  first by the argument check as `argument_invalid:<name>`, so a tool call does not
  reach `search_criterion_too_long`.
- The search runs last, after the window is labelled and after the ledger and
  type selectors, so a zero from a `complete` window is a checked zero. A voucher
  withheld for a foreign-currency amount has no amounts to compare: an `amount`
  search keeps it (listed in `withheld_vouchers`, never as an item) and the result
  stays `partial`.
  Every other criterion is decided on the fields a withheld voucher does carry.
- A held window is reused only for the same search: the search is part of the
  question a held window answers. `voucher_types` counts (`included`, `in_scope`)
  are taken before the search, so with a search they do not add up to `total`.

**Summaries.** `summarise_by` (`ledger`, `month`, `voucher_type`, `group`, `primary_group` or `cost_centre`) replaces
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
  same arguments without `summarise_by`, narrowed to the bucket, lists the rest: for a month bucket narrow `from` and `to`; for a ledger bucket pass that ledger when the call carries none; for a type bucket pass `voucher_class` (a superset when a class has child types) or the type's GUID (a type name is refused as `voucher_type_ambiguous` when the types with that name differ from the class of that name, or from the types whose reserved name it is).
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
  the vouchers behind them (for `cost_centre`, `totals` counts each allocated entry once and
  `vouchers_summarised` also counts vouchers with no allocation; see below). `excluded_from_buckets` counts the vouchers left out
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
- Ledger names in `ledger` buckets, and in the `members` of a `group` or `primary_group` bucket, are masked when parties are masked; group names are not (below).
- **`group` and `primary_group`** add each ledger's entries under the group it sits in:
  - A `group` bucket holds only the ledgers directly under that group, as the ledger master
    shows it (a ledger directly under the reserved root is under `Primary`, with `reserved_name`
    null). So a predefined group whose ledgers all sit in user sub-groups, such as Sundry Debtors
    split into two, has no bucket of its own, and Indirect Expenses has a smaller one than Tally's
    group total. The whole figure of a group, descendants included, is in **`subtree_totals`**: each
    group on any bucket's chain with its `depth` (1 for a group under the root), `vouchers`,
    `debit`, `credit` and `net` over everything under it, largest movement first, at most 60, with
    `subtree_totals_total` exact and `subtree_totals_complete`. Each row says what it `covers`: a
    `subtree_totals` row covers the group and everything under it, a `group` bucket only the
    ledgers directly under it, so a row is never added to a bucket of the same group. They overlap (a ledger counts under
    every group above it), so they do not add up to `totals`. Every page carries them. When
    `subtree_totals_complete` is false the groups past the 60th are not in the answer: the groups
    directly under the root are all in `primary_group`, and any other group's figure is the sum of
    its member ledgers' buckets (sum them from a `summarise_by: ledger` call, or narrow the window).
  - A `group` bucket carries `reserved_name` (empty for a group the book's user made), `chain`
    (every group from it up to the one under the root, nearest first, each with `name` and
    `reserved_name`) and `primary_group`. A user group is its own bucket, however deep it sits.
  - A `primary_group` bucket holds every ledger under the group directly under the root and is
    keyed by its reserved name, so a renamed predefined group stays one bucket and `group` shows
    its current name; a group the book's user made there has no reserved name and is keyed by its
    name, so two such groups are two buckets.
  - Each bucket lists `members`: the ledgers in it with their own `debit` and `credit`, largest
    movement first (never by name), at most 10, with `members_total` exact and `members_complete`.
    Where a bucket is complete, its members add up to it. Every entry of the selected vouchers is
    counted, also when `ledger` is given.
  - A group bucket cannot be narrowed with a selector, so its vouchers are traced through a member
    ledger with `ledger`; only 10 members are named, and `voucher_refs` names up to 5 vouchers.
  - Group names are shown as the book has them, as `parent` is in `ledger_masters` and
    `trial_balance` (a group may be named after a party); member ledger names are masked when
    parties are masked.
  - The groups are the book's masters read now, not the grouping in force on each voucher's date.
    Each ledger's group chain is read before the window and again after it and must be the same, or
    the call refuses: `group_snapshot_drifted` if a chain changed, `ledger_snapshot_drifted` if the
    set of ledgers did. (A group that no listed ledger sits under could change unseen.)
  - A ledger the ledger list does not hold, or whose chain cannot be walked to the root, refuses the
    whole summary as `summary_group_unresolved`, with `cause` one of `no_parent`, `group_absent`,
    `group_name_repeated`, `reserved_name_missing`, `cycle`, `exhausted`, `top_group_not_under_root`
    (the top group of the chain has no parent, or none that is the root) or `ledger_not_in_catalogue`
    (`no_parent` also covers a group name Tally returned that cannot be carried, one with control
    characters or over 1,024 bytes); `ledger` names the first ledger that could not be placed (masked
    when parties are masked; more may follow once it is corrected) and the group is not named: a group total that is short of an entry it could not place is the
    misleading answer, so there is no "unplaced" bucket. A ledger no voucher of the window touches
    does not matter. A voucher withheld for a foreign-currency amount is still in no bucket (the
    result is `partial`, as for every summary).
  - Cost, counted from the code and measured once in total (64 requests a call in the live run below): 30 requests on top of the window read, with or without
    `ledger` (its name is resolved against the same ledger list). A paired, identity-bracketed read
    is 6 requests; a group summary makes five: the company's marks first, to size the ledger list,
    then the ledger list and the group list before the window and again after it. A book whose
    master-alteration mark is above 11,983 is refused before the ledger list is read
    (`summary_group_book_too_large`, with `size`). The limit is provisional and computed, not
    measured: it is 16 MiB divided by an estimate of 1,400 bytes a ledger. The 16 MiB is a size chosen
    here (half the transport cap) and not verified as safe for the gateway; the 1,400 is the estimate
    the compliance ledger read uses, which that read measured at 1,104 bytes a ledger (a book of
    1,989 ledgers) and 1,221 (a real book of about 9,500) on its own list without balances; this list
    read 1,175 bytes a ledger in the live runs of 6 and 7 October (51,698 bytes both times, for a book of 44 ledgers: a small book, so
    the fixed part of the answer weighs in, and not a basis to raise the limit).
    The mark is an upper bound on ledgers (every other master raises it), so a smaller book may be
    refused, and the limit is about half of what the transport's own rule admits for the same list;
    it will not be raised without a measurement of bytes a ledger on this list. Then use the ledger, month or
    voucher_type summaries and `ledger_masters` for each ledger's group. This list has not been
    measured on a book of thousands of ledgers; a large book's ledger list is large, and a real book's group buckets may need several
    pages (use `offset`). For ledger totals over a month or more on a large book, `trial_balance` is
    the cheaper read.
  - A later page comes from the held window with the placements it was read with, so it reads no
    masters.
  - Checked once against a live Tally, and what was not (see the paragraph below): one run on one
    synthetic book. The tests also use a live ledger catalogue and group snapshot of that book and
    the same book's `trial_balance` (its parent column comes from the same ledger collection, so it
    checks the placement logic, while its amounts are independent), and scripted answers for the
    call itself, with a group snapshot derived from the live one for the end-to-end tests. Not
    measured: a large book, a group renamed or moved while a window is read, a held later page, the
    drift refusals.

- **`cost_centre`** (#1231) adds the entries' cost-centre allocations (`cost_centre_allocations`, protocol
  reference §8.2g) by category and centre:
  - A bucket is one cost centre under one cost category: `group` is the centre and `category` the
    category, both masked when parties are masked, as the masters tool masks them. A centre named
    under two categories has two buckets. Each allocation adds to the side its own amount's sign
    gives, as an entry's does. `members` lists the ledgers whose allocations fall in the bucket,
    as for a group bucket. With `ledger`, only that ledger's entries are counted
    (`entries_counted` is `selected_ledger`).
  - Each category allocates the whole entry: in the capture, one entry was allocated in full under
    two categories. So the categories overlap, and buckets of different categories are not added up.
    `category_totals` gives each category's `entries`, `debit`, `credit` and `net`. `totals` counts
    each allocated entry once. `entries_unallocated` holds the counted entries that carry no
    allocation; with `totals` they are every counted entry.
  - An empty summary says that nothing in the window was allocated, not that the company's Cost
    Centres setting is off: allocations were read on a book with the setting at No, so the summary
    neither reads nor refuses on the setting, and sends no request beyond the window's.
  - An entry whose allocations could not be read whole refuses the summary as
    `summary_cost_centre_unreadable`, with the entry's own reason as `cause`. An entry where one
    category's allocations do not add up to the entry's amount refuses it as
    `summary_cost_centre_allocation_mismatch`; no capture shows one. Another grouping of the same
    window is not refused.
  - Measured on one synthetic book with Cost Centres on (TallyPrime 7.2 Silver, 27 October 2026), on
    the saved raw answers of two vouchers keyed on Tally's screen: a Payment allocated to two
    centres of one category, and a Journal entry allocated in full under two categories. The built
    tool's summary has not been run against a live Tally.

A summary over a `partial` window is only as complete as that window: `state` and `reason` say which, and `basis` does not repeat them.

**Group summaries checked once against a live Tally** (7 October 2026; TallyPrime 7.1 Silver; a debug build of the
pull request's head, which is not in a published build, with the response budget raised to 2,000,000; the
synthetic book of 67 vouchers of the 6 October check below, plus one voucher dated after its year; read-only, one request at a
time). `summarise_by: group` over the year: `complete`, 12 buckets, 64 vouchers summarised, with exclusions
`cancelled` 1, `no_accounting_entries` 1 and `optional` 1 (together the book's 67), 18 `subtree_totals`. Every
bucket's debit, credit and voucher count equalled the 6 October run's ledger buckets added up by the trial
balance's own parent column; `totals`, `vouchers_summarised` and the exclusions equalled that run's; the
`subtree_totals` equalled the trial balance rolled up the group tree; each `chain`, `primary_group` and
`members` list agreed with the book's group tree. `summarise_by: primary_group`: `complete`, 8 buckets, the
same 64 vouchers and totals. Cost: 64 requests a call (34 for the window and 30 for the group modes, as
counted from the code), about 10.6 and 11.1 seconds; the answers were 45 KB and 24 KB. One run each, a debug
build; no large book, no group renamed or moved while a window was read, no held later page.

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
Tally. This book's mark (111) needed one count read; a large book needs many more. Every `ledger`, `month` or `voucher_type` summary or search call reads the window again at the
same cost (34 requests, 6 to 11 s). `ledger` adds 12 requests, four of them reads of the whole ledger
list, which grows with the ledger count; the `group` and `primary_group` summaries add 30 (above). For a month or more on a large book, read ledger totals with
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
  refuses the read as a protocol error (`agent_read_protocol_invalid`). A
  malformed `BILLDATE` refuses as `bill_allocation_date_invalid`. Both reach every reader that shares the voucher parser,
  not only `vouchers`: `voucher_presence`, the empty-window corroboration read and the desktop voucher
  screen. Write-side verification is not affected, because its fetch names no
  allocation fields.
- An allocation is read only when it has a `BILLTYPE`. An untyped one is skipped
  if it is a placeholder (no name) and its `BILLDATE` is not validated; if it has a
  name, it refuses the read (`bill_allocation_field_missing`).

### The `party` field of a voucher row

`party` is the voucher's `PARTYLEDGERNAME` as Tally answered it, and the parse takes that one element and no
other. For a Sales or Purchase voucher it names the customer or supplier. On a Receipt, Payment or Contra Tally can
answer the bank or cash ledger instead. On TallyPrime 7.1 it did for a Receipt and a Payment that ComplyEaze Bridge
imported with the party in `PARTYLEDGERNAME`, and a Contra imported without the element came back with its debit
bank ledger ([reference §9.13](../tally/TALLY_PROTOCOL_REFERENCE.md), the scoped correction of 2026-09-17).
Committed captures of a synthetic book's Payments carry the bank on some rows and the party on others. So the
counterparty of a Receipt, Payment or Contra is read from the voucher's `amounts`, not from `party`. `verify_import`
does not compare the field for a Receipt, Payment or Contra.

### Cost-centre allocations in voucher reads

Each ledger entry that carries cost-centre allocations has `cost_centre_allocations`, a list of
`{"category": ..., "cost_centre": ..., "amount": ...}` as Tally sent them, one row per centre, in Tally's order
(protocol reference §8.2g). The amount is signed as sent; the rows of one entry are not checked against the entry's
own amount.

- An entry without allocations has no such key. In the one capture of the read the vouchers tool sends, Tally printed no allocation element for such an entry.
  On an item invoice the read returns the sales or purchase ledger as a ledger entry with the goods line nested under
  it (§12a.13 of the protocol reference). An allocation list directly under that entry is read like any other; one
  inside the goods line is flagged `cost_centre_allocation_shape_unrecognised`. No captured item invoice carries
  allocations, so where Tally prints them there is not measured.
- **A list that cannot be read whole does not refuse the read.** The entry carries
  `cost_centre_allocations_unreadable` with one reason, and no partial list; the rest of the response is unchanged.
  The reasons: `cost_centre_name_missing`, `cost_centre_amount_missing` (an empty amount is never read as zero),
  `cost_centre_amount_unparsable`, `cost_centre_duplicate` (one centre named twice in one category),
  `cost_category_missing`, `cost_category_without_centres`, `cost_category_present_but_empty`,
  `cost_category_repeated` (one category named twice in one entry), `cost_centre_field_duplicated`,
  `cost_centre_text_undecodable` and `cost_centre_allocation_shape_unrecognised` (an allocation list at a depth the
  read does not take).
- Names are carried trimmed; the amount is carried exactly as Tally sent it (an amount with space around it is
  refused as `cost_centre_amount_unparsable`, as the entry's own amount would be). Under `mask_parties` the category and
  cost-centre names are masked like a party name (as the masters read masks the same names); the amounts are sent.
- An empty `CATEGORYALLOCATIONS.LIST` is not read as "no allocations": the entry carries
  `cost_category_present_but_empty`. No capture of the vouchers read shows an empty container; the empty containers on
  record are in import-verification reads, whose fetch does not ask for the allocation's fields, and there stood for
  a real allocation.
- A repeated child of a category or centre (a list included) reads as `cost_centre_field_duplicated`; no capture
  shows one.
- No request changed: the allocations were already in the answers the read asks for, and were dropped.
- It reaches the readers that return a voucher's entries from this parser: the vouchers read (and its search) and
  the entries the purchase and sales registers list; each marks the two names the same way, so `mask_parties` masks
  them in all of them. The presence, bill-trail and summary reads, `verify_import` and
  `ledger_movement` do not return them (`ledger_movement`'s fetch names three entry fields).

### Bank allocations in voucher reads

`vouchers` called with `bank_allocations: true` gives each ledger entry that has a bank allocation `bank_allocations`, a
list of at most eight, one per allocation as Tally sent it (protocol reference §8.2h; #1231, #1392). Each carries five
fields, `bank_date` (Tally's `BANKERSDATE`), `instrument_number`, `instrument_date`, `transaction_type` and `status`, and
each field is `{"state": "absent"}` (the element was not sent), `{"state": "empty"}` (sent with no text) or
`{"state": "value", "value": "..."}` (trimmed). Without the argument, and in every other answer that returns a voucher's
entries (the registers), the key is not there; the parse reads the allocations either way.

- **No reconciliation verdict.** Nothing in the list says whether an entry is reconciled. In the one capture, nothing was
  reconciled: every `bank_date` was empty and every `status` was `No`. A reconciled entry has not been captured, so what
  those fields read once an entry is reconciled, or cleared again, is not established, and an empty `bank_date` is not
  shown to mean "unreconciled".
- A transfer reference typed in the bank allocation of a Contra sits in its `instrument_number`, on both of the voucher's
  entries, and in no other field the read returns; one typed in the narration is only narration text, and the voucher's
  own `reference` stayed empty in the capture.
- An entry with no allocation, or only an allocation element with nothing in it (most entries' answers carry one), has
  no such key; that does not mean the bank entry has no allocation in Tally.
- **A list that cannot be read whole does not refuse the read.** The entry carries `bank_allocations_unreadable` with one
  reason and no partial list: `bank_allocation_field_duplicated`, `bank_allocation_text_undecodable`,
  `bank_allocation_date_unparsable` (a bank or instrument date that is not `YYYYMMDD`), `bank_allocations_too_many`
  (more than eight) or `bank_allocation_shape_unrecognised` (an allocation list inside another list of the entry, or an
  element inside a field that is read).
- Only those five fields are read. The names and account details an allocation can carry (the payee, bank, branch,
  account number) are not read or returned. The instrument number is any text typed or sent, usually a cheque number or a transfer
  reference, and is returned under every redaction mode, as the voucher's own `reference` is (a name typed inside it
  stays as it is). The `reference` search matches the voucher's own reference only, not this field.
- Measured on one synthetic book of five Contras keyed by hand (TallyPrime 7.1 Silver); other transaction types, other
  voucher types, a reconciled entry and other releases are not measured. No request changed.

### Goods lines in voucher reads

`vouchers` called with `goods_lines: true` gives each ledger entry of an invoice voucher that carries goods `goods_lines`,
one per line as Tally sent it (protocol reference §8.2i; #2015). Each line has `stock_item` and four fields,
`actual_quantity`, `billed_quantity`, `rate` and `amount`, each `{"state": "absent"}`, `{"state": "empty"}` or
`{"state": "value", "value": "..."}` (trimmed); a quantity or a rate keeps its unit as Tally wrote it (`666.667 Kgs`,
`30.00/Kgs`). Where Tally split the line by godown and batch, the line has `batches`, each with `godown`, `batch`, both
quantities and `amount` in the same form. A Purchase line's amount is negative and a Sales line's positive, the sign of
its ledger entry. Without the argument, and in every other answer that returns a voucher's entries (the registers), the
key is not there; the parse reads the lines either way.

- **Invoice vouchers.** In the captures Tally sent a goods line under a ledger entry only on an invoice-mode voucher; the read does not test the voucher's invoice flag and shows the lines of any entry that carries them. A Stock
  Journal answered an empty entry list in the one read of it (TallyPrime 7.2; that answer is not committed, and no
  committed capture holds a Stock Journal), and a Delivery Note, Receipt Note, Physical Stock or Manufacturing
  Journal is not captured, so an entry or a voucher without the key may still have moved stock.
- **Held pages.** The goods lines are parsed on every voucher read, so they count toward the 64 MiB of a held window
  (see "What is held" above) whether or not the argument is given: a window rich in goods lines may not be held, and a
  later page then reads it again.
- **A list that cannot be read whole does not refuse the read.** The entry carries `goods_lines_unreadable` with one
  reason and no partial list: `goods_line_batch_amounts_differ` (the batch amounts do not add up to the line's),
  `goods_line_amount_unparsable`, `goods_line_item_missing`, `goods_line_field_duplicated`,
  `goods_line_text_undecodable`, `goods_lines_too_many` (more than 1,000 lines on an entry or 100 batch rows on a line,
  bounds chosen, not measured) or `goods_line_shape_unrecognised` (a line or batch list at another depth, or an element
  inside a field that is read).
- Item, godown and batch names are masked under `mask_parties`, as the masters tool masks item and godown names.
- Measured on Sales and Purchase invoices of two synthetic books (TallyPrime 7.1), one of them also on 7.2; a Credit or
  Debit Note with items, discounts, additional cost, a bill of materials, a foreign-currency line and a voucher typed on
  Tally's screen are not measured. No request changed.

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
licence mode, or manually imported file. An unnumbered
Journal, Payment, Receipt or Contra is eligible for native posting, one voucher
per approval (a voucher that carries a voucher number is refused), and a saved
batch of 2 to 50 posts in one import after one approval of its summary. One
posting setting covers both, in the MCPB extension and on the command line (#1090).

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
   - A `missing` or `near_miss` row without `folded_twins` whose name is,
     character for character, an alias of exactly one live ledger also carries
     `alias_of`, that ledger's name as Tally lists it. The row is still not
     bound and Bridge never sends an alias: replace the name with `alias_of`
     only after an operator confirms that ledger is the one meant, then run
     `validate_masters` again.
3. Call `build_import_xml` with the payload. It checks exact decimal balance,
   company date extent, live masters, and local journal integrity,
   repeats the full catalogue to reject intervening changes, then writes `<data_dir>/imports/<batch_id>.xml` and records an append-only
   `agent-import-ledger.jsonl` line. The result's `path` is where a person finds
   that file, and `path_state` says how it was obtained: `resolved` (Windows
   reported the file's real location, which on a Microsoft Store install of
   Claude Desktop can be inside the package's own folder), `as_is` (macOS or
   Linux, shown as written), or `unresolved` (it could not be resolved; this is
   the logical path, not confirmed to exist). On a `Journal`, `voucher_number` and
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
   The refusal also applies when amending with a proposals file for a Cash-in-Hand
   bank ledger: the ledger must be changed first. An amendment given as inline
   `vouchers` names no statement ledgers, so this check does not run.
4. In Tally, with the intended company open, use **Gateway of Tally → Import →
   Vouchers** to import the file. Bridge does not dispatch this manual step.
   Alternatively, use the separately approved MCP voucher posting (or, for a
   Journal, the desktop posting) flow below instead of importing the file manually.
5. Call `verify_import` with the company GUID and batch ID. It reads the date
   window back, compares the exact signed ledger entries, reports missing or
   divergent rows and duplicates, adds a proof pair
   (`<batch>.proof.<time saved>.<SHA-256>.json` and `.md`) beside every earlier
   one, and appends the verification status, which names that pair current, to
   the local import ledger. No file is replaced: the journal decides which pair
   is current. The SHA-256 in the name is the JSON's, and only the JSON is read
   back and checked against it; the `.md` is a copy for people to read, and
   ComplyEaze Bridge never reads it back. Every pair is kept until you delete
   it; nothing in ComplyEaze Bridge deletes one. A batch last verified by an earlier build keeps its
   single `<batch>.proof.json`, which no longer changes. It compares the
   date, voucher type, the voucher number when you supplied one, the narration, the entries and, for any
   voucher but a Journal, the effective date (a value Tally leaves out is reported as not observed); it does
   **not** compare `PARTYLEDGERNAME`, which `Payment`, `Receipt` and `Contra` files carry — see the limits noted in reference §9.13.

### Skipped rows and the `transfers` section (#1392)

A transfer between two of the company's own bank accounts prints on both
statements and is booked once: a Contra from one statement, and the row mapped
`skip` on the other. `parse_bank_statement` writes proposals file version 2,
which a build needs for this; a version 1 file holding a skip or a Contra is
refused as `proposals_legacy` (parse the statement again).

- **What a statement build records.** Beside its vouchers: the statement's bank
  ledger GUID, the days it read in full, each Contra row's network reference
  and instrument (the cheque or number the statement prints), and each skipped
  row's date, amount, direction, other ledger, reference and instrument, with a
  flag on any row that has an equal amount going the other way the same day on
  its own statement. A statement whose every row is skipped saves no
  batch file: it appends one record to `agent-import-statement-skips.jsonl` and
  returns `state: skips_recorded`. No pairing is stored; the view is computed
  again from these records on every call.
- **The section.** Every successful build, and the first page of
  `verify_import`, returns `transfers`, except that a written build whose
  result fits the response cap only without it, not even beside the note that
  the check did not run, is answered without it and its envelope is marked
  `truncated`: `state` (`computed`, or
  `not_established` when the view could not be computed or the section was cut
  to fit), `skips_paired`, `skips_open`, `open`, `open_omitted`, `retired` and
  `next_steps`. Each open row is a `bridge_txn_id`, a date and a state, with a
  `batch_id` where one is involved: `not_found`, `tally_unchecked`, `ambiguous`,
  `other_side_unverified`, `held_elsewhere`, `posted_against_skip`, `matched_unconfirmed`,
  `reference_differs`, `reversed_same_day`, `not_established`, or
  `double_booked` for a Contra booked from both statements. No amount,
  reference, instrument, party or ledger is returned.
- **What `not_found` claims.** That the other account's statement went
  through ComplyEaze Bridge, was verified for that day, and holds no Contra of
  the row's day, amount, ledgers and direction that is not already paired, or
  matched by date, with another row, whatever its reference, and that
  ComplyEaze Bridge read every voucher of that day in Tally and found no Contra
  between the two accounts. No read of a whole day of Tally exists yet, so no row
  says `not_found` yet (#1392). A row pairs by an
  equal reference, or by an equal date (k = 0 until two real statements of one
  transfer have been measured), so a Contra the other bank printed on another
  day is not matched.
- **What `tally_unchecked` claims.** Only the first half of that: the other
  account's statement holds no such Contra, and whether Tally holds one is not
  established, because ComplyEaze Bridge did not read every voucher of that day in
  Tally, so a Contra entered in Tally directly is not in its records. Every row that
  would be `not_found` is `tally_unchecked` until a whole-day read exists. The
  assistant reads that day's vouchers with `vouchers` and looks for a voucher of
  the Contra voucher class, or of any other type such as a Journal or Payment,
  that carries the row's amount on the bank ledger the other account's statement
  was built for (and reads its narration where it was not dropped). It tells the
  person what it found, and keeps the skip; it does not look for a voucher dated
  another day or booked against another ledger, a transfer may print on different
  days, and a masked ledger name can read like another, so it compares amounts and
  dates too. A skip is withdrawn, on one side only, solely on the person's
  confirmation that no entry in Tally carries the transfer (`withdraw_skips`, as
  for any open row), and the step says not to book the transfer in Tally another
  way, because a later post of the other statement would book it twice. Reading
  the day sends its vouchers to the AI provider: `mask_parties` shortens party and
  ledger names, `drop_narration` drops narrations, and neither hides amounts.
- **What `restored` means.** A batch that a restore ComplyEaze Bridge recorded
  covers (see `verify_import`, `book_rolled_back`) is never found posted or
  verified for the transfer check, whatever its last status: a row it holds
  reads `held_elsewhere` naming it, its Contra is an unverified candidate, and
  its days give no coverage. Every row naming such a batch carries
  `"restored": true`, and `next_steps.restored` says what that means: no step
  inside ComplyEaze Bridge settles the row, so do not name it in
  `withdraw_skips` or build or amend its rows (that is advice, and the code refuses all
  three: a withdrawal as `withdraw_skip_paired` where a Contra carries the row, naming that batch
  with `"restored": true` when it is the covered one, or as `import_txn_already_posted` or
  `withdraw_skip_invalid` where the covered batch holds the row; an amendment of the batch's
  lineage as `import_amend_natively_posted`, since a recorded rollback covers only dispatched
  batches and a dispatched batch's lineage is never amended; a new build that posts a row the
  covered batch holds as `import_txn_already_posted`); check in Tally whether that
  batch's voucher for it is there and tell the person, and do not enter the
  transfer by hand or import that batch's file. A rollback never retires a skip
  that would be held without it.
- **What `held_elsewhere` and `posted_against_skip` mean.** A skipped row that
  a batch holds as a voucher is not a skip to pair. While no batch holding the
  row is found posted it reads `held_elsewhere`, with the batch to verify. Once
  a batch is found posted, the skip is retired (counted in `retired`) only if
  every batch found posted that holds the row has a build in its lineage that
  recorded the withdrawal: a build keeps the rows the person named in
  `withdraw_skips`, and they count once that build is sent or found posted and
  it was built after the newest record of the skip (a later build of the
  skipping statement, or a skip-only record of it, records the skip again; a
  withdrawal built at the very instant of the record does not count).
  Otherwise the posted voucher contradicts the skip, and the row stays open as
  `posted_against_skip`, with that batch: the other account's Contra may carry
  the transfer too, ComplyEaze Bridge does not compare the two, and no step
  inside ComplyEaze Bridge removes a voucher already in Tally. The row leaves
  the list only when a build in that batch's lineage that names it in
  `withdraw_skips` is found posted after the newest record of the skip, which
  the person confirms only when the other account's statement does not carry
  it; a batch ComplyEaze Bridge posted itself cannot be amended, so for it no
  step inside ComplyEaze Bridge clears the row. Two builds made before either
  is sent are both admitted, so this is how a skip and a posting of one row
  that were built apart are reported once both are sent and one is found
  posted. A skip may be withdrawn whether or not its statement was ever sent,
  so a statement parsed again with the row posted is built again naming the row
  in `withdraw_skips`, and the batch built first is neither sent nor imported:
  sent or imported and found posted, it leaves the row `posted_against_skip`,
  and once the skip is live `post_import` refuses that send as
  `skip_row_paired_elsewhere`. The answer of the build that names the
  row lists it as `held_elsewhere` with its own batch, which is the one to send
  and verify. The batch left unsent blocks the days of its rows for the other
  account's coverage until it is verified, and no step inside ComplyEaze Bridge
  clears that. A build that names a row held this way is judged against the
  other account's Contras like any skip. For a skip that was sent or is in a
  statement of only skipped rows (a live skip), an amendment of a batch whose
  row reads `posted_against_skip` is refused as `skip_row_paired_elsewhere`
  unless it names the row in `withdraw_skips`; a skip that was never sent does
  not refuse it. An amendment that names the row counts once it is found posted
  (`post_import` does not send amendments), and one that never reaches
  found posted counts for nothing. A held skip is judged against the Contras alone, without
  the other skips: a refusal as `paired` for it means a Contra may carry it,
  not that it is established that it does.
- **What `reference_differs` means.** The person sees the row listed once for
  each batch holding a Contra of the same day and amount whose reference
  differs from the row's, verifies that batch if it is not verified yet, and
  checks that Contra in Tally, because ComplyEaze Bridge cannot tell whether it
  is the row's other side: they keep the skip if it is, and may name the row in
  `withdraw_skips` if it is not.
- **What `ambiguous` and `reversed_same_day` list.** The row is listed once
  for each batch holding a Contra that could carry it, and ComplyEaze Bridge
  does not choose among them: the person checks them in Tally. A
  `reversed_same_day` row that no Contra matches is listed once, without a
  `batch_id`.
- **What `double_booked` compares.** Two Contras of one amount between the same
  two ledgers and the same way, one booked from each statement: where their
  references are equal, whatever their dates (the second build is refused as
  `transfer_double_booked`), and where their dates are equal and at least one
  of them carries no reference (both are listed). Nothing else is compared: a
  Receipt or Payment that posts a skipped row is reported as `held_elsewhere`
  until its batch is found posted, then as `posted_against_skip` unless the
  withdrawal was recorded, not here. No step inside ComplyEaze Bridge removes a
  voucher already in Tally yet; the step says not to send a batch not yet sent.
- **What `matched_unconfirmed` means.** The person sees the row listed with the
  batch whose verified Contra matches it by day and amount, where the statement
  records of the two accounts do not both cover that day, and keeps the skip.
  The row can be decided only once statement records of both accounts cover
  that day: for each account that needs a statement whose rows begin before
  that day and end after it, built and, where it books a voucher, verified (a
  statement that begins or ends on that day does not cover it, and a day
  blocked by a batch that holds no statement records is covered by none).
  Until then the row stays as it is.
- **What the records cannot see.** A batch that holds no statement records
  (built from inline vouchers, or by a release before them) can still book a
  transfer. Each Contra and statement row it holds makes the days it names (its
  date, and the date in its `st-` id) give no coverage for the ledgers it names,
  whatever the batch's status. A skip naming one of those ledgers that a single
  verified Contra matches by date on such a day is `matched_unconfirmed`, never
  paired. A skip with no candidate is `tally_unchecked` (or `not_found` after a
  whole-day read of Tally) only where the other account's windows still hold its
  day, and otherwise `not_established`;
  `reference_differs` and `reversed_same_day` are decided before either. A
  Contra with no statement record inside a statement's own builds blocks the
  same way; another voucher there does not.
  A record with no endpoint (from an early release) blocks the ledgers it names
  at every endpoint, and one with no ledger binding blocks every ledger at its
  endpoint, so such a record can turn every skip across its days
  `not_established`. A blocked day removes the whole window it falls in. A
  statement build blocks the days of its rows until it is verified, so one
  abandoned unsent keeps them `not_established`, and no step inside ComplyEaze
  Bridge clears it yet. A statement build whose send Tally rejected also stays
  unverified, and building that statement again is refused as
  `import_txn_already_posted`: a row waiting on it (`other_side_unverified`)
  has no step inside ComplyEaze Bridge either.
- **Refused before anything is written:** `skip_row_already_posted`,
  `skip_row_paired_elsewhere`, `transfer_double_booked`, `withdraw_skip_invalid`,
  `withdraw_skip_paired` and `transfer_view_not_established`, each with a
  `next_step`.
- **`withdraw_skips`.** Names skipped rows a build posts anyway, only when the
  person confirms the other account's statement does not carry them. A
  withdrawal is refused while exactly one verified Contra matches the row and
  nothing else claims that Contra. Where more than one verified Contra
  matches, the match is also claimed by another row or booked twice, the amount
  reverses the same day and the match is by date, or the only match carries a
  different reference and is verified, ComplyEaze Bridge cannot tell which
  entry the row belongs to: the withdrawal is then the person's explicit
  choice, and the batch of every matching Contra is listed on the row in the
  `transfers` section. While the only match is not yet verified, the
  withdrawal is refused and the step says to verify that entry first. So is
  one with any match in a batch not yet verified, whatever the row's state: the
  person can confirm only what Tally holds. The refusal is
  `withdraw_skip_paired`, and its `paired_rows` names each such batch with the
  row's state there (`paired`, `matched_unconfirmed` or
  `other_side_unverified`). A Contra matches a row by an equal reference, or
  by the same date and amount where one of the two carries no reference. One
  of the same day and amount whose reference differs from the row's is a match
  only for a row that nothing else matches. One dated another day, or with a
  third leg, does not match and does not refuse: verify every batch of the
  other account's statement before naming a row.
- **At send.** A batch built from a bank statement, or from vouchers given
  directly that post a statement row or withdraw a skip, is checked against
  these records when `post_import` sends it as well as when it was built, with
  the same refusals, before the post (before the dialog, or in the call that
  redeems an approval) and again under the admission lock. The step of each
  begins by saying nothing was sent and any approval of the batch is withdrawn,
  and, except for `transfer_view_not_established` (building it again meets the
  same unreadable record), ends by saying the batch must be built again. A refusal under the lock carries its code and a step that sends the
  caller back to `post_import` for the whole reason. The batch is judged as it
  was built, with its own records left out of the journal, and a row it withdrew is judged on the withdrawal the batch
  recorded (`withdrawn_skips`), whether it was built from a statement or from
  vouchers given directly, and counts only if the batch was built after the
  newest record of that skip: a skip recorded since refuses the batch, and a
  build naming the row again is newer. A batch that is not about transfers (no statement row, no statement, no
  withdrawal) is not checked.
- **Records a release cannot read.** The statement records refuse unknown
  fields, so records written by a later release make this one refuse every
  build that needs the view, and every send of such a batch
  (`transfer_view_not_established`); builds that do
  not need it go on with `transfers.state: not_established`. A missing
  skip-only file reads as no skip-only records, as a missing import journal
  reads as no batches.

The file path is deliberately not a direct-posting path. Masters must already
exist and match exactly. File generation requires fresh supported product/mode
observations; the checks do not make a later manual import atomic with the earlier reads.
In Education mode, voucher dates must be on day 1, 2, or 31 — for every
voucher type, not only `Journal`. A different requested date is refused as
`education_voucher_date_unsupported`; Bridge does not move it.
Optional narration and reference must contain 1–2,000 Unicode characters when
supplied; omit them when unused. Control characters and the reserved attribution
marker are refused, including XML entity-encoded marker spellings. Voucher
numbers contain 1–32 characters, cannot contain controls or `$`, and cannot begin
or end with white space (Tally read a number back without its end space). Lengths match
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

### CSV statement shell (`parse_bank_statement_csv`)

Takes `statement_path` (a local file, under the path rules of `parse_bank_statement`) and `controls_source`
(`prior_statement`, `bank_ledger_in_tally` or `this_file`), and reads the file with the CSV edge reader of the
`bridge-bank-statement` crate, which accepts one private bank's export layout and nothing else. A file the reader
refuses is refused with its category as the error code, `statement_<category>` and `:row_N` where a row is at fault; there
are fifteen categories, each its own code. A file that reads is still refused, as `statement_csv_account_line_unmeasured` (not the PDF tool's `statement_no_account_number_line`, which says a statement prints no account line):
whether that export prints an account number is not established, so nothing binds a file to an account and nothing is
proposed. With `controls_source` `this_file` the refusal's `cause` is `controls_this_file_derived_not_independent` and its next step says "derived, not independent": the
file's own first and last balance cannot show that rows are missing, so they are not an independent control. `controls_source` is
checked before the file is opened and ComplyEaze Bridge cannot verify the claim. The tool writes nothing to Tally or beside the statement
(like every read tool it adds receipt lines to the local log) and returns no row value. Part of #1320; the account line and the proposals follow once the export's account line is measured.

### Statement tie-out (`statement_tie_out`)

A read-only check of the bank ledger against a statement `parse_bank_statement`
has parsed, for use before `build_import_xml` (`stage` `before_build`) and
after a post or an import by hand (`stage` `after_post`). It writes nothing to
Tally and never posts. Its check of the import journal takes the shared
import-admission lock for a moment, so a build or post at the same time can be
refused as lock-busy, and it can be refused by theirs; nothing waits, so nothing
deadlocks.

- **A whole statement, or whole calendar months of it.** The parse records the
  statement's own first and last row dates in an optional `window` of the
  proposals file (`first_row_date`, `last_row_date`, `whole_statement`), and in
  `window.tie_out` where the file can be compared with the book: either
  `established`, with the dates and the statement's balance at the start of the
  first and the end of the last (`first_date`, `last_date`, `opening_balance`,
  `closing_balance`, basis `statement_rows_by_transaction_date`), or
  `not_established` with a reason. The schema string is unchanged, and an older
  build still reads such a file. For the whole statement the dates are its first
  and last row dates and the balances are the ones the caller supplied. For a
  `from`/`to` window that leaves rows out they are the window's own dates, which
  must be whole calendar months (from the 1st to a month's last day,
  `window_not_whole_calendar_months`) within the statement's rows
  (`window_beyond_statement_rows`); the opening is the supplied opening plus the
  net of the rows dated before the window, and the closing the supplied closing
  less the net of the rows dated after it. These are sums by transaction date,
  so the order the bank printed the rows in cannot move them, and every row's
  amount is proved by the running-balance check before they are taken. The
  tie-out checks the month rule again when it reads the file. For a file that
  carries `tie_out`, when every row in the window has a voucher in the file, the
  vouchers must move the bank ledger by exactly the closing less the opening, or
  the file is refused (`proposals_file_invalid`); an older whole-statement file
  is not checked this way. A file with no `window` is `not_established`
  (`window_not_recorded`), and so is a narrowed file written before the parse
  recorded `tie_out` (`window_narrower_than_statement`), answered from the file
  once the company is verified.
- **The file is not bound to a company.** It names a bank ledger, not the
  company it was parsed for; pass the company the statement belongs to.
- **Three reads of the ledger list**, none of vouchers: the position at the
  window's first date, at the day after its last date, and at the first date
  again. A change dated before the window, or an edit to the ledger itself (its
  opening balance, name or removal), made between the first and third read, is
  `book_changed_during_read`; a voucher dated inside the window and posted
  during the read moves only the end figure and is not caught. No ledger is read
  when the file alone answers (`window_not_recorded`,
  `window_narrower_than_statement`, `window_not_whole_calendar_months`,
  `window_beyond_statement_rows`) or the window starts before the company's
  books begin (`window_precedes_books_from`).
- **Three figures**, all in the statement's sign (book less statement: positive
  means the book shows more money in the bank), with the two dates:
  `opening_gap`, `closing_gap` (`before_build` adds what the file's vouchers put
  into the bank ledger) and `change_in_window`, each `tied`, `differs` or
  `not_established` with a reason. For `before_build` the closing figures are
  `not_established` when some rows of the statement have no voucher in the file
  (`rows_without_voucher`, whose count the result carries) or when the import
  journal already holds a row of the file (`file_already_partly_posted`, the same
  check a build makes before it writes a file); `after_post` is unaffected.
- **What the result reveals.** The book's own balances are not returned, but a
  gap plus the statement balance the caller supplied gives the book's balance,
  so they are derivable. For a window of whole calendar months the reverse also
  holds: the book's figure (which `ledger_movement` reads) less a gap gives the
  statement's balance at a month edge, and `change_in_window` with the book's
  movement gives the statement's net over those months. That is why a narrowed
  window must be whole calendar months within the statement's rows: what can be
  learnt this way is the statement's balance at month ends and its net per
  month, never a part of a month except the part-months at the statement's own
  start and end, whose net a month edge's balance gives together with the
  opening and closing supplied to the parse. A month in which every other row's amount is
  already known still gives that row's amount: a month of one row, or one row
  beside open cash lines, whose amounts `cash_questions` returns. The parse
  result already ties such a row to its party and direction through
  `rows_in_window` and `counterparties`. The result also carries the company block, the
  stage, the window's two dates (`first_date`, `last_date`) and the reading.
- **The sign is a type** (`BankSide`): Tally holds a debit balance as a negative
  number, a statement prints money in the account as positive, and
  `BankSide::from_book` is the only conversion.
- **No cause is given.** Uncleared cheques, deposits in transit, a missing entry,
  a repeated entry and the wrong bank ledger look alike; the reading says so,
  prescribes no remedy and checks the bank ledger only.
- **Run against a live book.** Every checked row of one run passed on 30 October
  2026 (UTC; 31 October on the lab computer), and one row was recorded as a note
  (licensed TallyPrime 7.2 Silver, one synthetic company, the published 0.7.0
  Windows package):
  - A window ending in the middle of a month answered `not_established`
    (`window_not_whole_calendar_months`) with no amount, after the company check
    alone: four requests and no ledger read.
  - Two windows of one whole calendar month each, on a bank ledger in the middle
    of its year, tied while the book also held a second bank ledger.
    `trial_balance` at each month end, and Tally's own Ledger display over the
    two months, agreed with the statement. An entry keyed by hand that is not on
    the statement showed as a closing gap and a change of its own amount, with
    the opening still tied.
  - A statement whose first and last rows fall in the middle of a month was read
    at its first row's date and the day after its last: a voucher dated the day
    before the first row counted in the opening, and one dated the day after the
    last row stayed out of the closing.
  - ComplyEaze Bridge's own post of that statement's vouchers (`post_import`,
    `posted_verified`) then tied at `after_post`, and `before_build` run again
    answered `file_already_partly_posted` for the closing and the change rather
    than counting the batch twice. A twin of one row keyed after the post showed
    at `after_post` as a closing gap and a change of its amount.

  Two earlier runs were on TallyPrime 7.1 Silver: one by an outside contributor
  on two synthetic companies (10 October 2026), only partly recorded in the pull
  request, and one on 14 October 2026 on the same synthetic company. That run had
  already measured whole-month windows on the bank ledger in the middle of its
  year before and after a hand import, reads after the ledger's last voucher, a
  window refused as `window_not_whole_calendar_months` with no ledger read, and
  `file_already_partly_posted`. New in this run are the second bank ledger,
  Tally's Ledger display as the reference, the statement edges in the middle of a
  month, ComplyEaze Bridge's own post, and TallyPrime 7.2. The tool-level tests
  replay captured responses with one value changed in memory, and the dated
  ledger read the tool relies on is the one described in protocol reference
  section 5.5. Their proposals files are written by hand, except in the
  round-trip test, whose file is written by the code the parse uses. Not
  measured: a statement whose last day is the end of a financial year, a book of
  1,000 or more vouchers, Education mode, a voucher posted inside the window
  during the reads, another bank's statement layout, and Gold. Evidence:
  `RESULTS.md` on the lab branch `lab/1211-tie-out-evidence-answers` at
  `7760d367`.

## Approved voucher posting

**Voucher posting is off by default in the MCPB extension** while four known
limits remain. Tally aims an import at a company by its name and cannot bind it to a company's GUID. Bridge's last request before the post checks that exactly one loaded company has the target's GUID and name, and that no other loaded company has the same name ignoring case and spacing; otherwise it refuses the post ([#607](https://github.com/ComplyEaze/bridge/pull/607)). A company renamed to, or loaded under, the target's name (or one differing only in case or spacing) in the moment after that check could still receive the voucher, if it has the voucher's ledgers. Bridge may flag afterwards that the loaded companies changed, but cannot always say where the voucher went, and cannot prevent it (accepted residual, [#574](https://github.com/ComplyEaze/bridge/issues/574)). A ledger renamed and replaced in that same moment means the post can land in the replacement ledger. Bridge marks the result as needing reconciliation when it sees that the ledger now resolves to a different master; a change that leaves the company's master mark unmoved, or is reverted before that check, is not seen, and a regroup in that moment is not detected ([#623](https://github.com/ComplyEaze/bridge/pull/623)). And Bridge has no tool to delete or undo a voucher it has posted, so a wrong post must be corrected by hand in Tally. It records the REMOTEID each post sends, but no delete tool exists yet ([#579](https://github.com/ComplyEaze/bridge/issues/579), [#582](https://github.com/ComplyEaze/bridge/pull/582)). And the approval window covers only ComplyEaze Bridge: another Tally connector in the same Claude Desktop that can change entries can do so without it.
The saved batch file is now checked byte for byte against the approved record
before posting ([#575](https://github.com/ComplyEaze/bridge/issues/575), fixed).
**Allow voucher posting, one voucher or a batch of up to 50** turns it on for
users who accept those risks. It is a new setting, so a posting choice saved
for an earlier version does not carry over: posting starts off.
For command-line installation, set `BRIDGE_AGENT_ENABLE_WRITES=true`.
This enables `build_import_xml`, `parse_bank_statement`, `post_import` and
`acknowledge_post_review`, which asks the local user, in its own native dialog, to
record that they reviewed a post whose masters check found a changed ledger. That
record changes no verification status and nothing in Tally (#239).
`verify_import` remains available so an uncertain saved batch can be checked
after posting is turned off. `BRIDGE_AGENT_ENABLE_IMPORT=true` alone exposes
the manual file workflow and bank-statement proposal preparation, while
verification remains available without either switch.

`BRIDGE_AGENT_ENABLE_WRITES=true` also lets `post_import` post a saved batch of 2
to 50 vouchers in one import, after one approval of the batch's summary: every
ledger's debit and credit totals, the money Receipts bring in and Payments take
out, and the standing cautions (#1090). In the MCPB extension this is the
setting **Allow voucher posting, one voucher or a batch of up to 50**
(`enable_posting_2026_10`). It is a new key, off by default, so a posting choice
saved for an earlier version, which covered one voucher at a time, does not
carry over. An empty value, or the key's unresolved
`${user_config.enable_posting_2026_10}` placeholder, reads as off; what Claude
Desktop passes for a new key on an upgrade is not yet measured. The separate
`BRIDGE_AGENT_ENABLE_BATCH_POST` is retired: while it is set, with any value,
and posting is on, `post_import` refuses as `setting_retired` until it is
removed; every other tool works. A batch is `posted_verified` only when Tally created
exactly that many vouchers, the readback verifies every one, and the company's
voucher mark moved by exactly that many. Otherwise it is
`reconciliation_required` (`batch_step_unconfirmed` when only the mark
was not confirmed). Review a doubted batch's vouchers in Tally and do not rebuild it;
`acknowledge_post_review` records that review, one doubt at a time, and changes
no verdict. Its `invoice_stop` value is the one exception to "review": it
releases the stop a sent Sales invoice that is not verified posted puts on its
company (`invoice_company_stopped`), after the person has checked Tally, in the
same dialog the model cannot answer.

`prepare_ledger_creates` and `post_ledger_creates` were added in release 0.7.0. The create path and the rule that refuses a near name were
measured on TallyPrime 7.1 Silver, on one synthetic book; other releases and
editions are not measured.

`BRIDGE_AGENT_ENABLE_MASTER_WRITES=true`, together with
`BRIDGE_AGENT_ENABLE_WRITES=true`, lists two more tools, which are not among the
ordinary default tools above: `prepare_ledger_creates` and `post_ledger_creates`.
Together they create ONE new ledger (a party under Sundry Debtors or Sundry
Creditors, a Suspense ledger, or a GST duty ledger whose head has been measured)
from typed fields: ComplyEaze Bridge builds the request itself, accepts no XML,
and can set no opening balance or parent. `prepare_ledger_creates` reads the
company's ledger and group names once, refuses a name that already exists as a
ledger, a group or an alias (`master_name_taken`), saves the request in the
local masters journal and returns a `batch_id` with the preview the person will
be shown (under `mask_parties` the ledger's name in it is masked; the dialog
shows it whole); it writes nothing to Tally. `post_ledger_creates` takes that
`batch_id`, asks the local user in a native dialog (whose first line says what each
answer does, as the post dialog's does), and sends the create once;
its approval, pending and approved answers behave as `post_import`'s do. A batch
already sent is never sent again: a later call reads the book back, or reports
what was recorded for a batch already settled.
With the setting off, posting off, or the retired `BRIDGE_AGENT_ENABLE_BATCH_POST`
still set, neither tool is listed and a call to either
is refused `master_writes_disabled`. A name that differs from an existing one
only in case, spaces or symbols is refused too, because TallyPrime can take such
a Create for the existing ledger and change it. If the ledger names TallyPrime
returns cannot be read, `prepare_ledger_creates` refuses
`master_ledger_index_unavailable` and nothing is created. After the one send
ComplyEaze Bridge reads the book back (the master marks of the loaded companies,
the paired ledger and group names, and the new ledger's parent, tax fields, GSTIN
and opening balance) and answers with one state: `posted_verified` (exactly one
ledger of the name, new, with the parent, tax fields, GSTIN and opening balance
as sent; bill-wise is not read back; no ledger or group that existed before
changed; the company's own master mark moved by exactly one, or, on a later call,
the ledger's `ALTERID` shows it as the first master change in the company after
the last read before the send, which `verification.checks.target_mark_step_one`
shows; no other loaded company's master mark moved),
`posted_into_existing_master` or `master_write_located_elsewhere` (loud: check
Tally, do not retry), `confirmed_absent` (only from a later call: its read found
the name absent and the company's master mark unmoved; a create that Tally
rejected reaches it too, and it does not show that Tally will not still run a
create that ComplyEaze Bridge stopped waiting for, so make sure Tally is idle and
answering before a new batch is prepared, which needs a new approval),
`confirmed_absent_after_change` (only from a later call: its read found the name
absent, no ledger or group that existed before changed or went missing, no ledger
or group that was not there before the send, and the company's master mark moved
forward since the send; whether the create ran, its ledger later deleted, or never
ran is not known; the batch is settled and its name released, and a new batch
needs a new approval),
`posted_unattributed` (only from a later call, when no answer from Tally to the
create is on file (it could not be recorded, or Tally never answered): the ledger
was found as sent, and its `ALTERID` shows it as the first master change in the
company after ComplyEaze Bridge's last read of it before sending the create, and
not changed since, which `verification.checks.first_change_after_intent` shows; the batch is settled and
its name released, but it is not verified, because nothing shows that change was
ComplyEaze Bridge's own create and not the same ledger made by someone else, and
a create Tally still holds could run later and change the ledger again: check the
ledger in Tally), `masters_after_post_unconfirmed` (the book could
not be read back; call again, which only reads), plain `posted_unverified`
(Tally answered, with any counters it gave, but the read-back could not be
attempted, so nothing was judged; see its `next_step`, never send again, check
Tally) or
`reconciliation_required` (with reasons). The answer of the call in which Tally
answered, and an answer that reports a read-back (it has `verification.moment`),
carry `answer_recorded`; a failure answer does not. False means ComplyEaze Bridge
has no record of Tally's answer on file (it could not be recorded, or Tally never
answered). In the answer of the call in which Tally answered, the counters shown
are then the only record to rely on, and a later call may have none and cannot
reach `posted_verified`: keep them and check the ledger in Tally. A later call
that finds no answer on file shows no counters.
`verdict_recorded` says whether the verdict was written to the journal, and
`settled` is true only when the state is final and the verdict is recorded. With
`verdict_recorded` false the state was judged but is not settled: call
`post_ledger_creates` with the same `batch_id` again, which reads the book back and
sends nothing. A later call for a sent batch never sends. It repeats the
read-back unless the batch already has a terminal verdict (it then answers from
the journal alone), another dispatch is using the Tally endpoint (the book is
then not read, and the answer is `masters_after_post_unconfirmed` with the reason
`endpoint_dispatch_busy`), or the batch's company cannot be matched to a loaded
company (it then answers from the journal and says what stopped the read-back).

Refusals and answer fields not named above include these.
`master_writes_need_licensed_mode` refuses a book in TallyPrime's Education mode,
before the person is asked, because no master write has been measured there.
`group_export_invalid` says the group names TallyPrime returned could not be read,
with a `cause` when one is known. `post_catalogue_unreadable` says the ledger and
group names read again after the approval differ between the two readings or
cannot be indexed. `master_approval_declined`, `master_approval_unavailable` and
`master_approval_timed_out` say the person declined the dialog, the dialog could
not be shown, or the person did not answer in time; each says nothing was sent,
and a declined or timed-out answer's `next_step` says that the next call with the
same `batch_id` opens a new dialog. `master_review_too_large` from
`post_ledger_creates` says the dialog's text was over its limit or, on Windows,
where the dialog is measured against the screen as the post's is, that it did not
fit the screen whole; nothing was sent. In an answer,
`verification.verdict_written` is true when this call appended the verdict (an
open verdict already on file in the same words is not appended again);
`verification.verdict_record_error` is the code of why the verdict was not
recorded; `verification.answer_record_error` is the code of why Tally's answer was
not recorded, in the `posted_unverified` answer given when the book could not be
read back; `verification.counters_note` says why no counters are shown; on a later call with
no answer on file, `verification.reasons` gives `ledger_altered_since_create` when
the ledger found was changed after the first master change that followed the last
read before the send (or another master change came first), and
`ledger_alter_id_not_after_intent` when it was last changed before that read;
either keeps the batch open; and
`verification.not_read_back` lists `bill_wise` when the ledger asked for
bill-wise, which the read-back does not fetch. An answer made from the journal
alone (a batch with a terminal verdict, or a book that could not be read back) has
a `journal` object with `response_recorded`, `counters` and `verdict`, takes its
`state` from the verdict (or is `posted_unverified` when only Tally's answer is on
file, and `reconciliation_required` when neither is), and, for a book that was not
read back, has `verification.attempted` false with the code that stopped it.

A create whose outcome is not settled holds its ledger name, and every name that
may be the same ledger, against a new create in every company this data folder
holds batches for, not only its own. `post_ledger_creates` refuses such a name
with `master_ledger_open_dispatch_overlaps` before it asks the person when it can
read its journal just before that check. When that read fails the check is
skipped, and the refusal then comes from the check under the journal's lock: after
the approval, and still before anything is sent. The check under the journal's
lock stays in every case, so a refusal can still come after an approval, and then
the approval lapses and the next call asks the person again. The refusal says this call sent
nothing. Its `error.hold` says what the journal showed: `held` (it names
`blocking_batch_ids`, the batches of the refused batch's own company that hold the
name, and `blocking_batches_other_companies`, each with its `batch_id` and
`company_guid`), `settled_meanwhile`, which only the check under the lock gives (the
holder settled since the call began: post this batch again), or `journal_unread`
(a journal read that did not show this batch: call again). The check under the lock
reads the journal again for its answer. When that read fails (the journal busy because
another call is writing to it, or unreadable for any other reason), every refusal the
lock made after reading the journal (all but `master_journal_unavailable`, which could
not read it) is answered from the journal the lock read when it refused. It names the
holders, or the send on file, as they were at the refusal: a send another call has
recorded since is not in it, so where it shows no send its message says only that this
call sent nothing and that no send of the batch was on file when it was refused
(bridge#2083).
When the check before the dialog finds a send of this
batch recorded since the call began by another call, and when a held-name refusal
finds it in the journal read that follows, the answer is `import_already_attempted`
with `error.hold` `send_of_this_batch_recorded_since`: call again later, which
only reads, and create nothing by hand. The check under the journal's lock
answers the same code without the `hold` field, as an outcome that may have been
sent: call again, which reads the book back when the send is on file, and create
nothing by hand meanwhile. For a held name, call `post_ledger_creates` with
each blocking `batch_id`. For a batch of another company, `list_companies` shows
only the companies loaded in Tally, so a `company_guid` not found there belongs to a
company that is not loaded; the call reads that book back and sends nothing if the
company is loaded, and otherwise answers to load the same companies and call again.
Post this batch again when they are settled, and create nothing in Tally by hand
meanwhile, because the earlier create may still be waiting in Tally and would
overwrite it.

Both absent states rest on the order Tally serves requests in: the read that finds
the name absent is sent after the create's send ended, and Tally has been seen to
serve such a read only after the create had run. That was seen on TallyPrime 7.2
Silver, for ledger creates, with every request on a new connection and with the
create, the read or both on a reused connection (#2176). Their answers carry
`verification.basis` `read_after_send_order`. So ComplyEaze Bridge gives either
absent state only when the read-back's Tally reports TallyPrime 7.2 Silver,
licensed. On any other the batch stays open, its name held, with a reason that
names what is not covered: `absent_not_measured_on_this_product` (the product is
not exactly TallyPrime), `absent_not_measured_on_gold`,
`absent_not_measured_on_this_release`, `absent_not_measured_in_education` or
`absent_tally_release_not_known`, and a next step that says so: calling again
on the same Tally will not close it, so on TallyPrime 7.1, Gold, Education or
another product such a batch holds its name until the company is read back on
7.2 Silver. Before this, `confirmed_absent` was given on any Tally. An absent
name kept open also names what else the read showed (`master_new_since_intent`,
`mark_went_back_since_intent`).

Later calls cannot settle these states, and their batches keep holding the name: a
read that found the name absent while the company's master mark moved back, or
while a ledger or group appeared that was not there before the send; a ledger
present with no counters, or with a field that differs; a ledger changed in Tally
after its create, or created after another master change, which reads
`ledger_altered_since_create` (its `ALTERID` is past the first change after the
last read before the send); a create Tally rejected, once the mark moves; and the
reasons `pre_image_master_missing`,
`pre_image_unreadable` and `master_index_not_usable`. A later call judges the book
as it then is, so a state stays unsettled only while the book stays as it was read.
A company that is not loaded or was renamed is not read back until the same
companies are loaded under the same names; the next call then reads the book back
and judges it. A book whose set of loaded companies changed is read and judged:
the answer is `reconciliation_required` with `company_scope_changed` among its
reasons, and it says to load the same companies and call again; a read that shows
a changed existing master or another company's mark moved keeps the state for
that. A read-back that read the book,
found the name absent and did not settle says that ComplyEaze Bridge cannot settle
the batch now, because a create it sent may still be waiting in Tally; the name
stays held, and a later call sends nothing, reads the book back, shows whether a
ledger of the name now exists and judges one that has appeared. No answer tells the
person to create the ledger in Tally by hand. An unconfirmed answer
and the answer given when the book could not be read back after Tally answered
say to call again later, which only reads, and to create nothing in Tally by hand
meanwhile. A failure answer that says the create may have been sent says the same.
When the journal could not be read for that answer, a failure other than a refusal under the
journal's lock made after the lock read the journal says it is not known whether a send was recorded: call `post_ledger_creates` with the same `batch_id`
again, which reads the book back and sends nothing if a send is on file,
otherwise asks for approval again before anything is sent, and is refused as
`master_batch_not_found`, sending nothing, if ComplyEaze Bridge holds no such batch; create nothing in Tally by hand
meanwhile. Of the party fields the read-back fetches, no PAN, bank detail, e-mail
address or phone number is put in an answer or a record; a GSTIN the caller
supplied, which contains the party's PAN, is in the preview and in the batch's
local record. The setting is a command-line setting only and is not in the MCPB
extension.

All three switches accept `true`/`false` or `1`/`0`; any other value stops startup, except that an empty `BRIDGE_AGENT_ENABLE_WRITES`, or the extension's unresolved placeholder for it, reads as off. No model-supplied argument can grant approval. Claude controls
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
   usable master refuses with `import_base_currency_undetermined`. A native
   post sends no voucher number, so before approval ComplyEaze Bridge reads the
   company's master mark and then each voucher type's voucher numbering
   (bridge#724): a voucher type that does not read Automatic refuses with
   `import_voucher_numbering_not_automatic`, and one that reads Automatic
   without the sub-method `Auto Retain` (it reads `Auto Renumber`, prints no
   sub-method, or prints one ComplyEaze Bridge does not recognise) refuses with
   `import_voucher_numbering_not_retain`. Either names each such voucher type
   and what was read in `refused_types`; a batch that holds both is refused
   as `import_voucher_numbering_not_automatic`. `build_import_xml` checks every
   voucher the same way, and writes a Journal that carries its own voucher
   number into a voucher type that reads Manual with duplicate numbers
   prevented as well: Tally keeps the number there and refuses one a Journal
   already holds. Under Manual with duplicates allowed a Payment carrying a
   held number was created as a second voucher (protocol reference §9.8), so
   a numbered Journal is refused there. Under Automatic
   numbering Tally discards a supplied number and assigns its own, so such a
   Journal is held to `Auto Retain` like a voucher without a number; it is
   refused on a voucher type set to "Renumber Vouchers", where Tally was seen
   to discard the number a Receipt carried and give it the number of its
   place in date order (protocol reference §9.8). With a
   type set to "Renumber Vouchers", numbers already in the book were seen to
   change when a voucher with an earlier date was entered by hand, and again
   when a voucher with an earlier date was sent by import without a number:
   the import's answer read CREATED 1, ALTERED 0 and no voucher's AlterID
   moved; a voucher dated after every other changed nothing (one book,
   TallyPrime 7.1 Silver, Receipts; a file of two Receipts, a Receipt that
   carried a number and a Journal imported without a number were measured
   there too, §9.8). A file of more than two vouchers, a mixed file, a
   Journal that carries a number or is dated after every other, Payment and
   Contra, Gold, other releases and the same through ComplyEaze Bridge's own
   post are not measured. On a release, an edition or a book whose series
   prints no sub-method, or words it differently, every Automatic voucher type
   refuses this way; a `masters` read of `voucher_types` shows what a book
   prints. A voucher
   type saved during that read refuses it with `masters_bound_premise_violated`
   (cause `masters_alter_id_above_master_mark`): run the call again. A book in
   Education mode is refused with `masters_education_unqualified`. Inside the
   queue, a change to the company's masters from that numbering read to the
   last read before the post refuses with `post_masters_moved`
   (`post_masters_unconfirmed` if it cannot be checked); re-run the post. This
   sees only changes that move the company's master AlterID (`ALTMSTID`):
   measured for ledger renames and creates made through the gateway, and each
   of thirteen saves of a voucher type's screen in Tally moved it by 2 (nine of
   a type holding no vouchers, and two in each of two runs on a type holding
   vouchers, all on one synthetic book, TallyPrime 7.1 Silver), and a ledger
   regrouped, created or deleted, or its opening balance changed, on Tally's
   own screens moved it by 1 (VERIFIED, one run each, protocol reference
   §11c.5); enabling cost centres, creating one and making a ledger applicable
   moved it by 3 (PARTIAL). Other screen edits, and whether posting a voucher
   moves it, are not yet measured. A queue catalogue re-read that does not
   parse as this company's catalogue refuses with `post_catalogue_unreadable`,
   whose `cause`
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
   the first answer did not list. Import a bank statement in whole-day batches:
   cut a batch only between days, at most 50 vouchers (the batch-post limit), so
   same-day rows of one amount are not split across batches; they pass this
   check together in one batch. A proposals file is whole days already, since
   `from` and `to` are days. A day with more than 50 rows does not fit one
   batch, and that case is open: ask the user. With posting on, a saved
   batch of up to 50 posts after one approval, so same-day twins post
   together in one batch or through a hand import of the built file. The list and step are
   withheld on a very small response budget, and the same refusal inside the
   queue, after approval, carries no list. Any other read inside the queue that fails before the post is refused
   with `post_queue_read_failed`, with a `cause` where one is known (a read the endpoint's
   gate held back keeps its own code, `tally_endpoint_busy` or `tally_endpoint_lock_unavailable`,
   and a withdrawn call is `request_cancelled`); nothing is sent, and
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
   each ledger's totals, and the overall totals. A batch of at most 10 vouchers
   also lists one line per voucher (type, date, amount, ledger and the start of the
   narration, quoted); no line shows a reference, and a batch of more than 10, or
   one whose lines do not fit or hold an unsafe narration or reference, shows totals
   only, with a sentence saying why. Equal ledger totals do
   not prove each voucher is right, so check the vouchers before building the batch.
   Choose **Post voucher** (for a batch,
   **Post N vouchers**) on macOS or **Yes** on Windows to permit this attempt. **Cancel** or Escape
   declines on macOS; Return may leave the dialog open. Windows defaults to
   **No**, and its window also takes the keyboard (Windows 10, a full-screen Tally,
   one screen, each tried once, #1780: on the 0.6.0 release candidate a Down arrow
   moved its buttons; on the published 0.6.0 a typed **n** declined it and a typed
   **y** approved it; macOS not measured). When you ask for a post, stop typing in Tally until you have
   answered the window with the mouse. Long or directionally ambiguous previews are refused; use the
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
response (no `STATUS` 0, `CREATED` equal to the voucher count, and `LASTVCHID`), the mark after
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
place). Tally gives vouchers keyed again after a restore the MasterIDs, GUIDs and
numbers the post's vouchers held (measured once on a synthetic company,
bridge#1050), and no identifier a read returns tells the two apart. So a
rollback, once seen, is recorded: when a check of a native post reads the
company's voucher mark below the floor any native post of the company had
proved (a mark Tally omits twice reads as a company holding no voucher, unless the same
check's own window read saw a voucher mark: it is then refused as
`voucher_mark_contradiction` and nothing is recorded), the
import journal keeps a record of it whenever it covers a post no earlier record
covers, so a second restore is recorded too (`post_span_binding.rollback_recorded`,
with `other_batches_covered`: how many of the company's other native posts it
covers, and the first ten, to verify again). A record that cannot be written
fails the check, to be run again. A post sees a restore too: when the voucher
mark its last read before the send gives is below such a floor, the record is
written before the post's own dispatch intent and before its approval is spent,
and the post's answer gives `rollback_recorded_at_post` (the count and the first
ten of the native posts it covers, to verify again). That record names one of
the posts it covers, not the post that read the mark, which has no intent yet;
the journal does not say which post saw it. A record a post cannot write
refuses the post as `post_rollback_unrecorded`, with nothing recorded for it
and nothing sent, and the approval withdrawn, so the next call asks again:
once that post's vouchers raised the mark, no later check could see the
restore.
From then on no native post of the same company (name, GUID, company number
and books-from date) at the same Tally address reads `posted_verified` if it
was sent before that check read the journal and the vouchers it had proved by
then (its binding, else Tally's answer unless it reported `STATUS` 0, else
its mark before the post) put it above the mark read:
`post_span_binding` is then `book_rolled_back` with `mark_read` and `seen_by`
(the batch whose check saw it, or, for a restore a post saw, one of the posts
the record covers, possibly this one), its vouchers read as matching content, not as
its own, and the plain line says to check them in Tally. The check still sees a
rollback only while the mark reads below some post's: a book restored and then
keyed or edited past the mark before any check is not seen, and a native voucher
keyed again with the same content can then read verified (measured once on
TallyPrime 7.1 Silver, with Tally's security off), so after restoring a backup
over a company, check its posted vouchers in Tally. A record is kept for good: a
wrong backup restored and then the right one leaves the posts it covers reading
rolled back, and a covered Sales invoice batch stops that company's invoices
until it is released; a repair or rewrite of the company's data that lowered the
voucher mark (not measured) would be recorded the same way. An older build refuses a journal holding the record: do not
downgrade after posting. Bridge still records those rows as posted, so a rebuilt batch holding
them is refused as `import_txn_already_posted`; re-entering them in that book is
the person's decision. The result's `post_span_binding` names the binding's
`state`: `bound`, `refused` with its `code`, `unsettled` with its `code` (for
example `binding_effective_date_not_observed`, when the read left out a
Payment, Receipt or Contra's effective date: never refused for it, and decided
again by the next verification),
`not_bound`, `book_rolled_back` or `not_applicable`.

A native post that sent no tag is never attributed by one: a row carrying its
batch's tag is a hand import of the batch's file, matched by content only. The
binding compares narration byte for byte, and so does every verification since
#1608: an attributed voucher whose narration differs from the one written reads
`posted_divergent`, with `narration` in its `diffs` (a voucher matched by content
only stays `matching_content_observed` and only lists it). A narration holding the one sequence
the agent readers are known to rewrite (a literal U+FFFD followed by `#`, digits
and `;`) is refused when the batch is built (`voucher_text_invalid`), and
`post_import` refuses a batch saved before that check in the same way, before
any request; it is still admitted for review and reconciliation. So is a
narration holding a character that draws nothing (Unicode
Default_Ignorable_Code_Point or Format, except ZWJ, ZWNJ and the prepended
concatenation marks). Other text,
such as Devanagari or the rupee sign, is admitted, and whether it reads back
byte for byte is not yet measured: a narration that reads back changed refuses
that post's binding for good.

A batch whose binding was refused (`post_span_binding` state `refused`) is not closed by anything ComplyEaze
Bridge offers. A refusal is final for the batch: `verify_import` on it again reads the same unless the book
changes (a changed book alters the vouchers' statuses, never the refusal), `acknowledge_post_review` reviews a
doubt only beside vouchers that read back verified, and an amendment of it is unavailable
(`import_amend_natively_posted`, as for any natively posted batch). The summary of such a post says that it cannot
be closed or amended; its vouchers are checked in Tally. Its rows stay refused in any other batch
(`import_txn_already_posted`), and `local_data_report` counts it as `not_settled_binding_refused` (or as
`not_settled_restored` once a recorded restore covers it).
After the person has checked its vouchers in Tally, `acknowledge_post_review` with `doubt` set to `content_only`
records that they did (bridge#1039), in the same review dialog. The same holds for a native post with no answer
from Tally recorded (`post_span_binding` state `not_bound`, code `binding_response_not_recorded`), which never
binds either. It is admitted only while the journal holds that state and a fresh readback shows it: never for a
binding refused as `span_identity_reused`, a book read as rolled back, a binding not yet settled or a batch that
reads `posted_verified`. The record, in the import journal, changes nothing in Tally and no status or verdict; it
binds no voucher, lifts no invoice stop and releases no row. `verify_import` reports it as `content_review`, with
its own `next_step`, and `local_data_report` counts the batch as `not_settled_reviewed_not_attributed` instead (a
recorded restore voids the review, and the batch then counts as `not_settled_restored`).
A later `posted_verified` of the batch, a later answer from Tally with an outcome, or a recorded rollback of the
book voids it. A build older than this record refuses the whole import journal once one is written, as for a
stop release: do not downgrade after recording one. Any other batch is refused before any read
(`ack_content_review_not_content_only`; `ack_batch_not_posted` for one never sent), as is a second review while
one stands (`ack_already_recorded`), and a readback that no longer shows the state. Nothing is written when the
batch's records change, or a rollback is recorded, while the dialog is open (`ack_changed_while_reviewing`).
A multi-voucher batch whose voucher mark step was not confirmed is a different case: the review of
`acknowledge_post_review` is kept on this computer and does not change its status.

A native post whose saved answer from Tally reported none of its vouchers created (every counter reported, none
created, one exception for each voucher sent, nothing else counted; for a batch, the post also recorded its
voucher step as matching) stays `reconciliation_required` for good, and ComplyEaze Bridge never sends it again:
Tally can still finish work its client gave up on, so a second send could book a voucher twice (bridge#1709).
Once the person says each voucher is in the book or will not be posted, `acknowledge_post_review` with `doubt`
set to `not_created` settles the batch, in its own dialog (`--confirm-settle`, titled to close the batch), which
the model cannot answer. It is admitted only while the journal holds that answer and a fresh readback shows the
batch neither `posted_verified`, bound, still binding nor rolled back (`ack_not_created_not_established`
otherwise; `ack_batch_not_posted` for a batch never sent; `ack_already_recorded` while one stands). The record,
in the import journal, changes nothing in Tally and no status or verdict, binds no voucher and sends nothing. Its
rows stay held, so a rebuild of them is still refused (`import_txn_already_posted`), and a Sales invoice keeps
its stop until the separate release (`doubt` set to `invoice_stop`). `verify_import` reports it as
`person_settlement`, with its own `next_step` and a binding summary that says a person settled the batch, and
`local_data_report` counts the batch as `settled_by_person` instead of `not_settled`. A later `posted_verified`,
a later answer from Tally with an outcome, a binding or a recorded rollback of the book voids it. Nothing is
written when the batch's records change while the dialog is open (`ack_changed_while_reviewing`). A build older
than this record refuses the whole import journal once one is written, as for a stop release: do not downgrade
after recording one.

A voucher of an untagged native post that was not bound (its binding refused or
its response lost), and that its content no longer finds (for example after an
edit in Tally), is `sent_not_attributed`, never `not_found`. In the post's own
readback only, when its own answer from Tally reported every counter, created
none of the vouchers sent and reported one exception for each, with nothing else
counted, and none of them is found, they are `tally_reported_not_created`
instead (bridge#1108). For a batch this also needs the company's voucher mark
read on both sides of the post and unmoved. The person is told to check that
each is not in Tally and enter it there by hand, not through Tally's Import
menu. A later check (`verify_import`, a repeated `post_import`, or the check
before a review is recorded) never reads a voucher as not created, since
someone may have entered it by hand and edited it since: it reads
`sent_not_attributed`. When the post's saved answer had that shape (every
counter reported, none created, one exception for each voucher sent, nothing
else counted), and for a batch the post itself recorded its voucher step as
matching, the `next_step` of each voucher the check cannot find says that
Tally reported the voucher as not created when it was sent, and to look for it
in Tally before entering it by hand; the status and the batch's state are the
same. A partly created batch is never read as not created: a
count does not say which voucher Tally rejected, and two vouchers of one batch
with the same content defeat matching by content. A binding refusal is final: an
edit to one voucher of a batch in Tally before the binding is made
(a deferred bind, or a later `verify_import`) refuses it for the whole batch,
whose vouchers are then matched by content only. Such a batch stays
`reconciliation_required`: the person checks its vouchers in Tally, and
`acknowledge_post_review` records no doubt review for it, because it records one
only beside vouchers that read back verified; with `doubt` set to `content_only`
it records that the person checked them, which settles nothing (bridge#1039). Its `post_span_binding.summary` says when the book holds,
for each voucher it sent or for some, a voucher with the same date, voucher type
and ledger entries that is neither cancelled nor optional, and that ComplyEaze
Bridge cannot tell whether that voucher is this post's; `local_data_report` counts it under `not_settled_binding_refused`
(under `not_settled_restored` once a recorded restore covers it),
or under `not_settled_reviewed_not_attributed` once a person has recorded checking it.
`voucher_presence` cannot identify a native post's
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
closing a batch whose binding was refused (bridge#1039); and seeing a restore
keyed past the post's mark before any check (bridge#1050).

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
downgrade after posting with it. From the first verification with the build that
added `<batch>.proof.<time saved>.<SHA-256>.json` (#911), each verification's status
record names the proof it saved, which an older connector refuses too: do not
downgrade after verifying with it. A downgrade before the first post or
verification with that build leaves the
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

### A bank line is posted at most once

ComplyEaze Bridge sends each bank-statement line to a company at most once from
this computer. Once a line has been sent, Bridge will not send it again as a new
voucher, even if the post failed, the reply was lost, or the voucher was later
changed or deleted in Tally. If Tally does not hold it, enter it in Tally
yourself; do not edit the statement to get it past Bridge. After each post
Bridge reads the books back and marks a voucher verified only when it finds
exactly the voucher its own post created. A correction built with
`amends_batch_id` is a file the person imports by hand, meant to alter the
vouchers in place and add none (measured over Tally's gateway, not yet through
its Import menu); Bridge refuses to build one for a batch it posted itself.

What carries the rule, by its code: the journal row check refuses a row another
batch of the company was sent with, by its transaction id (for a row with any
other id, also its date and amounts), whether or not that batch ended well
(`import_txn_already_posted`, #876, #898); the read-back before
the post names a voucher already in the book with the same date, type, ledgers
and amounts and refuses (`import_preexisting_identity`, #901, #908); a native
post is verified by binding its vouchers to their own exact AlterID span, with
their GUID and MasterID recorded in the journal (#864, #1054). A post whose
binding was refused, or whose reply was lost, is closed only by the person
checking the vouchers in Tally (a way to close it inside Bridge is #1039). This
was read from the code, and the cases below were not measured on a live Tally
unless a number links to the measurement.

What the rule does not cover:

1. The memory of what was sent is on this computer. Posting the same company's
   statement from a second computer, or after reinstalling Bridge or deleting
   its data folder, is checked only against what Tally holds, and a voucher
   changed since is not recognised.
2. Two downloads of the same statement from different places (for example net
   banking and a branch printout) may read a narration differently and so not be
   recognised as the same lines; a posted voucher changed in Tally since is then
   not recognised.
3. Entries typed by hand are recognised only if date, voucher type, ledgers and
   amounts all match.
4. Never import Bridge's saved file into Tally by hand after posting it from
   Bridge. The saved file carries a different REMOTEID from the post's, so a
   second voucher is expected (not measured), and Bridge would only report it at
   the next `verify_import`.
5. If a reply is lost, or someone works in Tally while Bridge posts, the batch
   may never read verified; Bridge shows any voucher it finds with the same
   content, or reports the line as sent but not found, and the person checks
   them in Tally.
6. After restoring a backup, check the posted vouchers in Tally.

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
a single attempt. Company discovery can retry transient failures, but a request
that timed out, or whose response body failed after the response began, is not
sent again; its commitments describe the terminal attempt, excluding earlier
attempts. Read failures retain
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
or date) and a next step. An amount in a row that cannot be read names that row
too. A refusal that is not about one row (the shape of the report, the book
window) has a `cause` and no `bill_row`. A due date
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
  window, or the tool counts a small book too), the data parts, then closing marks (only when the window was
  divided). Each is folded with the tool-level combination.
- **Extent check.** On a later page of a listing, the two book-extent requests
  that decide whether the held read can be reused, folded with the runtime
  combination (`continued_listing` in `src-tauri/src/agent_ledgers.rs`).
- **Corroboration.** On a book large enough to be counted, a census of the
  window one day wider on each side, with the marks already known (no read of
  the neighbouring days' vouchers), then the marks read again if the census took
  several requests, plus a scoped marks read if that window is empty too. A
  book that fits one request has the window read whole instead
  (`corroborate_empty_voucher_read` in `src-tauri/src/agent_vouchers.rs`).

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
- `statement_tie_out` (`src-tauri/src/agent_statement_tie_out.rs`): the company
  read, and then, when the file records no window or one it cannot compare,
  nothing more (evidence `partial`, with that reason). Otherwise:
  1. the company read;
  2. a runtime ledger read (currency masters, ledger export) at the window's
     first date;
  3. the same at the day after its last date;
  4. the same again at its first date.

  Each ledger read carries its own deadline and size limits, as for
  `ledger_movement`; none reads vouchers. The result is `partial` when any
  figure is not established.
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
  3. only when a voucher type is declared `manual` (20 requests): the company's master mark (6),
     then the voucher-type numbering (14: the mode probe and identity, the masters pair, the
     numbering pair, identity and mode again); the call is refused here, before any window
     request, when a declared-manual type is not Manual with exactly one series that prevents duplicate
     numbers in the book, the
     book holds no type of that exact name, or its numbering cannot be read (Education mode keeps
     the read's own code, `masters_education_unqualified`). The check reads the numbering as it
     is now, holds for the moment of the read, and does not know how existing vouchers were
     numbered when they were entered;
  4. the window read;
  5. the corroboration (if empty);
  6. the ledger catalogue again.

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
- `audit_test_run` (`read_audit_book` in `src-tauri/src/agent_audit_read.rs`), after the company read, each
  request a single unpaired read between two company-list checks and its evidence folded in this order:
  1. the company's two marks;
  2. the company object (checked against the verified GUID and books-from date);
  3. the groups, the ledgers for the period, the trial balance for the period and the voucher types;
  4. the voucher window: its own opening marks, then a census (always, a book that fits one request
     included; none for a book that has never held a voucher, which is then refused), then the planned
     parts, then its closing marks. Only the parts kept for the engine are folded into the evidence: the
     voucher parts and the closing marks, not the window's opening marks or its census. A call refused
     during the read carries the evidence of the company read only.
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
     it replays the first window's), unless the first window was divided, every
     part was admitted against a census that names each voucher's GUID, and its
     closing marks equal its opening marks: that read is not repeated (#1241);
  5. a scoped read of the company's marks (if the batch was posted natively
     and its voucher mark before the post was recorded, `current_voucher_mark`);
  6. the closing probe (if any voucher was not found);
  7. the ledger catalogue (if the saved masters check runs and the read
     succeeds; when it fails, nothing is folded and the check reports
     `check_unavailable`).

  `evidence.mode_opening` is the opening probe and `mode_closing` the closing
  one (or null). `company` is the company read. `voucher_read` is the first
  window's data parts only, and `voucher_read_corroboration` the second's, or
  `{"state": "not_sent", "reason": "counted_and_bracketed_read"}` when no second
  read was made.
  No named key covers the marks or census requests, which is what the lab
  capture in #726 showed. A later page read with `proof_sha256` sends no
  request: its request digest hashes
  `verify_import_page:<batch_id>:<offset>` and its response digest hashes
  the saved proof.
- `egress_log`, `local_data_report`, `parse_bank_statement`, `read_evidence`
  and `voucher_schema` read nothing from Tally. Their request digest hashes the tool's name, and
  their response digest hashes what they return (for `voucher_schema`, the
  name again), except that of `egress_log`: it hashes every receipt line the call read, which
  on an answer trimmed for size is more lines than the answer holds (`omitted_for_size` counts
  the difference).

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
`rows` sentence and its `page` rather than keeping a stale one. A partial read names every gap with its counts, and
the result names up to 20 ledgers of each kind that were left out. The codes stay in `result`. `profit_and_loss` and `balance_sheet` carry one too, with no `rows`:
when every result of the statement is established, the lead says so and what the derived lines passed
the comparison with (Tally's own Balance Sheet, and its own Profit and Loss when a profit and loss read it
as well); when any result is not established, the lead starts "Not established" and names each result
with its own state, the reason in words (the reasons are a closed list, so a new one is a compile error
until it has words) and, for a difference, how many lines did not tie (a Tally line that differs, a Tally line carrying an
amount that nothing derived was compared with, a derived line Tally has no counterpart for, and for a
profit and loss the Cost of Sales heading when it is off the derived cost of sales), says when the
derived lines are withheld, and gives one next step for each reason. So far `trial_balance`,
`profit_and_loss`, `balance_sheet`, `cash_flow` and `company_features` carry one, and `post_import` and `verify_import` carry one of
another shape (below); the other read tools and the refusals follow.

`post_import` (after a send was read back) and every page of `verify_import` carry a headline that
says how the batch ended, counted voucher by voucher from the verification's own rows, never from
tool arguments and never stored in the proof (a later page recomputes the same one from the saved
proof). The lead says whether the batch is confirmed: for a batch of one voucher it is a sentence about
that voucher, and for a batch of several vouchers it gives their number and, when some are not
confirmed, how many read back (`posted_verified`) and how many are not. The counts are always in `batch`, which holds `vouchers`, `confirmed`, `not_confirmed` (one count for each other row status,
and `unrecognised` for any status this build does not know) and `verdict` (`posted_verified` only when
every voucher is confirmed and the readback as a whole is). When every voucher reads back but the
batch as a whole is not confirmed (a duplicate, a doubt about the ledgers or the voucher mark, or an
answer from Tally that was not clean or is missing), the lead says so and does not call the batch
confirmed. When only some vouchers read back, the counts hold the rows only and cannot tell a doubt
about the whole batch from a doubt about one row, so the lead says that the vouchers read back and that
the batch is not confirmed, and never calls those vouchers confirmed. When some vouchers are not
confirmed, the lead says that each row says what was found and
gives a next step where there is one: only five statuses (`bound_not_in_window`, `book_rolled_back`,
`sent_not_attributed`, `tally_reported_not_created`, and `posted_divergent` on an invoice only) carry a `next_step`. Only a batch ComplyEaze Bridge dispatched natively
(`result.dispatch.state` present and not `not_dispatched`) carries one: a built batch not yet posted, a
file imported by hand and a refusal before anything was sent carry none, because the batch can still be
posted.

## Protocol and migration notes

The server negotiates MCP `2025-06-18` or `2024-11-05`, returning a supported
version when a client proposes a newer one. Initialization must precede tool
requests. Incoming frames are limited to 5 MB. Every tool answer obeys the configured
byte cap, including the JSON-RPC wrapper and newline. Control replies (to `initialize`,
`ping` and `tools/list`, and the refusal of an unknown method) are fixed by the build and
the request id and carry no book data, so they are held instead to a ceiling of
1,000,000 bytes, or the configured cap if that is larger: the tool list is sent whole
whatever the cap. A control reply that cannot fit returns `agent_response_too_large`;
the session remains usable. Text content contains the same
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
any Tally request, and so is a missing required argument (`<name>_required`, for
example `company_guid_required`).

`validate_masters` accepts 1–100 nonblank names, each at most 1024 characters.
Near-miss suggestions are limited to 25 names and 8192 UTF-8 bytes per requested
name; `candidate_count`, `candidate_count_is_lower_bound` and
`candidates_truncated` preserve ambiguity and count precision. A true lower-bound
flag means "at least N" even when no candidates are listed. Import
planning allows 1000 vouchers but at most 100 distinct ledger names per batch.
Repeated uses of a ledger do not consume additional distinct-name slots.
Voucher-type name and ledger selectors share the 1024-character bound
(`voucher_type_guid` is bounded at 128); ledger
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
Active vouchers with no accounting entry (a Stock Journal) are left out of movement
like cancelled and optional ones; an entry row with only some of its fields is refused.
Movement corroborates the complete opening-ledger snapshot after voucher reads
and rejects unknown entry names before selecting a ledger. Caller-specified
opening dates require a freshly observed supported product/mode and a valid native
boundary before and after the read; a prior status call or cached profile does not
grant admission.

Top-party ranking uses `gross_exposure`, with billed and unallocated receivable
and payable fields kept separate. `totals.scope` is `open_bills_only`.
`open_bills_total` counts every open bill in the requested direction (`direction`, default `both`; the bills `totals` and
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
most 42 vouchers, so a window of more than 5,376 of the company's vouchers is refused unless
the party is read by its ledger (below), and a smaller one may be when its parts measure heavier. The refusal comes before any
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
`direction`, `top`, `offset` and `limit`. The detail reads the company's window, and
`window.company_vouchers_read` counts the company's vouchers in it, not the party's. Only when that
read is refused as needing more requests than one call may spend is the party read by its own
ledger instead (#1356): when its vouchers carry the ledger's name as stored, the name holds only
ASCII letters, digits and single spaces and no word a TDL formula reads as an operator (`and`,
`or`, `not` and a few more), and no other ledger shares it but for case or spaces. That is the
scoped window read `ledger_movement` uses, with the window read's fields (protocol reference
section 11c.6): a census of the ledger in AlterID spans counts the party's vouchers, the planner
divides them by date into parts, each part is admitted against the census voucher by voucher, and
`window.party_vouchers_read` counts the party's vouchers. The detail then carries `window_scope`
(`basis` `ledger_scoped`, `vouchers_read`, and a `note`: only the vouchers Tally lists under the
ledger were read, and that list has been compared with a whole-book read only in windows where
every voucher carried the ledger). Where the ledger is not read, or its read finds nothing or
needs a day divided by AlterID (a part never observed ledger-scoped), the company window's
refusal stands and carries `window_scope` with `basis` `whole_book` and the `reason`
(`ledger_name_outside_scoped_alphabet`, `ledger_name_holds_formula_word`,
`ledger_vouchers_spell_it_otherwise`, `ledger_name_shared_but_for_case`,
`ledger_scope_plan_needs_day_split` or `ledger_scope_empty_not_established`). A refusal of the
ledger read itself keeps its own code, with `window_scope` `basis` `ledger_scoped`: a part not
admitted against the census (`voucher_window_part_not_admitted`, which is what a book that moved
between the census and the read looks like), a row with no line on the ledger
(`party_ledger_row_without_line`), or the party's own vouchers needing too many requests. A
refusal from a later step, after the party was read by its ledger (the closing marks read, a book
that moved, or the detail's checks of what was read), keeps its own code and carries
`window_scope` `basis` `ledger_scoped` too. The ledger census costs one request (sent twice, as
every read is) per 8,192 of the book's AlterIDs, spent after the company window's census, so such
a call can take about twice as long; that cost is unmeasured. A named bill's window never starts
before the books.

The company's two change marks are read before the statement of open bills and again after the
voucher window, and a detail whose marks moved between them is refused as
`voucher_window_changed_during_read` (a voucher entered or saved, a ledger renamed, or a report
that raises a mark; an export raised the voucher mark by 3 once): the bills, the ledger catalogue
and the vouchers are tied by the party's spelling, so a rename between those reads would have
tied different ledgers. It reads the marks twice and the window reads none of its own, so it costs one marks read more than before,
and any edit in Tally during the call refuses it, so run it again once. A change that moves
neither mark is not seen: one letter-case-only rename of a ledger moved neither (reference,
the walk-in ledger's spelling), so that rename can still answer as a finding. Allocations that net
to zero can still read `tied`. The three reads are not otherwise compared. Measured on one synthetic book only
(reference 12a.14); a second synthetic book (captures of 9 October 2026) also showed an `as_of`
earlier than the last voucher, a post-dated voucher and an advance later adjusted by an invoice
reading as Tally's own screen does, bill for bill. Not measured: a client book, an opening bill with
a credit sign, more than one opening bill.

The read of Tally's Bills Receivable and Payable reports comes before any party detail, and on a
large book it can be the step that fails: on one synthetic book of about 30,000 vouchers, at the
end of its first year (31 March 2026), one of its requests was held past the 20-second limit and
the whole call was refused as `native_outstandings_read_failed`, with no figure returned, while the
same book three months into the year answered (#1356). A Bills report request cut at the limit has
its own cause, `native_bills_report_deadline_exceeded` (#2119); another request of the read cut the
same way keeps `request_deadline_exceeded`. The Bills cause's next step tells the assistant not to
repeat the call. Tally can go on building the report after the limit (a read sent straight after
was refused for time too), so it also says to wait and let `tally_status` answer before the next
read, and that an earlier `as_of` with fewer open bills answered on that book. Where the limit
falls on a given book is not measured. A call withdrawn while the read runs finishes the request in
flight and sends nothing more (#778).

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
`unallocated` figures do not. What is still to be measured (a party term against an unfiltered
control, a per-ledger report, the cost on a large book) is in #1356, the follow-up to #945.

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
remain available for local recovery. A verification writes its proof JSON and
Markdown under new names, then appends the ledger status that names them current,
under one admission lock; it replaces and removes no earlier file, so a failure
before the append leaves the earlier proof current. Builds create
the journal first, write and sync staged XML, then expose the importable filename. An interrupted
build, or a status append whose outcome is unknown because its rollback failed,
leaves a recovery journal and blocks further import
admission until the local files and ledger are reconciled. A verification
stopped around its journal append leaves `imports/.proof-publication`. Compare
the status record in its `update.json` with the journal's last line, as JSON
(the file is indented, the line is not):

- the same record: the append finished, and its proof is current;
- an earlier, whole record: the append never began, and the earlier proof is
  current;
- a torn last line: the append stopped part-way. Keep a copy of the journal,
  then remove only that partial line;
- no `update.json`: the stop came before the record was written, and the
  journal is as it was.

Then remove the folder. Proof files that no journal record names are left
over from the stopped verification; nothing reads them. Preserve the journal
and generated XML; do not delete it merely to retry. This is explicit
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
otherwise `partial` with reason `nonempty_window_unqualified`. An empty window is
corroborated by a second read widened by a day on each side, and is `complete`
only when that read is also empty and the company has no vouchers (reason
`company_has_no_vouchers`). It is `partial` with `empty_uncorroborated` when the
widened read is also empty but the company has vouchers, and `partial` with
`empty_window_wider_read_nonempty` when the widened read found rows on the
neighbouring days, none inside the window (a row inside it refuses the read as
`window_contradicted`). That last case means the window's own read found nothing
and the neighbouring days' rows could not count it, so a genuinely empty window
next to a busy day cannot reach `complete` today. A book that has
held a voucher is counted however small, so a few vouchers can be `complete` (#1029);
only a book that has never held one sends no census (Tally omits its mark), and a nonempty window read without one is `partial`. Voucher selectors are applied
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
Each active voucher that has entries must have signed amounts that sum
exactly to zero before movement arithmetic or ledger selection. An unequal
projection returns `voucher_entries_unbalanced`, retaining source evidence.
This necessary check cannot detect an omitted subset that itself balances.

Evidence-history and egress-log reads use the smallest of the requested limit,
`BRIDGE_AGENT_MAX_ROWS`, and the 256-record ceiling. Omitted history sets
`truncated`; egress-log reads also report omissions from the bounded tail scan.
An `egress_log` answer names its order (`result.order`, `newest_first`). When the receipts it
would return are over the response byte cap, it keeps the newest whole receipts that fit the
cap as the response is really encoded, sets `truncated`, and counts the receipts left out for
size in `result.omitted_for_size`. This tool has no offset, so the receipts cut for size
cannot be reached by it; a single receipt that is alone over the cap is refused
(`agent_response_too_large`). Receipts left out by the bounded tail scan are not counted in
`omitted_for_size`. A receipt a stopped process left cut short (a torn tail) is never read as a
receipt (#2172). Until the next receipt is written it is reported only by its size, in
`result.unrepaired_tail_bytes`. Writing the next receipt repairs it, by appending, never by
removing bytes: a `#` line, then a `torn_record` marker giving the torn bytes' offset, length and
sha256. `result.torn_records` lists the markers among the receipts read. A line that is neither a
whole receipt nor exactly vouched for by the marker after it, or invalid UTF-8 or JSON in a
receipt, returns `egress_log_incomplete`. The marker is a repair note, not proof against
tampering. The same repair keeps the Terms of Use record (`terms-acceptance.jsonl`) usable: a torn
last line there is never an acceptance, and accepting again repairs it.

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
