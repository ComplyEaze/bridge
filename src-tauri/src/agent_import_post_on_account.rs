//! The On Account mark of an approval dialog (#1234): which ledgers a saved
//! batch records as approved to take entries with no bill allocation, how a
//! dialog line shows one, and the check every dialog text passes. A module of
//! its own so that nothing outside it can make a `ReviewText`: a dialog shape
//! cannot return its text from `review_text` without that check. The text is
//! a plain string again from there to the approval, so a path that built one
//! elsewhere is not stopped by the type.

use super::super::ImportLedgerLine;
use super::{line_break_names_are_marked, ReviewLedgerName};
use std::collections::BTreeSet;

/// What a dialog line shows before the quoted name of a ledger its batch
/// records as approved to take entries On Account (#1234). The mark stands
/// before the name, beside the amount it belongs to.
pub(super) const ON_ACCOUNT_MARK: &str = "On Account  ";
/// What the mark says, once, in place of the blank line under the marked
/// lines, so it costs a dialog no line. It claims what the saved batch
/// records and what the file holds, not where Tally puts the amount: the
/// ledger's flag is the build's reading, and the post reads it again only for
/// the ledgers that were not approved.
pub(in super::super) const ON_ACCOUNT_LEGEND: &str =
    "On Account: a bill-wise ledger when this batch was built. Its entries carry no bill allocation.";
/// A dialog text that would show an approved bill-wise ledger without its
/// mark. No shape here produces one; a shape added later that forgets the
/// mark is refused with this rather than shown.
pub(super) const ON_ACCOUNT_UNMARKED: &str = "import_review_on_account_unmarked";
/// A dialog text that does not pass `line_break_names_are_marked`: for one,
/// a text that would show a ledger name ending in a line break without its
/// mark or without the line that explains the mark. No shape here produces
/// one; it is refused as layout text rather than shown.
const LINE_BREAK_UNMARKED: &str = "import_review_layout_text";

/// The ledgers a saved batch records as approved to take entries On Account,
/// as its approval dialog marks them. The record is the build's (#1234 slice
/// 1); the person's click on a dialog that marks them is what covers them. A
/// batch saved before the record existed marks nothing, and is refused for
/// posting where its record is asked for (`admit_saved_voucher`).
pub(in super::super) struct OnAccountMarks<'a> {
    recorded: Option<BTreeSet<&'a str>>,
}

/// The text of an approval dialog as `review_text` returns it. Its field is
/// private to this module, so only `OnAccountMarks::seal` makes one.
pub(super) struct ReviewText(String);

impl ReviewText {
    pub(super) fn into_string(self) -> String {
        self.0
    }
}

impl<'a> OnAccountMarks<'a> {
    pub(in super::super) fn of(line: &'a ImportLedgerLine) -> Self {
        Self {
            recorded: line
                .on_account_approved
                .as_ref()
                .map(|approved| approved.iter().map(|item| item.ledger.as_str()).collect()),
        }
    }

    /// Whether the batch records exactly this ledger name.
    pub(in super::super) fn marks(&self, ledger: &str) -> bool {
        self.recorded
            .as_ref()
            .is_some_and(|recorded| recorded.contains(ledger))
    }

    /// A ledger's name as a line that carries its amount shows it: the name
    /// as every line writes it, the mark before it when the batch records it.
    pub(super) fn named(&self, ledger: ReviewLedgerName<'_>) -> String {
        let shown = ledger.shown();
        if self.marks(ledger.as_str()) {
            format!("{ON_ACCOUNT_MARK}{shown}")
        } else {
            shown
        }
    }

    /// The line under a block of lines naming `ledgers`: the legend when one
    /// of them is marked, blank otherwise.
    pub(in super::super) fn legend<'b>(
        &self,
        ledgers: impl IntoIterator<Item = &'b str>,
    ) -> &'static str {
        if ledgers.into_iter().any(|ledger| self.marks(ledger)) {
            ON_ACCOUNT_LEGEND
        } else {
            ""
        }
    }

    /// The finished text of `line`'s dialog. Refused when a recorded ledger
    /// the batch names has no line that ends in two spaces, its marks and
    /// its quoted name, or the legend is missing; and when the text does not
    /// pass [`line_break_names_are_marked`] for the ledgers the batch names
    /// (bridge#626). It is a tripwire for a shape that forgets a mark or its
    /// line, not a second rendering: which lines carry them is pinned by the
    /// tests of each shape.
    pub(super) fn seal(
        &self,
        line: &ImportLedgerLine,
        preview: String,
    ) -> Result<ReviewText, String> {
        let mut marked = line
            .vouchers
            .iter()
            .flat_map(|voucher| voucher.entries.iter())
            .map(|entry| entry.ledger.as_str())
            .filter(|ledger| self.marks(ledger))
            .peekable();
        if marked.peek().is_some() && !preview.lines().any(|shown| shown == ON_ACCOUNT_LEGEND) {
            return Err(ON_ACCOUNT_UNMARKED.into());
        }
        if marked.any(|ledger| {
            ReviewLedgerName::parse(ledger).map_or(true, |ledger| {
                // The ledger's own line: two spaces, its marks, its name.
                let named = format!("  {}", self.named(ledger));
                !preview.lines().any(|shown| shown.ends_with(&named))
            })
        }) {
            return Err(ON_ACCOUNT_UNMARKED.into());
        }
        let named = line
            .vouchers
            .iter()
            .flat_map(|voucher| voucher.entries.iter())
            .filter_map(|entry| ReviewLedgerName::parse(&entry.ledger).ok());
        if !line_break_names_are_marked(&preview, named) {
            return Err(LINE_BREAK_UNMARKED.into());
        }
        Ok(ReviewText(preview))
    }
}

/// Whether `text` holds the opening of the legend, `\bon\s+account\s*:`, in
/// any case and anywhere in it. `\s` is any whitespace and `\b` sees a letter,
/// digit or `_` as a word character: "PAID ON ACCOUNT" and "Commission
/// account: 12" are ordinary text.
pub(super) fn reads_like_the_legend(text: &str) -> bool {
    let chars: Vec<char> = text.chars().map(|c| c.to_ascii_lowercase()).collect();
    let starts = |at: usize, word: &str| {
        chars[at..]
            .iter()
            .take(word.len())
            .copied()
            .eq(word.chars())
    };
    let past_spaces = |at: usize| at + chars[at..].iter().take_while(|c| c.is_whitespace()).count();
    (0..chars.len()).any(|at| {
        let word_before = at > 0 && (chars[at - 1].is_alphanumeric() || chars[at - 1] == '_');
        if word_before || !starts(at, "on") {
            return false;
        }
        let account = past_spaces(at + 2);
        account > at + 2
            && starts(account, "account")
            && chars.get(past_spaces(account + 7)) == Some(&':')
    })
}
