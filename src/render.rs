//! Markdown rendering: per-card field tables, the cross-card matrix, and the paste-able setup bundle.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::catalog::{Card, Catalog, Tier};

/// The marker in a hand-written skill that is replaced by the card's generated field reference.
pub const FIELDS_MARKER: &str = "<!-- FIELDS -->";

/// The marker in the onboarding skill that is replaced by the generated list of Vault fields.
pub const VAULT_FIELDS_MARKER: &str = "<!-- VAULT_FIELDS -->";

/// Escape a cell for a markdown table.
fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

/// The card's field reference: header facts, then one table per page.
pub fn card_reference(cat: &Catalog, card: &Card) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "### Card facts: {}\n", card.name);
    let _ = writeln!(out, "- **Official URL (only this one):** {}", card.url);
    let _ = writeln!(out, "- **Filing window:** {}", card.window);
    let _ = writeln!(out, "- **Who files:** {}", card.who);
    let _ = writeln!(out, "- **Exempt:** {}", card.exempt);
    let _ = writeln!(out, "- **Last verified:** {}", card.verified);
    if !card.handoffs.is_empty() {
        let _ = writeln!(out, "- **Always the user's (hand over):**");
        for h in &card.handoffs {
            let _ = writeln!(out, "  - {h}");
        }
    }
    let _ = writeln!(out, "\n### Fields, page by page\n");
    let _ = writeln!(
        out,
        "Tier: 🔐 VAULT = traveller's Vault entry · 🧠 RECALL = memory → email → calendar → derived · ✍️ ASK = ask every time, never saved.\n"
    );
    for page in &card.page {
        let _ = writeln!(out, "#### {}\n", page.title);
        if let Some(n) = &page.notes {
            let _ = writeln!(out, "{n}\n");
        }
        if page.field.is_empty() {
            continue;
        }
        let _ = writeln!(out, "| Site label | Key | Tier | Req | Format / options |");
        let _ = writeln!(out, "|---|---|---|---|---|");
        for f in &page.field {
            let tier = cat.fields.get(&f.key).map_or("?", |c| c.tier.badge());
            let req = match (&f.when, f.required) {
                (Some(w), _) => format!("if {w}"),
                (None, true) => "yes".to_owned(),
                (None, false) => "no".to_owned(),
            };
            let _ = writeln!(
                out,
                "| {} | `{}` | {} | {} | {} |",
                cell(&f.label),
                f.key,
                tier,
                cell(&req),
                cell(f.format.as_deref().unwrap_or(""))
            );
        }
        let _ = writeln!(out);
    }
    if !card.sources.is_empty() {
        let _ = writeln!(out, "Sources: {}\n", card.sources.join(" · "));
    }
    out
}

/// The cross-card matrix: every canonical field, its tier and source, and which cards use it.
pub fn matrix(cat: &Catalog) -> String {
    let mut out = String::new();
    let codes: Vec<&str> = cat.cards.iter().map(|c| c.code.as_str()).collect();
    let _ = writeln!(
        out,
        "# Field matrix: which answers each card needs, and where they come from\n"
    );
    let _ = writeln!(
        out,
        "Generated from `catalog/` by `cargo run -- render`. Do not edit by hand.\n\n\
         One answer covers every card that uses its key, so the bot never asks the same thing twice.\n"
    );
    for tier in [Tier::Vault, Tier::Recall, Tier::Ask] {
        let heading = match tier {
            Tier::Vault => "🔐 VAULT: identity, asked once, saved to the traveller's Vault entry",
            Tier::Recall => {
                "🧠 RECALL: found in memory, email or calendar before asking; saved to memory"
            }
            Tier::Ask => "✍️ ASK: declarations, asked fresh on every card, never saved",
        };
        let _ = writeln!(out, "## {heading}\n");
        let _ = writeln!(
            out,
            "| Key | Question | Source / saved to | {} |",
            codes.join(" | ")
        );
        let _ = writeln!(out, "|---|---|---|{}", "---|".repeat(codes.len()));
        for key in &cat.order {
            let f = &cat.fields[key];
            if f.tier != tier {
                continue;
            }
            let marks: Vec<&str> = cat
                .cards
                .iter()
                .map(|c| {
                    if c.fields().any(|cf| cf.key == *key) {
                        "✓"
                    } else {
                        ""
                    }
                })
                .collect();
            let _ = writeln!(
                out,
                "| `{}` | {} | {} | {} |",
                f.key,
                cell(&f.label),
                cell(&f.source()),
                marks.join(" | ")
            );
        }
        let _ = writeln!(out);
    }
    out
}

/// The onboarding checklist: every VAULT-tier field, the Vault slot that holds it, and the cards that need it.
pub fn vault_checklist(cat: &Catalog) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "### Vault fields to collect (generated from the catalog)\n\n\
         | Key | Ask as | Vault slot | Needed by |\n|---|---|---|---|"
    );
    for key in &cat.order {
        let f = &cat.fields[key];
        if f.tier != Tier::Vault {
            continue;
        }
        let cards: Vec<&str> = cat
            .cards
            .iter()
            .filter(|c| c.fields().any(|cf| cf.key == *key))
            .map(|c| c.code.as_str())
            .collect();
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} |",
            f.key,
            cell(&f.label),
            cell(&f.source()),
            cards.join(", ")
        );
    }
    out
}

