# Row 2161-C v2: results (Windows 10, TallyPrime 7.2 Silver, BRIDGE BATCH LAB, BUILD-M at 187e882b)

**First line: STOPPED at B2, the idle read X0. The v2 `taskbar-uia.ps1` (the COM client) created the client (`client=com8`), then threw at line 124 before reading anything: `Cannot convert argument "uia", with value: "System.__ComObject", for "Buttons" to type "IUIAutomation"`. `uia-x0.txt` holds the `E` line only, with no `B` line and no `Z` line. The sheet's known answer is not met. No cell was run, no box was opened, and Q1 and Q2 are not measured.** (verified)

Sheet `lab/2161-c/RUN-SHEET.md` at a3cc26ec, a forward commit on cc441415 that changes only `taskbar-uia.ps1`, the sheet's note and B2's read check. SHA256.txt 12 of 12, by sha256 and size. v1's answers are one folder up (`lab/2161-c-answers/`). Every step and stamp is in `notes.txt` (UTC).

## Before the sitting

| check | result |
|---|---|
| B1 | BUILD-M as fetched for v1 today, re-hashed equal: zip 1940bd97… = the digest, `bridge_mcp.exe` 9bda7144…, `pdfium.dll` 79d4676b…. The runner is v1's, unchanged: sha256 ba0003a967f6f5b8aa35483196b63ec9906f0b9eff27526187bcc9c4ead2f627. (verified) |
| P0 | Before loading, the desktop app held the foreground. Tally was brought forward by `SwitchToThisWindow` on its own window. BRIDGE BATCH LAB was then loaded alone (its name typed as a filter, one row, Enter), GUID a30f23c3-4819-4334-96ab-ffa6fba70976. ALTVCHID 87, ALTMSTID 224, the answer byte-equal to v1's P0. One tally.exe on 9001, `GET /status` 200. (verified) |
| B4 | As v1: 1536x864 logical, 1920x1080 physical, 125 %, one monitor, Windows 10 build 19045, Tally maximised. (verified) |
| B5 before | Nothing left over. (verified) |

## B2, the idle read X0 (the stop)

Tally's title was clicked by the runner, and Tally was in front. One read set X took 9.48 s.
- **`uia-x0.txt`, whole:** `E 2026-11-04T06:37:23.537Z dpi_context=ok tray=0x20138 client=com8`. (verified)
- **The script's error output** (in the runner's timeline, `x0-timeline.txt`): `Cannot convert argument "uia", with value: "System.__ComObject", for "Buttons" to type "IUIAutomation": "Cannot convert the "System.__ComObject" value of type "System.__ComObject" to type "IUIAutomation"."`, at `taskbar-uia.ps1:124 char:16` (`foreach ($l in [TUia]::Buttons($uia, $tray))`), `MethodArgumentConversionInvalidCastArgument`. (verified)
- **`rect-x0.txt`, `cover-x0.txt` and the `x0` copies** were as in v1. (verified)
- **Hypothesis, unverified:** `[TUia]::Create` returns the CUIAutomation8 object to PowerShell, which wraps it as a plain `System.__ComObject`. PowerShell's binder cannot cast that to the C# `[ComImport]` `IUIAutomation` parameter of `Buttons`. Keeping the object inside C# would avoid that crossing. (hypothesis)

## Close (C10)

- **MARKS while loaded:** ALTVCHID 87, ALTMSTID 224, the answer byte-equal to P0's. (verified)
- **B5 at the end:** nothing left over. Every input re-hashed equal. (verified)
- **Before the shut:** the owner's browser held the foreground. Tally was brought forward by `SwitchToThisWindow` on its own window, and nothing was done to the browser. (verified)
- **The shut:** Company > Shut gave Select Company with none loaded. One tally.exe, 9001 LISTENING, `GET /status` 200. No XML request after the shut. (verified)

## Not measured

Q1 and Q2: no cell ran. Step HA did not run.
