fn main() {
    // The release date compiled into the program (#2391): one line, YYYY-MM-DD, written by
    // `scripts/next-version.mjs --apply`. A missing or unreadable file gives an empty date, which
    // the program reads as "no notice", never as an old build.
    println!("cargo:rerun-if-changed=release-date.txt");
    let date = std::fs::read_to_string("release-date.txt").unwrap_or_default();
    println!("cargo:rustc-env=BRIDGE_RELEASE_DATE={}", date.trim());
    tauri_build::build()
}
