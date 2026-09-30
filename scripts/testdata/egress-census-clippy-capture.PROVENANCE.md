# Clippy capture for the egress census tests

`egress-census-clippy-capture.jsonl` is the unedited output of `cargo clippy --offline --lib
--message-format=json -- --force-warn clippy::disallowed_methods --force-warn clippy::disallowed_types`
on the crate below, run with clippy 1.96.0 on macOS (arm64) on 2026-09-30. It is a real capture
(compiler artifacts, one `dead_code` warning, four egress-lint messages, `build-finished`), not
authored here.

The only change is that the absolute path of the scratch directory the crate was built in is
replaced by `/work/egress-census-capture` everywhere it appears, so no local path is committed.

`clippy.toml` beside the crate: `std::process::Command::new` and `std::net::TcpStream::connect` as
`disallowed-methods`.

```rust
// src/lib.rs
macro_rules! spawn {
    () => {
        std::process::Command::new("c")
    };
}

mod other;

pub fn plain() {
    let _ = std::process::Command::new("a");
}

#[expect(clippy::disallowed_methods, reason = "reviewed")]
pub fn reviewed() {
    let _ = std::net::TcpStream::connect("127.0.0.1:1");
}

#[allow(clippy::all)]
pub fn hidden() {
    let _ = std::process::Command::new("b");
}

// src/other.rs
pub fn via_macro() {
    let _ = spawn!();
}
```

What it shows: a plain call, a call under `#[expect]`, a call under `#[allow(clippy::all)]` and a
call made by a macro defined in one file and used in another all fire (four messages). For the
macro call the primary span is in `src/lib.rs` and its outermost expansion site is `src/other.rs`;
the census reports the expansion site.

What it does not show: paths on Windows, a workspace member below a `crates/` directory, or any of
the other methods in `src-tauri/clippy.toml`. The census tests derive Windows-style, `crates/`-member and
absolute path variants from it by string substitution; none was captured on Windows, so the Windows path shape
is authored, and the first Windows CI run is the first real evidence for it.

## Measurements on the same crate (not committed as captures)

Run on 2026-09-30 with clippy 1.96.0 on macOS (arm64), each in a fresh copy of the crate, counting
`clippy::disallowed_methods` messages in `--message-format=json` output (confidence: verified for
this crate and toolchain, not for the workspace):

| Change to the run | Firings |
| --- | --- |
| none (the capture above) | 4 |
| `RUSTFLAGS=--cap-lints allow` | 0 (no compiler message of any kind) |
| `-A clippy::all -A warnings` before the `--force-warn` flags | 4 |
| `[lints.clippy] all = "allow"` in Cargo.toml | 4 |
| `#![allow(warnings)]` and `#![allow(clippy::style)]` at the top of `src/lib.rs` | 4 |
| a passing `cargo clippy -- -D warnings` first (only the `#[expect]`ed call remains), then the census run, same profile | 1 (the run re-linted and reported) |
| the same, with `--release` on the census run | 1 |
