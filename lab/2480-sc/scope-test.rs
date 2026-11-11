// Row 2480-SC's scope print: master's LedgerScopeName::new (bridge-tally-protocol/src/ledger_scope.rs), the rule a name
// must pass before master writes it into CHILDOF, run on each name of the file SC_NAMES (id, number, name;
// tab-separated), printing each line with ADMITTED or REFUSED and the refusal's code appended. Scratch only: copied
// into bridge-tally-protocol/tests/ of a checkout of master, run once, never committed to master.
use bridge_tally_protocol::ledger_scope::LedgerScopeName;

#[test]
fn print_scope() {
    let path = std::env::var("SC_NAMES").expect("SC_NAMES names the input file");
    let input = std::fs::read_to_string(path).expect("the input file reads");
    let mut out = String::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let name = line.split('\t').nth(2).expect("three fields");
        let verdict = match LedgerScopeName::new(name) {
            Ok(admitted) => {
                assert_eq!(admitted.as_str(), name, "an admitted name is kept as given");
                "ADMITTED".to_string()
            }
            Err(error) => format!("REFUSED {}", error.code()),
        };
        out.push_str(&format!("{line}\t{verdict}\n"));
    }
    let target = std::env::var("SC_SCOPE").expect("SC_SCOPE names the output file");
    std::fs::write(target, out).expect("the output file writes");
}
