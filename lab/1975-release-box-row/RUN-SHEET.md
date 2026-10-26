# 1975 release box (#1975, Part of #1828): the invoice-stop release box shown on a real Windows screen, and closed with No

Not for merge. Written 26 Oct 2026 from pull request #1975's own body ("a Windows build showing the release box on a lab book is to follow"). `SHA256.txt` lists every file here with its bytes; check it before step 0.

On #1975's build, releasing an invoice stop asks in its own box: titled "ComplyEaze Bridge — release the invoice stop for this company?", with Yes, No and Cancel, No the default, topmost, and fit-checked like the post and review boxes. This row shows that box once and records what it shows. **Yes is never pressed.**

## Why no company, and no Tally
An invoice stop is set only by a Sales invoice that ComplyEaze Bridge sent to Tally and could not verify as posted. Sales is not among the voucher types any build may send (the qualified list holds Journal, Payment, Receipt and Contra, at #1975's head too), so no book can hold an invoice stop and `acknowledge_post_review` cannot reach the release box through Tally on any build today. The box is therefore shown the way the box-fit rows of #1879 showed theirs: through the executable's own release child mode (`bridge_mcp.exe --confirm-release`), fed a synthetic text on stdin. Nothing is sent to Tally, nothing is read from it, and no company is loaded or named in Tally.

## Rules
`LAB-COMMON.md` does not apply: there is no Tally step. These rules hold:
- No key is pressed on the PC while the box is open: it takes the keyboard focus, and Y or Enter on Yes would answer Yes. The box is answered with the MOUSE, on No, by the person the run coordinator names.
- Yes is never pressed. A token printed to stdout means Yes was pressed: report it at once.
- The display setting is not changed.

## Build
The `bridge_mcp` program of pull request #1975 at head `af0efdb511420a696314daaffb92ae1477cddf88`, built on the lab PC in a clean checkout of exactly that commit: `cargo build --locked --release --manifest-path src-tauri/Cargo.toml -p bridge --bin bridge_mcp`. If the run coordinator names a CI artifact of that head instead, verify its zip digest and take the exe from a copy of the `.mcpb` (its sha256 must equal `binary_sha256` in the artifact's `mcpb-smoke.json`). Record the commit and the exe's sha256 before step 1. If the pull request's head is no longer `af0efdb5` when the row starts, STOP and ask which commit to build.

## Files
- `inputs/release-probe.txt`: the child's input, `<nonce>\n1\n<displayed text>` with no final line break. The displayed text is what the release's parent writes (the statement line, the release answer text wrapped at 100 characters, then a release preview) with synthetic values: company "BRIDGE STOP PROBE", invoice "SYN-INV-0001", batch `bridge-00000000-0000-4000-8000-000000000002` (the synthetic sentinel of the merge gate's registry). 13 lines.
- `run-release-fit.ps1`: a copy of the box-fit item's `run-fit.ps1` (`lab/1879-box-fit-run-sheet` at 959d3c17) with only the child flag, the case list and its hash changed. It checks the input's hash, opens the box through the child mode with its stdout written to a file, prints the window rectangle against the work area (physical pixels), then the child's exit status, elapsed time and stdout. Run `-DryRun` first (prints, opens nothing).

## Steps
0. Check `SHA256.txt`. Record the exe's sha256, the screen's resolution and scale and the work area. Copy `inputs\` into a new folder and run nothing from this branch's folder.
1. `powershell -NoProfile -File run-release-fit.ps1 -Exe <exe> -InputDir <inputs folder> -Case release-probe -OutDir <new empty folder> -DryRun`. EXPECT it to print the case, the exe hash, the screen and the command, and to open nothing.
2. The same without `-DryRun`. The box opens. The person named by the run coordinator, using the mouse only:
   - RECORD the window's title exactly (EXPECT "ComplyEaze Bridge — release the invoice stop for this company?");
   - RECORD its text line by line against `inputs/release-probe.txt`'s displayed text (EXPECT the same lines);
   - RECORD the buttons and which is the default (EXPECT Yes, No, Cancel; No the default);
   - RECORD whether the box came to the front by itself and whether the whole box (title bar, every line, the buttons) is on the screen;
   - a screenshot of the whole screen, if the lab's screenshots show the box (else the person's words, recorded verbatim);
   - then click **No**.
3. RECORD the script's output whole: the window line (rectangle, work area, the four off-screen flags), `cmd exit=`, the elapsed milliseconds and the stdout file's size and sha256. EXPECT `cmd exit=1` (declined) and an empty stdout (0 bytes). `cmd exit=77` means the box was judged not to fit and was cancelled: a FINDING if the window line showed it wholly on the screen. Any token in stdout: report at once (Yes was pressed).

PASS = steps 0 to 3 recorded whole: the title, the text, the buttons and default, the fit and the exit. A title or default other than expected, a box not wholly on the screen, or an exit other than 1 is a FINDING for #1975: report it whole.

## Not measured by this row
The release reached through `acknowledge_post_review` on a real invoice stop (not reachable on any build until Sales may be sent); the macOS window; a release text too tall for the screen; Yes.

## Evidence
The script's output files and the stdout file by name, size and sha256, the person's recorded words, the build's commit and exe sha256, the screen setting, screenshots by name, size and sha256 (never pushed), and a notes file with UTC times. Evidence on a new branch `lab/1975-release-box-row-answers` with `SHA256.txt`; send the SHA to the run coordinator before pushing, and push only after the pre-push scan and the coordinator's yes.
