# 1688 ledger aliases (#1688): what Tally's ledger lists return for ledgers with one alias, two aliases and a non-ASCII alias, whether an alias can equal another ledger's name, and what ComplyEaze Bridge 0.7.0 reads from them (Part of #1688)

Not for merge. About two hours. `SHA256.txt` lists every other file here with its bytes; check it before P0.

**Writes to the book: YES, to THE COMPANY only, undone at the end by restoring BK-1688.**
- Part K keys aliases on three existing ledgers on Tally's own Ledger Alteration screen, and tries one alias that Tally may refuse.
- Part I creates up to five ledgers by request, each with one alias.
- Nothing else is changed. No voucher is posted. No ComplyEaze Bridge write tool is called.

## Why
- **What is built.** Since 0.7.0 (#1937, #2028), ComplyEaze Bridge's ledger catalogue keeps every name after the first of a ledger's first language list as an alias, once each, apart from empty or unusable names and names equal to the ledger's own (`distinct_aliases`). The twin check counts aliases, `validate_masters` names the ledger a requested alias belongs to (`alias_of`), and a twin found only through an alias is named (`through_alias`). It never sends an alias: a voucher naming one is refused (`masters_not_exact`).
- **What waits on this row.** #1688's last part resolves a requested alias to its own ledger in the build, so that the voucher is sent with the ledger's own name. That is built only on what Tally is seen to return.
- **What has never been measured:**
  - those reads through ComplyEaze Bridge's own build. The alias tests use a committed capture edited to add an alias;
  - a non-ASCII alias;
  - two aliases on Bridge's catalogue request. They were seen only on the requests of `lab/masters-m1-answers`;
  - an alias equal to another ledger's name in lowercase on Tally's screen. `lab/1688-b1-alias` saw capitals, a doubled space and the exact text refused there, and did not try lowercase;
  - an alias set by an import at all, and with it an alias equal to another ledger's name, folding equal to one, or equal to another ledger's alias. Ledgers whose names differ only by a trailing line break, or by case and one, coexist in a book (protocol reference 9.4e), so the gateway's rules for names are not the screen's.
- **What this row decides:**
  - **Q1, the reads.** For one alias, two aliases and a non-ASCII alias, do the three ledger lists (`CAT`, `CV2` and `IDX`) list each alias under the ledger's first language list, after its own name, byte for byte as keyed? Does `validate_masters` then give `alias_of` for each?
  - **Q2, the screen.** Does Tally's Ledger Alteration screen refuse an alias that equals another ledger's name in lowercase?
  - **Q3, the import shape.** Does a ledger Create that carries a language list with an alias set that alias?
  - **Q4, a clash by import.** When an imported alias equals another ledger's own name, folds equal to one, or equals another ledger's alias, does Tally refuse the Create, create a twin, or alter another ledger?
  - **Q5, the twin check.** For any twin that exists after Q2 or Q4, does `validate_masters` name it?

**Expected:** Tally's answers are not predicted. Every reading is a result. Only the STOP rules end the row early.

## Rules and company
`LAB-COMMON.md` in this folder applies in full. It is v3, a byte copy from `lab/2214-mailing-cut-drain` at 3ba77c56, sha256 5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6. Where it and this sheet differ, the stricter holds.

**Who acts.** The lab does every step, on the owner's standing words, and records each as LAB-COMMON C11 directs, naming the word:
- the raw requests, the creates included: 26 Oct 2026, the lab's curl requests, writes included, for shape discovery;
- master writes on a synthetic book, the screen keying of Part K included: 28 Oct 2026;
- the backup (C5);
- the restore (C6) at Close, on the run coordinator's RESTORE word for this row.

The go-ahead and any STOP ruling come from the run coordinator.

**THE COMPANY** is `BRIDGE ALIAS LAB` (synthetic), company number 100070, with the GUID `lab/1688-b1-alias`'s `MANIFEST.txt` records (it begins `021bda0c`). The company and its three Lab ledgers were keyed for that run (21 Oct 2026), which then tried aliases and saved none. It should hold five ledgers:
- `Cash`;
- `Profit & Loss A/c`;
- `Lab Other Ledger`, GUID suffix `-000000cf`, under Sundry Debtors;
- `Lab Plain Ledger`, GUID suffix `-000000ce`, under Sundry Debtors;
- `Lab Sales`, GUID suffix `-000000d0`, under Sales Accounts.

None has an alias, and the book holds no voucher. On 21 Oct `ledger_masters` read `snapshot.master_alter_id` 210 and `voucher_alter_id` null; no raw MARKS read was kept. If the company is absent, or P0's reads differ from this, record the difference and STOP.

**Raw requests.** None is sent but these:
- **MARKS:** `requests/marks.utf16le.xml`, 2214's MARKS request with only the company changed;
- **CAT:** `requests/ledger-catalogue.utf16le.xml`, the standard ledger catalogue (V1: `Name`, `GUID`, `Parent` and the two company fields). It is byte-identical to `lab/1688-b1-alias`'s `r1-ledger-catalogue.utf16le.xml`, which is master's `shape-lab-standard-ledger-catalogue` request fixture with only the company changed. It has no GUID filter;
- **CV2:** `requests/ledger-catalogue-v2.utf16le.xml`, the import ledger catalogue (V2: V1 and `IsBillWiseOn`). This is the request `validate_masters` sends (`render_import_ledger_catalog_request`). It is IDX with only its `AlterID` line taken out;
- **IDX:** `requests/ledger-name-index.utf16le.xml`, master's `masters-lab-ledger-index.request.utf16le.xml` (the V3 ledger name index: V2 and `AlterID`, read by `prepare_ledger_creates`), with only the company changed.

  The three are ComplyEaze Bridge's renderings of one collection, each with one more method than the last: taking IDX's `AlterID` and `IsBillWiseOn` lines out gives CAT byte for byte. Bridge's own sends differ from them in the company name and possibly in leading whitespace;
- C2 c's and C10's `GET /status`;
- the five creates of Part I, each at most once.

Every read's size bound is C3's 2,000,000 bytes. Every other request is ComplyEaze Bridge's own, through its tools.

**THE READS** are MARKS, CAT, CV2 and IDX, sent in that order, each once, by C3's line. Each answer is decoded and parsed as C4 directs. For CAT, CV2 and IDX, record per LEDGER row:
- the `NAME` attribute;
- the GUID;
- the parent;
- for CV2 and IDX, `ISBILLWISEON`, and for IDX `ALTERID`;
- every `NAME` under every `NAME.LIST` of every `LANGUAGENAME.LIST`, in document order, each with its length in characters and its code points in hex.

Also record the count of LEDGER rows, and of `LANGUAGENAME.LIST` blocks in each row. Every GUID must begin with THE COMPANY's, else STOP. Keep each table as a file.

**Named exceptions to LAB-COMMON C3 and C4,** for Part I's creates only:
1. **The answer.** C4's EXPECT (`CREATED` equal to the objects sent, every other counter 0) is replaced by the three answers Part I calls results. Any other answer is an I-HALT, not a STOP, unless the marks rule below makes it one.
2. **The read after a write.** C3's "if a read does not show it, STOP" holds for I0, as I0's check. For I1 to I4, a read that does not show the alias as sent is a result: record it.

**Marks.** ALTMSTID moves by one for each save Tally accepts (a screen accept, a `CREATED` 1 or an `ALTERED` 1), and by nothing for a refusal (`ERRORS` 1, or a screen refusal). Any other move = STOP, an I-HALT answer included. ALTVCHID must not move. If it is absent from the row (a book with no voucher), record that; it is not a STOP.

**Keys.**
- **The Gateway, the ledger list that Alter > Ledger opens, and the Ledger Alteration screen** are the only screens used.
- **No key is pressed but:**
  - the arrows and Enter of Select Company at P0, and at Close after the restore;
  - the Backup screen at S0;
  - in Part K: Gateway of Tally > Alter > Ledger, the ledger's name typed in the list's search line, Enter, then the step's alias text in the `(alias)` lines under Name. Enter ends each alias line. Accept the screen with Ctrl+A;
  - Ctrl+Q to leave a Ledger Alteration screen without saving, and only after a screenshot shows that screen's title;
  - the Enter that K3 and K4 name for their boxes;
  - **back to the Gateway:** after Ctrl+A or Ctrl+Q, take a screenshot. If it shows the Gateway, press nothing. If it shows the ledger list (its title, and no master screen and no box), press Escape once: the list is not a master screen, so C7's rule on Escape does not apply to it. Take a screenshot again: it must show the Gateway. Anything else = STOP;
  - Company > Shut, and the Restore screen with its two prompts, at Close (C6);
  - Company > Shut at C10.
- **Never** Ctrl+Q or Escape at the Gateway of Tally, and never Escape on a master screen (C7).

Licensed mode only; Education mode = STOP.

## Build and inputs
- **The build:**
  - ComplyEaze Bridge 0.7.0 as published (tag `mcp-v0.7.0`). Use its `bridge_mcp.exe` from the release's MCP client extension, the one the lab installs. Record the extension file's name, bytes and sha256, and the exe's sha256, at P0 and again at Close. A different second exe sha256 = STOP.
  - `initialize`'s `serverInfo.version` must read `0.7.0`. Another version = STOP: an older build has no `alias_of`.
- **The server:**
  - It runs over stdio with:
    - `BRIDGE_TALLY_HOST=127.0.0.1` and `BRIDGE_TALLY_PORT=9001`;
    - a new empty `BRIDGE_AGENT_DATA_DIR`, `C:\bridge-lab\1688\data`, listed first: absent or empty, else STOP;
    - `BRIDGE_TERMS_ACCEPTED=true` and `BRIDGE_AGENT_MAX_BYTES=5000000`;
    - redaction unset.
  - Writes stay off. Record the environment's key names.
  - `tools/list` must hold `list_companies`, `validate_masters`, `ledger_masters` and `egress_log`; any one absent = STOP.
- **THE BRIDGE READS** (Part B) are three calls, in this order:
  1. **`validate_masters`** for THE COMPANY, with `ledgers` set to these 18 names, in this order:
     1. `Lab Plain Ledger`
     2. `Lab Plain Alias`
     3. `LAB PLAIN ALIAS`
     4. `Lab Other Ledger`
     5. `Lab Other First`
     6. `Lab Other Second`
     7. `lab other ledger`
     8. `Lab Sales`
     9. `Lab Café Sales` (é is U+00E9)
     10. `Lab Alias Shape`
     11. `Lab Shape Alias`
     12. `Lab Alias Script`
     13. `लैब खाता` (U+0932 U+0948 U+092C U+0020 U+0916 U+093E U+0924 U+093E)
     14. `Lab Alias Clash`
     15. `Lab Alias Fold`
     16. `LAB SALES`
     17. `Lab Alias Shared`
     18. `Lab No Such Ledger`

     The list is the same at every Part B, typed exactly. The names not yet keyed are controls.

     Record the call's answer whole. For each row record `match_state`, `alias_of`, `folded_twins`, `through_alias` and any other field.
  2. **`ledger_masters`** with `fields=basic`. Record `total` and every item.
  3. **`egress_log`** with `limit` 10, covering the two calls. From each record's `request_trail`, record each send's kind, outcome, `held_ms`, request bytes and response bytes.

  No row's reading is a STOP. A refusal of a whole call is a result: record its code and message whole, and go on.
- **The creates:**
  - `scripts/make_creates.py` writes them. Run `python -I make_creates.py C:\bridge-lab\1688\creates` into a new empty folder.
  - It prints one line per file in `SHA256.txt`'s form (sha256, bytes, `requests/creates/<file>`). Those five lines must equal the five `requests/creates/` lines of `SHA256.txt`. Any difference = STOP, and nothing is sent.
  - Each is one Import of one `LEDGER ... ACTION="Create"` under Sundry Debtors. Its name comes only from a `LANGUAGENAME.LIST` whose `NAME.LIST` holds the ledger's own name and then one alias, with `LANGUAGEID` ` 1033`, the shape the catalogue answers return.

    | file | own name | alias |
    |---|---|---|
    | `create-i0` | Lab Alias Shape | Lab Shape Alias |
    | `create-i1` | Lab Alias Script | लैब खाता |
    | `create-i2` | Lab Alias Clash | Lab Other Ledger (another ledger's own name, exact) |
    | `create-i3` | Lab Alias Fold | LAB SALES (another ledger's own name, in capitals) |
    | `create-i4` | Lab Alias Shared | Lab Plain Alias (another ledger's alias, after K1) |
- **Folders:** the working folder is `C:\bridge-lab\1688\`, with evidence in `C:\bridge-lab\1688\evidence\`. BK-1688 goes to the new empty folder `C:\bridge-lab\1688\bk\`.

## Before the sitting
- **P0.**
  - LAB-COMMON C1 and C2: THE COMPANY in Select Company's list and loaded alone, else STOP.
  - Hash every file of this folder against `SHA256.txt`. Generate the creates and compare them as above.
  - Then THE READS, the at-rest set (r0):
    - **MARKS:** EXPECT ALTMSTID 210, and record ALTVCHID or its absence.
    - **CAT:** EXPECT 5 LEDGER rows, as THE COMPANY lists them, each with one `NAME` in its one language list. EXPECT the answer byte-identical (8,516 bytes, sha256 `3b3dab36824005bc90c0d8bed6744a620a71dbb0a3264273051239ea5066f88d`) to `lab/1688-b1-alias`'s `ans-s1-ledger-catalogue-before.xml`. If not byte-identical but the five rows and names agree, record the difference; it is not a STOP.
    - **CV2 and IDX:** 5 LEDGER rows each, with the same names as CAT. Record each `ISBILLWISEON` and IDX's `ALTERID`.

    A ledger count, name, GUID or alias other than THE COMPANY's above = STOP.
- **S0.** BK-1688 by C5, before any write. Record the marks after it (ms0).
- **Processes.** `tasklist /FO CSV /NH`, kept as a file: no `bridge_mcp.exe`, and no ComplyEaze Bridge desktop app, whoever started it. One running = STOP: it could send requests this sheet does not list.
- **Start the server.** Save `initialize` and `tools/list` whole. Then:
  - `list_companies`: EXPECT exactly THE COMPANY, with `identity_state` `verified_tuple`;
  - **B0:** THE BRIDGE READS. EXPECT `ledger_masters` `total` 5 and `snapshot.master_alter_id` equal to ms0's ALTMSTID. Either differing = STOP: Bridge does not see the book THE READS see.

## Part K: aliases keyed on the screen
For each step, before Ctrl+A:
- take a screenshot of the whole Ledger Alteration screen;
- zoom it on the Name and `(alias)` lines;
- record the text as shown.

A step's text not shown exactly as the step gives it: Ctrl+Q (nothing is saved), record it as "not keyed", and go to the next step. For K1 and K2 that is a STOP instead.

After each step:
- a screenshot of the whole Tally window: no dialog, and the Gateway shown;
- then THE READS.

Record the marks before and after.

- **K1. One alias.**
  - On `Lab Plain Ledger`: alias `Lab Plain Alias`.
  - EXPECT the screen to accept it, and ALTMSTID +1.
  - EXPECT the `Lab Plain Ledger` row of each of CAT, CV2 and IDX to list `Lab Plain Ledger`, then `Lab Plain Alias`.
  - The screen refusing it, or a read without it = STOP.
- **K2. Two aliases.**
  - On `Lab Other Ledger`: aliases `Lab Other First`, then `Lab Other Second`, on two `(alias)` lines.
  - EXPECT ALTMSTID +1, and each of CAT, CV2 and IDX listing the own name, then the two aliases in that order.
  - A refusal, or a read without both aliases = STOP.
- **K3. A non-ASCII alias.**
  - On `Lab Sales`: alias `Lab Café Sales`, with é as U+00E9.
  - If it is accepted: EXPECT ALTMSTID +1. Record the row's names in each of CAT, CV2 and IDX, in code points.
  - Before Ctrl+A, record whether the alias line shows é. Typed text can arrive as e and a combining accent (U+0065 U+0301) rather than U+00E9: CAT's code points say which, and either is a result.
  - **This step names one prompt:** a box that refuses the alias, on leaving the alias line or at Ctrl+A. Take a screenshot and copy its text exactly. Then press Enter once and take a screenshot. If the box is still up, press no further key, and STOP. Otherwise Ctrl+Q, and record "refused", with ALTMSTID unchanged. Any other box is C7.
  - If the screen refuses it, or cannot type é: record that. It is a result, and Part I's `create-i1` covers a non-ASCII alias by import.
- **K4. An alias that folds equal to another ledger's name.**
  - On `Lab Plain Ledger`: add a second alias, `lab other ledger`, all lowercase: `Lab Other Ledger`'s own name in another case.
  - **This step names one prompt.** The box `lab/1688-b1-alias` saw, with the words "Oops! Name already exists. Enter a different name.", may appear on leaving the alias line or at Ctrl+A:
    - take a screenshot, and copy its text exactly. Text other than those words is C7: press no key;
    - press Enter once, and take a screenshot. If the box is still up, press no further key, and STOP;
    - then Ctrl+Q from the Ledger Alteration screen;
    - record "refused", with ALTMSTID unchanged.

    Any other box is C7.
  - If the screen accepts it, Ctrl+A. Record "accepted", with ALTMSTID +1, and the names of `Lab Plain Ledger` in each of CAT, CV2 and IDX. A twin exists then, and Q5 reads it at BK.
- **BK.** THE BRIDGE READS.

## Part I: aliases set by import (each create a NEW SHAPE)
**The creates.** The lab sends `create-i0` to `create-i4`, each once, in order, by C3's line.

**Before each create:**
- send MARKS. Go on only if the answer holds exactly one COMPANY row, THE COMPANY's, with ALTMSTID equal to the last one recorded;
- check the file's sha256 against `SHA256.txt`.

**After each create:**
- record the answer's every counter, `LINEERROR`, `LASTMID` and `LASTVCHID`;
- take a screenshot of the whole Tally window, with no dialog, before anything else is sent;
- then THE READS;
- for every ledger row in IDX whose `ALTERID`, names, parent or `ISBILLWISEON` differ from the IDX before this create, record before and after.

**Which answers are results.** An answer of exactly one of these is a result:
- `CREATED` 1, and every other counter 0;
- `ERRORS` 1 with a `LINEERROR`, and every other counter 0;
- `ALTERED` 1, and every other counter 0.

Any other answer = I-HALT: no further create is sent, and the row goes on to BI and Close. ALTMSTID must move by one for `CREATED` 1 or `ALTERED` 1, and stay unchanged for `ERRORS` 1 or an I-HALT answer; anything else = STOP.

**An `ALTERED` answer pauses the row.** A Create of a new name that alters another ledger is reported to the run coordinator at once, with the rows before and after. Nothing further is sent or keyed until the coordinator's word: to go on with the next create, or to go to BI and Close.

- **I0. The shape check.** EXPECT `CREATED` 1, and each of CAT, CV2 and IDX holding a new row `Lab Alias Shape` that lists `Lab Alias Shape`, then `Lab Shape Alias`. Anything else = I-HALT: the import did not set the alias, so I1 to I4 would test nothing.
- **I1. A non-ASCII alias.** Record the answer, and the new row's names in code points. EXPECT the alias's code points equal to the file's: U+0932 U+0948 U+092C U+0020 U+0916 U+093E U+0924 U+093E.
- **I2. An alias equal to another ledger's own name.** The alias is `Lab Other Ledger`. Record which of the three answers came. If `ALTERED` 1, record which existing row changed, by GUID, and how.
- **I3. An alias that folds equal to another ledger's own name.** The alias is `LAB SALES`. Record it as for I2.
- **I4. An alias equal to another ledger's alias.** The alias is `Lab Plain Alias`, K1's. Record it as for I2.
- **BI.** THE BRIDGE READS.

## Close
1. **Stop the server** by closing its standard input, and record its exit code and error stream. Hash the exe, and re-hash the inputs.
2. **Restore BK-1688 by C6,** on the run coordinator's RESTORE word. The fallback (C6) is `lab/1688-b1-alias`'s BK1, `TDBK1800_100070.001`, 24,104 bytes, sha256 `5c1d904f3fb3abb67e096a98c796f091eb9627f28b13b29d728d7a30a53488df`, kept on the lab PC, restored only on the run coordinator's word.
3. **THE READS:**
   - EXPECT MARKS equal to ms0;
   - EXPECT CAT and CV2 byte-identical to r0's answers;
   - EXPECT IDX byte-identical to r0's IDX answer. If it differs, it differs only where its rows do: compare them as tables and record it.

   Any row or name differing from r0 = STOP. The book was not restored.
4. **Start the server again** with the same environment, and take THE BRIDGE READS (BC). EXPECT every row and item equal to B0's, apart from times and the snapshot's id. Stop the server as in step 1.
5. **Processes:** no `bridge_mcp.exe` or ComplyEaze Bridge desktop app left running.
6. **C10:** MARKS before the shut, then Shut Company, then `GET /status` only.

STOP RULES:
- C7 and C8 of LAB-COMMON;
- any dialog but K3's and K4's named boxes;
- a second `tally.exe`, or a ComplyEaze Bridge process at P0;
- a mark that moves other than as this sheet says;
- K1's or K2's check;
- a GUID outside THE COMPANY's;
- Close's restore check;
- Ctrl+Q or Escape at the Gateway (an incident: press no key, take a screenshot, STOP);
- an exe sha256 or server version that is not the one named;
- a request without an answer in 10 minutes (C3: leave it running, and send it never again).

An I-HALT is not a STOP: it ends Part I only.

## What the row records
- **The step table:** one line per step (K1 to K4, I0 to I4). Each line holds:
  - the act;
  - accepted, refused (with the text) or not keyed, or the create's counters and `LINEERROR`;
  - ALTMSTID before and after;
  - the touched rows' names in each of CAT, CV2 and IDX, in document order;
  - any other row that changed.
- **The name tables** of each set of THE READS (r0, after each step, and at Close).
- **The Bridge tables** of B0, BK, BI and BC: each of the 18 rows' fields, and `ledger_masters`'s items. `alias_of` appears only on a `missing` or `near_miss` row without `folded_twins`, and only for an alias of exactly one live ledger (`annotate_alias_of`). So a name that is a live ledger's own name, or an alias that I4 gives a second owner, carries no `alias_of`. Read K1 to K3's `alias_of` at BK, and I0's and I1's at BI.
- **Reading it (for the requester):**

| reading | meaning |
|---|---|
| K1, K2 and K3 listed after the own name, byte for byte, with BK's `alias_of` naming the right ledger for each, and I0 and I1 likewise at BI | Bridge's catalogue reads Tally's aliases as keyed, and #1688's resolution can be built on the catalogue's alias list. |
| A row's aliases in another order, missing, or in a second language list | The parse rule "every name after the first of the first list is an alias" is wrong for that shape. Report it; nothing is built on it. |
| K4 refused | Tally's screen refuses a lowercase twin alias as well. Together with `lab/1688-b1-alias`, a twin through an alias cannot be keyed on this release's screen. |
| K4 accepted | A twin can be keyed. BK shows whether Bridge names it. |
| I0 `CREATED` with its alias | An import sets aliases. I2 to I4 then show whether the gateway admits the clashes the screen refuses. |
| I2, I3 or I4 `ERRORS` | The gateway refuses that clash too. |
| I2, I3 or I4 `CREATED`, with the alias kept | The gateway makes a twin the screen refuses. BI shows whether Bridge names it. |
| I2, I3 or I4 `ALTERED`, or an alias dropped silently | A Create carrying an alias changed another ledger, or kept less than it was sent. Report it at once, with the rows before and after. |
| Any twin that BK or BI does not name in `folded_twins` | A gap in the twin check. Report it with the rows. |
| Anything else | Record it whole and infer nothing. |

**Confidence:** one sitting on one synthetic book and one release, so PARTIAL.

**Not covered:**
- Gold;
- aliases of groups;
- a second language;
- a voucher that names an alias (`lab/626-twin-answers` t7, 13 Oct, sent one and saw it posted to the ledger and read back under the ledger's own name);
- an alias sent by ComplyEaze Bridge, which never sends one;
- `prepare_ledger_creates` on a name that is an alias.

## Evidence
Everything but the screenshots and the backup is committed, since the book is synthetic. Captured answers are kept byte for byte:
- **RESULTS.md:** every step, stamp, mark, answer, byte count and duration, and the tables above;
- **every raw answer** (MARKS, CAT, CV2, IDX and the five creates), with its headers file, size and sha256;
- **the ComplyEaze Bridge answers** of B0, BK, BI and BC, whole, saved as the tool returned them;
- **`notes.txt`**, `restore-plan.txt`, the C2 observation files, the `tasklist` files, and `SCREENSHOTS.txt`. The screenshots stay on the lab PC, listed by name, size and sha256;
- **`SHA256.txt`** over the committed files, with sizes.

Evidence goes on a new branch, `lab/1688-ledger-aliases-answers`. Send its head to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
