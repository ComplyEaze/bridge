//! Print the layout of a statement PDF as `extract_pages` reads it, with every
//! value masked: the measurement to take before writing a profile for a bank
//! this crate does not read yet.
//!
//! A profile's anchors, column bounds and end anchor are measured through
//! `extract_pages`, never through `pdftotext` (the two differ by about a line
//! per page). The statement is a customer's file that the person writing the
//! layout must not read, so this prints as little as it can of what a person,
//! an account or a transaction could be told from. Two facts shape every rule
//! below: a masked word's *width* identifies it (a name's letters, an amount's
//! digits), and a width can be worked out from any printed position next to a
//! masked word (the next word's left edge, a neighbour's right edge, a column
//! edge). So **no position of a masked word is printed on its line at all**.
//!
//! * a word is printed as typed only when it is exactly one of a closed list of
//!   statement words ([`ALLOWED`], with an optional colon) and no masked word
//!   comes before it on its line; after a masked word it prints as `L` (it may be
//!   part of a name, or a debit/credit flag). What a page repeats, or what looks
//!   like a label, is never a reason to print a word: a holder's name, address and
//!   account number repeat on every page;
//! * every other word is masked: every run of digits becomes `9+` (commas inside
//!   a number belong to it, so neither the number of digits nor the Indian or
//!   Western grouping shows; a date is `9+-9+-9+`), a run of capitals `A+`, a run
//!   of lower-case letters `a+` (one letter stays `A` or `a`), `@` and `/` become
//!   `#`, and any non-ASCII symbol `?`; other punctuation is kept. An amount (a
//!   number with a point and one or two places, with whatever sign, bracket,
//!   currency sign or `Cr`/`Dr` around it) becomes `9+.99` with those affixes
//!   masked by the same rules, so a debit and a credit look alike;
//! * the left edge of a word is printed only on a line of nothing but listed words
//!   (a header row): on any other line it can depend on the width of a masked word
//!   or of a centred or right-aligned line. A right edge is never printed on a
//!   line, and y is printed in whole points;
//! * where the amount columns end is given per page, with no line attached and no
//!   count: the whole points at which three or more amounts end, at least a quarter
//!   of the page's amounts, and which begin at three or more different points
//!   (a right-aligned column). Amounts that all begin at one point and end at one
//!   have one width, which would give their digit count, so such an edge is not
//!   printed. Where a text column begins is read from its header word's edge;
//! * a line whose masked words repeat a line already printed three times is
//!   not printed again; the number left out is counted at the end;
//! * pages 1 to 3 and the last page line by line, the others only at their top
//!   and bottom.
//!
//! The output is **still a fingerprint of a statement** (line counts, shapes and
//! the column edges). The output file is created readable by its owner only (on
//! Unix) and is never overwritten (a file that exists is refused). It stays with
//! the person who ran this: it is not for an issue, a pull request, a
//! transcript or any public place. What it cannot show is a bank's own header
//! wording that is not on the list; add the words that the output shows as masked
//! headers to the list in a later change, one bank at a time.
//!
//! ```text
//! cargo run --example layout_probe -- \
//!     --pdfium /path/to/libpdfium.dylib --pdf statement.pdf \
//!     [--password-file statement.password] --out layout.txt
//! ```

use bridge_bank_statement::geometry::{lines, Line, Page};
use bridge_bank_statement::pdf::{engine, extract_pages};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

const TOP_LINES: usize = 12;
const BOTTOM_LINES: usize = 3;
const FULL_PAGES: usize = 3;

struct Arguments {
    pdfium: PathBuf,
    pdf: PathBuf,
    password_file: Option<PathBuf>,
    out: PathBuf,
}

fn arguments() -> Result<Arguments, String> {
    let mut named: BTreeMap<String, String> = BTreeMap::new();
    // No argument is ever echoed: a misplaced token may be the password.
    // Read as text without ever echoing it: a non-text argument is refused by position.
    let collected: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|argument| {
            argument
                .into_string()
                .map_err(|_| "an argument is not valid text".to_string())
        })
        .collect::<Result<_, _>>()?;
    let mut args = collected.into_iter().enumerate();
    while let Some((position, key)) = args.next() {
        let Some(name) = key
            .strip_prefix("--")
            .filter(|name| ["pdfium", "pdf", "password-file", "out"].contains(name))
        else {
            return Err(format!(
                "argument {} is not an option of this tool",
                position + 1
            ));
        };
        let (_, value) = args
            .next()
            .ok_or_else(|| format!("argument {} needs a value", position + 1))?;
        named.insert(name.to_string(), value);
    }
    let mut need = |name: &str| {
        named
            .remove(name)
            .ok_or_else(|| format!("--{name} is required"))
    };
    let pdfium = need("pdfium")?.into();
    let pdf = need("pdf")?.into();
    let out = need("out")?.into();
    Ok(Arguments {
        pdfium,
        pdf,
        out,
        password_file: named.remove("password-file").map(PathBuf::from),
    })
}

