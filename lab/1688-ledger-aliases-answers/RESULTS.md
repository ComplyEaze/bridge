# 1688 ledger aliases: Tally lists one, two and a non-ASCII alias after the ledger's own name in all three ledger lists, and 0.7.0's validate_masters names the owner of each (alias_of); an imported Create sets an alias; a Create whose alias is another ledger's own name ends EXCEPTIONS 1 (I-HALT) (Part of #1688)

Sheet `lab/1688-ledger-aliases` at 33a02077 (LAB-COMMON v3). BRIDGE ALIAS LAB (synthetic), GUID `021bda0c-b1b1-44c8-8279-5212ce87c9c7`. TallyPrime 7.2 Silver. 30 Oct 2026, 20:45 to 21:11 UTC. One sitting, so the result is **PARTIAL**.

**Build:** published 0.7.0.
- Extension file `bridge-tally-mcp-v0.7.0-windows-x64.mcpb`, 15,598,419 B, sha256 291a4ed9…; `bridge_mcp.exe` sha256 48d2f743…. Both were the same at P0 and at Close.
- `initialize` reported 0.7.0, and `tools/list` holds the four tools the sheet needs.
- Environment keys: `BRIDGE_TALLY_HOST`, `BRIDGE_TALLY_PORT`, `BRIDGE_AGENT_DATA_DIR` (a new folder), `BRIDGE_TERMS_ACCEPTED`, `BRIDGE_AGENT_MAX_BYTES`. Writes off, redaction unset.
- Every Bridge call went to this server: pid 5768 at B0, BK and BI; pid 2612 at BC.

## P0 and S0
- **MARKS r0:** ALTMSTID **210**; ALTVCHID absent (the book has no voucher).
- **CAT, CV2 and IDX:** 5 LEDGER rows each, with THE COMPANY's GUIDs, one language list each and no alias.
  - CAT is 8,516 B. It equals `lab/1688-b1-alias`'s answer except inside `CMPINFO` (its COMPANY, LEDGER, CURRENCY and TAXUNIT count lines). Recorded, not a stop.
- **BK-1688:** 24,104 B, sha256 da9289cb…. The marks after it (ms0) were 210.
- **B0:** `ledger_masters` total 5, master_alter_id 210 = ms0.

## Part K (the screen)
| step | act | result | ALTMSTID | names in CAT, CV2 and IDX (each identical) |
|---|---|---|---|---|
| K1 | alias `Lab Plain Alias` on Lab Plain Ledger | **accepted** | 210 → 211 | `Lab Plain Ledger`, `Lab Plain Alias` |
| K2 | aliases `Lab Other First`, `Lab Other Second` on Lab Other Ledger | **accepted** | 211 → 212 | `Lab Other Ledger`, `Lab Other First`, `Lab Other Second` |
| K3 | alias `Lab Café Sales` on Lab Sales | **accepted**, with no box | 212 → 213 | `Lab Sales`, `Lab Café Sales`; the é is **U+00E9** (precomposed), 14 characters |
| K4 | second alias `lab other ledger` on Lab Plain Ledger | **not keyed** | 213 → 213 | unchanged; CAT and IDX byte-identical to K3's |

- **One language list.** Every alias sits in the ledger's single `LANGUAGENAME.LIST`, after the own name, in keyed order, byte for byte.
- **K4 could not be keyed as given.** Tally's alias line capitalised each typed word: `lab other ledger` showed as `Lab Other Ledger`. The step's text was not shown exactly, so the screen was left by Ctrl+Q (no prompt; nothing saved). **Q2 stays open:** this screen does not take an all-lowercase alias as typed.

## BK (THE BRIDGE READS after Part K)
- **`alias_of` names the right owner for each keyed alias:**
  - `Lab Plain Alias` → `Lab Plain Ledger`;
  - `Lab Other First` and `Lab Other Second` → `Lab Other Ledger`;
  - `Lab Café Sales` → `Lab Sales`.

  All four are `near_miss`.
- **No `alias_of` for the controls:** `LAB PLAIN ALIAS` (the K1 alias in capitals) carries none. Nor do `lab other ledger` and `LAB SALES`, whose candidate is the ledger by `normalized_equal`.
- No row carries `folded_twins` or `through_alias`. `ledger_masters` total 5, master_alter_id 213.

## Part I (creates by import; each preceded by a MARKS read equal to the last)
| step | alias | answer | ALTMSTID | result |
|---|---|---|---|---|
| I0 | `Lab Shape Alias` on new `Lab Alias Shape` | `CREATED` 1, rest 0 | 213 → 214 | new row (-000000d1) lists `Lab Alias Shape`, `Lab Shape Alias` in CAT, CV2 and IDX: **an import sets an alias** |
| I1 | `लैब खाता` on new `Lab Alias Script` | `CREATED` 1, rest 0 | 214 → 215 | new row (-000000d2); the alias's code points equal the file's (U+0932 U+0948 U+092C U+0020 U+0916 U+093E U+0924 U+093E) |
| I2 | `Lab Other Ledger` (another ledger's own name) on new `Lab Alias Clash` | **`EXCEPTIONS` 1**, every other counter 0, no `LINEERROR` | 215 → 215 | **I-HALT**: not one of the three result answers. No row created or changed; the reads are byte-identical to I1's |
| I3, I4 | | not sent (the I-HALT ends Part I) | | |

- After each create: a capture equal to the Gateway capture (no dialog), and every LASTMID and LASTVCHID was 0.
- The whole I2 answer is `IMPORTRESULT` with `EXCEPTIONS` 1 and nothing else.

## BI (after Part I)
- `Lab Shape Alias` → `alias_of` `Lab Alias Shape` (`near_miss`).
- `लैब खाता` → `alias_of` `Lab Alias Script` (`missing`, with no candidates).
- The two new ledgers are `exact`. The K-step aliases are as at BK.
- No `folded_twins` or `through_alias`. `ledger_masters` total 7, master_alter_id 215.
- The MARKS read after Part I was 215.

## Close
- **The server** exited 0 with an empty error stream. The exe and every input re-hashed equal.
- **BK-1688 restored** by C6, on the run lead's word for this row (`restore-plan.txt`): Accept, then Replace for THE COMPANY's own folder.
- **THE READS after the restore:** MARKS, CAT, CV2 and IDX are each **byte-identical to r0's**.
- **BC** (a new server, same environment) equals B0 apart from the snapshot id and time (master_alter_id 210).
- **C10:** MARKS 210, Company > Shut, no company loaded, `GET /status` 200.

## Deviations, each on the run lead's recorded word
1. **Four idle `bridge_mcp.exe` processes.** Children of the desktop client (two from the installed extension, two from a master dev build) were running at P0, where the sheet's Processes rule says stop. They were waived and are listed in `p0-tasklist.txt` and `notes.txt`. Every Bridge call went to the lab's own server.
   - Added check: before Part K and after Part I, MARKS read exactly 210 and 215.
   - The idle children's own egress cannot be seen; that is acceptable for an idle client.
2. **The sheet's one Escape from the ledger list reaches Master Alteration's List of Masters, not the Gateway.** After K1 the row stopped there, and a second Escape (from that list) reached the Gateway. K2 to K4 used two Escapes, each followed by a screenshot.
3. **`make_creates.py` was given its folder from a bash shell**, which dropped the backslashes. It wrote into a stray folder under `sheet\scripts`. The five files (each byte-identical to the committed ones) were moved to `creates\`.

Not covered (the sheet's list): Gold; group aliases; a second language; a voucher naming an alias; an alias sent by Bridge; `prepare_ledger_creates` on an alias. Also I3 and I4, not sent after the I-HALT.
