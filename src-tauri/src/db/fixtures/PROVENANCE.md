# Provenance: `mirror-sqlcipher-4.10.0.db.hex`

The hex text of an encrypted local mirror written by the build before #2432's library bump. It is used to check that a mirror created by that build still opens under the current library (#2432).

- **Written by** `connect_encrypted` at commit b27ad20e6, the base of the pull request that moved `libsqlite3-sys` from 0.37.0 to 0.40.0 (that commit's `src-tauri/Cargo.lock` has `libsqlite3-sys` 0.37.0).
- **Written on** macOS, with the CommonCrypto provider. It is opened in CI on Windows, under OpenSSL, and on macOS.
- **Content:** synthetic only. The key is 32 bytes of `0x5a`, a test key that protects nothing. One table, `upgrade_proof (id INTEGER PRIMARY KEY, value TEXT NOT NULL)`, holds one row, `WRITTEN_BY_SQLCIPHER_4_10`.
- **The file:** stored as lowercase hex, 64 characters a line, because the merge gate admits no binary file without the owner's attestation. Decoded, it is 8,192 bytes, sha256 `deda6cd0485cbaf09e87afae267b15eb6921948c202967c04f0a33816fe236d8`. The pool was closed, so no `-wal` or `-shm` file remained. No plaintext occurs in it: a byte search for the row, the table name and `SQLite format` found none.

The committed text file, as the fixture gates check it:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `mirror-sqlcipher-4.10.0.db.hex` | 16,640 | `ebf1f8e0e8e8a4bbd9328349114542725ed4a6d2f4871d656e1fdf25a2de1873` |

Do not edit or re-wrap the `.hex` file. This directory is covered by the fixture byte-integrity gate.

## How it was made

The file is encrypted throughout, so nothing in it shows which library wrote it. The record of that is the run below. At b27ad20e6, this test was appended to `src-tauri/src/db/encrypted_tests.rs`; it was not committed:

```rust
#[tokio::test]
#[ignore = "writes the upgrade fixture; run by hand only"]
async fn scratch_write_upgrade_fixture() {
    let out = std::env::var("UPGRADE_FIXTURE_OUT").expect("UPGRADE_FIXTURE_OUT");
    let database = PathBuf::from(out);
    let pool = connect_encrypted(&database, Zeroizing::new(vec![0x5a_u8; MIRROR_KEY_BYTES]))
        .await
        .expect("create the mirror");
    sqlx::query("CREATE TABLE upgrade_proof (id INTEGER PRIMARY KEY, value TEXT NOT NULL);")
        .execute(&pool)
        .await
        .expect("create table");
    sqlx::query("INSERT INTO upgrade_proof (value) VALUES ('WRITTEN_BY_SQLCIPHER_4_10');")
        .execute(&pool)
        .await
        .expect("insert");
    let sqlite: String = sqlx::query_scalar("SELECT sqlite_version();").fetch_one(&pool).await.unwrap();
    let cipher: String = sqlx::query_scalar("PRAGMA cipher_version;").fetch_one(&pool).await.unwrap();
    let journal: String = sqlx::query_scalar("PRAGMA journal_mode;").fetch_one(&pool).await.unwrap();
    println!("FIXTURE sqlite={sqlite} cipher={cipher} journal={journal}");
    pool.close().await;
}
```

It was run from `src-tauri` with `UPGRADE_FIXTURE_OUT` set to a new file path:

```
cargo nextest run -p bridge --locked --run-ignored only -E 'test(scratch_write_upgrade_fixture)' --no-capture
```

The run printed this line, from the library that wrote the file:

```
FIXTURE sqlite=3.50.4 cipher=4.10.0 community journal=wal
```

Running it again writes a different file, because SQLCipher picks a new random salt for each database. A rerun therefore gives a fresh old-build mirror to test with; it will not reproduce this file's sha256.