fn read_password(path: &PathBuf) -> Result<String, String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = std::fs::metadata(path)
            .map_err(|_| "password file unreadable")?
            .permissions()
            .mode();
        if mode & 0o077 != 0 {
            return Err("password file must not be readable by group or others (chmod 600)".into());
        }
    }
    let text = std::fs::read_to_string(path).map_err(|_| "password file unreadable")?;
    Ok(text.trim_end_matches(['\r', '\n']).to_string())
}

/// The only words printed as typed: statement vocabulary that no person,
/// account or transaction owns. Matched whole, case-sensitively, with or
/// without a trailing colon.
const ALLOWED: &[&str] = &[
    "Date",
    "Value",
    "Txn",
    "Tran",
    "Transaction",
    "Narration",
    "Description",
    "Particulars",
    "Details",
    "Remarks",
    "Chq",
    "Cheque",
    "Ref",
    "No",
    "No.",
    "Reference",
    "Withdrawal",
    "Withdrawals",
    "Debit",
    "Debits",
    "Deposit",
    "Deposits",
    "Credit",
    "Credits",
    "Balance",
    "Opening",
    "Closing",
    "Total",
    "Amount",
    "Dr",
    "Cr",
    "INR",
    "Page",
    "of",
    "Statement",
    "Account",
    "Summary",
    "Period",
    "From",
    "To",
    "Brought",
    "Carried",
    "Forward",
    "B/F",
    "C/F",
];

fn allowed(text: &str) -> bool {
    ALLOWED.contains(&text.strip_suffix(':').unwrap_or(text))
}

/// The masked form of an amount: a number with a point and one or two places, with
/// anything around it (a sign, brackets, a currency sign, `Cr`, `Dr`, `Rs.`). The
/// affixes are masked by the general rules, so a debit and a credit look alike. A
/// number with no point is not an amount (an account number, a phone number, a page
/// number, a year) and goes through the digit rule.
fn amount(text: &str) -> Option<String> {
    let first = text.find(|c: char| c.is_ascii_digit())?;
    let last = text.rfind(|c: char| c.is_ascii_digit())?;
    let (prefix, core, suffix) = (&text[..first], &text[first..=last], &text[last + 1..]);
    let (whole, places) = core.split_once('.')?;
    let ok = whole.chars().all(|c| c.is_ascii_digit() || c == ',')
        && (1..=2).contains(&places.len())
        && places.chars().all(|c| c.is_ascii_digit());
    ok.then(|| format!("{}9+.99{}", mask_runs(prefix), mask_runs(suffix)))
}

fn mask(text: &str) -> String {
    if allowed(text) {
        return text.to_string();
    }
    if let Some(masked) = amount(text) {
        return masked;
    }
    mask_runs(text)
}

fn mask_runs(text: &str) -> String {
    #[derive(PartialEq, Clone, Copy)]
    enum Run {
        Digit,
        Upper,
        Lower,
    }
    fn flush(out: &mut String, run: Option<Run>, len: usize) {
        match (run, len) {
            (Some(Run::Digit), _) => out.push_str("9+"),
            (Some(Run::Upper), 1) => out.push('A'),
            (Some(Run::Upper), _) => out.push_str("A+"),
            (Some(Run::Lower), 1) => out.push('a'),
            (Some(Run::Lower), _) => out.push_str("a+"),
            (None, _) => {}
        }
    }
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let (mut run, mut len) = (None, 0_usize);
    for (position, &character) in chars.iter().enumerate() {
        // A comma between two digits is part of the number, so grouping does not show.
        if character == ','
            && run == Some(Run::Digit)
            && chars
                .get(position + 1)
                .is_some_and(|next| next.is_numeric())
        {
            continue;
        }
        let kind = if character.is_numeric() {
            Some(Run::Digit)
        } else if character.is_uppercase() {
            Some(Run::Upper)
        } else if character.is_alphabetic() {
            Some(Run::Lower)
        } else {
            None
        };
        if kind.is_some() && kind == run {
            len += 1;
            continue;
        }
        flush(&mut out, run, len);
        (run, len) = (kind, usize::from(kind.is_some()));
        if kind.is_none() {
            out.push(match character {
                '@' | '/' => '#',
                c if c.is_ascii_punctuation() => c,
                _ => '?',
            });
        }
    }
    flush(&mut out, run, len);
    out
}

