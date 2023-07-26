use crate::{cards::Card, proxy_builder::ProxyBuilder};

macro_rules!const_template_field{
    ($($name:ident),+) => {
        $(
            const $name : &'static str = stringify!(<!--$name-->);
        )+
    }
}

pub struct SvgCard(String);
pub struct SvgNormal(SvgCard);
pub struct SvgReverse(SvgCard, SvgCard);
pub struct SvgSaga(SvgCard);

impl SvgCard {
    fn new() -> Self {
        Self(format!(
            r#"{Self::CARD_NAME}
{Self::MANA_COST}
{Self::TYPE_LINE}"#
        ))
    }

    fn add_css(&mut self, css: &str) -> &mut self {
        self.0.replace(Self::CSS, css);
        self
    }

    fn proxy_card(&mut self, card: &[Card]) -> &mut Self {
        let Some(card) = card.first() else {
            return self;
        };

        self.name(&card.name);
        self.type_line(&card.type_line);
        self.mana_cost(&card.type_line);
        self
    }

    const_template_field!(
        CSS
        CARD_NAME,
        MANA_COST,
        ART_FILENAME,
        TYPE_LINE,
        RULES_TEXT,
        FLAVOR_TEXT,
        ART_CREDITS
    );
}

impl SvgNormal {}

impl ProxyBuilder for SvgCard {
    type Output = String;

    fn build(&self) -> Self::Output {
        self.0.clone()
    }

    fn name(&mut self, name: &str) -> &mut Self {
        self.0.replace(Self::CARD_NAME, name);
        self
    }

    fn type_line(&mut self, type_line: &str) -> &mut Self {
        self.0.replace(from, to)
    }

    fn mana_cost(&mut self, mana_cost: &str) -> &mut Self {
        todo!()
    }

    fn art_filename(&mut self, art_filename: &std::path::Path) -> &mut Self {
        todo!()
    }

    fn art_credits(&mut self, artist: &str) -> &mut Self {
        todo!()
    }

    fn border_color(&mut self, color: crate::proxy_builder::BorderColor) -> &mut Self {
        todo!()
    }

    fn legendary_decor(&mut self, is_legedary: bool) -> &mut Self {
        todo!()
    }
}
