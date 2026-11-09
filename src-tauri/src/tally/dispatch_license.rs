//! The licence a send needs, sealed.
//!
//! `post_approved_import` sends only on a licence its `before_dispatch` closure
//! returned, and asks the licence whether it covers the request. Exactly two
//! licences exist, and both are defined in this file: `()`, for a voucher post
//! (its intent is appended inside the closure), and [`RecordedIntent`], for a
//! master post. `Sealed` is nameable only here, so no other module of the crate
//! can add a third; a source-text test holds the count at two.
//!
//! A [`RecordedIntent`] is made only by the master journal's `append_intent`,
//! which calls [`RecordedIntent::issue`] once the intent line is synced under the
//! exclusive lock; a source-text test holds that to one call. It names the batch
//! and the digest of the bytes the intent recorded, and it covers a request only
//! if that request is a master request bound to the same batch and carrying
//! exactly those bytes.
use super::{ApprovedImport, PostKind};

mod sealed {
    /// Nameable only inside `dispatch_license`: the seal.
    pub trait Sealed {}
}

/// What `before_dispatch` hands the send: the licence to send this request.
pub(crate) trait DispatchLicense: sealed::Sealed {
    /// Whether this licence covers sending `request`. Consumed: one licence,
    /// one send.
    fn licenses(self, request: &ApprovedImport) -> bool;
}

impl sealed::Sealed for () {}

impl DispatchLicense for () {
    fn licenses(self, request: &ApprovedImport) -> bool {
        request.post_kind() == PostKind::Vouchers
    }
}

/// An intent that is in the master journal, synced, under the exclusive lock. The
/// send it licenses takes this by value: no master send is built without one.
#[must_use = "the send consumes the recorded intent"]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RecordedIntent {
    batch_id: String,
    native_request_sha256: String,
}

impl RecordedIntent {
    /// Only the master journal's `append_intent` calls this, after the intent is
    /// recorded. It is `pub(crate)` because the journal is in another module
    /// tree; a source-text test holds the calls to that one.
    pub(crate) fn issue(batch_id: String, native_request_sha256: String) -> Self {
        Self {
            batch_id,
            native_request_sha256,
        }
    }

    /// A recorded intent for the runtime's own tests, which cannot reach the
    /// journal. Compiled under bare `cfg(test)` only.
    #[cfg(test)]
    pub(crate) fn issued_for_test(batch_id: &str, native_request_sha256: &str) -> Self {
        Self::issue(batch_id.to_string(), native_request_sha256.to_string())
    }

    #[cfg(test)]
    pub(crate) fn batch_id(&self) -> &str {
        &self.batch_id
    }

    #[cfg(test)]
    pub(crate) fn native_request_sha256(&self) -> &str {
        &self.native_request_sha256
    }
}

impl sealed::Sealed for RecordedIntent {}

impl DispatchLicense for RecordedIntent {
    fn licenses(self, request: &ApprovedImport) -> bool {
        request.post_kind() == PostKind::Masters
            && request.master_batch_id() == Some(self.batch_id.as_str())
            && request.xml_sha256() == self.native_request_sha256
    }
}

#[cfg(test)]
#[path = "dispatch_license_tests.rs"]
mod tests;
