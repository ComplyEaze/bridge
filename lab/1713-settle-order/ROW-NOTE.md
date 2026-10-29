# Row note for `lab/1713-settle-order` (the go-ahead record ORDER-ROWS.md asks for)

Not for merge. This note names what `ORDER-ROWS.md` leaves to the run coordinator. The go-ahead is given only when
every field below is filled; the two fields marked **RUN COORDINATOR** are not filled by this note, and until they
are, no row runs.

## The sheet

- **This branch's head before this note:** `5fdd02d66dfa0d96024e4baad1cb99feefb8fa6a` (the order rows, based on master
  `e9836b7e4`). The note itself is the commit after it.
- **Files of this sheet, by sha256** (equal to `SHA256.txt` at that head):

| file | sha256 |
| --- | --- |
| `ORDER-ROWS.md` | `526d319297fc221a6f8bc36199145537bb2696f377ef1e6c3c22039efe649992` |
| `LAB-COMMON.md` | `5370c6d288cd0f54a4ac547c513ec1648dd8bec0ea5a327d46b0de519a6369c6` |
| `order.py` | `858c7acf7b4c4634bfc1e1fc4d74cb51604611dae9f80fcc680361afb1bad6ec` |

## The earlier sheet

- **Head:** `811aed8b511b3af3cc855688e315ad6e9daf0361` of `lab/1713-abandoned-ledger-create`, the head its rows L2 to
  L5 ran at (its answers: `lab/1713-abandoned-ledger-create-answers` at `4a64d9ae`).
- **Files taken from it, by sha256** (equal to its `SHA256.txt` at that head):

| file | role here | sha256 |
| --- | --- | --- |
| `requests/busy-year.xml` | A | `ad2d4aa96d98041cd69cc52b4d2d280feeaab561a9558f4190e8b64db30ee749` |
| `requests/ledgers.xml` | C and the observation reads | `3eb31c35d3b911421742005af98e5e87453fbfc7dbc0d812ac1856761a3f280b` |
| `requests/marks.xml` | MARKS | `0464f624a2f7205a4f03668b2f6da7703589fe7bd590b4d22f2ec50c3a32542b` |
| `requests/l2-create.xml` | the template of every create B | `705095a3aa08612fffc4a5510ba3200c77e884089667facb8d6624f4ec5a9eff` |
| `read.py` | reading A, C and the observation reads by structure | `6b3ed7b4ac0da90c3e81daf64392636edf75e910f5a93ba77481763cd6f812ac` |

## THE COMPANY

- **BRIDGE ABANDON LAB**, company number 100096, GUID `5f4825c4-5d89-47fa-9872-6397cca3a591`, books from 1 April 2025:
  the synthetic company of the earlier sheet, created on Tally's screen for it. TallyPrime 7.2 Silver, licensed, one
  currency.
- **Its state:** as the earlier sheet's last backup BKE left it: 48,000 Journals, and the probe ledgers `Lab Probe L0`,
  `L1`, `L1B` and `L2` to `L5`. Its master mark read 216 after L5. None of this sheet's names (`Lab Order <row>`) is
  among them.
- **The busy read A, measured:** 186.645 s at the earlier sheet's M1 (`busy ok rows 48000`, 238,211,294 B; D = 187 s),
  above this sheet's floor of 120 s. Step 0's own D is measured again and governs.

## The build of R0

- **Commit:** master `e9836b7e40268014ddc5fa48758a5feb6e9efd3a`, this sheet's base.
- **CI run:** `ci.yml` push run 40929229503 on master (conclusion success; its Windows bundle smoke, seam positive
  control and native checks succeeded).
- **Artifact:** `unsigned-bundle-smoke-windows-latest`, id 12922333403, 57,155,003 B, sha256 (as GitHub reports it for
  the artifact archive) `b1c8557493ee72d68bbb45b9eec26575f18653ecb3ebc979650b29721f3290c6`.
- **`bridge_mcp.exe`:** its sha256 is recorded by the lab when the archive is unpacked, after the archive's own
  sha256 is checked against the value above (a mismatch = STOP). It is not known before the download.
- **Settings for R0:** posting on (`BRIDGE_AGENT_ENABLE_WRITES`) and ledger creation on
  (`BRIDGE_AGENT_ENABLE_MASTER_WRITES`), typed by the owner; batch posting not set; party masking absent; a fresh
  empty data folder `C:\bridge-lab\1713-order\r0-data\`.

## N1

**N1 is not run.** Its file must be one of the earlier sheet's bulk Journal imports that kept the gateway busy 60 s
or more in that sheet's records. None did. The record is `evidence/bulk/bulk-answers.txt` of the earlier sheet's
answers: of 192 imports, the longest was `bulk-083` at 6.825 s by curl's `time_total`, and every other was shorter.
RESULTS says so.

## Backups and the restart row

- **BK0 folder:** `C:\bridge-lab\1713-order\BK0\`, new and empty before step 2.
- **BK-pre folder (restart row only):** `C:\bridge-lab\1713-order\BK-pre\`, new and empty before that row.
- **Ending the one `tally.exe` (restart row only):** in PowerShell, `Stop-Process -Id <PID> -Force`, with the PID
  recorded at step 0, and no other means.

## Recorded by the run coordinator

- **RUN COORDINATOR:** the OK for this sheet's writes (one ledger per row) and for the download of the R0 build, with
  its date and where it is recorded.
- **RUN COORDINATOR:** the owner's word of 28 Oct 2026 for the optional restart row, as recorded.
