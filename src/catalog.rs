//! The field catalog: canonical fields (`catalog/fields.toml`) and one file per card (`catalog/cards/*.toml`).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::Deserialize;

/// Where a field's value comes from, and where an answer to it is saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    /// Identity kept in the traveller's Vault entry; asked once, ever.
    Vault,
    /// Trip facts and preferences found in memory, email or calendar.
    Recall,
    /// Declarations answered fresh on every card; never stored.
    Ask,
}

impl Tier {
    /// The marker used on the gap form and in tables.
    pub fn badge(self) -> &'static str {
        match self {
            Tier::Vault => "🔐 VAULT",
            Tier::Recall => "🧠 RECALL",
            Tier::Ask => "✍️ ASK",
        }
    }
}

/// The Vault slot that holds a `vault` field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VaultSlot {
    Name,
    Address,
    Phone,
    Email,
    /// A `key=value` line in the entry's Hints.
    Hint,
    /// A Vault Login entry (site credentials).
    Login,
}

impl fmt::Display for VaultSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VaultSlot::Name => "Contact › Name",
            VaultSlot::Address => "Contact › Address",
            VaultSlot::Phone => "Contact › Phone",
            VaultSlot::Email => "Contact › Email",
            VaultSlot::Hint => "Contact › Hints",
            VaultSlot::Login => "Login",
        };
        f.write_str(s)
    }
}

/// Where a `recall` answer is remembered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Save {
    /// The dated trip log in memory.
    Trip,
    /// A lasting memory preference.
    Preference,
}

/// A canonical field shared by all cards.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub key: String,
    pub label: String,
    pub tier: Tier,
    pub vault_slot: Option<VaultSlot>,
    pub save: Option<Save>,
    pub find: Option<String>,
    pub note: Option<String>,
}

impl Field {
    /// Where the value is looked up and where an answer goes, in one phrase.
    pub fn source(&self) -> String {
        let base = match self.tier {
            Tier::Vault => match self.vault_slot {
                Some(VaultSlot::Hint) => format!("Vault hint `{}=`", self.key),
                Some(slot) => format!("Vault {slot}"),
                None => "Vault".to_owned(),
            },
            Tier::Recall => {
                let to = match self.save {
                    Some(Save::Trip) => "trip log",
                    Some(Save::Preference) => "memory preference",
                    None => "memory",
                };
                format!(
                    "{} → saved to {to}",
                    self.find.as_deref().unwrap_or_default()
                )
            }
            Tier::Ask => "Ask every time; never saved".to_owned(),
        };
        match &self.note {
            Some(n) => format!("{base}. {n}"),
            None => base,
        }
    }
}

#[derive(Debug, Deserialize)]
struct FieldsFile {
    field: Vec<Field>,
}

/// One field on a card's page, pointing at a canonical field.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardField {
    /// The label as printed on the official site.
    pub label: String,
    /// The canonical field key (`catalog/fields.toml`).
    pub key: String,
    #[serde(default = "yes")]
    pub required: bool,
    /// A condition under which the field shows or becomes required.
    pub when: Option<String>,
    /// Site format, option list or transformation rule.
    pub format: Option<String>,
}

fn yes() -> bool {
    true
}

/// A page (step) of a card's site.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Page {
    pub title: String,
    pub notes: Option<String>,
    #[serde(default)]
    pub field: Vec<CardField>,
}

/// One arrival card.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Card {
    /// ISO 3166 alpha-2 code of the country.
    pub code: String,
    pub name: String,
    /// The site skill that walks this card.
    pub skill: String,
    pub url: String,
    pub window: String,
    pub who: String,
    pub exempt: String,
    /// Steps that are always the user's (CAPTCHA, login, final submit, …).
    #[serde(default)]
    pub handoffs: Vec<String>,
    /// When the facts were last checked against the live site / official sources.
    pub verified: String,
    #[serde(default)]
    pub sources: Vec<String>,
    pub page: Vec<Page>,
}

impl Card {
    /// Every card field in page order.
    pub fn fields(&self) -> impl Iterator<Item = &CardField> {
        self.page.iter().flat_map(|p| p.field.iter())
    }
}

/// The whole catalog.
#[derive(Debug, Clone)]
pub struct Catalog {
    pub fields: BTreeMap<String, Field>,
    /// Field keys in file order (for stable output).
    pub order: Vec<String>,
    pub cards: Vec<Card>,
}

impl Catalog {
    /// Load `fields.toml` and every `cards/*.toml` under `dir`, cards sorted by code.
    pub fn load(dir: &Path) -> Result<Self> {
        let path = dir.join("fields.toml");
        let text =
            fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        let file: FieldsFile =
            toml::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;

        let order = file.field.iter().map(|f| f.key.clone()).collect();
        let mut fields = BTreeMap::new();
        for f in file.field {
            let key = f.key.clone();
            if fields.insert(key.clone(), f).is_some() {
                anyhow::bail!("{}: duplicate field key `{key}`", path.display());
            }
        }

        let cards_dir = dir.join("cards");
        let mut cards = Vec::new();
        let entries =
            fs::read_dir(&cards_dir).with_context(|| format!("reading {}", cards_dir.display()))?;
        for entry in entries {
            let p = entry?.path();
            if p.extension().is_some_and(|e| e == "toml") {
                let text =
                    fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
                let card: Card =
                    toml::from_str(&text).with_context(|| format!("parsing {}", p.display()))?;
                cards.push(card);
            }
        }
        cards.sort_by(|a, b| a.code.cmp(&b.code));
        Ok(Self {
            fields,
            order,
            cards,
        })
    }

    /// The card with this country code (case-insensitive).
    pub fn card(&self, code: &str) -> Option<&Card> {
        self.cards
            .iter()
            .find(|c| c.code.eq_ignore_ascii_case(code))
    }

