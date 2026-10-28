# Run sheet for #2062: capture one cancel of a voucher in ComplyEaze Bridge's native shape

Not for merge. LAB-COMMON v3 (C1 to C11, `LAB-COMMON.md` here, a byte copy from `lab/1211-tie-out-rows-run-sheet` at 539f191d) applies in full; where the row and it differ, the stricter holds. One row, `C1-cancel.md`, on BRIDGE SHAPE LAB. Record the TallyPrime version, the edition, the licence mode and the date.

**WHY.** #2062 adds a candidate cancel renderer: the native post's request with `ACTION="Cancel"` in place of `ACTION="Create"` and every other byte the same. Nothing sends it, and nothing may rely on it until one live answer shows what it does (AGENTS.md P1). Protocol reference §9.14 observed a cancel once, with a request built for that run, not by this renderer. This row sends the renderer's own bytes once and records the request, the answer, the marks before and after, and the voucher as read back, then restores the book.

**THE REQUESTS.** Two files in `requests/`, both rendered by ComplyEaze Bridge's code at fcc39affad69b891ffe3f5a98b6d0302e9c69abd (branch `fix/2062-cancel-renderer`), and encoded as its dispatch encodes a request (UTF-16LE with a byte order mark):
- `w1-create.xml` is `render_native_vouchers_xml`'s request for one Journal, the bytes a native post sends, with a version 4 REMOTEID minted for this row.
- `w2-cancel.xml` is the candidate `render_native_cancel_xml` for the same voucher and REMOTEID. It differs from `w1-create.xml` in five bytes only, the letters of `Create` and `Cancel`.
They are sent by `curl.exe` as C3 sets out (a lab decision of 26 Oct 2026 allows a lab row to send a request by curl, writes included, to find a shape). The shape `w2-cancel.xml` is meant to become is `render_native_cancel_xml` (`src-tauri/src/agent_import_cancel.rs`). The voucher is posted by curl rather than by `post_import` because the cancel file must name the voucher's REMOTEID, and `post_import` mints that at dispatch time. The Create bytes are the native post's own (a test compares the renderer with the live-captured requests byte for byte). Neither file asks for a report or collection, so the request-ID preflight finds nothing to check.

**Reads.** Every read is a ComplyEaze Bridge tool call, one at a time. MARKS means `ledger_masters` (first page, no `snapshot_id`): M = `snapshot.master_alter_id`, V = `snapshot.voucher_alter_id`; `snapshot.reused` must read false. The voucher read is `vouchers` for 15 June 2026 alone; record each row's `date`, `voucher_type`, `narration`, `guid`, `master_id`, `alter_id`, `cancelled`, `optional` and `amounts`, whole.

## WHO ACTS

Steps marked [OWNER] are as `lab/0.7.0-auto-order/ORDER.md` at 6a69df744c73e96db777fac1d04f2d140a4e727c sets out under "WHO ACTS". The run coordinator's word, where the row asks for it (go-ahead, RESTORE, the evidence push), comes from the run coordinator.

## Build

Any ComplyEaze Bridge build the lab already holds that has `list_companies`, `ledger_masters` and `vouchers` (the build sends no write in this row). Hash the exe before the first call and at the end; any other hash is a STOP. Record the version the tool reports. Run it over stdio, ONE launch for the row, with `BRIDGE_TALLY_HOST=127.0.0.1`, `BRIDGE_TALLY_PORT=9001`, ONE new empty `BRIDGE_AGENT_DATA_DIR` (`C:\bridge-lab\2062\data\`), `BRIDGE_TERMS_ACCEPTED=true` (the person responsible for the lab accepting the Terms of Use), and `BRIDGE_AGENT_ENABLE_WRITES`, `BRIDGE_AGENT_REDACTION` and `BRIDGE_AGENT_MAX_BYTES` unset.

`SHA256.txt` lists every file here.
