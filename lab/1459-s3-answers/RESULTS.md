# S3 (#1459) on the lab PC, 5 Nov 2026: ended at step 0c, K1b not run

The sitting ended at step 0c. The outside probe K1b did not run: the lab PC refused the probe file. Which side is which was not established, so no step after 0c and no row was run. No extension tool was called, nothing reached Tally after step 0's GET /status, nothing was posted and nothing was deleted.

## Setup readings

| step | reading | mark |
|---|---|---|
| 0a | Windows 10.0.19045, 22H2. BRIDGE CORPUS DENSE the one company loaded (by the owner); Gateway current date Tuesday, 31-Mar-2026; no dialog; no Educational mode. tasklist: one tally.exe, pid 1200, exit 0. netstat: 9001 LISTENING on the tally pid; 37 LISTENING lines; exit 0. GET /status: exit 0, 200, 51 bytes. | verified |
| 0b | Exactly one package of the desktop client: version 4.4212.0.0, SignatureKind Developer; the same version `lab/1584-msix-paths-answers` recorded. | verified |
| 0c | K1a (in) ran, exit 0; its output is kept on the lab PC by sha256, with a summary in `evidence/K1a-in-summary.txt`. K1b (out) not run: the lab PC refused the probe file (attempts at 14:48:00Z, 14:55:49Z and 15:00:51Z, in `notes.txt`). K1c not run. Classification: not established. The KA comparison needs K1b: not made. | K1a verified; K1b and K1c not run |
| 0d, 0d2, 0e | not run | not run |

O1 to O5: not run. Rows S3-1459-1 to S3-1459-4: not run, because the outside reading every row rests on is not established. C10: skipped, as nothing reached Tally after 0a; BRIDGE CORPUS DENSE stays loaded as the owner left it.

## What it decides

Nothing for acceptance (a) of #1459: the installed extension's `build_import_xml` was not called, so the shown path, its `path_state` and its opening from outside were not measured (U).

Confidence: one sitting on one computer, ended before the question was measured: not established.
