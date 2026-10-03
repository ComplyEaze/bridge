// SPDX-License-Identifier: Apache-2.0
//! Port of the part of the reference engine's `party_identity` that `entity_269st_gap` uses: which
//! ledgers are parties, the PAN each carries and where it came from, and the entities that a shared
//! PAN binds. Not ported, because nothing here reads it: the party's key, address and source, the
//! unbound and missing-PAN lists and the PAN/GSTIN mismatch report.
//!
//! A ledger is a party only by its group chain or by the engagement's own list, never by its name.
//! A ledger under Duties & Taxes or named as a round-off ledger is never one. A PAN is never
//! cross-derived from a GSTIN unless the engagement opts in, and then only into a gap, never over a
//! recorded one. A chain that is incomplete and does not already settle the answer refuses the test
//! rather than defaulting to "not a party" (`PARTY-chain-incomplete`).
//!
//! Divergence, deliberate, and not parity: the reference reads the optional `[party_identity]` table
//! leniently (a table that is not a table, or a key of the wrong type, is read as empty or by
//! Python's truthiness, so a string for a list becomes the set of its characters). Here every such
//! value refuses with a typed `Config` error, and only this test fails.

use std::collections::{BTreeMap, BTreeSet};

use crate::book::Book;
use crate::error::{AuditError, Result};
use crate::support::{py_lower, py_upper};

const DUTIES_TAXES_GROUP: &str = "Duties & Taxes";
const DEFAULT_PARTY_GROUPS: [&str; 2] = ["Sundry Debtors", "Sundry Creditors"];

/// Words that say nothing about whose name it is.
const NAME_STOPWORDS: [&str; 11] = [
    "pvt", "private", "ltd", "limited", "and", "co", "company", "the", "llp", "inc", "corp",
];

/// Where a party's PAN came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanSource {
    Masters,
    ClientConfig,
    DerivedFromGstin,
}

/// The engagement's optional `[party_identity]` table.
#[derive(Debug, Clone, Default)]
pub struct PartyConfig {
    pub derive_pan_from_gstin: bool,
    pub party_groups: Vec<String>,
    pub additional_party_ledgers: BTreeSet<String>,
    pub excluded_ledgers: BTreeSet<String>,
    pub round_off_ledgers: BTreeSet<String>,
    pub overrides: BTreeMap<String, PartyOverride>,
}

/// One ledger's override: a field fills a gap a master left, never replaces what it holds. Empty
/// when not given.
#[derive(Debug, Clone, Default)]
pub struct PartyOverride {
    pub name: String,
    pub pan: String,
    pub gstin: String,
    pub address: String,
}

const KEYS: [&str; 6] = [
    "derive_pan_from_gstin",
    "party_groups",
    "additional_party_ledgers",
    "excluded_ledgers",
    "round_off_ledgers",
    "overrides",
];

fn config_error(detail: String) -> AuditError {
    AuditError::Config(format!("party_identity: {detail}"))
}

fn strings(table: &toml::Table, key: &str) -> Result<Vec<String>> {
    let Some(value) = table.get(key) else {
        return Ok(Vec::new());
    };
    value
        .as_array()
        .ok_or_else(|| config_error(format!("[party_identity].{key} is not a list")))?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_string)
                .ok_or_else(|| config_error(format!("[party_identity].{key} holds a non-string")))
        })
        .collect()
}

