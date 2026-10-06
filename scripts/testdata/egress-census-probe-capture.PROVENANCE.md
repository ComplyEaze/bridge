# Second clippy capture for the egress census tests

`egress-census-probe-capture.jsonl` is the unedited output of `cargo clippy --offline --lib
--message-format=json -- --force-warn clippy::disallowed_methods --force-warn clippy::disallowed_types`
on the crate below, with clippy 1.96.0 on macOS (arm64) on 2026-09-30 (the first capture is described in
`egress-census-clippy-capture.PROVENANCE.md`). The only change is that the absolute path of the scratch
directory is replaced by `/work/probe2`, so no local path is committed.

`main/clippy.toml`: `std::process::Command::new` and `std::process::Commnad::new` (a typo) as
`disallowed-methods`, `std::net::TcpListener` as `disallowed-types`. `main` depends on `ext` by path;
neither has a `[workspace]`, so `ext` is not a workspace member.

```rust
// ext/src/lib.rs
#[macro_export]
macro_rules! spawn_ext {
    () => { std::process::Command::new("z") };
}
pub fn nonmember_call() {
    let _ = std::process::Command::new("y");
}

// main/src/lib.rs
pub fn via_external_macro() { let _ = ext_macros::spawn_ext!(); }
pub fn calls_nonmember() { ext_macros::nonmember_call(); }
pub fn uses_listed_type() -> Option<std::net::TcpListener> { None }
```

What it shows (confidence: verified for this toolchain, on this crate):
- A listed call made by a dependency's own `macro_rules!` macro fires, and its outermost expansion
  site is the call in `main/src/lib.rs`.
- The `disallowed_types` message reads `use of a disallowed type ...` with code
  `clippy::disallowed_types`.
- The typo'd path is reported as a warning with no code, the text "`std::process::Commnad::new` does
  not refer to a reachable function", located in `clippy.toml`; nothing fires for it.
- `ext`'s own call inside `nonmember_call` is not reported at all: clippy does not lint a path
  dependency that is not a workspace member.

What it does not show: a proc-macro's expansion, or how the census behaves on Windows.

## Integrity digest

The row is the SHA-256 of the file's committed bytes, which
`scripts/check-fixture-provenance.mjs` checks (#838). A digest pins the bytes as
committed and claims nothing about where they came from: the Capture column
repeats only what this note says above.

| Fixture | Bytes | SHA-256 (integrity digest) | Capture |
| --- | ---: | --- | --- |
| `egress-census-probe-capture.jsonl` | 6,932 | `4fac918393c902145e65cab62de8232ac1729c467b7916de3bc6cae92d341e61` | clippy output with one path replaced, as above |
