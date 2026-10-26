# 697 clause21a_candidates real-book parity (#697): one admitted audit read kept, then compared. READ-ONLY sitting.

Not for merge. Written 26 Oct 2026. `SHA256.txt` lists every file here with its bytes; check it before step B0.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy of the one in `lab/699-sft-candidates-sheet` at fdd474a3
(itself a byte copy from `lab/626-1038-1050-run-sheets` at 8c602c15), sha256
5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6.

THE COMPANY is the book named for this run at step B0, with its company number and GUID. Nothing in this item writes to it: every
step is a read, and no write, backup or restore is made.

This item differs from LAB-COMMON in four places, and the stricter rule holds everywhere else:
1. **Requests.** Every read goes through the build's own tools (below), not curl. C3's request files are not used.
2. **Step 0.** C2 step 0 runs WITHOUT its MARKS request (as item 699's Phase B ran it): the marks come from `trial_balance`.
3. **Writes.** None. C5 (backup) and C6 (restore) do not apply.
4. **The book.** C1 says synthetic companies only. A client book may be THE COMPANY only under the owner's rule for client books
   (below); otherwise THE COMPANY is a synthetic lab book.

## The book: the owner's rule
A client book is THE COMPANY only when all of these hold:
- the owner grants it **for this run, in chat**, naming the book; a grant for an earlier run does not carry over;
- `BRIDGE_LAB_TARGET_GUID` is set to that book's GUID **on purpose**, for this run: the build keeps a read only for the target
  company, so setting it is the act that keeps the book's raw answers on this PC;
- **no raw client capture is ever committed or pushed**: not the kept folder, not a part, not a dump, not a tool result;
- the kept folder (B3), the two dumps (C1, C2) and every tool result file are **deleted at the end of the review cycle** that reads
  them, by exact path, with a directory listing afterwards showing them gone, and the deletion is reported to the run coordinator.

The alternative is a synthetic lab book: `BRIDGE SFT LAB` (item 699), or another synthetic `BRIDGE` company. The comparison (C3)
refuses a book with no clause 21(a) candidate on either side (empty-vs-empty), so THE COMPANY must hold at least one debit on a
ledger under Purchase Accounts, Direct Expenses or Indirect Expenses that the test's keyword table matches, or that sits in a
voucher crediting a Capital Account ledger. `BRIDGE SFT LAB` as item 699 keys it has no ledger under those three groups, so on its
own it gives a capture rehearsal (B2 to B4), not a record. A synthetic book used here is named, with its GUID, before B0.

## The build (record at B0)
A named build of master that includes PR #1949 (branch `lab-audit-read-capture`, head
aa689accfb00d89b190e38ed30f4ece997e963ab): with the `lab-writes` feature, lab writes on, port 9001 and `BRIDGE_LAB_TARGET_GUID` set
to the company read, `audit_test_run` keeps the read it had the tax-audit crate admit under
`<data dir>\lab\audit-reads\<manifest sha256>\` (`manifest.json` and `parts\<name>`) and adds one line to
`<data dir>\lab\lab-manifest.jsonl`. Its answer to the caller is unchanged.
- The commit: a master commit whose history holds #1949's merge (a commit whose subject ends `(#1949)`). Record its full SHA and
  the tax-audit crate's tree at it (`git rev-parse <commit>:src-tauri/crates/bridge-tax-audit`). #1949 not merged into it = STOP.
- Built with `cargo build --release --locked --manifest-path src-tauri/Cargo.toml -p bridge --features lab-writes`, toolchain
  1.96.0. Record the exe's path, bytes and sha256 (Get-FileHash).
- The tools list of that exe lists `audit_test_run`. With lab writes on it also lists the lab write tools (`lab_import_masters`, `lab_import_vouchers`): record the list at B0 and call none of them; only the tools the steps name are called. It need not admit `clause21a_candidates`: the read is the same for every test,
  so `cash_44ab` (admitted) captures it.

## The environment (record each value at B0; nothing else set)
Run the exe as the stdio server with exactly:
- `BRIDGE_TERMS_ACCEPTED=true` (the owner allowed the lab to set it on 25 Oct 2026; without it every call refuses `terms_not_accepted`);
- `BRIDGE_TALLY_HOST=127.0.0.1`;
- `BRIDGE_AGENT_ENABLE_TAX_AUDIT=true`;
- `BRIDGE_LAB_WRITES=1`;
- `BRIDGE_TALLY_PORT=9001` (any other port refuses as `lab_port_not_9001`);
- `BRIDGE_LAB_TARGET_GUID=<THE COMPANY's GUID>`; `BRIDGE_LAB_DENY_GUIDS` as the lab's standing config has it, or absent;
- `BRIDGE_AGENT_DATA_DIR` as the lab's standing config has it (record the folder; the kept read lands under it);
- no `BRIDGE_AGENT_REDACTION` (a redaction setting refuses `audit_test_run` as `audit_redaction_unsupported`).

## Phase B: the read (READ-ONLY, tool calls only, one at a time)
B0. Check `SHA256.txt`. Run C2 step 0 WITHOUT its MARKS request. The marks come from `trial_balance` for `2025-04-01` to
    `2026-03-31` (offset 0, a fresh read): record `snapshot.master_alter_id`, `snapshot.voucher_alter_id` and `snapshot.reused`,
    which must be false (true = STOP). Record the build and the environment above. List `<data dir>\lab\audit-reads\` (absent, or
    the folders already there, by name): the folder B2 makes must not be among them.

B1. `list_companies`. EXPECT THE COMPANY open with the GUID recorded, and that GUID equal to `BRIDGE_LAB_TARGET_GUID`. Anything
    else: STOP.

B2. `audit_test_run`, ONCE: `company_guid` = that GUID, `from` = `2025-04-01`, `to` = `2026-03-31`, `test` = `cash_44ab`,
    `cash_groups` and `bank_groups` = the book's own cash and bank groups, as the run coordinator names them before B0 (record
    them; the same lists go into C1's client config). Keep the whole result as a file, with its size and sha256.
    - A refusal: record its code. Only `C6-moved` or `voucher_window_changed_during_read` may be called once more, after C2 step 0
      (a retry makes a new read; a folder left by the first call is listed, not reused).
    - `lab_audit_read_not_target`, `lab_audit_read_exists`, `lab_evidence_write_failed` or `lab_dir_unavailable`: STOP.
    - `audit_part_deadline_exceeded`: STOP (its own text says not to retry).
    - EXPECT (recorded, not predicted): `result.state`, each figure and finding id with its value, and the code and count of every
      failed book check.

B3. The kept read. List `<data dir>\lab\audit-reads\` again: EXPECT exactly one new folder. Record its name, and for
    `manifest.json` and every file under `parts\` the name, bytes and sha256. Read the LAST line of
    `<data dir>\lab\lab-manifest.jsonl` (parse it as JSON): EXPECT `tool` = `audit_test_run`, `company_guid` = THE COMPANY's GUID,
    `manifest_sha256` = the folder's name = the sha256 of its `manifest.json`, and `parts` = the number of files under `parts\`.
    Record `at` and `read_id`. Any difference: STOP. Then `local_data_report`: EXPECT the `lab` class to count these files and
    bytes (record its figures).

B4. Run C2 step 0 again, WITHOUT its MARKS request, and the same `trial_balance` call (offset 0; `snapshot.reused` false, else
    STOP). Its two marks equal B0's (nothing was written). Then C10.

## Phase C: the comparison (run by the run coordinator, not the lab)
Run where the kept folder is: for a client book, on the lab PC (the folder and the dumps never leave it); for a synthetic book, on
the lab PC or, after the privacy scan, wherever the run coordinator holds the reference. The reference engine is not in this
repository: it is "the reference engine at pin 6168aeaa", the pin of `cash_44ab`'s record, run by the run coordinator.

C1. The client config: the reference engine's client config for THE COMPANY, with `[roles]` `cash_groups` and `bank_groups` as B2
    sent them, and NO `[clause21a]` table and NO `[partners]` table, so both sides see what `audit_test_run` passes the test (no
    extra terms, no partner ledgers; a partner's interest or remuneration credited to capital is then a personal candidate on
    both sides). Record its sha256. A config with either table is a different comparison, recorded separately if made.

C2. The reference's dump, from the tax-audit crate directory of the build's commit:
    `python parity/python_golden.py <reference engine> <client toml> <python dump> --read <kept dir> --test clause21a_candidates`
    (the crate's provenance notes give the `uv run` line and its packages). It refuses unless the reference's keyword table equals
    the crate's copy byte for byte (sha256 28cb01475ab4ca72b8f9fb1cfb29ef4dcace3f56614d6fb64703d6685b4721b3): that refusal is a
    STOP, not a result. Record the dump's bytes and sha256.

C3. From the same directory:
    `cargo run --release --locked --example local_parity -- clause21a_candidates <rules> <client toml> <kept dir> <python dump> <rust dump>`
    where `<rules>` is the reference engine's full rules file. Write stdout and stderr to a file and read the file.
    EXPECT `test_id=clause21a_candidates identical_json=true`, `differences=0`, exit code 0, and `figures` at least 2 (two per
    item with a candidate). `PARITY REFUSED: both sides report zero figures` = no record: the book has no candidate; choose
    another book. Any difference: STOP and send the output's numbers (not its listed differences, which can name a ledger) to the
    run coordinator.

What this gives: the real-book parity record `clause21a_candidates` needs before `audit_test_run` admits it, at reference pin
6168aeaa and the crate tree recorded at B0. Until C3 passes on a book, that record is PENDING.

## Evidence and after
- A client book: the kept folder, the dumps, the tool results and the client config stay on the lab PC, listed in the notes by
  name, bytes and sha256 only, and are deleted at the end of the review cycle (above).
- A synthetic book: the kept folder may be pushed as a fixture candidate only after the privacy scan and the synthetic-UUID
  registry check pass; the dumps likewise.
- Either way, push only the result JSON summaries: for each C3 run, one JSON with the build commit, the crate tree, the reference
  pin, the client config's sha256, the kept folder's name (the manifest sha256), the two dumps' sha256, `identical_json`,
  `figures`, `findings`, `book_violations`, `result_violations`, `differences` and `rust_ms`, as numbers and hashes; no ledger,
  group, party or company name, and no listed difference. Push branch `lab/697-clause21a-parity-answers` with SHA256.txt and a
  MANIFEST (each tool call with its arguments, the window, the build path and sha256; for a client book, each argument other than
  the dates and the test by its sha256, never its value). Send the SHA to the run coordinator before pushing.
