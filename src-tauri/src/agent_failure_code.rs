//! Failure codes as typed values (#1942). A code a tool failure carries can reach the egress
//! receipt, so it is declared here, in a closed enum per module, and a [`FailureCode`] is made only
//! from one of them: no string a failure site writes can become one. A site not yet converted still
//! builds its code from a string (`From<String>` for `ToolFailure`); `failure_code_sites_tests`
//! counts those per file, and the count may only fall.

/// A failure code made from a declared variant. Its constructor is private to this module, so
/// every value is one of the strings the enums below declare.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct FailureCode(&'static str);

impl FailureCode {
    pub(crate) fn as_str(self) -> &'static str {
        self.0
    }
}

/// Declares a closed set of failure codes, each variant with its exact string, and the only ways
/// from it to a [`FailureCode`] and to a tool failure.
macro_rules! failure_codes {
    (
        $(#[$meta:meta])*
        pub(crate) enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $code:literal,)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum $name {
            $($(#[$variant_meta])* $variant,)+
        }

        impl $name {
            /// Every variant, for the test that pins each one's string.
            #[cfg(test)]
            pub(crate) const ALL: &'static [$name] = &[$($name::$variant,)+];

            pub(crate) fn as_str(self) -> &'static str {
                match self {
                    $($name::$variant => $code,)+
                }
            }
        }

        impl From<$name> for FailureCode {
            fn from(code: $name) -> Self {
                FailureCode(code.as_str())
            }
        }

        impl From<$name> for super::ToolFailure {
            fn from(code: $name) -> Self {
                FailureCode::from(code).into()
            }
        }
    };
}

failure_codes! {
    /// The refusals of `gst_uncertain_vouchers` itself.
    pub(crate) enum GstUncertainCode {
        /// The count read found more vouchers in the window than the tool reads.
        WindowTooLarge = "gst_uncertain_window_too_large",
        /// The wide read returned fewer vouchers than the count read found.
        WideReadShort = "gst_uncertain_wide_read_short",
        /// More vouchers to list than this server's row limit.
        RowsOverLimit = "gst_uncertain_rows_over_limit",
    }
}

#[cfg(test)]
#[path = "agent_failure_code_tests.rs"]
mod tests;
