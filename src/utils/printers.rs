use std::fmt::Display;

use crate::{
    atomic_cards::{
        cardoids::Cardoid,
        cards::Card,
        types::{Side, Type},
    },
    proxy::Proxy,
    utils::{
        iter::IterExt,
        symbolics::{replace_symbols, DiscordEmoji, NothingReplacer, RulesTextSymbolReplacer},
    },
};

pub enum ToText<'a> {
    Proxy(&'a Proxy),
    Cardoid(&'a Cardoid),
    Card(&'a Card),
}

pub struct TextPrinter<'a, R>(pub &'a R, pub ToText<'a>)
where
    R: RulesTextSymbolReplacer;

impl<'a, R> Display for TextPrinter<'a, R>
where
    R: RulesTextSymbolReplacer<Item = String> + TextFormatting,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            ToText::Proxy(p) => {
                TextPrinter(self.0, ToText::Cardoid(&p.cardoid)).fmt(f)?;

                write!(f, "\n{}{}", self.0.line_start(), self.0.hr())?;

                if !p.category.is_empty() {
                    write!(f, "\n{}category: {}", self.0.line_start(), p.category)?;
                }
                if !p.tags.is_empty() {
                    write!(
                        f,
                        "\n{}tags: {}",
                        self.0.line_start(),
                        &p.tags.iter().map(Clone::clone).collvect().join(", ")
                    )?;
                }
                if p.repeats > 1 {
                    write!(f, "\n{}copies: {}", self.0.line_start(), p.repeats)?;
                }
            }
            ToText::Cardoid(c) => {
                let face = c.face();
                if let Some(b_side) = c.side(Side::B) {
                    write!(f, "{}{}", self.0.line_start(), self.0.em(&face.name))?;
                    write!(f, "\n{}{}\n", self.0.line_start(), self.0.strong("SIDE A"))?;
                    TextPrinter(self.0, ToText::Card(face)).fmt(f)?;
                    write!(f, "\n{}{}\n", self.0.line_start(), self.0.strong("SIDE B"))?;
                    TextPrinter(self.0, ToText::Card(b_side)).fmt(f)?;
                } else {
                    TextPrinter(self.0, ToText::Card(face)).fmt(f)?;
                }
            }
            ToText::Card(c) => {
                let mut name = &c.face_name;
                if name.is_empty() {
                    name = &c.name;
                }
                write!(
                    f,
                    "{}{} {}",
                    self.0.line_start(),
                    self.0.em(&name),
                    replace_symbols(self.0, &c.mana_cost).join("")
                )?;
                write!(
                    f,
                    "\n{}({}) {}",
                    self.0.line_start(),
                    self.0.indicator(&c.colors),
                    c.type_line
                )?;
                for line in c.text.lines() {
                    write!(
                        f,
                        "\n{}{}",
                        self.0.line_start(),
                        replace_symbols(self.0, line).join("")
                    )?;
                }
                if c.types.contains(&Type::Planeswalker) {
                    write!(f, "\n{}[{}]", self.0.line_start(), c.loyalty)?;
                }
                if c.types.contains(&Type::Battle) {
                    write!(f, "\n{}[{}]", self.0.line_start(), c.defense)?;
                }
                if c.types.contains(&Type::Creature) {
                    write!(f, "\n{}{}/{}", self.0.line_start(), c.power, c.toughness)?;
                }
            }
        };

        Ok(())
    }
}

trait TextFormatting {
    fn em(&self, text: &str) -> String;
    fn strong(&self, text: &str) -> String;
    fn line_start(&self) -> String;
    fn hr(&self) -> String;
}

impl TextFormatting for NothingReplacer {
    fn line_start(&self) -> String {
        "".to_string()
    }

    fn em(&self, text: &str) -> String {
        text.to_string()
    }

    fn strong(&self, text: &str) -> String {
        text.to_string()
    }

    fn hr(&self) -> String {
        "###".to_string()
    }
}

impl TextFormatting for DiscordEmoji {
    fn line_start(&self) -> String {
        "> ".to_string()
    }

    fn em(&self, text: &str) -> String {
        format!("*{}*", text)
    }

    fn strong(&self, text: &str) -> String {
        format!("**{}**", text)
    }

    fn hr(&self) -> String {
        "".to_string()
    }
}