/// What a word is, for deciding what is printed about it.
#[derive(Debug, PartialEq, Eq)]
enum Kind {
    Listed,
    Amount,
    Other,
}

fn kind(text: &str) -> Kind {
    if allowed(text) {
        Kind::Listed
    } else if amount(text).is_some() {
        Kind::Amount
    } else {
        Kind::Other
    }
}

fn print_line(out: &mut String, label: &str, line: &Line) {
    // One decimal would show whether a masked word has ascenders or descenders.
    let _ = write!(out, "  {label} y={:.0} |", line.y);
    let listed = |word: &bridge_bank_statement::geometry::Word| kind(&word.text) == Kind::Listed;
    // Only a line of nothing but listed words (a header row) keeps its edges: on a line with
    // a masked word, a left edge can depend on a centred or right-aligned line's width.
    let header_row = line.words.iter().all(listed);
    let mut masked_before = false;
    for word in &line.words {
        if listed(word) && !masked_before {
            if header_row {
                let _ = write!(out, " [{:.0}]", word.x0);
            }
            let _ = write!(out, " {}", word.text);
        } else if listed(word) {
            // A listed word after a masked one may be part of a name or a debit/credit flag.
            out.push_str(" L");
        } else {
            masked_before = true;
            let _ = write!(out, " {}", mask(&word.text));
        }
    }
    out.push('\n');
}

/// The masked words of a line, without positions: what "the same line" means for the repeat cap.
fn shape(line: &Line) -> String {
    let mut masked_before = false;
    let mut words = Vec::new();
    for word in &line.words {
        if kind(&word.text) == Kind::Listed && masked_before {
            words.push("L".to_string());
        } else {
            masked_before |= kind(&word.text) != Kind::Listed;
            words.push(mask(&word.text));
        }
    }
    words.join(" ")
}

const SHAPE_REPEATS: usize = 3;
const EDGE_SHARED: usize = 3;

/// Where amounts end on a page's lines, in whole points: for each right edge, how many
/// amounts end there and the left edges those amounts begin at.
#[derive(Default)]
struct Edges {
    right: BTreeMap<i64, (usize, BTreeSet<i64>)>,
    amounts: usize,
}

impl Edges {
    fn add(&mut self, line: &Line) {
        for word in &line.words {
            if kind(&word.text) == Kind::Amount {
                let entry = self.right.entry(word.x1.round() as i64).or_default();
                entry.0 += 1;
                entry.1.insert(word.x0.round() as i64);
                self.amounts += 1;
            }
        }
    }

    /// The edges to print, as whole points without a count. An edge is printed only when
    /// many amounts end there AND they begin at several different points: that is a
    /// right-aligned column. Amounts that end at one point and all begin at one point have
    /// one width, which would give their digit count.
    fn right_shared(&self) -> Vec<i64> {
        let quarter = self.amounts.div_ceil(4);
        self.right
            .iter()
            .filter(|(_, (n, lefts))| *n >= EDGE_SHARED.max(quarter) && lefts.len() >= EDGE_SHARED)
            .map(|(x, _)| *x)
            .collect()
    }
}

