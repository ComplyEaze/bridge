# Provenance: `mirror-sqlcipher-4.10.0.db.hex`

The hex text of an encrypted local mirror written by the build before #2432's library bump. It is used to check that a mirror created by that release still opens under the current library (#2432).

- **Written by** `connect_encrypted` at commit b27ad20e6, the base of the pull request that moved `libsqlite3-sys` from 0.37.0 to 0.40.0. That build reported SQLite 3.50.4, SQLCipher `4.10.0 community` and journal mode `wal`.
- **Written on** macOS, with the CommonCrypto provider. It is opened in CI on Windows, under OpenSSL, and on macOS.
- **Content:** synthetic only. The key is 32 bytes of `0x5a`, a test key that protects nothing. One table, `upgrade_proof (id INTEGER PRIMARY KEY, value TEXT NOT NULL)`, holds one row, `WRITTEN_BY_SQLCIPHER_4_10`.
- **The file:** stored as lowercase hex, 64 characters a line, because the merge gate admits no binary file without the owner's attestation. Decoded, it is 8,192 bytes, sha256 `deda6cd0485cbaf09e87afae267b15eb6921948c202967c04f0a33816fe236d8`. The pool was closed, so no `-wal` or `-shm` file remained. No plaintext occurs in it: a byte search for the row, the table name and `SQLite format` found none.
- **How it was made:** a temporary test, not committed, created the table and row through the pool and closed it.
