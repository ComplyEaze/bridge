# Lab helper (#2161-C v3 N2, not product code): the lab's side of cell n for the re-run the run lead ordered with no
# desktop-tool screenshot in the cell. When the runner asks for the copy before the cell (cmd/n-pre-ready), it takes a
# PrintWindow capture of Tally's window (capwin.ps1, shots/n-pre-printwindow.png) instead, then writes cmd/n-pre-done
# saying so. It never writes cmd/n-ha-shot-done (no control shot: the hold runs to its 12 s limit, as in N1). When the
# runner writes cmd/n-post-ready, it takes the same PrintWindow capture after the cell (shots/n-post-printwindow.png).
# Sends no key, clicks nothing. Usage: bash n2-words.sh
R=<LAB_DIR>; cd $R
cap() { h=$(powershell -NoProfile -Command "[int64](Get-Process tally | Where-Object { \$_.MainWindowTitle -like 'TallyPrime:9001*' } | Select-Object -First 1).MainWindowHandle" | tr -d '\r'); powershell -NoProfile -ExecutionPolicy Bypass -File $R/capwin.ps1 -Handle $h -Out "<LAB_DIR>\\shots\\$1.png" >/dev/null 2>&1; echo "$(date -u +%FT%T.%3NZ) capture shots/$1.png $(wc -c < shots/$1.png) B sha256 $(sha256sum shots/$1.png | cut -c1-64)"; }
for i in $(seq 1 600); do [ -f cmd/n-pre-ready ] && break; sleep 0.25; done
cap n-pre-printwindow
echo "$(date -u +%FT%T.%3NZ) no desktop-tool copy (the run lead's word for N2); a PrintWindow capture of Tally's window instead, shots/n-pre-printwindow.png" > cmd/n-pre-done
echo "$(date -u +%FT%T.%3NZ) wrote cmd/n-pre-done"
for i in $(seq 1 1200); do [ -f cmd/n-post-ready ] && break; sleep 0.25; done
cap n-post-printwindow