impl PartyConfig {
    /// The optional `[party_identity]` table; empty when absent. A key it does not know refuses,
    /// as does a value of the wrong type.
    pub fn from_toml(raw: Option<&toml::Value>) -> Result<Self> {
        let Some(raw) = raw else {
            return Ok(Self::default());
        };
        let table = raw
            .as_table()
            .ok_or_else(|| config_error("[party_identity] is not a table".to_string()))?;
        if let Some(unknown) = table.keys().find(|k| !KEYS.contains(&k.as_str())) {
            return Err(config_error(format!(
                "[party_identity].{unknown} is not a key"
            )));
        }
        let derive_pan_from_gstin = match table.get("derive_pan_from_gstin") {
            None => false,
            Some(v) => v.as_bool().ok_or_else(|| {
                config_error(
                    "[party_identity].derive_pan_from_gstin is not true or false".to_string(),
                )
            })?,
        };
        let mut overrides = BTreeMap::new();
        if let Some(raw) = table.get("overrides") {
            let by_ledger = raw.as_table().ok_or_else(|| {
                config_error("[party_identity].overrides is not a table".to_string())
            })?;
            for (ledger, v) in by_ledger {
                let fields = v.as_table().ok_or_else(|| {
                    config_error(format!(
                        "[party_identity].overrides.{ledger} is not a table"
                    ))
                })?;
                let text = |key: &str| -> Result<String> {
                    match fields.get(key) {
                        None => Ok(String::new()),
                        Some(v) => v.as_str().map(str::to_string).ok_or_else(|| {
                            config_error(format!(
                                "[party_identity].overrides.{ledger}.{key} is not a string"
                            ))
                        }),
                    }
                };
                if let Some(unknown) = fields
                    .keys()
                    .find(|k| !["name", "pan", "gstin", "address"].contains(&k.as_str()))
                {
                    return Err(config_error(format!(
                        "[party_identity].overrides.{ledger}.{unknown} is not a key"
                    )));
                }
                overrides.insert(
                    ledger.clone(),
                    PartyOverride {
                        name: text("name")?,
                        pan: text("pan")?,
                        gstin: text("gstin")?,
                        address: text("address")?,
                    },
                );
            }
        }
        Ok(Self {
            derive_pan_from_gstin,
            party_groups: strings(table, "party_groups")?,
            additional_party_ledgers: strings(table, "additional_party_ledgers")?
                .into_iter()
                .collect(),
            excluded_ledgers: strings(table, "excluded_ledgers")?.into_iter().collect(),
            round_off_ledgers: strings(table, "round_off_ledgers")?.into_iter().collect(),
            overrides,
        })
    }
}

/// Ledgers that are one legal person, bound by a shared PAN, never by name.
#[derive(Debug, Clone)]
pub struct EntityBinding {
    pub pan: String,
    /// Canonical ledger names, sorted.
    pub ledgers: Vec<String>,
    /// Per ledger, aligned with `ledgers`.
    pub pan_sources: Vec<PanSource>,
    /// Disclosure only, never a binding criterion: whether every ledger name shares a word (of a
    /// single ledger, whether its name has one).
    pub names_agree: bool,
}

#[derive(Debug, Default)]
pub struct PartyIndex {
    entities: Vec<EntityBinding>,
    entity_of: BTreeMap<String, usize>,
}

impl PartyIndex {
    /// The binding a ledger belongs to, or `None` when it has no PAN to bind on (or is no party).
    pub fn entity_for_ledger(&self, ledger: &str) -> Option<&EntityBinding> {
        self.entity_of.get(ledger).map(|i| &self.entities[*i])
    }
}

/// The PAN segment of a GSTIN (characters 3 to 12), upper-cased, or `None` when it is not shaped
/// like a PAN (AAAAA9999A). A GSTIN is not always built on a PAN: a deductor may register on a TAN
/// (AAAA99999A), and the two are told apart by the shape, so a TAN gives none.
pub fn pan_from_gstin(gstin: &str) -> Option<String> {
    let chars: Vec<char> = gstin.chars().collect();
    if chars.len() < 12 {
        return None;
    }
    let segment = py_upper(&chars[2..12].iter().collect::<String>());
    let s: Vec<char> = segment.chars().collect();
    let pan_shaped = s.len() == 10
        && s[..5].iter().all(char::is_ascii_uppercase)
        && s[5..9].iter().all(char::is_ascii_digit)
        && s[9].is_ascii_uppercase();
    pan_shaped.then_some(segment)
}

fn name_tokens(name: &str) -> BTreeSet<String> {
    let lower = py_lower(name);
    let mut out = BTreeSet::new();
    let mut run = String::new();
    for c in lower.chars().chain(std::iter::once('\n')) {
        if c.is_ascii_lowercase() {
            run.push(c);
        } else if !run.is_empty() {
            let word = std::mem::take(&mut run);
            if !NAME_STOPWORDS.contains(&word.as_str()) && word.len() > 2 {
                out.insert(word);
            }
        }
    }
    out
}

