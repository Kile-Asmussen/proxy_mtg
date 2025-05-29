use build_html::Html;
use clap::Parser;
use std::path::PathBuf;

use crate::{
    html::{RenderContext, RenderSettings},
    proxy::decklist::DeckList,
};

#[derive(Parser, Debug, Clone)]
pub struct Build {
    #[arg(short, long, value_name = "FILE")]
    pub output: PathBuf,
}

impl Build {
    pub fn dispatch(&self, decklist: &DeckList) -> anyhow::Result<()> {
        let mut render = RenderContext::new(RenderSettings {
            color: true,
            reminder_text: true,
        });

        for proxy in decklist {
            render.add_proxy(proxy);
        }

        std::fs::write(&self.output, render.into_file().to_html_string())?;

        Ok(())
    }
}
