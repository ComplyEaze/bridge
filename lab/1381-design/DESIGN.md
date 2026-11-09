# Creating one stock item from a sibling's GST details, for #1381: design note

Not for merge. Part of #1381. Written 8 Nov 2026. Lines are cited at master 1f5dc024; paths are under `src-tauri/` unless they start with `docs/`. Nothing here is built. Each claim carries a marker:
- **verified**: read in master's code, or the protocol reference's VERIFIED;
- **partial**: one live run (the 5 Nov run 1381-M: sheet `lab/1381-stock-item-create` at 5bbd6fb2, answers `lab/1381-stock-item-create-answers` at d8948467, TallyPrime Silver 7.2, one synthetic company `BRIDGE STOCK CREATE LAB`, each row once);
- **unverified**: derived, or not measured.

The issue's comment of 5 Nov records the 1381-M results and the maintainer's ruling that the Gold row is not run. This note takes both as given.

## 1. What master already has

### The approved master path of #690 (verified: it is for ledgers only, and Create only)
| piece | where | reusable for a stock item |
|---|---|---|
| Import envelope over all masters | `crates/bridge-tally-protocol/src/import_envelope.rs:12` (`ImportReport::AllMasters`) | yes, as it is |
| Counter parser; a counter Tally did not report is not a zero | `crates/bridge-tally-protocol/src/import_outcome.rs:390` (`parse_import_outcome`); `src/agent_master_tools.rs:364` (`reported_counters`) | yes, as it is |
| Counter check against the batch | `src/agent_master_verdict.rs:516` (`counters_check`) | yes, with a count of 1 |
| Recheck before dispatch: marks unchanged, name still absent | `src/agent_master_post.rs:152` (`recheck_master_admission`), `:230` (`check_name_free`), `:338` (`admit_free_name`, which takes a `LedgerCreateItem`) | the logic, yes; the types are ledger types |
| Read-back and verdict | `src/agent_master_verify.rs:300` (`read_master_book_back`), `:459` (`verify_master_batch`); `src/agent_master_verdict.rs:697` (`judge`) | the shape, yes; the fields judged are a ledger's |
| Approval window | `src/tally/approved_import.rs:67` (`MastersPrompt`), `:75` (`masters_words`, which says "ledgers"), `:437` (`confirm_masters`, which takes a `LedgerCreateBatch`) | needs a stock-item variant of the prompt and admission |
| Dispatch | `src/tally/runtime.rs:3499` (`post_approved_import`), the one path (P8) | yes |
| Tools and gate | `src/agent_catalog.rs:298` (`MASTER_WRITE_TOOLS` = `prepare_ledger_creates`, `post_ledger_creates`); listed only under `BRIDGE_AGENT_ENABLE_MASTER_WRITES` (`src/agent_master_tools.rs:1-4`) | a new pair under the same gate |
| Ledger renderer and typed batch | `crates/bridge-tally-protocol/src/ledger_create.rs:19`; `src/tally/ledger_create_item.rs:39` (`MasterName`), `:265` (at most 10 per batch), `:286` | the pattern only; a stock item takes exactly one |

### Reads (verified unless marked)
- The `stock_items` kind of `masters` fetches `NAME, PARENT, GUID, MASTERID, ALTERID, HSNDETAILS.LIST, GSTDETAILS.LIST` (`crates/bridge-tally-protocol/src/native_masters.rs:121-125`). The renderer is at `:241` and the parser at `:408`; the row's `StockItem { hsn, gst }` detail is at `:280-285`. It counts `LANGUAGENAME.LIST` (`:589`) but returns no names. It fetches no `BASEUNITS`, `GSTAPPLICABLE` or `GSTTYPEOFSUPPLY`.
- The two lists are typed observations: `GstDetailsObservation` (`crates/bridge-tally-protocol/src/gst_details.rs:30`) and `HsnDetailsObservation` (`hsn_details.rs:28`), read from a stock item by `stock_tax_details.rs:91`. Absent, empty and zero are kept apart, and an entry with children the parser does not model says so (`other_content_skipped`, `gst_details.rs` module doc).
- 1381-M's own item read (`requests/items.utf16le.xml` on the sheet branch) fetched master's list plus `BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY, LANGUAGENAME.LIST`, and all three scalars arrived for the keyed item (**partial**). That read carries the company's GUID and stayed on the lab PC, so there is no fixture of it.
- An item's alias arrives as a second name entry in `LANGUAGENAME.LIST/NAME.LIST/NAME` (**partial**: `docs/tally/TALLY_PROTOCOL_REFERENCE_MEASUREMENTS_AND_OPEN_QUESTIONS.md` §12a.13, line 1597, on 7.1 Silver).
- Size: the masters read refuses a book it cannot hold as `masters_too_large` (`src/tally/runtime_masters.rs:98`, admitted at `:131`).