fn report(pages: &[Page]) -> String {
    let all: Vec<Vec<Line>> = pages.iter().map(|page| lines(page)).collect();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "pages {}; lines per page {:?}",
        pages.len(),
        all.iter().map(Vec::len).collect::<Vec<_>>()
    );
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut omitted = 0_usize;
    for (page_index, page_lines) in all.iter().enumerate() {
        let count = page_lines.len();
        let _ = writeln!(out, "page {} ({count} lines)", page_index + 1);
        let full = page_index < FULL_PAGES || page_index + 1 == all.len();
        let mut edges = Edges::default();
        for (index, line) in page_lines.iter().enumerate() {
            let near_edge = index < TOP_LINES || count - 1 - index < BOTTOM_LINES;
            if !(full || near_edge) {
                continue;
            }
            // Every line of a printed region counts toward the column edges, also a line
            // whose shape is not printed again: the edges need the many rows.
            edges.add(line);
            let times = seen.entry(shape(line)).or_insert(0);
            *times += 1;
            if *times > SHAPE_REPEATS {
                omitted += 1;
            } else {
                print_line(&mut out, &format!("L{}", index + 1), line);
            }
        }
        let right = edges
            .right_shared()
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        if !right.is_empty() {
            let _ = writeln!(out, "  amounts end at (whole points): {right}");
        }
    }
    let _ = writeln!(
        out,
        "lines left out as repeats of a shape already printed {SHAPE_REPEATS} times: {omitted}"
    );
    out
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("layout_probe: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The output is a fingerprint of a statement: readable by its owner only, and a file
/// that exists is never overwritten (it could be the statement).
fn write_private(path: &PathBuf, text: &str) -> std::io::Result<()> {
    use std::io::Write as _;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)?.write_all(text.as_bytes())
}

