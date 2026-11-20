# Provenance: `mirror-release-0.7.0.db.hex`

The hex text of an encrypted local mirror written by release 0.7.0, the last release before #2432's library bump. It is used to check that a mirror a user has from that release opens and migrates under the current library (#2432).

- **Written by** release 0.7.0 (tag `mcp-v0.7.0`, commit fcfb5ff69): `connect_encrypted`, then `TallyMirrorRepository::migrate`, the two calls the app makes when it opens the mirror. That release's `src-tauri/Cargo.lock` has `libsqlite3-sys` 0.37.0, and the build reported SQLite 3.50.4, SQLCipher `4.10.0 community` and journal mode `wal`. Its migration files are the same as master's at the time of writing.
- **Written on** macOS, with the CommonCrypto provider. It is opened in CI on Windows, under OpenSSL, and on macOS.
- **Content:** synthetic only. The key is 32 bytes of `0x5a`, a test key that protects nothing. Every mirror migration is applied: the release records versions 2 to 28 without 7 and 24, as this build does for a new mirror, and `tally_endpoints` holds one row, `endpoint-written-by-0-7-0` for `http://127.0.0.1:9000`. The file was vacuumed and the pool closed, so no `-wal` or `-shm` file remained.
- **The file:** stored as lowercase hex, 64 characters a line, because the merge gate admits no binary file without the owner's attestation. Decoded, it is 647,168 bytes, sha256 `dbabe86c236aa07566dd6ce8ee06f8b83cd36c0c02a7b90f225e99b0c411e1f5`. No plaintext occurs in it: a byte search for `SQLite format`, `CREATE TABLE`, `tally_endpoints`, `tally_schema_migrations`, the row's id and its address found none.

The committed text file, as the fixture gates check it:

| File | Bytes | SHA-256 |
| --- | ---: | --- |
| `mirror-release-0.7.0.db.hex` | 1,314,560 | `05cb61db961c3e89b9b9ce14b0a95bdc46352693774ba14d505a8c834f0f802c` |

Do not edit or re-wrap the `.hex` file. This directory is covered by the fixture byte-integrity gate.

## How it was made

The file is encrypted throughout, so nothing in it shows which library wrote it. The record of that is the run below. At `mcp-v0.7.0`, this test was appended to `src-tauri/src/db/encrypted_tests.rs`; it was not committed:

```rust
#[tokio::test]
#[ignore = "writes the upgrade fixture; run by hand only"]
async fn scratch_write_release_mirror_fixture() {
    let database = PathBuf::from(std::env::var("UPGRADE_FIXTURE_OUT").expect("UPGRADE_FIXTURE_OUT"));
    let pool = connect_encrypted(&database, Zeroizing::new(vec![0x5a_u8; MIRROR_KEY_BYTES]))
        .await
        .expect("create the mirror");
    crate::db::tally_mirror::TallyMirrorRepository::new(pool.clone())
        .migrate()
        .await
        .expect("migrate the mirror");
    sqlx::query(
        "INSERT INTO tally_endpoints (id, canonical_origin, created_at_unix_ms, last_observed_at_unix_ms) \
         VALUES ('endpoint-written-by-0-7-0', 'http://127.0.0.1:9000', 1, 2);",
    )
    .execute(&pool)
    .await
    .expect("insert");
    let versions: Vec<i64> =
        sqlx::query_scalar("SELECT version FROM tally_schema_migrations ORDER BY version;")
            .fetch_all(&pool)
            .await
            .unwrap();
    let sqlite: String = sqlx::query_scalar("SELECT sqlite_version();").fetch_one(&pool).await.unwrap();
    let cipher: String = sqlx::query_scalar("PRAGMA cipher_version;").fetch_one(&pool).await.unwrap();
    let journal: String = sqlx::query_scalar("PRAGMA journal_mode;").fetch_one(&pool).await.unwrap();
    sqlx::query("VACUUM;").execute(&pool).await.expect("vacuum");
    println!("FIXTURE sqlite={sqlite} cipher={cipher} journal={journal} versions={versions:?}");
    pool.close().await;
}
```

It was run from `src-tauri` with `UPGRADE_FIXTURE_OUT` set to a new file path:

```
cargo nextest run -p bridge --locked --run-ignored only -E 'test(scratch_write_release_mirror_fixture)' --no-capture
```

The run printed this line, from the library that wrote the file:

```
FIXTURE sqlite=3.50.4 cipher=4.10.0 community journal=wal versions=[2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 25, 26, 27, 28]
```

Running it again writes a different file, because SQLCipher picks a new random salt for each database. A rerun therefore gives a fresh release mirror to test with; it will not reproduce this file's sha256.
