# 1780-K: does the ledger-create box take the keyboard when it opens? Yes, on BUILD-K (Part of #1780)

Sheet `lab/1780-ledger-box-keyboard` at 1f273633, with the run lead's erratum (no key in Tally from B2 until C10, and none while a box is open). No ComplyEaze Bridge server ran; every box came from the product's dialog child, fed a fixed text, and was answered No by mouse. BUILD-K: #2171 head ab3e7ce7, CI run 40927702754, artifact 12921849410, exe sha256 3adf9bbffd248f4f03345402c4e8f35cfc142b3dc56c5d5d742d06ee86707ba8. BUILD-H: exe sha256 a946979561b61f51e6dab3539d3de98fd48c8f103f61d2e3abe787acf3240c23. BRIDGE BATCH LAB loaded alone (marks 87 / 224 at P0 and at C10). Windows 10.0.19045, TallyPrime 7.2 Silver, one monitor, 1920 x 1080 at 125 %. ForegroundLockTimeout 200,000 ms. 30 Oct 2026, 07:30 to 07:40 UTC.

| cell | box | build | timing | Fg at T0 | FIRST to the box in front | box-fit | exit |
|---|---|---|---|---|---|---|---|
| K1-H | ledger | H | late | Tally | **never** (Tally in front throughout) | none | 1 |
| K1-K1 | ledger | K | late | Tally | **0.174 s** (first block) | 2 lines, fits=true | 1 |
| K1-K2 | ledger | K | late | Tally | **0.146 s** (first block) | 1 line, fits=true | 1 |
| K2-H | ledger | H | early | Tally | **never** | none | 1 |
| K2-K | ledger | K | early | Tally | **0.169 s** (first block) | 1 line, fits=true | 1 |
| K3-L1 | post | K | late | Tally | 0.201 s | 1, fits=true | 1 |
| K3-L2 | post | K | late | Tally | 0.167 s | 1, fits=true | 1 |
| K3-E1 | post | K | early | Tally | 2.123 s | 1, fits=true | 1 |
| K3-E2 | post | K | early | Tally | 0.149 s | 1, fits=true | 1 |

Every box: extended style 0x00010108, topmost 1; stdout 0 bytes. Readings, by the sheet's table:
- **K1.** The change gives the ledger box the keyboard under the late-open timing, as its first line says: K1-H's box never took the foreground, and both K1-K boxes were in front at their first trace block.
- **K2.** With Tally `Fg` at T0 and the box opening about 1 s after an input to Tally, the old build's box (`MB_SETFOREGROUND` alone) still did not take the foreground. The new build's box took it early as well.
- **K3.** The post's box was in front within 3 s of `FIRST` in all four cells; K3-E1 after 2.1 s (row 1780-H's H1: 2.4 s).
- **K4 (K1-K1).** After the click into Tally at T0+12, Tally held the foreground at C+0 and C+6, and the ledger box stayed z 1 with topmost 1. System-modal topmost is kept and the helper does not raise the box again.
- **The ledger box is now measured, and fits:** box-fit box=(688,342,1252,764) work=(0,0,1920,1030) fits=true (row 1780-H's L2 read 713,370,1228,740 before it was measured). In K1-K1 the second box-fit line came after C+6's read.

FAIL conditions: none. No key, no Yes, no request to Tally beyond C2's and C10's reads, Tally's screen unchanged beyond focus, marks equal at C10, no leftover at B5 (before, after K2-K, end), B2 and B3 met.

Deviation: K1-H's read at T0+6 (rect, z-order, capture) was not taken. The first runner named its read function `R`, which PowerShell resolved to the built-in alias of Invoke-History. The trace, which decides K1-H, is whole. The runner was fixed before K1-K1 (both versions' sha256 are in `notes.txt`; v2 is under `scripts/`).

Not measured: the server path (`post_ledger_creates`), the review and release boxes, whether a key reaches the box (no key was pressed), more than one run of most cells, a second monitor, Windows 11, macOS.