### Name folding (verified)
- `MasterNameIndex::collisions` (`crates/bridge-tally-core/src/master_name_index.rs:159`) matches a candidate against every name and alias under three keys (`crates/bridge-tally-core/src/master_binding.rs:2052`, `MasterNameKeys`).
- `MasterKind` is `Ledger | Group` only (`master_name_index.rs:40`). No stock-item code uses the index.

### Stock item create (verified)
- The only Create is the lab harness's `render_stock_item_xml` (`src/agent_lab_import.rs:741`, behind `lab-writes`). It writes the name, parent, unit, an HSN code and openings, and no GST lists. It is not a production builder.

### Docs (verified)
- Guide §3.6 (`docs/tally/IMPLEMENTATION_GUIDE.md:865`) and reference §9.4 (`TALLY_PROTOCOL_REFERENCE_WRITE_RESPONSES_AND_MASTERS.md:448`): a re-create is a silent Alter.
- §9.4b (`:511`): the ledger fold. Line 729 says the stock-item rule is unverified; 1381-M's R4a and R4b now answer case and a doubled inner space (**partial**).
- A sent GST list replaces the stored one on an Alter (`:502-503`, **partial**). 1381-M's R3 showed the same on a Create that reached an existing item (**partial**).

## 2. The smallest first slice that ends in a live check (P1)

**Slice 1: one item, copied from one named Goods sibling, end to end.** The sales ledger as a source, a Services sibling, a sibling that is not Taxable, and any second item stay out of slice 1. Each is refused by name with a typed code, never guessed.

1. **Read.** One candidate stock-item read: master's `stock_items` request with `BASEUNITS, GSTAPPLICABLE, GSTTYPEOFSUPPLY` and `LANGUAGENAME.LIST` added to its fetch list, nothing else changed. Row 1381-R1 (`lab/1381-r1-answers` at e870ba8c, S3) found three new elements in the answer, each with a value: `BASEUNITS`, `GSTAPPLICABLE` and `GSTTYPEOFSUPPLY`. `LANGUAGENAME.LIST` was already returned by the shipped read, although that read's fetch list does not name it: 1 element with 3 descendants in both reads. The candidate keeps all four names, because that is the request the row measured (**verified**, one book). Its parser returns every name entry, the three scalars, and the two typed lists. The book's units and stock groups are read through the shipped masters kinds. If those kinds do not return them, slice 1 refuses (unverified: not checked here).
2. **Admit before any window.**
   - Refuse when the name folds to any name entry of any item (§3, `ItemNameAbsent`). The fold is the full ledger fold of §9.4b, wider than measured for items, because refusing more is the safe side (unverified that items match by every key).
   - Refuse when the sibling is missing, is not one row, or lacks any of the six fields. The six are parent, unit, applicability, type of supply, the GST list and the HSN list. Also refuse when a list carries content the parser does not model, or when the type of supply is not `Goods`.
   - Refuse a parent or unit the book's read does not hold. That has to be checked here, because 1381-M R5 showed Tally refusing it in-band only (**partial**).
   - Refuse a book the read cannot hold (`masters_too_large`).
3. **Window.** It shows the name, and the six fields with their source ("copied from the item *S*"). The two lists are shown as they will be written, with every dated entry, state row, duty head and rate, never one "18%".
4. **Render.** A new builder writes one `STOCKITEM ACTION="Create"` from the shown draft only. Every value is escaped once as XML text from a typed field. **It never copies the sibling's XML.** 1381-M's `make_r2.py` copied the answer's own characters, which is right for a lab run and forbidden to Bridge (P7). So this is a new shape: the lab row below measures it. Row 1381-R1 b settled the form (`lab/1381-r1-answers` at e870ba8c, S5 and S6). With the three texts `GSTAPPLICABLE` ` Applicable`, `STATENAME` ` Any` and `GSTRATEVALUATIONTYPE` ` Not Applicable` written as read (b1, `requests/b1-create.utf16le.xml`), Tally answered CREATED 1 and every other counter 0. It stored no `GSTAPPLICABLE`, and its state-wise block held no state and no rates. With the same three written in Tally's marked form, `&#4;` before each (b2, `requests/b2-create.utf16le.xml`), the item read back equal to the sibling's in all six fields (**verified**, one book, one copy of each form). So the builder writes only the marked form. Those three fields are closed-set types whose only values are the three above, each rendered marked. Any other value is refused before a request is sent, and there is no plain path.
5. **Recheck and post.** `recheck_master_admission`'s logic runs: the marks must be unchanged since the window, and the name must still be absent from the book. Then the post goes through `post_approved_import`.
6. **Outcome** (§3, `StockItemCreateOutcome`):
   - success only on CREATED 1 with every other counter reported and 0;
   - ALTERED anything is a stop. The answer cannot say which item changed (1381-M R3: LASTMID 0, **partial**). So Bridge reads the items again and names the one whose ALTERID equals the new master mark. It also tells the person that the item's GST and HSN lists were replaced, and that restoring them is a manual alteration;
   - EXCEPTIONS with a LINEERROR is a typed refusal.
