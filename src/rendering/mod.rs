pub mod dual_faced;
pub mod general;
pub mod manafont;
pub mod normal;
pub mod parsing;
pub mod reminders;
pub mod tokens;
pub mod verticalia;

use normal::normal_layout_proxy;

use crate::{
    atomic_cards::types::CardLayout,
    html::{Document, Element, Tag},
    proxy::Proxy,
    rendering::{
        dual_faced::{flip_layout_proxy, genuine_dual_face_proxy},
        general::empty_card,
        tokens::token_proxy,
        verticalia::{class_layout_proxy, saga_layout_proxy},
    },
};

#[derive(Clone, Copy)]
pub struct RenderSettings {
    pub in_color: bool,
    pub testing: bool,
    pub remninder_text: Option<bool>,
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
                CardLayout::Normal => normal_layout_proxy(proxy),
                CardLayout::Class => class_layout_proxy(proxy),
                CardLayout::Saga => saga_layout_proxy(proxy),
                CardLayout::Flip => flip_layout_proxy(proxy),
                CardLayout::Transform => genuine_dual_face_proxy(proxy),
                CardLayout::ModalDfc => genuine_dual_face_proxy(proxy),
                CardLayout::Token => token_proxy(proxy),
                _ => vec![empty_card(proxy.cardoid.face(), proxy)],
            })
        }
    }

    pub fn into_file(self) -> anyhow::Result<Document> {
        let mut html_pages = Document::new()
            .title("PROXIES")
            .head_link("preconnect", "https://fonts.googleapis.com")
            .head(Element::new(Tag::link).attr("rel", "preconnect").attr("href", "https://fonts.gstatic.com").flag("crossorigin"))
            .head_link("stylesheet", "https://fonts.googleapis.com/css2?family=Amarante&family=Crimson+Text:ital,wght@0,400;0,600;0,700;1,400;1,600;1,700&family=Inconsolata:wght@200..900&display=swap")
            .head_link("stylesheet", "https://cdn.jsdelivr.net/npm/mana-font@latest/css/mana.css");

        if self.settings.testing {
            html_pages = html_pages
                .head_link("stylesheet", "../css/page-layout.css")
                .head_link("stylesheet", "../css/font-settings.css")
                .head_link("stylesheet", "../css/card-geometry.css");

            html_pages = if self.settings.in_color {
                html_pages.head_link("stylesheet", "../css/full-color.css")
            } else {
                html_pages.head_link("stylesheet", "../css/monochrome.css")
            }
            .head_link("stylesheet", "../css/card-colors.css");
        } else {
            html_pages = html_pages
                .inline_style("./css/page-layout.css")?
                .inline_style("./css/font-settings.css")?
                .inline_style("./css/card-geometry.css")?;

            html_pages = if self.settings.in_color {
                html_pages.inline_style("./css/full-color.css")?
            } else {
                html_pages.inline_style("./css/monochrome.css")?
            }
            .inline_style("./css/card-colors.css")?;
        }

        let mut pages = vec![];
        {
            let mut page = vec![];
            let mut row = vec![];
            for card in self.cards {
                if page.len() >= 3 {
                    pages.push(page);
                    page = vec![];
                }
                if row.len() >= 3 {
                    page.push(row);
                    row = vec![];
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
                    html_row = html_row.node(card);
                }

                html_page = html_page.node(html_row);
            }

            html_pages = html_pages.body(html_page);
        }

        Ok(html_pages)
    }
}
