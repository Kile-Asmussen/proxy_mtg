use clap::Parser;
use std::path::PathBuf;

use crate::{
    proxy::decklists::DeckList,
    rendering::{RenderContext, RenderSettings},
};

#[derive(Parser, Debug, Clone)]
pub struct Build {
    #[arg(value_name = "FILE")]
    pub output: PathBuf,
    #[arg(long)]
    pub reminders: bool,
    #[arg(long)]
    pub color: bool,
}

impl Build {
    pub fn dispatch(&self, decklist: &DeckList) -> anyhow::Result<()> {
        let mut render = RenderContext::new(RenderSettings {
            color: self.color,
            reminder_text: self.reminders,
        });

        for proxy in decklist {
            render.add_proxy(proxy);
        }

        std::fs::write(&self.output, format!("{}", render.into_file()))?;

        Ok(())
    }
}
