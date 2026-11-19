@echo off
rem Lab helper (2214 export cut on the current app; new, not product code): lists, to evidence\<label>-listing.txt, the files
rem (not folders) of the current user's Downloads, Documents and Desktop folders, one level each, oldest write time first,
rem each followed by a line saying whether its dir succeeded (dir ok, or dir FAILED; read by && and ||, since an ERRORLEVEL expansion inside the for block is fixed when the block is parsed). Reads only. Usage: listdirs.cmd <label>
set O=<lab folder>\evidence\%1-listing.txt
echo listing %1 at %DATE% %TIME% > %O%
for %%D in ("%USERPROFILE%\Downloads" "%USERPROFILE%\Documents" "%USERPROFILE%\Desktop") do (
  echo == %%~D >> %O%
  dir /a-d /t:w /o:d "%%~D" >> %O% 2>&1 && (echo dir ok >> %O%) || (echo dir FAILED >> %O%)
)
echo done >> %O%
