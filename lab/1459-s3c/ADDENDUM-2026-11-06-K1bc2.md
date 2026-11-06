# #1459 sitting S3c: addendum K1bc2 (6 Nov 2026)

Not for merge. An addendum to `DEVIATION-2026-11-06.md` in this folder; everything there still holds. Nothing here has been run.

## What happened at K1bc

0ac, 0bc and K1ac passed. K1bc (out, N6 `d9cb26f9`) was started by the WMI create (return value 0); the lab PC then blocked the processes involved and refused the lab session's next command. No `K1bc-out.json` and no `K1bc-out.exit` were written, and no `d9cb26f9` file was found under X or the package copy. **K1bc is kept as not run.** The lab PC's settings were changed by the owner for this sitting, and the PC was restarted.

## The changes

1. **0ac2** is taken again, as 0ac, before K1bc2: Tally and the processes are read after the change (`evidence\0ac2-step0.txt`, `shots\0ac2-screen.png`).
2. **K1bc2** replaces K1bc: the same probe line on the out side, with step label `K1bc2` and a fresh nonce N7 = `f4c872bd`, drawn from a cryptographic source and unequal to every earlier nonce of #1459 (`341c7fc2`, `afc70663`, `db4823cc`, `6886521a`, `34e7f032`, `d9cb26f9`) and to every nonce file already under X (`107fe768`, `21f2fd7b`, `dbc59b37`, `5b713adc`). Files: `run\K1bc2-out.cmd`, `evidence\K1bc2-out.json`, `evidence\K1bc2-out.exit`.
3. K1cc, 0dc and 0d2c follow as in the deviation, K1cc reading N7's out file. K1bc's files (`run\K1bc-out.cmd`) stay as they are.

If the lab PC blocks or removes a file or a command again, that is recorded with its time, the step stops and is reported; no other name, folder, file type or launcher is tried.
