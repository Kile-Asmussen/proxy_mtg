use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

use build_html::{HtmlContainer, HtmlElement, HtmlPage, HtmlTag};
use fragments::HtmlExt;
use normal::normal_card;

use crate::{atomic_cards::types::Layout, proxy::Proxy};

pub mod fragments;
pub mod general;
pub mod normal;

pub struct RenderSettings {
    pub color: bool,
    pub reminder_text: bool,
}

pub struct RenderContext {
    pub settings: RenderSettings,
    pub cards: Vec<HtmlElement>,
}

impl RenderContext {
    pub fn new(settings: RenderSettings) -> Self {
        Self {
            settings,
            cards: vec![],
        }
    }

    pub fn add_proxy(&mut self, proxy: &Proxy) {
        use Layout::*;
        let card = match proxy.layout() {
            Adventure => todo!(),
            Aftermath => todo!(),
            ArtSeries => todo!(),
            Augment => todo!(),
            Case => todo!(),
            Class => todo!(),
            DoubleFacedToken => todo!(),
            Emblem => todo!(),
            Flip => todo!(),
            Host => todo!(),
            Leveler => todo!(),
            Meld => todo!(),
            ModalDfc => todo!(),
            Mutate => todo!(),
            Normal => normal_card(proxy, &self.settings),
            Planar => todo!(),
            Prototype => todo!(),
            ReversibleCard => todo!(),
            Saga => todo!(),
            Scheme => todo!(),
            Split => todo!(),
            Token => todo!(),
            Transform => todo!(),
            Vanguard => todo!(),
        };

        for _ in 0..proxy.repeats {
            self.cards.push(card.clone())
        }
    }

    pub fn into_file(self) -> HtmlPage {
        let mut html_pages = HtmlPage::new()
            .with_title("PROXIES")
            .with_head_link("https://fonts.googleapis.com", "preconnect")
            .with_head_link_attr(
                "https://fonts.gstatic.com",
                "preconnect",
                [("crossorigin", "crossorigin")],
            )
            .with_stylesheet("https://fonts.googleapis.com/css2?family=Amarante&family=Crimson+Text:ital,wght@0,400;0,600;0,700;1,400;1,600;1,700&family=Inconsolata:wght@200..900&display=swap")
            .with_stylesheet("../css/colors.css")
            .with_stylesheet("../css/dimensions.css")
            .with_stylesheet("../css/fonts.css")
            .with_stylesheet("../css/page.css")
            .with_stylesheet("../css/card.css")
            .with_stylesheet("../css/magic-font.css");

        let mut pages = vec![];
        {
            let mut page = vec![];
            let mut row = vec![];
            for card in self.cards {
                if page.len() >= 3 {
                    pages.push(mem::replace(&mut page, vec![]))
                }
                if row.len() >= 3 {
                    page.push(mem::replace(&mut row, vec![]))
                }
                row.push(card);
            }
            if !row.is_empty() {
                page.push(row);
            }
            if !page.is_empty() {
                pages.push(page);
            }
        }

        for page in pages {
            let mut html_page = HtmlElement::new(HtmlTag::Div).with_classes(["page"]);

            for row in page {
                let mut html_row = HtmlElement::new(HtmlTag::Div).with_classes(["card-row"]);

                for card in row {
                    html_row.add_element(card);
                }

                html_page.add_element(html_row);
            }

            html_pages.add_html(html_page);
        }

        html_pages
    }
}
