#!/bin/bash
# Lab helper (2119-L, not product code): step 2: sends ref-receivable once by c3-send-t.ps1, and takes a capture of Tally's window every 30 s while it runs.
cd <LAB_DIR> || exit 9
rm -f evidence/.ref-done
( i=0; while [ ! -f evidence/.ref-done ]; do sleep 1; i=$((i+1)); if [ $((i % 30)) -eq 0 ] && [ ! -f evidence/.ref-done ]; then python -I -X utf8 cap.py s2-during-$((i/30)) >> evidence/s2-during-captures.txt 2>&1; fi; done ) &
powershell -NoProfile -ExecutionPolicy Bypass -File <LAB_DIR>/c3-send-t.ps1 -Req "<LAB_DIR>\req\ref-receivable.utf16le.xml" -Sha 6520d94ec6d013e3e44d9af62ee9c9830a65085b4cb5b54710340df0d03c750f -AnsDir "<LAB_DIR>\ans" -Ans s2-ref-receivable.xml -Bound 20000000 -Log "<LAB_DIR>\evidence\send-log.txt"
echo "send exit $?"
touch evidence/.ref-done; wait
