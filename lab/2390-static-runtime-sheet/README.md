# lab/2390-static-runtime-sheet

One sheet for #2390: the Windows `bridge_mcp.exe` built with the C runtime linked in. It reads the new file's import table and starts it to run `initialize` and `tools/list`. It needs the `.mcpb` that a CI run of the pull request produced, handed to the lab by the person who queues the sheet (see "The input"). No Tally, no install, no uninstall. See `static-runtime-run.md`. Results go to `reviews/ds1/lab-results/CRT2/`.