/// Fill the `<!-- FIELDS -->` marker of a hand-written card skill with the generated reference.
pub fn expand_skill(source: &str, reference: &str) -> String {
    source.replace(FIELDS_MARKER, reference.trim_end())
}

/// Where the published files are served from; substituted for `{{RAW}}` in the setup header.
pub const RAW_BASE: &str = "https://raw.githubusercontent.com/juntao/grokbot-arrival-cards/main";

/// The bot's full definition in one file: the setup header (`bot/SETUP_HEADER.md`, written for the user's Main
/// Bot and ending on the `## PROFILE` heading), the profile, then every skill to save.
pub fn setup_bundle(header: &str, profile: &str, skills: &[(String, String)]) -> String {
    let mut out = header.trim_end().replace("{{RAW}}", RAW_BASE);
    let _ = writeln!(out, "\n\n{}\n\n---\n\n## SKILLS\n", profile.trim_end());
    for (name, body) in skills {
        let _ = writeln!(
            out,
            "### SKILL `{name}`\n\n````markdown\n{}\n````\n",
            body.trim_end()
        );
    }
    out
}

/// Render everything under `out_dir`: `FIELD_MATRIX.md`, `skills/*.md`, `PROFILE.md`, `SETUP_PROMPT.md`.
pub fn render_all(cat: &Catalog, bot_dir: &Path, out_dir: &Path) -> Result<Vec<String>> {
    let skills_out = out_dir.join("skills");
    fs::create_dir_all(&skills_out)
        .with_context(|| format!("creating {}", skills_out.display()))?;
    let mut written = Vec::new();
    let mut write = |path: &Path, text: &str| -> Result<()> {
        fs::write(path, text).with_context(|| format!("writing {}", path.display()))?;
        written.push(path.display().to_string());
        Ok(())
    };

    write(&out_dir.join("FIELD_MATRIX.md"), &matrix(cat))?;

    let read = |p: &Path| fs::read_to_string(p).with_context(|| format!("reading {}", p.display()));
    let profile = read(&bot_dir.join("PROFILE.md"))?;
    write(&out_dir.join("PROFILE.md"), &profile)?;

    // Onboarding and the orchestrator first, then the card skills in catalog order.
    let onboarding = read(&bot_dir.join("skills/arrival-card-onboarding.md"))?;
    if !onboarding.contains(VAULT_FIELDS_MARKER) {
        anyhow::bail!("arrival-card-onboarding has no {VAULT_FIELDS_MARKER} marker");
    }
    let mut skills = vec![
        (
            "arrival-card-onboarding".to_owned(),
            onboarding.replace(VAULT_FIELDS_MARKER, vault_checklist(cat).trim_end()),
        ),
        (
            "arrival-card-fill".to_owned(),
            read(&bot_dir.join("skills/arrival-card-fill.md"))?,
        ),
    ];
    for card in &cat.cards {
        let src = read(&bot_dir.join("skills").join(format!("{}.md", card.skill)))?;
        if !src.contains(FIELDS_MARKER) {
            anyhow::bail!("skill {} has no {FIELDS_MARKER} marker", card.skill);
        }
        skills.push((
            card.skill.clone(),
            expand_skill(&src, &card_reference(cat, card)),
        ));
    }
    for (name, body) in &skills {
        write(&skills_out.join(format!("{name}.md")), body)?;
    }
    write(
        &out_dir.join("SETUP_PROMPT.md"),
        &setup_bundle(&read(&bot_dir.join("SETUP_HEADER.md"))?, &profile, &skills),
    )?;
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_are_escaped() {
        assert_eq!(cell("a|b\nc"), "a\\|b c");
    }

    #[test]
    fn marker_is_replaced() {
        assert_eq!(
            expand_skill("x\n<!-- FIELDS -->\ny", "TABLE\n\n"),
            "x\nTABLE\ny"
        );
    }

    #[test]
    fn bundle_lists_every_skill_fenced() {
        let b = setup_bundle(
            "see {{RAW}}/x\n\n## PROFILE\n",
            "P",
            &[("a".into(), "body a".into()), ("b".into(), "body b".into())],
        );
        assert!(b.contains("## PROFILE\n\nP"));
        assert!(b.starts_with(&format!("see {RAW_BASE}/x")));
        assert!(b.contains("### SKILL `a`\n\n````markdown\nbody a\n````"));
        assert!(b.contains("### SKILL `b`"));
    }

    #[test]
    fn real_catalog_renders_every_card() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let cat = Catalog::load(&root.join("catalog")).expect("catalog loads");
        let m = matrix(&cat);
        assert!(m.contains("| `passport_no` |"));
        let v = vault_checklist(&cat);
        assert!(v.contains("| `passport_photo` |") && v.contains("| `passport_no` |"));
        assert!(!v.contains("`health_symptoms`") && !v.contains("`arrival_date`"));
        for card in &cat.cards {
            let r = card_reference(&cat, card);
            assert!(r.contains(&card.url));
            assert!(
                !r.contains("| ? |"),
                "{} has a field with an unknown tier",
                card.code
            );
        }
    }
}
