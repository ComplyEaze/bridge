# 1738-COUNTS: no transfer double booking (A) and no skip posted against (C) in any of the lab's 29 import journals; two not-created sends (B), one in the package copy and one in a lab data folder (Part of #1738)

Sheet `lab/1738-journal-counts` at fa13e544 (README 45a2ae0b…, `count_journals.py` 4c58805a…, both equal to `SHA256.txt`). 30 Oct 2026, 21:22 to 21:24 UTC.
- **No Tally request.** Nothing on the lab PC was changed but the new folder `C:\bridge-lab\1738c`.
- **Copies:** every journal was copied by one WMI out step (`run\copy.cmd`). The step's environment line read `LOCALAPPDATA=<home>\AppData\Local` and `USERPROFILE=<home>`.
- **Every copy stands:** for each file, the source's size and sha256 before the copy, the source's after, and the copy's are equal (`notes.txt`). No journal was void.

| label | journal | lines | batches | sent | found | A equal-reference pairs | A both sent or found | A missing-reference same-day pairs | B not-created sends | C skips posted against |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| Q | present (174,198 B; skips absent) | 256 | 38 | 32 | 31 | 0 | 0 | 0 | **1** | 0 |
| R | present (1,074 B; skips absent) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L1 | present | 12 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| L2 | present | 9 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L3 | present | 12 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| L4 | present | 7 | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 |
| L5 | present | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| L6 | present | 8 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L7 | present | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L8 | present | 7 | 1 | 1 | 0 | 0 | 0 | 0 | **1** | 0 |
| L9 | present | 8 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L10 | present | 4 | 2 | 0 | 2 | 0 | 0 | 0 | 0 | 0 |
| L11 | present (skips present) | 3 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L12 | present (skips present) | 2 | 1 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| L13 | present (skips present) | 10 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L14 | present | 5 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 |
| L15 to L19, L21 | present (each) | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L20 | present | 3 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L22 | present | 21 | 2 | 2 | 2 | 0 | 0 | 0 | 0 | 0 |
| L23 | present | 8 | 1 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L24 | present | 4 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L25 | present | 2 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| L26 | present (skips present) | 11 | 2 | 1 | 1 | 0 | 0 | 0 | 0 | 0 |
| L27 | present | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

- **Parsing:** every line parsed (`lines_not_parsed` and `skip_lines_not_parsed` are 0 everywhere), and every count exited 0.
- **Contras:** only L12 (2), L13 (4) and L14 (4) hold Contras with statement facts (`contras_with_statement_facts`); none of them pairs.
- **A and C are zero in every journal.** Each count is meant as an over-count of what ComplyEaze Bridge reports, so on this PC no transfer double booking of either class, and no skipped row posted by another lineage, exists.
- **B is 1 in Q and 1 in L8:** a latest dispatch answer in the not-created shape. Both are leads for `verify_import`. Their batch ids stay on the lab PC (`kept\Q-ids.json`, `kept\L8-ids.json`, listed by sha256 in `kept-local.txt`).
- **Folder names:** each `L` folder is given as `C:\bridge-lab\<folder-N>\…`; the real names are in `kept\labels.txt` on the lab PC.
- **Copies:** the journals and their copies were not pushed.
