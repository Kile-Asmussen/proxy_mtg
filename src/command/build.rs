use clap::Parser;
use itertools::{EitherOrBoth, Itertools};
use std::path::{Path, PathBuf};

use crate::{
    atomic_cards::types::CardLayout,
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
    #[arg(long, conflicts_with = "scryfall_art")]
    pub all_scryfall_art: bool,
    #[arg(long, conflicts_with = "all_scryfall_art")]
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
            scryfall: if self.all_scryfall_art {
                Some(true)
            } else if self.scryfall_art {
                None
            } else {
                Some(false)
            },
        };
        let mut render = RenderContext::new(settings)?;

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

            if proxy.layout() != &CardLayout::Token {
                if let Some(true) = settings.scryfall {
                    let name = proxy.name.clone();
                    proxy.set_scryfall_arts(|| {
                        render.scryfall_client.get_scryfall_card_art(&name)
                    })?;
                } else if let None = settings.scryfall {
                    let name = proxy.name.clone();
                    proxy.add_scryfall_arts(|| {
                        render.scryfall_client.get_scryfall_card_art(&name)
                    })?;
                }
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
