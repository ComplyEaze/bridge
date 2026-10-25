import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("ledger investigation remains an explicit, bounded Overview action", async () => {
  const [app, screen, styles] = await Promise.all([
    readFile(new URL("../src/main.tsx", import.meta.url), "utf8"),
    readFile(new URL("../src/LedgerEntriesScreen.tsx", import.meta.url), "utf8"),
    readFile(new URL("../src/styles.css", import.meta.url), "utf8"),
  ]);
  assert.match(app, /import \{ LedgerEntriesScreen \} from "\.\/LedgerEntriesScreen";/);
  assert.match(app, /type View = .*"ledger_entries"/);
  assert.match(app, /onClick=\{\(\) => setView\("ledger_entries"\)\}/);
  assert.match(app, /<LedgerEntriesScreen[\s\S]*?onReadActivity=\{changeChildTallyReadActivity\}/);
  assert.doesNotMatch(app.slice(app.indexOf('<nav aria-label="ComplyEaze Bridge navigation">'), app.indexOf("</nav>")), /Investigate ledger/);
  assert.match(screen, /config\.host, config\.port, company\.name, company\.guid, company\.company_number, company\.books_from_yyyymmdd, company\.canonical_origin, ledger, from, to/);
  assert.match(screen, /return \(\) => \{ requestVersion\.current \+= 1; \};/);
  assert.match(screen, /submittedScope === latestScope\.current/);
  assert.match(screen, /fetch_selected_ledger_entries/);
  assert.match(screen, /onSubmit=\{\(event\) => \{[\s\S]*?investigate\(\)/);
  assert.match(screen, /shows vouchers observed for this selected window before filtering\. Each next page makes a new observation/);
  assert.match(screen, /Voucher entries and counterpart lines/);
  assert.match(screen, /voucher\.cancelled && "cancelled"/);
  assert.match(screen, /voucher\.optional && "optional"/);
  assert.match(screen, /result\.total > 0 && result\.items\.length === 0/);
  assert.match(screen, /Show first entries/);
  assert.match(screen, /result\.items\.length > 0 && result\.offset \+ result\.items\.length < result\.total/);
  assert.match(styles, /\.ledger-investigation-form\s*\{[\s\S]*?grid-template-columns:/);
  assert.match(styles, /@media \(max-width: 760px\)[\s\S]*?\.ledger-investigation-form/);
});

test("desktop command invokes the one shared selected-voucher operation", async () => {
  const [agent, vouchers, commands] = await Promise.all([
    readFile(new URL("../src-tauri/src/agent.rs", import.meta.url), "utf8"),
    readFile(new URL("../src-tauri/src/agent_vouchers.rs", import.meta.url), "utf8"),
    readFile(new URL("../src-tauri/src/commands.rs", import.meta.url), "utf8"),
  ]);
  assert.match(vouchers, /pub\(crate\) async fn selected_voucher_operation/);
  assert.match(vouchers, /pub\(super\) async fn vouchers[\s\S]*?selected_voucher_operation\(self, args\)/);
  assert.match(agent, /pub\(crate\) async fn desktop_selected_vouchers[\s\S]*?vouchers::selected_voucher_operation_for_verified\([\s\S]*?&server/);
  assert.match(commands, /fetch_selected_ledger_entries[\s\S]*?desktop_selected_vouchers/);
  // The voucher source read is bounded before it is sent (protocol reference
  // §11c): `read_entry_window_rows`, which parses each row under the composite
  // policy (#674), plans it through `read_voucher_window` with the shape's
  // limits, as `read_entry_window_shaped` does (neither pattern may cross into
  // another function); a type filter alone switches it
  // to the class-resolving shape (#625). The ledger catalogue is read before
  // and again after it: before, `read_resolvable_ledgers` keeps both spellings of
  // each ledger so a typed name resolves by either (#1085); after, the drift check
  // `read_ledger_catalogue` stays on the spelling the voucher rows carry.
  assert.match(vouchers, /read_resolvable_ledgers\b(?:(?!\bfn\s)[\s\S])*?read_entry_window_rows\((?:(?!\bfn\s)[\s\S])*?read_ledger_catalogue\b/);
  assert.match(vouchers, /async fn read_entry_window_rows(?:(?!\bfn\s)[\s\S])*?self\.read_voucher_window\((?:(?!\bfn\s)[\s\S])*?WindowReadLimits::for_shape\(shape\)/);
  assert.match(vouchers, /type_selector\.is_some\(\) \{\s*VoucherReadShape::ClassEntryWildcard\s*\} else \{\s*VoucherReadShape::EntryWildcard/);
  assert.match(vouchers, /fn read_entry_wildcard_window[\s\S]*?VoucherReadShape::EntryWildcard/);
  // The page is taken from the filtered and selected rows: the ledger filter runs first, then the rows are held for later
  // pages and `render_page_body` cuts the page: for a listing it calls `page_items` (which applies `skip(offset)` then
  // `take(limit)`), for a summary the bucket page. A served later page calls the same `render_page_body` on the held rows
  // (no pattern may cross into another function).
  assert.match(vouchers, /filter_voucher_rows_for_ledger(?:(?!\bfn\s)[\s\S])*?render_page_body\(server, &rows, summary\.as_ref\(\), \(offset, limit\)\)/);
  assert.match(vouchers, /fn render_page_body(?:(?!\bfn\s)[\s\S])*?page_items\(server, rows, offset, limit\)/);
  assert.match(vouchers, /fn page_items(?:(?!\bfn\s)[\s\S])*?skip\(offset\)(?:(?!\bfn\s)[\s\S])*?take\(limit\)/);
  assert.match(vouchers, /render_page_body\(self, &snapshot\.rows, summary\.as_ref\(\), \(offset, limit\)\)/);
});
