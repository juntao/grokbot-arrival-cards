//! Build tool for the Arrival Card Grok Bot template.
//!
//! The bot itself runs only on skill text (Grok Bot templates carry no custom code). This tool keeps that text
//! consistent: it validates the field catalog and renders the card skills, the field matrix and the setup message.

mod catalog;
mod render;

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use catalog::Catalog;

#[derive(Parser)]
#[command(
    version,
    about = "Validate and render the Arrival Card Grok Bot template"
)]
struct Cli {
    /// Directory holding fields.toml and cards/*.toml.
    #[arg(long, default_value = "catalog")]
    catalog: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate the catalog; exits non-zero on any issue.
    Check,
    /// Validate, then render PROFILE, skills, FIELD_MATRIX and SETUP_PROMPT into the output directory.
    Render {
        /// Hand-written profile and skills.
        #[arg(long, default_value = "bot")]
        bot: PathBuf,
        /// Output directory for the generated, paste-ready files.
        #[arg(long, default_value = "dist")]
        out: PathBuf,
    },
    /// Print one card's field reference (as embedded in its skill) to stdout.
    Card {
        /// Country code: CN, JP, SG or TW.
        code: String,
    },
}

fn run(cli: Cli) -> Result<ExitCode> {
    let cat = Catalog::load(&cli.catalog)?;
    let issues = cat.validate();
    if !issues.is_empty() {
        for i in &issues {
            eprintln!("error: {i}");
        }
        eprintln!("{} issue(s) in {}", issues.len(), cli.catalog.display());
        return Ok(ExitCode::FAILURE);
    }
    match cli.command {
        Command::Check => {
            let n: usize = cat.cards.iter().map(|c| c.fields().count()).sum();
            println!(
                "ok: {} canonical fields, {} cards, {n} card fields",
                cat.fields.len(),
                cat.cards.len()
            );
        }
        Command::Render { bot, out } => {
            for path in render::render_all(&cat, &bot, &out)? {
                println!("wrote {path}");
            }
        }
        Command::Card { code } => match cat.card(&code) {
            Some(card) => print!("{}", render::card_reference(&cat, card)),
            None => {
                let known: Vec<&str> = cat.cards.iter().map(|c| c.code.as_str()).collect();
                eprintln!("unknown card `{code}`; known: {}", known.join(", "));
                return Ok(ExitCode::FAILURE);
            }
        },
    }
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::FAILURE
        }
    }
}
