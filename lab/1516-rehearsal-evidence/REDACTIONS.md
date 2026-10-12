# REDACTIONS

Item 1 (#1516 batch rehearsal) text evidence from the Windows lab, 12 Oct 2026. Only .json and .txt files are here; screenshots and Tally backups stay on the lab PC.
Rules: the Windows user name is replaced with `<user>`, the host name with `<host>`, and the JSON-escaped user profile prefix with `<userprofile>`; nothing else is changed. Process logs and files carrying local account details are withheld.
`SHA256-originals.txt` hashes every original file (withheld ones included); `SHA256-redacted.txt` hashes the files in this folder as published.

| File | Replaced token kinds and counts | Note |
|---|---|---|
| p0-tally-screen-check.txt | WITHHELD | names a non-lab company and a local path; kept on the lab PC |
| password-file-acl.txt | WITHHELD | names the Windows account and host; kept on the lab PC |
| s0-rename.txt | WITHHELD | local user paths; kept on the lab PC |
| s0.2-result.txt | WITHHELD | process log, kept on the lab PC |
| s0.3-attempt1-terms-not-accepted.json | WITHHELD | carries a lab annotation (process wording), kept on the lab PC |
| s0.3-log.txt | WITHHELD | process log, kept on the lab PC |
| s04-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| s1-log.txt | WITHHELD | process log, kept on the lab PC |
| s3-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| s4-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| s5-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| s6a-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| s6b-build.json | Windows user name x1, user profile path x1 | bytes changed, so response_sha256 no longer matches this copy |
| times.txt | WITHHELD | process log, kept on the lab PC |

Every file not listed above is byte-identical to its original.
