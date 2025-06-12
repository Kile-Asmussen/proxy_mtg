use clap::Parser;
use std::path::{Path, PathBuf};

use crate::{
    proxy::decklists::DeckList,
    rendering::{RenderContext, RenderSettings},
};

#[derive(Parser, Debug, Clone)]
pub struct Build {
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    #[arg(value_name = "FILE")]
    pub decklist: PathBuf,
    #[arg(long)]
    pub in_color: bool,
    #[arg(long)]
    pub testing: bool,
    #[arg(long, conflicts_with = "no_reminder_text")]
    pub reminder_text: bool,
    #[arg(long, conflicts_with = "reminder_text")]
    pub no_reminder_text: bool,
    #[arg(long)]
    pub scryfall_art: bool,
}

impl Build {
    pub fn decklist_file(&self) -> &Path {
        &self.decklist
    }

    pub fn dispatch(&self, decklist: &mut DeckList) -> anyhow::Result<()> {
        let settings = RenderSettings {
            in_color: self.in_color,
            testing: self.testing,
            remninder_text: if self.reminder_text {
                Some(true)
            } else if self.no_reminder_text {
                Some(false)
            } else {
                None
            },
        };
        let mut render = RenderContext::new(settings);

        println!(
            "Rendering {} cards",
            decklist
                .iter()
                .map(|p| p.cardoid.printed_cards() * p.repeats)
                .sum::<usize>()
        );

        for proxy in decklist {
            if let Some(b) = settings.remninder_text {
                proxy.reminder_text = b;
            }
            render.add_proxy(proxy);
        }

        if let Some(output) = &self.output {
            std::fs::write(&output, format!("{}", render.into_file()?))?;
        } else {
            println!("{}", render.into_file()?);
        }

        Ok(())
    }
}
