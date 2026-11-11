// Row 2480-SK's key print: master's letters_and_digits_key (bridge-tally-core/src/master_binding.rs), run on each name
// of the file SK_NAMES (row, id, number, name; tab-separated), printing each line with the key appended. Scratch only:
// copied into bridge-tally-core/tests/ of a checkout of master, run once, never committed to master.
use bridge_tally_core::master_binding::letters_and_digits_key;

#[test]
fn print_keys() {
    let path = std::env::var("SK_NAMES").expect("SK_NAMES names the input file");
    let input = std::fs::read_to_string(path).expect("the input file reads");
    let mut out = String::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let name = line.split('\t').nth(3).expect("four fields");
        // A non-ASCII character of the key is written as its \u{..} escape, so that no word of
        // this file mixes Latin letters with another script; the escape is one to one.
        let key: String = letters_and_digits_key(name)
            .chars()
            .map(|c| if c.is_ascii() { c.to_string() } else { c.escape_unicode().to_string() })
            .collect();
        out.push_str(&format!("{line}\t{key}\n"));
    }
    let target = std::env::var("SK_KEYS").expect("SK_KEYS names the output file");
    std::fs::write(target, out).expect("the output file writes");
}
