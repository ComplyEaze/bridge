//! A stand-in for the native approval dialog's child process, for the unit
//! tests of `confirm_with` and `confirm_review_with` in
//! `src/tally/approved_import.rs` only (#702). It is an example, so neither
//! `tauri build` nor `scripts/package-mcpb.mjs` ships it, and
//! `scripts/check-no-test-seam.mjs` proves its marker is absent from every
//! shipped executable.
//!
//! Its first act is to create an empty `<path>.ran`, where `<path>` is the
//! path it was started by (`argv[0]`, as the parent passed it). It then reads
//! its whole input and writes the answer scripted in `<path>.answer`: that
//! file's first line is the exit code, and the rest is written byte for byte,
//! with `{NONCE}` replaced by the nonce on the input's first line. Its last
//! act before exiting is to save the input it read to `<path>.ran`, so a test
//! that finds the input there knows the stand-in ran to its end.
//!
//! Each test row runs its own link to this executable with its own file, so
//! no row reads another's answer, and no environment variable is shared
//! between test threads. The path comes from `argv[0]`, not from
//! `current_exe()`: every link is the same file, and only the path the parent
//! started tells them apart.
use std::ffi::OsString;
use std::io::{Read, Write};

/// Present in this example's executable, and in no other.
const STANDIN_MARKER: &str = "bridge-test-approval-standin-1b1f4e18";

fn beside_me(extension: &str) -> OsString {
    let mut path = std::env::args_os().next().expect("argv[0]");
    path.push(extension);
    path
}

fn main() {
    std::hint::black_box(STANDIN_MARKER);
    std::fs::write(beside_me(".ran"), "").unwrap();
    let script = std::fs::read_to_string(beside_me(".answer")).expect("a scripted answer");
    let (code, answer) = script.split_once('\n').expect("an exit code line");
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let nonce = input.split('\n').next().unwrap_or_default();
    let code: i32 = code.parse().unwrap();
    let mut stdout = std::io::stdout();
    stdout
        .write_all(answer.replace("{NONCE}", nonce).as_bytes())
        .unwrap();
    stdout.flush().unwrap();
    std::fs::write(beside_me(".ran"), &input).unwrap();
    std::process::exit(code);
}
