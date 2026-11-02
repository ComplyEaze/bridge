# Results: #2246 L2 repeat, with #2272 in the build, 2 Nov 2026

**All four decisive points hold. A build the journal holds answers built. With the transaction folder held open, it still answers built, with a warning naming the OS error. A leftover empty folder clears at the next admission, and the next build goes through. No caller was told a recorded batch was refused. The first L2's fault did not recur on its own in the 3 unforced inside builds here (L2b, L2d, T1), so the warning was reached only by forcing it.**

Sheet `lab/2246-split-journal` at f0533b80, steps 8 to 12. Build: head af335415, run 41472315786, artifact 13156956680, zip digest equal, `bridge_mcp.exe` sha256 ca94ad0b8a12530abbce1fc609042d34a5b971064fe14a8513ea85b16518e3da, equal to the package's smoke file and before and after. BRIDGE CORPUS DENSE, TallyPrime 7.2 Silver, loaded alone. A backup was taken first; Tally was only read. Nothing was posted, no box opened. L3 was not run: it stays with the owner.

## The L2 steps, from a fresh folder
| row | side | outcome | seconds | journal after |
|---|---|---|---|---|
| L2a | out | built, path resolved | 7.72 | 1,080 B, 1 line |
| L2b | in | **built**, path resolved | 7.52 | 2,160 B, 2 lines |
| L2c | out | built | 7.63 | |
| L2d | in | built | 7.60 | 4,320 B, 4 lines |

The journal grew in place; the package copy of the folder stayed absent. No `.build-publication` remained after any of these. L2b's `local_data_report` reads `interrupted_write_folders` 0 (1 at the first L2) and batches 2. [verified]

This is the L2-A outcome of the first L2, now with no refusal after it.

## The decisive points, forced
| row | what was done | outcome | |
|---|---|---|---|
| T1 | an empty `.build-publication` made by hand in the lab folder, then an inside build | built; the folder gone | clears at admission |
| T2 | a holder opened the transaction folder without delete sharing, during an inside build, for 10 s | built, `isError` false, batch and path returned; a sixth warning; the folder present and empty after | built, with a warning |
| T3 | after the holder let go, an outside build | built; the folder gone | second build goes through |

T2's warning, whole: "This batch is built and on record, but its temporary folder could not be removed (os error 32). The next build or post clears the folder if it is empty; if anything is left in it, they are refused until a person checks it."

Error 32 is Windows' sharing violation, and it comes from this row's holder, so it shows the path works and says nothing about what held the folder in the first L2. That remains not established. Across this row 10 builds answered built and none returned `import_publication_recovery_required`. [verified]

## Split causes
Each on its own lab folder, by an outside client with posting on: a clean build first, then the lay-out, then a build and a post.

| cause | how | build | post |
|---|---|---|---|
| `other_journal_holds_batches` | a copy of a lab journal with batches at the package path | refused with this cause | refused, no box, `attempt_recorded` false |
| `other_journal_not_read` | a folder at that journal's name | refused with this cause | refused, no box, `attempt_recorded` false |
| `package_copy_journal_not_located` | a dangling junction at that name | refused with this cause | refused, no box, `attempt_recorded` false |
| `journal_not_located` | a dangling junction at this side's own journal | **did not stick:** refused `import_ledger_unavailable`, no cause; which check answered was not read | no batch to post |
| `package_copies_not_listed` | **not attempted:** it needs the Packages folder under the local app data folder unreadable, a change to a real system folder | - | - |

A first try at `journal_not_located` laid its junction from inside the package; Windows put it in the package copy, so that row (`s8`) answered `other_journal_not_read`. That was the lay-out's error, not the product's. It was repeated once from outside (`s9`).

**What each refusal says to do:**
- **A build, any cause:** `import_journal_split` with a `remediation` naming both groups below, ending "Apart from that one try, do not retry here." It adds: do not move, merge or delete either folder; uninstalling, resetting or repairing that Store app removes its copy and the journal in it; `bridge_mcp --local-data-report --show-paths` outside <desktop client> lists the package copies.
- **`other_journal_holds_batches`, `other_journal_not_read`:** the post's message: nothing was posted; post from the Store app's copy instead; that copy does not see this saved Journal or anything posted from here, so check its rows in Tally first; do not move or delete either folder.
- **`package_copy_journal_not_located`** (and by the code `journal_not_located`, `package_copies_not_listed`): the post's message: could not tell whether a package keeps a journal; restart this computer and post again; if refused again, stop posting on this computer and keep the batch; do not move or delete either folder.

The two post messages differ as #2246 intends: one names a copy to post from, the other names none. The messages for the two causes not reached are read from the code, not seen.

## Left behind
Folders `BridgeLab2246-R4` to `R9`, real and package sides, with their batches and journal lines, never posted; three dangling junctions, in rows s7, s8 and s9. The first L2's folders are untouched. Nothing was deleted. Company shut; none loaded; one tally.exe; `GET /status` 200.

**Confidence:** one computer, one Store package, processes started from the lab shell and by WMI: PARTIAL. L3, the extension inside <desktop client>, is not measured. The split cases are lay-outs made by hand on lab folders, not a real install's history.
