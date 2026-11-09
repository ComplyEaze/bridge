# Lab helper (row 2162-RP, from row 2162-GY's shot.sh): one capture of the whole Tally window to shots/<name>.png, with its UTC time.
H=$(powershell -NoProfile -Command '[int64](Get-Process tally | Select-Object -First 1).MainWindowHandle'); powershell -NoProfile -File <lab folder>/rc070/tools/capwin.ps1 -Handle $H -Out "<LAB_DIR>/shots/$1.png" | tail -1; date -u +%FT%T.%3NZ
