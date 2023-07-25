use regex::{Captures, Regex};

use crate::cards::{AtomicCards, Card, Layout};

fn english_flavor_text(card: &Card) -> Option<&str> {
    card.foreign_data
        .iter()
        .find(|foreign| foreign.language == "English")
        .map(|foreign| foreign.flavor_text.as_str())
}

pub trait ProxyTemplate {
    type Output;

    fn applies_to(&self, layout: Layout) -> bool;

    fn from_cards(&self, cards: &[Card]) -> Option<Self::Output>;

    fn proxy(&self, name: &str, atomic: &AtomicCards) -> Option<Self::Output> {
        let cards = &atomic.data.get(name)?[..];

        if !cards.iter().all(|c| self.applies_to(c.layout)) {
            return None;
        }

        self.from_cards(cards)
    }
}

pub struct TemplateSet<T>(Vec<T>);

impl<T: ProxyTemplate> ProxyTemplate for TemplateSet<T> {
    type Output = <T as ProxyTemplate>::Output;

    fn applies_to(&self, layout: Layout) -> bool {
        self.0.iter().any(|t| t.applies_to(layout))
    }

    fn from_cards(&self, cards: &[Card]) -> Option<Self::Output> {
        for t in &self.0 {
            if t.applies_to(cards[0].layout) {
                return t.from_cards(cards);
            }
        }
        None
    }
}

pub struct SimpleTemplate;

impl ProxyTemplate for SimpleTemplate {
    type Output = String;

    fn applies_to(&self, layout: Layout) -> bool {
        layout == Layout::Normal
    }

    fn from_cards(&self, cards: &[Card]) -> Option<Self::Output> {
        if cards.len() != 1 {
            return None;
        }

        let card = &cards[0];

        Some(format!(
            r#"----
{name}   {mc}
{type_line}
{rules_text}
----"#,
            name = card.name,
            mc = card.mana_cost,
            type_line = card.type_line,
            rules_text = card.text
        ))
    }
}

pub struct DiscordTemplate;

impl ProxyTemplate for DiscordTemplate {
    type Output = String;

    fn applies_to(&self, layout: Layout) -> bool {
        layout == Layout::Normal
    }

    fn from_cards(&self, cards: &[Card]) -> Option<Self::Output> {
        if cards.len() != 1 {
            return None;
        }
        let card = &cards[0];

        let mana_cost = Self::replace_symbols(&card.mana_cost);
        let text = Self::replace_symbols(&card.text).replace('\n', "\n> ");

        Some(format!(
            r#"> {name} {mc}
> {type_line}
> {rules_text}"#,
            name = card.name,
            mc = mana_cost,
            type_line = card.type_line,
            rules_text = text
        ))
    }
}

impl DiscordTemplate {
    fn replace_symbols(text: &str) -> String {
        let text = text
            .replace("{W}", ":sunny:")
            .replace("{U}", ":droplet:")
            .replace("{B}", ":skull:")
            .replace("{R}", ":fire:")
            .replace("{G}", ":deciduous_tree: ")
            .replace("{T}", ":arrow_heading_down:");

        Regex::new(r"\{(\d+)\}")
            .unwrap()
            .replace_all(&text, |cap: &Captures<'_>| {
                if let Some(m) = cap.get(1) {
                    match m.as_str() {
                        "0" => ":zero:",
                        "1" => ":one:",
                        "2" => ":two:",
                        "3" => ":three:",
                        "4" => ":four:",
                        "5" => ":five:",
                        "6" => ":six:",
                        "7" => ":seven:",
                        "8" => ":eight:",
                        "9" => ":nine:",
                        s => return format!("**[{}]**", s),
                    }
                } else {
                    cap.get(0).unwrap().as_str()
                }
                .into()
            })
            .into_owned()
    }
}