/// Classify every ledger of the book and bind the parties that carry a PAN into entities.
pub fn build_party_index(book: &Book, config: &PartyConfig) -> Result<PartyIndex> {
    let is_party = |ledger: &crate::book::Ledger| -> Result<bool> {
        if config.excluded_ledgers.contains(&ledger.name)
            || config.round_off_ledgers.contains(&ledger.name)
            || ledger.under(DUTIES_TAXES_GROUP)
        {
            return Ok(false);
        }
        if DEFAULT_PARTY_GROUPS.iter().any(|g| ledger.under(g))
            || config.party_groups.iter().any(|g| ledger.under(g))
            || config.additional_party_ledgers.contains(&ledger.name)
        {
            return Ok(true);
        }
        if !ledger.chain_complete {
            return Err(AuditError::refused(
                "PARTY-chain-incomplete",
                format!(
                    "ledger {:?} has an incomplete group chain {:?} that does not already contain \
Duties & Taxes or a configured party group; whether it is a party cannot be decided without the \
rest of the chain",
                    ledger.name, ledger.chain
                ),
            ));
        }
        Ok(false)
    };
    // (ledger, display name, PAN, where it came from) of every party that carries a PAN.
    let mut bound: Vec<(String, String, String, PanSource)> = Vec::new();
    for ledger in book.ledgers.values() {
        if !is_party(ledger)? {
            continue;
        }
        let over = config.overrides.get(&ledger.name);
        let from_over = |f: fn(&PartyOverride) -> &String| over.map_or("", |o| f(o).as_str());
        // Masters win whenever they hold a value; the engagement only fills a gap.
        let gstin = if ledger.gstin.is_empty() {
            from_over(|o| &o.gstin)
        } else {
            ledger.gstin.as_str()
        };
        let (mut pan, mut source) = if !ledger.pan.is_empty() {
            (ledger.pan.clone(), Some(PanSource::Masters))
        } else if !from_over(|o| &o.pan).is_empty() {
            (
                from_over(|o| &o.pan).to_string(),
                Some(PanSource::ClientConfig),
            )
        } else {
            (String::new(), None)
        };
        // Derived only when the engagement opts in, and only into a gap.
        if pan.is_empty() && config.derive_pan_from_gstin {
            if let Some(derived) = pan_from_gstin(gstin) {
                pan = derived;
                source = Some(PanSource::DerivedFromGstin);
            }
        }
        let Some(source) = source.filter(|_| !pan.is_empty()) else {
            continue; // a party with no PAN to bind on
        };
        let display = match from_over(|o| &o.name) {
            "" => ledger.name.clone(),
            name => name.to_string(),
        };
        bound.push((ledger.name.clone(), display, pan, source));
    }
    let mut by_pan: BTreeMap<&str, Vec<&(String, String, String, PanSource)>> = BTreeMap::new();
    for party in &bound {
        by_pan.entry(party.2.as_str()).or_default().push(party);
    }
    let mut index = PartyIndex::default();
    for (pan, mut members) in by_pan {
        members.sort_by(|a, b| a.0.cmp(&b.0));
        // A single-ledger entity never reaches a gap row, so its disclosure is never read.
        let names_agree = {
            let mut sets = members.iter().map(|m| name_tokens(&m.1));
            let first = sets.next().unwrap_or_default();
            !sets
                .fold(first, |acc, s| acc.intersection(&s).cloned().collect())
                .is_empty()
        };
        let at = index.entities.len();
        for m in &members {
            index.entity_of.insert(m.0.clone(), at);
        }
        index.entities.push(EntityBinding {
            pan: pan.to_string(),
            ledgers: members.iter().map(|m| m.0.clone()).collect(),
            pan_sources: members.iter().map(|m| m.3).collect(),
            names_agree,
        });
    }
    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::book::Ledger;

    /// The expected values are what the reference's `pan_from_gstin` gives for the same text.
    #[test]
    fn a_pan_is_the_pan_shaped_segment_of_a_gstin_and_nothing_else() {
        for (gstin, want) in [
            ("27ABCDE1234F1Z5", Some("ABCDE1234F")),
            ("27abcde1234f1z5", Some("ABCDE1234F")),
            ("27ABCDE1234f1Z5", Some("ABCDE1234F")),
            ("27ABCDE1234F", Some("ABCDE1234F")),
            ("  ABCDE1234F1Z5", Some("ABCDE1234F")),
            ("27ABCD12345F1Z5", None), // a TAN
            ("27ABCDE1234", None),     // fewer than twelve characters
            ("", None),
            ("29ABCDEF234F1Z5", None),
            ("27ABCDE12345F1Z", None),
            ("ABCDE1234F", None),
            ("27ÄBCDE1234F1Z5", None),
            ("27ABCDESS234F1Z5", None),
        ] {
            assert_eq!(pan_from_gstin(gstin).as_deref(), want, "{gstin:?}");
        }
    }

    fn config(text: &str) -> Result<PartyConfig> {
        let value: toml::Value = toml::from_str(text).unwrap();
        PartyConfig::from_toml(value.get("party_identity"))
    }

    #[test]
    fn the_party_identity_table_is_read_typed_and_refused_when_it_is_not() {
        let c = config(
            "[party_identity]\nderive_pan_from_gstin = true\nparty_groups = [\"G\"]\n\
additional_party_ledgers = [\"A\"]\nexcluded_ledgers = [\"E\"]\nround_off_ledgers = [\"R\"]\n\
[party_identity.overrides.L]\nname = \"N\"\npan = \"P\"\ngstin = \"S\"\naddress = \"D\"\n",
        )
        .unwrap();
        assert!(c.derive_pan_from_gstin);
        assert_eq!(c.party_groups, ["G"]);
        assert!(c.additional_party_ledgers.contains("A"));
        assert!(c.excluded_ledgers.contains("E"));
        assert!(c.round_off_ledgers.contains("R"));
        let o = &c.overrides["L"];
        assert_eq!(
            (&*o.name, &*o.pan, &*o.gstin, &*o.address),
            ("N", "P", "S", "D")
        );
        assert!(!config("").unwrap().derive_pan_from_gstin);
        let empty = config("[party_identity]\n").unwrap();
        assert!(empty.overrides.is_empty() && !empty.derive_pan_from_gstin);
        for bad in [
            "party_identity = 5",
            "[party_identity]\nunknown_key = 1",
            "[party_identity]\nderive_pan_from_gstin = \"yes\"",
            "[party_identity]\nparty_groups = \"G\"",
            "[party_identity]\nparty_groups = [1]",
            "[party_identity]\nadditional_party_ledgers = \"A\"",
            "[party_identity]\nexcluded_ledgers = [1]",
            "[party_identity]\nround_off_ledgers = {}",
            "[party_identity]\noverrides = []",
            "[party_identity.overrides]\nL = 1",
            "[party_identity.overrides.L]\npan = 1",
            "[party_identity.overrides.L]\nunknown = \"x\"",
        ] {
            assert!(matches!(config(bad), Err(AuditError::Config(_))), "{bad}");
        }
    }

    fn book_of(ledgers: &[(&str, &[&str], bool)]) -> Book {
        Book {
            ledgers: ledgers
                .iter()
                .map(|(name, chain, complete)| {
                    (
                        (*name).to_string(),
                        Ledger {
                            name: (*name).to_string(),
                            parent: chain[0].to_string(),
                            chain: chain.iter().map(|g| (*g).to_string()).collect(),
                            chain_complete: *complete,
                            master_opening_paise: 0,
                            guid: format!("invented-{name}"),
                            masterid: None,
                            pan: "ABCDE1234F".to_string(),
                            gstin: String::new(),
                        },
                    )
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn an_incomplete_chain_that_settles_nothing_refuses_and_one_that_settles_it_does_not() {
        let cfg = PartyConfig::default();
        let refused = build_party_index(&book_of(&[("Unplaced", &["Legacy"], false)]), &cfg)
            .expect_err("whether it is a party cannot be decided");
        assert_eq!(refused.code(), Some("PARTY-chain-incomplete"));
        // Already settled by the known part: Duties & Taxes (not a party), a party group (a party).
        let settled = book_of(&[
            ("Tax", &["Duties & Taxes"], false),
            ("Debtor", &["Sundry Debtors"], false),
        ]);
        let index = build_party_index(&settled, &cfg).unwrap();
        assert!(index.entity_for_ledger("Tax").is_none());
        assert!(index.entity_for_ledger("Debtor").is_some());
        // Named by the engagement as a party, or excluded, it is settled as well.
        let named = PartyConfig {
            additional_party_ledgers: ["Unplaced".to_string()].into(),
            ..PartyConfig::default()
        };
        assert!(build_party_index(&book_of(&[("Unplaced", &["Legacy"], false)]), &named).is_ok());
        let excluded = PartyConfig {
            excluded_ledgers: ["Unplaced".to_string()].into(),
            ..PartyConfig::default()
        };
        assert!(
            build_party_index(&book_of(&[("Unplaced", &["Legacy"], false)]), &excluded).is_ok()
        );
        let round_off = PartyConfig {
            round_off_ledgers: ["Unplaced".to_string()].into(),
            ..PartyConfig::default()
        };
        assert!(
            build_party_index(&book_of(&[("Unplaced", &["Legacy"], false)]), &round_off).is_ok()
        );
        // A complete chain that is no party and no tax is simply no party.
        let plain = book_of(&[("Expense", &["Indirect Expenses"], true)]);
        assert!(build_party_index(&plain, &cfg)
            .unwrap()
            .entity_for_ledger("Expense")
            .is_none());
    }

    fn debtor(name: &str, pan: &str, gstin: &str) -> Ledger {
        Ledger {
            name: name.to_string(),
            parent: "Sundry Debtors".to_string(),
            chain: vec!["Sundry Debtors".to_string()],
            chain_complete: true,
            master_opening_paise: 0,
            guid: format!("invented-{name}"),
            masterid: None,
            pan: pan.to_string(),
            gstin: gstin.to_string(),
        }
    }

    fn book_with(ledgers: Vec<Ledger>) -> Book {
        Book {
            ledgers: ledgers.into_iter().map(|l| (l.name.clone(), l)).collect(),
            ..Default::default()
        }
    }

    /// The reference's single-ledger entity always "agrees" with itself, whatever its name holds.
    #[test]
    fn a_single_ledger_entity_agrees_with_itself_even_with_no_name_word() {
        let book = book_with(vec![debtor("M/s", "PAN-ALONE-1", "")]);
        let index = build_party_index(&book, &PartyConfig::default()).unwrap();
        let entity = index.entity_for_ledger("M/s").unwrap();
        assert_eq!(entity.ledgers, vec!["M/s".to_string()]);
        assert!(entity.names_agree);
    }

    /// With derivation opted in, a PAN the engagement supplies still comes before one derived
    /// from the GSTIN: derivation only fills a gap nothing else filled.
    #[test]
    fn an_override_pan_comes_before_one_derived_from_the_gstin() {
        // Built at run time so no source line is shaped like a real identifier.
        let segment = ["DDDDD", "3333", "D"].concat();
        let gstin = format!("29{segment}1X7");
        let book = book_with(vec![debtor("Customer", "", &gstin)]);
        let mut config = PartyConfig {
            derive_pan_from_gstin: true,
            ..PartyConfig::default()
        };
        let derived = build_party_index(&book, &config).unwrap();
        let entity = derived.entity_for_ledger("Customer").unwrap();
        assert_eq!(
            (entity.pan.as_str(), entity.pan_sources.as_slice()),
            (segment.as_str(), &[PanSource::DerivedFromGstin][..])
        );
        config.overrides.insert(
            "Customer".to_string(),
            PartyOverride {
                pan: "PAN-OVR-1".to_string(),
                ..PartyOverride::default()
            },
        );
        let overridden = build_party_index(&book, &config).unwrap();
        let entity = overridden.entity_for_ledger("Customer").unwrap();
        assert_eq!(
            (entity.pan.as_str(), entity.pan_sources.as_slice()),
            ("PAN-OVR-1", &[PanSource::ClientConfig][..])
        );
    }
}
