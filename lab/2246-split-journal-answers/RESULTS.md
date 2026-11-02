# Results: #2246 split journal on a real Store install, rows L1 and L2, 2 Nov 2026

**L1 passed every EXPECT. L2 did not: an inside build on a data folder the outside side created wrote its batch and journal line, then refused with `import_publication_recovery_required` and left an empty `imports\.build-publication`. That empty folder then refused every build from either side. It reproduced from a fresh folder. Nothing was posted; no box opened; the marks did not move.**

Sheet `lab/2246-split-journal` at f0533b80; build e9c780f3, 0.7.0, run 41184397066, artifact 13036317536, zip digest equal, exe sha256 equal before and after. BRIDGE CORPUS DENSE, TallyPrime 7.2 Silver, loaded alone. Windows 10 22H2, standard user at Medium level.

## Which side is which (steps 0 to 3) [verified]
Out's package family is none, and its new file is in the real folder. In's new file is seen from out only under the package's copy. `ka.txt` opens from both sides with the same volume and id. `R\J` 1074 B and `Q\J` 174198 B are different files, as the sheet needs.

## L1: the two journals already here [verified]
| row | side | outcome | seconds |
|---|---|---|---|
| L1a report | out | exit 0; `package_copies` lists `Q` | - |
| L1b report | in | exit 0; `package_copies` lists `Q` | - |
| L1c build | in | batch built, `result.path` under `Q`, `path_state` resolved | 10.63 |
| L1d build | out | `import_journal_split`, `other_journal_holds_batches`, no path | 1.32 |
| L1d post | out | `import_journal_split`, the same cause, no box | 0.149 |

The four paths named in #2246's body did not fire on either side. M 328 and V 29900 did not move. `R\J` stayed 1074 B and `R\imports` did not change. `Q\J` grew to 175278 B with L1c's batch.

## L2: a new folder, from nothing
| row | side | folder | outcome | seconds | journal bytes after |
|---|---|---|---|---|---|
| L2a | out | R2 | batch built, path resolved | 7.45 | 1076 |
| L2b | in | R2 | **EXPECT missed:** `import_publication_recovery_required`, cause null, `batch_id` present, no `result.path` | 7.49 | 2152 |
| L2c | out | R2 | the same code, no `batch_id` | 1.27 | 2152 |
| L2d | in | R2 | the same code, no `batch_id` | 1.25 | 2152 |
| R3a | out | fresh | batch built, path resolved | 7.17 | not read |
| R3b | in | fresh | the same code as L2b, `batch_id` present, no `result.path` | 7.05 | 2152 |

- **L2 outcome:** L2-A for the journal: it grew in place, the same file from both sides, and `Q2\J` stayed absent. The sheet's EXPECT of a batch at L2b, L2c and L2d was not met. [verified]
- **The batch is on record though the caller was refused:** after L2b, `imports` held both XML files and an empty `.build-publication`, and the journal held two lines with status `built`. In and out listed the same. [verified]
- **Reproduced once from a fresh folder**, the only repeat run. [verified]
- **Likely path:** at e9c780f3, `agent_import_persistence.rs`: the journal append succeeded, `remove_dir_all` of the transaction folder failed after emptying it, and `require_settled` then refuses any build while that folder exists. [hypothesis: read from the code and the folder left behind]
- **Why the folder stayed:** not established. L1c, inside on its own folder, finished clean, and so did R3a, outside on its own. The services on the PC are in the run lead's message, not here.

## Not run
L3 (needs the run lead's word). The second allowed repeat.

## Files
Each row's `answers/`, `client-log.txt`, `launch-env.json` and server error stream; `evidence/` (probe JSON, reports, the listing); `notes.txt`; `kept-local.txt` (originals by sha256); `SCREENSHOTS.txt` (the captures stay local); `substitutions.txt`; `SHA256.txt`. `whoami /groups` stays local; its level is above.

**Confidence:** one computer, one Store package, processes started from the lab shell and by WMI: PARTIAL for any other install. The inside process's package family reads `none` here; the redirection is what shows it is inside.
