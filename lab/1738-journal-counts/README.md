# Windows item 1738-COUNTS: how many transfer double bookings, not-created sends and skips posted against exist in the lab's import journals (Part of #1738)

Not for merge. **No Tally request, no write anywhere but a new scratch folder, no download, no owner act, Claude Desktop untouched.** The journals are copied, the copies are read by `count_journals.py`, and only counts and batch ids leave the PC. About 15 minutes.

## What it decides

#1738's design note (v2) recommends building no route until these states are shown to exist. On today's code an equal-reference transfer pair reaches Tally only from a journal written before #1625/#1714 or from two journals for one company, and `lab/1584-msix-paths-answers` (7c330007) measured two journals on this PC: the package copy's (`Q`, 37 batches before that sheet) and the real folder's (`R`, created by that sheet). This item counts, in each journal on the PC:
- **A**: Contra pairs from two statement banks and two lineages with the same company, endpoint and amount, each Contra decided per lineage (the newest build holding it, the newest statement entry naming it): pairs with an equal bank reference (the class whose build is refused), how many of those have both sides sent or found posted, and pairs where one side has no reference and the dates are equal (the class that is only listed);
- **B**: batches whose latest dispatch answer is the not-created shape (every counter present, created to cancelled 0, exceptions equal to the vouchers sent);
- **C**: skipped statement rows that a batch of another lineage, found posted and not rolled back, holds as a voucher, with no withdrawal recorded after the skip by a build of that lineage that was sent or found posted.

A batch counts as found posted by a verification record, or, in a journal written before those records, by its own build line's status.

Each count is meant as an over-count of what ComplyEaze Bridge itself reports: every condition the product applies and the script does not can only add candidates (`count_journals.py` says which). So **a zero is decisive**, and a non-zero is a lead to check, later, with `verify_import` on the batches it names. The script was checked against synthetic lines only (each count fires on a positive case and stays zero on its negative), not against a real journal.

## Files

`count_journals.py` (on this branch; check it against `SHA256.txt` before use). It reads the two files named on its command line, prints JSON, and writes nothing.

## Steps

- **0. Setup.** A new empty folder `C:\bridge-lab\1738c\` with `run\`, `copies\` and `out\`. Copy `count_journals.py` there; its sha256 must equal `SHA256.txt`'s, else STOP. The run coordinator's grant for this item is held (no Tally, so no LAB line is needed for Tally; record the time).
- **1. Copy the journals, from outside the package.** Inside the lab session's shell, the real folder's journal path resolves to the package copy (`lab/1584-msix-paths-answers`, O2-in), so every copy is made by an out step: the 1584 sheet's WMI method (`Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{CommandLine='cmd.exe /c C:\bridge-lab\1738c\run\copy.cmd'}`), with `copy.cmd` written as ASCII by the lab and recorded in `notes.txt`. For each of:
  - `Q`: `%LOCALAPPDATA%\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl` and `agent-import-statement-skips.jsonl` beside it;
  - `R`: `%LOCALAPPDATA%\Bridge\agent\agent-import-ledger.jsonl` and its skips file;
  - **every other** `agent-import-ledger.jsonl` under `C:\bridge-lab\` (the data folders earlier sheets set with `BRIDGE_AGENT_DATA_DIR`), each with its skips file, labelled `L1`, `L2`, … in `notes.txt` by its folder;
  
  the out step first writes `echo %LOCALAPPDATA% & echo %USERPROFILE%` to `out\env-out.txt` (a WMI-started process may not carry them), and every path in `copy.cmd` is absolute, with `Q` named by its full `...\Packages\<family>\LocalCache\Local\Bridge\agent` path, never through `%LOCALAPPDATA%`. It records the source's size and sha256, copies it with `copy /b` into `copies\<label>\`, then records the source's size and sha256 again and the copy's. All three hashes equal = the copy stands; a source that changed during the copy = copy once more; changed again = that journal is VOID (record it). A skips file that does not exist is recorded as absent (`if exist`), not as an error. A journal that does not exist is recorded as absent.
- **2. Count.** In the lab shell, for each label: `python -I C:\bridge-lab\1738c\count_journals.py C:\bridge-lab\1738c\copies\<label>\agent-import-ledger.jsonl C:\bridge-lab\1738c\copies\<label>\agent-import-statement-skips.jsonl > C:\bridge-lab\1738c\out\<label>.json` (omit the second path when the skips file was absent), with its exit code. Then the same with `--ids` added, into `kept\<label>-ids.json`: the batch ids each count names, which stay on the lab PC. An exit code other than 0, or `lines_not_parsed` above 0, is recorded as found; the count still stands for the lines it read.
- **End.** Nothing is deleted on the lab PC; the copies stay in `C:\bridge-lab\1738c\copies\`.

## Evidence

On a new branch `lab/1738-journal-counts-answers`, folder `lab/1738-journal-counts-answers/`:
- `RESULTS.md`: one table, one row per label (`Q`, `R`, `L1`…): present or absent, lines, batches, sent, found, A equal-reference pairs, A both sent or found, A missing-reference pairs, B, C. No batch id (they tie to real books; `kept\*-ids.json` stays on the PC and is listed in `kept-local.txt` by sha256);
- `out\*.json` (counts and batch ids only), `notes.txt` (UTC times, each command, each exit code, each size and sha256, the label of each folder), `run\copy.cmd`, `SHA256.txt`.
- **Never pushed:** the journals and their copies, which hold the companies' books, and the `--ids` outputs. In `notes.txt` each `L` label's folder is given only as `C:\bridge-lab\<folder-N>\...` (a folder name can carry a company's name), with the real names in `kept-local.txt` on the PC; the user's home becomes `<home>` and the package family `<family>`. Before the push, a case-insensitive count over every pushed file of the user name, the home folder's last part and each `L` folder's real name must be 0.
- Before the push: the pre-push scan; no `Co-Authored-By` or "Generated with" line in any commit. The SHA goes to the run coordinator, and the push waits for its yes.

## Not measured here

Whether ComplyEaze Bridge itself would report a counted pair (that needs `verify_import`, which reads Tally and writes a status record, so it is a separate item if A is above zero); journals on any other computer.
