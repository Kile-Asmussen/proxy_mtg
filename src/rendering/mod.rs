pub mod general;
pub mod manafont;
pub mod normal;

use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
};

use normal::normal_card;

use crate::{
    atomic_cards::types::CardLayout,
    html::{Document, Element, Node, Tag},
    proxy::Proxy,
    rendering::general::empty_card,
};

pub struct RenderSettings {
    pub in_color: bool,
    pub reminder_text: bool,
}

pub struct RenderContext {
    pub settings: RenderSettings,
    pub cards: Vec<Element>,
}

impl RenderContext {
    pub fn new(settings: RenderSettings) -> Self {
        Self {
            settings,
            cards: vec![],
        }
    }

    pub fn add_proxy(&mut self, proxy: &Proxy) {
        for _ in 1..=proxy.repeats {
            self.cards.append(&mut match proxy.layout() {
                CardLayout::Normal => normal_card(proxy, &self.settings),
                _ => vec![empty_card(proxy.cardoid.face())],
            })
        }
    }

    pub fn into_file(self) -> Document {
        let mut html_pages = Document::new()
            .head(Element::new(Tag::title).text("PROXIES"))
            .head_link("preconnect", "https://fonts.googleapis.com")
            .head(Element::new(Tag::link).attr("rel", "preconnect").attr("href", "https://fonts.gstatic.com").flag("crossorigin"))
            .head_link("stylesheet", "https://fonts.googleapis.com/css2?family=Amarante&family=Crimson+Text:ital,wght@0,400;0,600;0,700;1,400;1,600;1,700&family=Inconsolata:wght@200..900&display=swap")
            .head_link("stylesheet", "https://cdn.jsdelivr.net/npm/mana-font@latest/css/mana.css")
            .head_link("stylesheet", "../css/colors.css")
            .head_link("stylesheet", "../css/dimensions.css")
            .head_link("stylesheet", "../css/fonts.css")
            .head_link("stylesheet", "../css/page.css")
            .head_link("stylesheet", "../css/card.css")
            .head_link("stylesheet", "../css/magic-font.css");

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
            let mut html_page = Element::new(Tag::div).class(["page"]);

            for row in page {
                let mut html_row = Element::new(Tag::div).class(["card-row"]);

                for card in row {
                    html_row = html_row.elem(card);
                }

                html_page = html_page.elem(html_row);
            }

            html_pages = html_pages.body(Node::Element(html_page));
        }

        html_pages
    }
}
