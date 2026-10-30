@echo off
(echo LOCALAPPDATA=%LOCALAPPDATA%& echo USERPROFILE=%USERPROFILE%) > C:\bridge-lab\1738c\out\env-out.txt
mkdir C:\bridge-lab\1738c\copies\Q 2>nul
echo label Q > C:\bridge-lab\1738c\out\copy-Q.txt
if exist "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
  copy /b "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\Q\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-Q.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("C:\bridge-lab\1738c\copies\Q\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\Q\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-Q.txt)
if exist "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
  copy /b "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\Q\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-Q.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "<home>\AppData\Local\Packages\<family>\LocalCache\Local\Bridge\agent\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-Q.txt
  for %%A in ("C:\bridge-lab\1738c\copies\Q\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-Q.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\Q\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-Q.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-Q.txt)
mkdir C:\bridge-lab\1738c\copies\R 2>nul
echo label R > C:\bridge-lab\1738c\out\copy-R.txt
if exist "<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
  copy /b "<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\R\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-R.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "<home>\AppData\Local\Bridge\agent\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("C:\bridge-lab\1738c\copies\R\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\R\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-R.txt)
if exist "<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
  copy /b "<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\R\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-R.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "<home>\AppData\Local\Bridge\agent\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-R.txt
  for %%A in ("C:\bridge-lab\1738c\copies\R\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-R.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\R\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-R.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-R.txt)
mkdir C:\bridge-lab\1738c\copies\L1 2>nul
echo label L1 > C:\bridge-lab\1738c\out\copy-L1.txt
if exist "C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
  copy /b "C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L1\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L1.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\<folder-1>\egress-copy\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L1\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L1\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L1.txt)
if exist "C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
  copy /b "C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L1\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L1.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\<folder-1>\egress-copy\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L1.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L1\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L1.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L1\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L1.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L1.txt)
mkdir C:\bridge-lab\1738c\copies\L2 2>nul
echo label L2 > C:\bridge-lab\1738c\out\copy-L2.txt
if exist "C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
  copy /b "C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L2\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L2.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\<folder-2>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L2\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L2\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L2.txt)
if exist "C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
  copy /b "C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L2\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L2.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\<folder-2>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L2.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L2\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L2.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L2\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L2.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L2.txt)
mkdir C:\bridge-lab\1738c\copies\L3 2>nul
echo label L3 > C:\bridge-lab\1738c\out\copy-L3.txt
if exist "C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
  copy /b "C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L3\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L3.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\<folder-3>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L3\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L3\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L3.txt)
if exist "C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
  copy /b "C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L3\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L3.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\<folder-3>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L3.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L3\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L3.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L3\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L3.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L3.txt)
mkdir C:\bridge-lab\1738c\copies\L4 2>nul
echo label L4 > C:\bridge-lab\1738c\out\copy-L4.txt
if exist "C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
  copy /b "C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L4\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L4.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\<folder-4>\data-before\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L4\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L4\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L4.txt)
if exist "C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
  copy /b "C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L4\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L4.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\<folder-4>\data-before\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L4.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L4\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L4.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L4\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L4.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L4.txt)
mkdir C:\bridge-lab\1738c\copies\L5 2>nul
echo label L5 > C:\bridge-lab\1738c\out\copy-L5.txt
if exist "C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
  copy /b "C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L5\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L5.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\<folder-5>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L5\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L5\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L5.txt)
if exist "C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
  copy /b "C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L5\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L5.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\<folder-5>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L5.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L5\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L5.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L5\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L5.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L5.txt)
mkdir C:\bridge-lab\1738c\copies\L6 2>nul
echo label L6 > C:\bridge-lab\1738c\out\copy-L6.txt
if exist "C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
  copy /b "C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L6\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L6.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\<folder-6>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L6\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L6\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L6.txt)
if exist "C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
  copy /b "C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L6\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L6.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\<folder-6>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L6.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L6\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L6.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L6\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L6.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L6.txt)
mkdir C:\bridge-lab\1738c\copies\L7 2>nul
echo label L7 > C:\bridge-lab\1738c\out\copy-L7.txt
if exist "C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
  copy /b "C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L7\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L7.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\<folder-7>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L7\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L7\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L7.txt)
if exist "C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
  copy /b "C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L7\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L7.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\<folder-7>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L7.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L7\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L7.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L7\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L7.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L7.txt)
mkdir C:\bridge-lab\1738c\copies\L8 2>nul
echo label L8 > C:\bridge-lab\1738c\out\copy-L8.txt
if exist "C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
  copy /b "C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L8\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L8.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\<folder-8>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L8\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L8\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L8.txt)
if exist "C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
  copy /b "C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L8\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L8.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\<folder-8>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L8.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L8\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L8.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L8\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L8.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L8.txt)
mkdir C:\bridge-lab\1738c\copies\L9 2>nul
echo label L9 > C:\bridge-lab\1738c\out\copy-L9.txt
if exist "C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
  copy /b "C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L9\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L9.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\<folder-9>\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L9\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L9\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L9.txt)
if exist "C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
  copy /b "C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L9\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L9.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\<folder-9>\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L9.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L9\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L9.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L9\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L9.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L9.txt)
mkdir C:\bridge-lab\1738c\copies\L10 2>nul
echo label L10 > C:\bridge-lab\1738c\out\copy-L10.txt
if exist "C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
  copy /b "C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L10\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L10.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\<folder-10>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L10\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L10\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L10.txt)
if exist "C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
  copy /b "C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L10\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L10.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\<folder-10>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L10.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L10\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L10.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L10\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L10.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L10.txt)
mkdir C:\bridge-lab\1738c\copies\L11 2>nul
echo label L11 > C:\bridge-lab\1738c\out\copy-L11.txt
if exist "C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
  copy /b "C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L11\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L11.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\<folder-11>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L11\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L11\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L11.txt)
if exist "C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
  copy /b "C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L11\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L11.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\<folder-11>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L11.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L11\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L11.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L11\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L11.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L11.txt)
mkdir C:\bridge-lab\1738c\copies\L12 2>nul
echo label L12 > C:\bridge-lab\1738c\out\copy-L12.txt
if exist "C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
  copy /b "C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L12\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L12.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\<folder-12>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L12\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L12\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L12.txt)
if exist "C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
  copy /b "C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L12\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L12.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\<folder-12>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L12.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L12\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L12.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L12\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L12.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L12.txt)
mkdir C:\bridge-lab\1738c\copies\L13 2>nul
echo label L13 > C:\bridge-lab\1738c\out\copy-L13.txt
if exist "C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
  copy /b "C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L13\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L13.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\<folder-13>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L13\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L13\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L13.txt)
if exist "C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
  copy /b "C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L13\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L13.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\<folder-13>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L13.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L13\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L13.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L13\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L13.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L13.txt)
mkdir C:\bridge-lab\1738c\copies\L14 2>nul
echo label L14 > C:\bridge-lab\1738c\out\copy-L14.txt
if exist "C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
  copy /b "C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L14\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L14.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\<folder-14>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L14\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L14\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L14.txt)
if exist "C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
  copy /b "C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L14\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L14.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\<folder-14>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L14.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L14\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L14.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L14\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L14.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L14.txt)
mkdir C:\bridge-lab\1738c\copies\L15 2>nul
echo label L15 > C:\bridge-lab\1738c\out\copy-L15.txt
if exist "C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
  copy /b "C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L15\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L15.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\<folder-15>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L15\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L15\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L15.txt)
if exist "C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
  copy /b "C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L15\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L15.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\<folder-15>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L15.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L15\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L15.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L15\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L15.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L15.txt)
mkdir C:\bridge-lab\1738c\copies\L16 2>nul
echo label L16 > C:\bridge-lab\1738c\out\copy-L16.txt
if exist "C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
  copy /b "C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L16\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L16.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\<folder-16>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L16\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L16\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L16.txt)
if exist "C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
  copy /b "C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L16\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L16.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\<folder-16>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L16.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L16\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L16.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L16\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L16.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L16.txt)
mkdir C:\bridge-lab\1738c\copies\L17 2>nul
echo label L17 > C:\bridge-lab\1738c\out\copy-L17.txt
if exist "C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
  copy /b "C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L17\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L17.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\<folder-17>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L17\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L17\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L17.txt)
if exist "C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
  copy /b "C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L17\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L17.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\<folder-17>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L17.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L17\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L17.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L17\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L17.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L17.txt)
mkdir C:\bridge-lab\1738c\copies\L18 2>nul
echo label L18 > C:\bridge-lab\1738c\out\copy-L18.txt
if exist "C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
  copy /b "C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L18\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L18.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\<folder-18>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L18\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L18\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L18.txt)
if exist "C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
  copy /b "C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L18\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L18.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\<folder-18>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L18.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L18\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L18.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L18\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L18.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L18.txt)
mkdir C:\bridge-lab\1738c\copies\L19 2>nul
echo label L19 > C:\bridge-lab\1738c\out\copy-L19.txt
if exist "C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
  copy /b "C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L19\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L19.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\<folder-19>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L19\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L19\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L19.txt)
if exist "C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
  copy /b "C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L19\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L19.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\<folder-19>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L19.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L19\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L19.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L19\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L19.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L19.txt)
mkdir C:\bridge-lab\1738c\copies\L20 2>nul
echo label L20 > C:\bridge-lab\1738c\out\copy-L20.txt
if exist "C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
  copy /b "C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L20\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L20.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\<folder-20>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L20\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L20\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L20.txt)
if exist "C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
  copy /b "C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L20\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L20.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\<folder-20>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L20.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L20\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L20.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L20\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L20.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L20.txt)
mkdir C:\bridge-lab\1738c\copies\L21 2>nul
echo label L21 > C:\bridge-lab\1738c\out\copy-L21.txt
if exist "C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
  copy /b "C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L21\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L21.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\<folder-21>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L21\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L21\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L21.txt)
if exist "C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
  copy /b "C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L21\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L21.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\<folder-21>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L21.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L21\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L21.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L21\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L21.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L21.txt)
mkdir C:\bridge-lab\1738c\copies\L22 2>nul
echo label L22 > C:\bridge-lab\1738c\out\copy-L22.txt
if exist "C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
  copy /b "C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L22\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L22.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\<folder-22>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L22\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L22\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L22.txt)
if exist "C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
  copy /b "C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L22\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L22.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\<folder-22>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L22.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L22\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L22.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L22\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L22.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L22.txt)
mkdir C:\bridge-lab\1738c\copies\L23 2>nul
echo label L23 > C:\bridge-lab\1738c\out\copy-L23.txt
if exist "C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
  copy /b "C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L23\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L23.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\<folder-23>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L23\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L23\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L23.txt)
if exist "C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
  copy /b "C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L23\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L23.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\<folder-23>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L23.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L23\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L23.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L23\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L23.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L23.txt)
mkdir C:\bridge-lab\1738c\copies\L24 2>nul
echo label L24 > C:\bridge-lab\1738c\out\copy-L24.txt
if exist "C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
  copy /b "C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L24\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L24.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\<folder-24>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L24\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L24\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L24.txt)
if exist "C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
  copy /b "C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L24\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L24.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\<folder-24>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L24.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L24\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L24.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L24\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L24.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L24.txt)
mkdir C:\bridge-lab\1738c\copies\L25 2>nul
echo label L25 > C:\bridge-lab\1738c\out\copy-L25.txt
if exist "C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
  copy /b "C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L25\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L25.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\<folder-25>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L25\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L25\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L25.txt)
if exist "C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
  copy /b "C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L25\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L25.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\<folder-25>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L25.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L25\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L25.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L25\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L25.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L25.txt)
mkdir C:\bridge-lab\1738c\copies\L26 2>nul
echo label L26 > C:\bridge-lab\1738c\out\copy-L26.txt
if exist "C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
  copy /b "C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L26\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L26.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\<folder-26>\r1\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L26\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L26\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L26.txt)
if exist "C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
  copy /b "C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L26\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L26.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\<folder-26>\r1\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L26.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L26\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L26.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L26\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L26.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L26.txt)
mkdir C:\bridge-lab\1738c\copies\L27 2>nul
echo label L27 > C:\bridge-lab\1738c\out\copy-L27.txt
if exist "C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl" (
  echo == agent-import-ledger.jsonl source before >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
  copy /b "C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl" "C:\bridge-lab\1738c\copies\L27\agent-import-ledger.jsonl" >> C:\bridge-lab\1738c\out\copy-L27.txt 2>&1
  echo == agent-import-ledger.jsonl source after >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\<folder-27>\data\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
  echo == agent-import-ledger.jsonl copy >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L27\agent-import-ledger.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L27\agent-import-ledger.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
) else (echo == agent-import-ledger.jsonl absent >> C:\bridge-lab\1738c\out\copy-L27.txt)
if exist "C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl" (
  echo == agent-import-statement-skips.jsonl source before >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
  copy /b "C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl" "C:\bridge-lab\1738c\copies\L27\agent-import-statement-skips.jsonl" >> C:\bridge-lab\1738c\out\copy-L27.txt 2>&1
  echo == agent-import-statement-skips.jsonl source after >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\<folder-27>\data\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
  echo == agent-import-statement-skips.jsonl copy >> C:\bridge-lab\1738c\out\copy-L27.txt
  for %%A in ("C:\bridge-lab\1738c\copies\L27\agent-import-statement-skips.jsonl") do echo size %%~zA >> C:\bridge-lab\1738c\out\copy-L27.txt
  certutil -hashfile "C:\bridge-lab\1738c\copies\L27\agent-import-statement-skips.jsonl" SHA256 >> C:\bridge-lab\1738c\out\copy-L27.txt
) else (echo == agent-import-statement-skips.jsonl absent >> C:\bridge-lab\1738c\out\copy-L27.txt)
echo %ERRORLEVEL% > C:\bridge-lab\1738c\out\copy.exit
