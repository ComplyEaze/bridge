Where a built import file is shown on a Store install of the AI desktop app: one measured run

What this folder is: the files of one run on a Windows computer where the AI desktop app is installed as an MSIX package (<from the Microsoft Store / directly>). The run used the release-candidate build and a synthetic company. Nothing was posted to Tally and nothing was deleted.

Paths in these files: the user's home folder is written <home> and the Store package's folder is written <package>. The files kept on the lab computer hold the real strings.

What was asked: is the location that build_import_xml prints for the import file one that File Explorer opens, and what does it say when the file is somewhere else?

What was found (fill from the files, one line each; say "not measured" where a step did not run):
- Packaged (MSIX) install confirmed: <yes/no>; signed as <Store/Developer/Enterprise> (step S3).
- path_state in the answer: <resolved / as_is / unresolved>.
- The printed path starts with the verbatim prefix: <yes/no>.
- The printed path exists, seen from a shell outside the package: <True/False>.
- The file's SHA-256 equals the one in the answer: <yes/no>.
- The logical path (data folder as built) exists, seen from outside the package: <True/False>.
- The package path (<home>\AppData\Local\Packages\<package>\LocalCache\Local\Bridge\agent\imports\<file>) exists, seen from outside the package: <True/False>.
- The printed path equals the package path: <yes/no>.
- Which of the two import folders holds the file: <package / real / both / neither>.
- The real import folder existed before the build: <yes/no>.
- The same three checks from a shell inside the package: <as recorded, or not measured>.

What this run does not show: other computers, other versions of the app, files other than the import file, other files a tool names, the privacy policy's folder line, what an uninstall or reset of the app removes, and whether Tally's Import screen finds the file (not opened).

Files in this folder: each listed with its size and SHA-256 in MANIFEST.txt.