    /// Every consistency problem in the catalog; empty when it is sound.
    pub fn validate(&self) -> Vec<String> {
        let mut issues = Vec::new();

        for f in self.fields.values() {
            let k = &f.key;
            match f.tier {
                Tier::Vault if f.vault_slot.is_none() => {
                    issues.push(format!("field `{k}`: vault tier needs `vault_slot`"))
                }
                Tier::Recall if f.save.is_none() => {
                    issues.push(format!("field `{k}`: recall tier needs `save`"))
                }
                Tier::Recall if f.find.as_deref().is_none_or(str::is_empty) => {
                    issues.push(format!("field `{k}`: recall tier needs `find`"));
                }
                Tier::Vault | Tier::Ask if f.save.is_some() => {
                    issues.push(format!("field `{k}`: only recall fields take `save`"));
                }
                Tier::Recall | Tier::Ask if f.vault_slot.is_some() => {
                    issues.push(format!("field `{k}`: only vault fields take `vault_slot`"));
                }
                _ => {}
            }
            if !k
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
            {
                issues.push(format!("field `{k}`: keys are snake_case ascii"));
            }
        }

        let mut codes = BTreeSet::new();
        for c in &self.cards {
            if !codes.insert(c.code.as_str()) {
                issues.push(format!("card {}: duplicate code", c.code));
            }
            if !c.url.starts_with("https://") {
                issues.push(format!("card {}: url must be https", c.code));
            }
            if c.page.is_empty() {
                issues.push(format!("card {}: no pages", c.code));
            }
            for p in &c.page {
                let mut labels = BTreeSet::new();
                for f in &p.field {
                    if !self.fields.contains_key(&f.key) {
                        issues.push(format!(
                            "card {} / {}: unknown field key `{}`",
                            c.code, p.title, f.key
                        ));
                    }
                    if f.label.trim().is_empty() {
                        issues.push(format!(
                            "card {} / {}: empty label for `{}`",
                            c.code, p.title, f.key
                        ));
                    }
                    if !labels.insert(f.label.as_str()) {
                        issues.push(format!(
                            "card {} / {}: duplicate label `{}`",
                            c.code, p.title, f.label
                        ));
                    }
                }
            }
        }

        // Every canonical field should be used by at least one card; an unused one is dead weight on the form.
        let used: BTreeSet<&str> = self
            .cards
            .iter()
            .flat_map(Card::fields)
            .map(|f| f.key.as_str())
            .collect();
        for k in &self.order {
            if !used.contains(k.as_str()) {
                issues.push(format!("field `{k}`: not used by any card"));
            }
        }
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn field(key: &str, tier: Tier) -> Field {
        Field {
            key: key.into(),
            label: key.into(),
            tier,
            vault_slot: (tier == Tier::Vault).then_some(VaultSlot::Hint),
            save: (tier == Tier::Recall).then_some(Save::Trip),
            find: (tier == Tier::Recall).then(|| "email".into()),
            note: None,
        }
    }

    fn catalog(fields: Vec<Field>, card_keys: &[&str]) -> Catalog {
        let order = fields.iter().map(|f| f.key.clone()).collect();
        let card = Card {
            code: "XX".into(),
            name: "Test card".into(),
            skill: "xx-card".into(),
            url: "https://example.gov".into(),
            window: "any".into(),
            who: "all".into(),
            exempt: "none".into(),
            handoffs: vec![],
            verified: "2026-11-11".into(),
            sources: vec![],
            page: vec![Page {
                title: "One".into(),
                notes: None,
                field: card_keys
                    .iter()
                    .map(|k| CardField {
                        label: format!("Label {k}"),
                        key: (*k).into(),
                        required: true,
                        when: None,
                        format: None,
                    })
                    .collect(),
            }],
        };
        Catalog {
            fields: fields.into_iter().map(|f| (f.key.clone(), f)).collect(),
            order,
            cards: vec![card],
        }
    }

    #[test]
    fn sound_catalog_has_no_issues() {
        let c = catalog(
            vec![
                field("passport_no", Tier::Vault),
                field("arrival_date", Tier::Recall),
            ],
            &["passport_no", "arrival_date"],
        );
        assert!(c.validate().is_empty(), "{:?}", c.validate());
    }

    #[test]
    fn unknown_and_unused_keys_are_reported() {
        let c = catalog(vec![field("passport_no", Tier::Vault)], &["pasport_no"]);
        let issues = c.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.contains("unknown field key `pasport_no`"))
        );
        assert!(issues.iter().any(|i| i.contains("`passport_no`: not used")));
    }

    #[test]
    fn tier_rules_are_enforced() {
        let mut bad = field("dob", Tier::Vault);
        bad.vault_slot = None;
        let mut ask = field("health_symptoms", Tier::Ask);
        ask.save = Some(Save::Trip);
        let c = catalog(vec![bad, ask], &["dob", "health_symptoms"]);
        let issues = c.validate();
        assert!(
            issues
                .iter()
                .any(|i| i.contains("`dob`: vault tier needs `vault_slot`"))
        );
        assert!(
            issues
                .iter()
                .any(|i| i.contains("`health_symptoms`: only recall fields take `save`"))
        );
    }

    #[test]
    fn real_catalog_is_sound() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("catalog");
        let c = Catalog::load(&dir).expect("catalog loads");
        assert_eq!(
            c.cards.iter().map(|c| c.code.as_str()).collect::<Vec<_>>(),
            [
                "CN", "ID", "IN", "JP", "KH", "KR", "MV", "MY", "NZ", "PH", "SG", "TH", "TW", "VN"
            ]
        );
        assert_eq!(c.validate(), Vec::<String>::new());
    }
}
