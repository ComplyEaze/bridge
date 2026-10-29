// SPDX-License-Identifier: Apache-2.0
//! The rules' own year comes from `[meta]`, and a `[meta]` date that is not a plain date fails
//! the parse instead of being read as some other day.

use bridge_tax_audit::rules::{Rules, VENDORED};

const START: &str = "previous_year_start = 2025-04-01\n";

#[test]
fn a_year_date_that_is_not_a_plain_date_fails_the_parse() {
    assert_eq!(VENDORED.matches(START).count(), 1);
    for replacement in [
        "previous_year_start = \"2025-04-01\"\n",
        "previous_year_start = 2025-04-01T00:00:00\n",
        "",
    ] {
        let text = VENDORED.replacen(START, replacement, 1);
        assert!(Rules::parse(&text).is_err(), "{replacement:?}");
    }
    assert!(Rules::parse(VENDORED).is_ok());
}