fn run() -> Result<(), String> {
    let arguments = arguments()?;
    let password = match &arguments.password_file {
        Some(path) => read_password(path)?,
        None => String::new(),
    };
    if arguments.out == arguments.pdf || arguments.password_file.as_ref() == Some(&arguments.out) {
        return Err("--out must not be the statement or the password file".into());
    }
    let bytes = std::fs::read(&arguments.pdf).map_err(|_| "statement unreadable")?;
    let engine = engine(&arguments.pdfium).map_err(|refusal| refusal.category.to_string())?;
    let pages =
        extract_pages(engine, &bytes, &password).map_err(|refusal| refusal.category.to_string())?;
    drop(password);
    write_private(&arguments.out, &report(&pages))
        .map_err(|_| "output file not writable, or it exists already")?;
    println!("wrote the layout of {} pages", pages.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use bridge_bank_statement::geometry::Word;

    /// A word of `text` from `x0` to `x1` on row `row` (rows are 12 points apart).
    fn w(row: usize, x0: f64, x1: f64, text: &str) -> Word {
        let y = row as f64 * 12.0;
        Word::new(x0, y, x1, y + 9.0, text)
    }

    fn line_of<'a>(out: &'a str, label: &str) -> &'a str {
        out.lines()
            .find(|l| l.starts_with(&format!("  {label} ")))
            .unwrap()
    }

    #[test]
    fn only_a_listed_word_is_printed_as_typed_and_only_exactly() {
        assert_eq!(mask("Date"), "Date");
        assert_eq!(mask("Balance:"), "Balance:");
        assert_eq!(mask("date"), "a+");
        assert_eq!(mask("Dates"), "Aa+");
        assert_eq!(mask("RAMESH:"), "A+:");
        assert_eq!(mask("Name:"), "Aa+:");
    }

    #[test]
    fn digits_and_letters_lose_their_values_and_long_runs_their_lengths() {
        assert_eq!(mask("01-04-2025"), "9+-9+-9+");
        assert_eq!(mask("123456789012"), "9+");
        assert_eq!(mask("\u{0967}\u{0968}\u{0969}"), "9+");
        assert_eq!(mask("ramesh.k@example.com"), "a+.a#a+.a+");
        assert_eq!(mask("UPI/123456789012/Ramesh"), "A+#9+#Aa+");
        // A tax-id-shaped text, assembled so the source holds no such literal.
        assert_eq!(mask(&["AAAAA", "9999", "A"].concat()), "A+9+A");
        assert_eq!(mask("\u{20b9}"), "?");
    }

    #[test]
    fn every_amount_form_is_masked_whole_and_a_bare_number_is_not_one() {
        assert_eq!(mask("1,23,456.78"), "9+.99");
        assert_eq!(mask("12.5"), "9+.99");
        assert_eq!(mask("-1,000.00"), "-9+.99");
        assert_eq!(mask("(500.00)"), "(9+.99)");
        assert_eq!(mask("1,23,456.78Cr"), "9+.99Aa");
        assert_eq!(mask("99.5DR"), "9+.99A+");
        assert_eq!(mask("1000.5Cr"), "9+.99Aa");
        assert_eq!(mask("Rs.500.00"), "Aa.9+.99");
        assert_eq!(mask("500.00-"), "9+.99-");
        assert_eq!(mask("+500.00"), "+9+.99");
        assert_eq!(mask("\u{20b9}1,000.00"), "?9+.99");
        assert_eq!(mask("\u{2212}1,000.00"), "?9+.99");
        assert_eq!(mask("1,00,000"), "9+");
        assert_eq!(mask("1,000"), "9+");
        assert_eq!(mask("500"), "9+");
        assert_eq!(mask("2025"), "9+");
        assert_eq!(mask("42"), "9+");
        assert_eq!(mask("7"), "9+");
        assert_eq!(mask("INR12.50ON05"), "A+9+.9+A+9+");
        assert_eq!(mask("12.500"), "9+.9+");
        assert_eq!(mask("\u{096b}.\u{0966}\u{0966}"), "9+.9+");
        assert_eq!(kind("1,23,456.78"), Kind::Amount);
        assert_eq!(kind("Rs.500.00"), Kind::Amount);
        assert_eq!(kind("123456789012"), Kind::Other);
        assert_eq!(kind("Ramesh"), Kind::Other);
        assert_eq!(kind("Narration"), Kind::Listed);
        assert!(mask("\u{0930}\u{092e}\u{0947}\u{0936}").is_ascii());
    }

    #[test]
    fn a_holder_block_repeated_on_every_page_is_masked() {
        let one = vec![
            w(0, 40.0, 80.0, "RAMESH"),
            w(0, 90.0, 130.0, "KUMAR"),
            w(1, 40.0, 80.0, "Account"),
            w(1, 90.0, 110.0, "No:"),
            w(1, 120.0, 200.0, "123456789012"),
            w(2, 40.0, 80.0, "Date"),
            w(2, 90.0, 140.0, "Narration"),
            w(2, 400.0, 440.0, "Balance"),
        ];
        let out = report(&[one.clone(), one.clone(), one.clone(), one]);
        assert!(!out.contains("RAMESH") && !out.contains("KUMAR"), "{out}");
        assert!(!out.contains("123456789012"), "{out}");
        assert!(
            out.contains("Narration") && out.contains("Balance"),
            "{out}"
        );
    }

    #[test]
    fn a_header_row_keeps_its_edges_and_a_listed_word_after_a_masked_one_is_hidden() {
        let header = vec![
            w(0, 40.0, 80.0, "Date"),
            w(0, 300.0, 340.0, "Balance"),
            w(0, 400.0, 420.0, "Dr"),
        ];
        // A data row: a name, then listed words (they may be part of a name or a Dr/Cr flag).
        let row = vec![
            w(1, 40.0, 100.0, "Ramesh"),
            w(1, 300.0, 340.0, "Balance"),
            w(1, 400.0, 420.0, "Dr"),
        ];
        let out = report(&[[header, row].concat()]);
        let body = |label| line_of(&out, label).split('|').nth(1).unwrap().to_string();
        assert_eq!(body("L1"), " [40] Date [300] Balance [400] Dr");
        assert_eq!(body("L2"), " Aa+ L L");
        assert!(
            line_of(&out, "L1").contains("y=0 |") && line_of(&out, "L2").contains("y=12 |"),
            "{out}"
        );
    }

    #[test]
    fn a_line_with_a_masked_word_prints_no_edge_at_all_whatever_its_alignment() {
        // A centred title with a name, a right-aligned name, and a listed word first on a
        // line that also holds a masked amount: none prints a position, and no right edge is
        // printed anywhere.
        let one = vec![
            w(0, 120.0, 180.0, "Statement"),
            w(0, 184.0, 200.0, "of"),
            w(0, 204.0, 260.0, "Account"),
            w(0, 264.0, 330.0, "RAMESH"),
            w(1, 448.0, 520.0, "KUMAR"),
            w(2, 40.0, 80.0, "Balance:"),
            w(2, 83.0, 125.0, "1,23,456.78"),
        ];
        let out = report(&[one]);
        let body = |label| line_of(&out, label).split('|').nth(1).unwrap().to_string();
        assert_eq!(body("L1"), " Statement of Account A+");
        assert_eq!(body("L2"), " A+");
        assert_eq!(body("L3"), " Balance: 9+.99");
        let printed = out.lines().filter(|l| l.starts_with("  L"));
        assert!(printed.into_iter().all(|l| !l.contains('[')), "{out}");
    }

    #[test]
    fn a_right_aligned_amount_column_has_its_edge_printed_and_nothing_else_about_amounts() {
        // Sixteen amounts: twelve end at 440, beginning at four different points (digit counts);
        // three end at 470 (15%); one at 462.
        let mut words = Vec::new();
        for r in 0..16 {
            let x1 = match r {
                0..=11 => 440.0,
                12..=14 => 470.0,
                _ => 462.0,
            };
            words.push(w(
                r,
                x1 - 40.0 - (r % 4) as f64 * 6.0,
                x1,
                &format!("{}.00", 1000 + r),
            ));
        }
        let out = report(&[words]);
        assert!(
            out.contains("amounts end at (whole points): 440\n"),
            "{out}"
        );
        assert!(!out.contains("470") && !out.contains("462"), "{out}");
        assert!(!out.contains("masked words begin"), "{out}");
        // Two amounts ending at one point are not enough.
        let two = vec![w(0, 400.0, 440.0, "1.00"), w(1, 394.0, 440.0, "2.00")];
        assert!(!report(&[two]).contains("amounts end"));
    }

    #[test]
    fn amounts_of_one_width_print_no_edge_because_it_would_give_their_digits() {
        // A left-aligned column: every amount begins at 400 and ends at 452 (nine characters).
        let words: Vec<Word> = (0..8)
            .map(|r| w(r, 400.0, 452.0, &format!("1{r}.00")))
            .collect();
        let out = report(&[words]);
        assert!(!out.contains("amounts end"), "{out}");
        assert!(!out.contains("452") && !out.contains("400"), "{out}");
    }

    #[test]
    fn pages_one_to_three_and_the_last_are_printed_whole_and_the_others_at_their_edges() {
        // Every line is distinct (two listed words, never repeated), so no cap applies.
        let n = ALLOWED.len();
        let pages: Vec<Page> = (0..6)
            .map(|p| {
                (0..30)
                    .flat_map(|j| {
                        let k = p * 30 + j;
                        vec![
                            w(j, 40.0, 80.0, ALLOWED[k % n]),
                            w(j, 200.0, 240.0, ALLOWED[(k / n) % n]),
                        ]
                    })
                    .collect()
            })
            .collect();
        let out = report(&pages);
        // Pages 1 to 3 and 6 in full (4 x 30), pages 4 and 5 their first 12 and last 3 lines.
        assert_eq!(
            out.lines().filter(|l| l.starts_with("  L")).count(),
            4 * 30 + 2 * 15,
            "{out}"
        );
    }

    #[test]
    fn a_line_shape_is_printed_three_times_and_then_counted() {
        let rows: Vec<Word> = (0..6)
            .flat_map(|r| vec![w(r, 40.0, 80.0, "Ramesh"), w(r, 400.0, 440.0, "500.00")])
            .collect();
        let out = report(&[rows]);
        assert_eq!(
            out.lines().filter(|l| l.starts_with("  L")).count(),
            3,
            "{out}"
        );
        assert!(out.contains("printed 3 times: 3"), "{out}");
    }

    #[test]
    fn rows_that_differ_only_by_a_listed_word_after_a_masked_one_are_one_shape() {
        // A debit/credit flag after a name must not make two shapes: the number of lines
        // printed would then show how the flags are spread.
        let rows: Vec<Word> = (0..6)
            .flat_map(|r| {
                let flag = if r % 2 == 0 { "Dr" } else { "Cr" };
                vec![w(r, 40.0, 80.0, "Ramesh"), w(r, 400.0, 420.0, flag)]
            })
            .collect();
        let out = report(&[rows]);
        assert_eq!(
            out.lines().filter(|l| l.starts_with("  L")).count(),
            3,
            "{out}"
        );
        assert!(
            out.lines()
                .filter(|l| l.starts_with("  L"))
                .all(|l| l.ends_with("| Aa+ L")),
            "{out}"
        );
    }

    #[test]
    fn the_repeat_cap_counts_a_shape_across_pages() {
        let one = vec![w(0, 40.0, 80.0, "Ramesh"), w(0, 400.0, 440.0, "500.00")];
        let out = report(&[one.clone(), one.clone(), one.clone(), one.clone(), one]);
        assert_eq!(
            out.lines().filter(|l| l.starts_with("  L")).count(),
            3,
            "{out}"
        );
        assert!(out.contains("printed 3 times: 2"), "{out}");
    }
}
