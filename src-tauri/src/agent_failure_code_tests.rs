use super::*;

/// Each declared code is the string the failure site wrote before it was typed, byte for byte, so
/// typing a site changes nothing a caller or a receipt sees (#1942).
#[test]
fn each_declared_code_is_the_string_its_site_wrote() {
    let strings: Vec<&str> = GstUncertainCode::ALL
        .iter()
        .map(|code| code.as_str())
        .collect();
    assert_eq!(
        strings,
        [
            "gst_uncertain_window_too_large",
            "gst_uncertain_wide_read_short",
            "gst_uncertain_rows_over_limit",
        ]
    );
    for code in GstUncertainCode::ALL {
        assert_eq!(FailureCode::from(*code).as_str(), code.as_str());
    }
}

/// A tool failure made from a declared code carries that code's string.
#[test]
fn a_tool_failure_from_a_declared_code_carries_its_string() {
    for code in GstUncertainCode::ALL {
        assert_eq!(super::super::ToolFailure::from(*code).code, code.as_str());
    }
}