7. **Read back.** The new item is read with the same candidate read. Every field must equal the shown draft. An absent field is not a value, and a value Tally filled itself (1381-M R1's `Primary` parent) is a difference.
8. **No bill.** Nothing is posted under this approval (acceptance 6). The tool's answer ends at the read-back.

**The live check that ends slice 1:** on `BRIDGE STOCK CREATE LAB`, from a backup, using a lab build:
- `prepare` refuses a case variant and a doubled-space variant of an existing item;
- `prepare` and `post` create `SC Copied Goods` from `SC Sibling Goods`, and the read-back passes field for field;
- the restore is checked.

That repeats R2, R4a and R4b through Bridge's own code (unverified until run).

## 3. Typed states (P2, P3)

| type | constructed only by | makes impossible |
|---|---|---|
| `StockItemCatalogue` | a parse of the candidate read whose company GUID is the verified company's, on a book admitted by size | an existence check on a partial or foreign read |
| `ItemNameAbsent<'a>` | `StockItemCatalogue::name_absent(&'a self, name, marks)`, only when the index (with `MasterKind::StockItem` added) finds no collision against any name entry | a window, or a Create, for a name that exists, differs only by fold, or is an alias |
| `SiblingGst` | the one catalogue row the person named, when all six fields are present and both lists are `Entries` with no skipped content | a Tally default standing in for an absent field (1381-M R1) |
| `GoodsSibling` | `SiblingGst` whose type of supply is `Goods` and taxability is `Taxable` | slice 1 creating a Services or exempt item, which is unmeasured |
| `StockItemDraft` | `ItemNameAbsent` + `GoodsSibling` + a parent and unit found in the book's read | a Create for an absent group or unit |
| `ShownDraft` | the approval window, from a `StockItemDraft` | writing a field the window did not show (acceptance 3). The renderer takes only this |
| `StockItemCreateOutcome` | the counter parse: `Created` only on CREATED 1 with every other counter reported and 0; `AlteredExisting { item }`; `Refused { line_error }`; `Unreported` | acceptance 4's success while any counter is absent or non-zero |
| `ReadBackVerdict` | comparing a fresh catalogue row to the `ShownDraft`: `Equal` or `Differs { fields }`, where absent is never equal to a value | acceptance 5 passing over a field Tally filled |

`Created` alone does not make a `VerifiedItem`; only `Created` with `ReadBackVerdict::Equal` does. A later bill build may take a `VerifiedItem` as evidence. It is never built inside this approval.

## 4. The lab row slice 1 still needs

**One row, 1381-R1, on `BRIDGE STOCK CREATE LAB` at its BK0 state, from a backup and with a checked restore:**
- **a.** The candidate read (§2, step 1) through the build's own renderer, compared with 1381-M's kept item read: the same rows and fields. It decides whether the read is a shape a release may send.
- **b.** The candidate Create rendered by the build's builder from `SC Sibling Goods`'s typed values, sent once. EXPECT CREATED 1, with the read-back equal field for field to the sibling's. It decides whether re-rendering typed values stores what copying the bytes stored. This is the one new write shape.
- **c.** Key one alias on `SC Sibling Goods` on the screen, then read it: the alias appears as a second name entry (§12a.13 says so on 7.1, **partial**). It decides whether the alias refusal can rest on the name entries.

**Also owed before the fixture tests, and not a Tally row:** the company's identifier in the synthetic-uuid registry, and the kept reads (items, groups, units, marks) pushed as fixtures. #2380 and #2382 did the same for two other books.

**Not needed for slice 1, because slice 1 refuses these cases:** a Services sibling, a sibling that is not Taxable, the sales ledger as a source, and the folds beyond case and spacing. Each needs its own row before a later slice admits it. Gold is not owed, by the 5 Nov ruling.

## 5. Not decided here

- The refusal codes' names and the remedy text.
- Whether the new tool pair is two tools, or a `kind` on the ledger pair. A second pair keeps the ledger path's types closed, so it is recommended (unverified preference).
- The docs updates (P9): §9.4b's stock-item line, and a stock-item sentence next to guide §3.6. Both are pinned files, so that pull request needs an acknowledgement file.
