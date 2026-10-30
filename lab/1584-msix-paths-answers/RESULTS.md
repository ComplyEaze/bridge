# 1584-MSIX: an in-package build writes its import file into the package's private copy and shows that real path (I-A); the profile wire lock refuses across the boundary both ways, the AppData wire lock does not; OUT-FIRST stopped at X0 (port 9011 has history) (Part of #1584, Part of #1459)

Sheet `lab/1584-msix-paths` at 92bf3ac7 (README 2a20b592…, OUT-FIRST cb7af358…). 30 Oct 2026, 20:22 to 20:37 UTC.
- **Windows:** 10.0.19045 (22H2).
- **<desktop client> package:** one Store package, `<desktop client>` 4.4212.0.0, family `<family>`, SignatureKind Developer.
- **Build:** the published 0.7.0 `bridge_mcp.exe` (48d2f743…) and `pdfium.dll` (79d4676b…), copied from the 0.7.0 bundle already on the lab PC. No download, no install. <desktop client> Desktop was not quit.
- **Tally:** BRIDGE CORPUS DENSE, loaded alone.

## Package-family table

| process | started from | package family |
|---|---|---|
| lab shell probes (K1a, K1c, I2-in, O2-in, X0-in) | the lab session's shell | none |
| out probes (K1b, I2-out, O2-out, X0-out) | WMI `Win32_Process Create` | none |
| I1 client and its server | lab shell | none, none |
| O1 client and its server | WMI | none, none |
| c-in client and server; c-out client and server | lab shell; WMI | none, none; none, none |

**Out steps:** every out step used the WMI method. The Run box needs a system-key grant this session does not have, and none was asked for.

## K1 (known answers)
- **Out: decided on the first try.**
  - K1b's family is `none`.
  - Its new file `N2-out` landed in the real `LA\bridge-lab-1584`, not in the package copy.
  - The in side (K1c) sees it there.
- **In: family `none`, and redirected.** K1a's new file `N1-in` is seen from out only under `LA\Packages\<family>\LocalCache\Local\bridge-lab-1584`, never in the real folder. So the in processes carry no package identity, yet their new AppData files are redirected.
- **KA instrument: passes.** It is the same file from both sides, with final path `\\?\C:\bridge-lab\1584\ka.txt`.
- **Decision 2, what Windows reports inside for a redirected file:** opened by its logical name `LA\bridge-lab-1584\N1-in.txt`, all three forms name the **real package path**:
  - `final` (flags 0);
  - `opened` (flags 8);
  - `nt` (`\Device\HarddiskVolume3\…\Packages\<family>\LocalCache\Local\bridge-lab-1584\…`).
  It is one file with the copy out sees (same vol and id).
- **Known folders:** `LocalAppData-0x44000` equals `-0x4000` on both sides, so the redirection-target flag changes nothing here. `Profile-0x4000` equals `USERPROFILE` on both sides, so the P rows are not void.
- **Before any process of this item ran (K1b):**
  - R, the real `LA\Bridge\agent`, was **absent**.
  - Q, the package copy, held the journal (173,124 B), the import files, `K.lock` and `K.wire.lock`.
  - P held `K.lock` and `K.wire.lock`.

## I (in): outcome **I-A**
- **I0** (`--local-data-report --show-paths`, in) printed `folder_path` as the logical R path with `folder` present, but its counts are Q's: journal 173,124 B and 37 batches.
- **I1** build: deviation, option (a) on the recorded word. CORPUS DENSE holds no Indirect or Direct Expenses ledger, so the voucher was Cash Dr 1.00 / DN Sales Cr 1.00, Journal, 2025-04-01, `lab1584-i1`.
  - It answered `result.path` = `<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\imports\bridge-84ee0bff-….xml`, with `path_state` `resolved`.
- **I2:**
  - From out, only that Q name opens (the R name is absent), and its sha256 equals `result.sha256`.
  - From in, all three names open the one Q file.
  - **The path shown is the real package path, and a person outside can open it.**
  - The Q journal grew 173,124 → 174,198 B; R stayed absent.

