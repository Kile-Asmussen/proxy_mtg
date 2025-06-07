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
    pub color: bool,
    #[arg(long)]
    pub testing: bool,
}

impl Build {
    pub fn decklist_file(&self) -> &Path {
        &self.decklist
    }

    pub fn dispatch(&self, decklist: &DeckList) -> anyhow::Result<()> {
        let mut render = RenderContext::new(RenderSettings {
            in_color: self.color,
            testing: self.testing,
        });

        println!(
            "Rendering {} cards",
            decklist
                .iter()
                .map(|p| p.cardoid.printed_cards())
                .sum::<usize>()
        );

        for proxy in decklist {
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
