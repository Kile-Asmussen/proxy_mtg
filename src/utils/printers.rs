use std::fmt::Display;

use crate::{
    atomic_cards::{
        cardoids::Cardoid,
        cards::Card,
        types::{Side, Type, WUBRG},
    },
    proxy::Proxy,
    utils::iter::IterExt,
};

pub enum ToText<'a> {
    Proxy(&'a Proxy),
    Cardoid(&'a Cardoid),
    Card(&'a Card),
}

impl<'a> Display for ToText<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToText::Proxy(p) => {
                ToText::Cardoid(&p.cardoid).fmt(f)?;

                write!(f, "\n###")?;

                if !p.category.is_empty() {
                    write!(f, "\ncategory: {}", p.category)?;
                }
                if !p.tags.is_empty() {
                    write!(
                        f,
                        "\ntags: {}",
                        &p.tags.iter().map(Clone::clone).collvect().join(", ")
                    )?;
                }
                if p.repeats > 1 {
                    write!(f, "\ncopies: {}", p.repeats)?;
                }
            }
            ToText::Cardoid(c) => {
                let face = c.face();
                if let Some(b_side) = c.side(Side::B) {
                    write!(f, "{}", &face.name)?;
                    write!(f, "\nSIDE A\n")?;
                    ToText::Card(face).fmt(f)?;
                    write!(f, "\nSIDE B\n")?;
                    ToText::Card(b_side).fmt(f)?;
                } else {
                    ToText::Card(face).fmt(f)?;
                }
            }
            ToText::Card(c) => {
                let mut name = &c.face_name;
                if name.is_empty() {
                    name = &c.name;
                }
                write!(f, "{} {}", &name, &c.mana_cost)?;
                write!(f, "\n({}) {}", WUBRG::render(&c.colors), c.type_line)?;
                for line in c.text.lines() {
                    write!(f, "\n{}", line)?;
                }
                if c.types.contains(&Type::Planeswalker) {
                    write!(f, "\n[{}]", c.loyalty)?;
                }
                if c.types.contains(&Type::Battle) {
                    write!(f, "\n<{}>", c.defense)?;
                }
                if c.types.contains(&Type::Creature) {
                    write!(f, "\n{}/{}", c.power, c.toughness)?;
                }
            }
        };

        Ok(())
    }
}