## O (out): outcome **O-A**
- **O0** (out) printed the same `folder_path`, with `folder` **missing**. It counted only P's 2,370 lock files.
- **O1** (out, `lab1584-o1`, the same pair) built into the real R, which it created: `result.path` = `<home>\AppData\Local\Bridge\agent\imports\bridge-bbcc471d-….xml`, `resolved`.
  - From out, only the R name opens, and its sha256 matches.
  - From in, the R name also opens the real file (reads merge).
  - A new real journal was created (1,074 B).
  - O1's `local_data_report`: journal 1 batch.
- **The two sides now keep two journals:** in → Q (174,198 B), out → R (1,074 B).

## L (lock files, port 9001)
| file | before (K1b, out) | after O2 | same file both sides |
|---|---|---|---|
| `P-K.wire.lock` | present | present | **yes** (one vol and id) |
| `P-K.lock` | present | present | yes |
| `R-K.wire.lock` | absent | out: the real file (created by O1); in: opens the Q file | **no: two files** |
| `R-K.lock` | absent | out: absent; in: the Q file | n/a |
| `*.snapshot.lock` | absent everywhere | absent | n/a |

- This matches the sheet's EXPECT: the in side's AppData file did not exist at the real path before I1, so in and out each got their own.
- **Lease counts:** Q-leases 2,565 (2,522 `.lock`, 38 snapshot, 5 wire); P-leases 2,370 (2,330, 34, 5, 1 other); R-leases 0 before and 1 (`wire`) after O1. From in, R-leases is listed as Q's 2,565.

## C (contention, `tally_status`; holders `-Seconds 900`)
| row | holder | call | answer | s |
|---|---|---|---|---:|
| KH | out, scratch.lock; then in | none | out `HELD`; in `BUSY win32=33` | |
| C0 | none | c-in; c-out | both answered | 0.49; 0.19 |
| C1s | out, P wire | c-out | `tally_endpoint_busy` | 9.85 |
| C1 | out, P wire | c-in | **`tally_endpoint_busy`** | 10.12 |
| C1r | (released) | c-in | answered | 0.42 |
| C2s | in, P wire | c-in | `tally_endpoint_busy` | 10.11 |
| C2 | in, P wire | c-out | **`tally_endpoint_busy`** | 9.80 |
| C2r | (released) | c-out | answered | 0.08 |
| C3 | out, R wire (the real file) | c-in | **answered** (the in client's AppData lock is the Q file) | 0.49 |
| C4 | in, R wire as the in shell names it (the Q file) | c-out | **answered** | 0.08 |

- **No row was void under the sheet's rules.** No answer was `tally_endpoint_lock_unavailable`.
- **The profile lock holds across the boundary both ways. The AppData lock alone is split**, as #1584 described.

**C1s was run three times:**
- **First:** the lab's shell quoting broke the holder's path (OPEN_FAILED, no call).
- **Second:** the runner's read of the holder log collided with the holder writing `HELD`, so `HELD` was never logged and no call was sent. Stopped.
- **Third** (C1s-3): it ran. The runner had marked it void with a 1 s margin of its own; under the sheet's rule it stands. The margin was removed before C1.

All three are in `evidence/C-rows.txt` and `notes.txt`.

## E
Not run. This lab session does not know that the extension's tools run without a <desktop client> Desktop permission prompt, and the run lead said to skip E.

## OUT-FIRST: **stopped at X0**
- Nothing listens on 9011 (0 of 34 listeners).
- **The 9011 lock files already exist:** from out, `Q-K2.lock`, `Q-K2.wire.lock`, `P-K2.lock` and `P-K2.wire.lock` are present (last written 12 to 29 Oct); `R-K2.*` are absent.
- **The port has history**, so the item does not run; the sheet says another port needs a sheet change.
- Nothing was sent to 9011, and X1 to X4 were not run.
- Probe files are in `out-first/`.

## End
- All four clients were stopped (exit 0), and no `bridge_mcp.exe` of this item is running.
- Nothing was deleted. BRIDGE CORPUS DENSE is left loaded.
- **Left behind, never to be posted:** the `lab1584-i1` (Q) and `lab1584-o1` (R) batches and their journal lines, the K1 probe files, the real R folder and its wire lock (`kept-local.txt`).

## Privacy
- Every pushed file is a substituted copy (`substitutions.txt`):
  - vol and id pairs → `file=<first 12 hex digits of sha256(vol|id)>`;
  - the home folder in every form → `<home>`; the user name → `<user>`;
  - the package family → `<family>`;
  - the company number → `<company_number>`.
- The home folder's 8.3 form is the same as its long name.
