use crate::{
    atomic_cards::types::WUBRG,
    html::{Element, Node, Tag},
    rendering::general::cost_symbol,
    utils::{symbolics::RulesTextSymbolReplacer, ToS},
};

#[derive(Default, Clone, Copy)]
pub struct ManaFontSymbolics;

impl RulesTextSymbolReplacer for ManaFontSymbolics {
    type Item = Node;

    fn map_symbol(&self, matched: &str) -> Self::Item {
        // fn split_mana(c: &str) -> Element {
        //     Element::new(Tag::i).class(["ms", c, "ms-cost", "ms-shadow"])
        // }

        match matched {
            "{C}" => cost_symbol("ms-c"),
            "{W}" => cost_symbol("ms-w"),
            "{U}" => cost_symbol("ms-u"),
            "{B}" => cost_symbol("ms-b"),
            "{R}" => cost_symbol("ms-r"),
            "{G}" => cost_symbol("ms-g"),
            "{S}" => cost_symbol("ms-s"),
            "{1}" => cost_symbol("ms-1"),
            "{2}" => cost_symbol("ms-2"),
            "{3}" => cost_symbol("ms-3"),
            "{4}" => cost_symbol("ms-4"),
            "{5}" => cost_symbol("ms-5"),
            "{6}" => cost_symbol("ms-6"),
            "{7}" => cost_symbol("ms-7"),
            "{8}" => cost_symbol("ms-8"),
            "{9}" => cost_symbol("ms-9"),
            "{10}" => cost_symbol("ms-10"),
            "{11}" => cost_symbol("ms-11"),
            "{12}" => cost_symbol("ms-12"),
            "{13}" => cost_symbol("ms-13"),
            "{14}" => cost_symbol("ms-14"),
            "{15}" => cost_symbol("ms-15"),
            "{16}" => cost_symbol("ms-16"),
            "{17}" => cost_symbol("ms-17"),
            "{18}" => cost_symbol("ms-18"),
            "{19}" => cost_symbol("ms-19"),
            "{20}" => cost_symbol("ms-20"),
            "{T}" => cost_symbol("ms-tap"),
            "{Q}" => cost_symbol("ms-untap"),
            // "{W/P}" => "(:sunny:/:drop_of_blood:)",
            // "{U/P}" => "(:droplet:/:drop_of_blood:)",
            // "{B/P}" => "(:skull:/:drop_of_blood:)",
            // "{R/P}" => "(:fire:/:drop_of_blood:)",
            // "{G/P}" => "(:deciduous_tree:/:drop_of_blood:)",
            // "{C/P}" => "(:diamond_shape_with_a_dot_inside:/:drop_of_blood:)",
            // "{2/W}" => "(:two:/:sunny:)",
            // "{2/U}" => "(:two:/:droplet:)",
            // "{2/B}" => "(:two:/:skull:)",
            // "{2/R}" => "(:two:/:fire:)",
            // "{2/G}" => "(:two:/:deciduous_tree:)",
            // "{C/W}" => "(:diamond_shape_with_a_dot_inside:/:sunny:)",
            // "{C/U}" => "(:diamond_shape_with_a_dot_inside:/:droplet:)",
            // "{C/B}" => "(:diamond_shape_with_a_dot_inside:/:skull:)",
            // "{C/R}" => "(:diamond_shape_with_a_dot_inside:/:fire:)",
            // "{C/G}" => "(:diamond_shape_with_a_dot_inside:/:deciduous_tree:)",
            s => return s.into(),
        }
        .into()
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        Node::Text(non_matched.s())
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }

    fn indicator(
        &self,
        indicate: &std::collections::BTreeSet<crate::atomic_cards::types::WUBRG>,
    ) -> Self::Item {
        if indicate.is_empty() {
            return Node::Text("".s());
        }

        let class = vec![
            format!("ms"),
            format!("ms-ci"),
            format!("ms-ci-{}", indicate.len()),
            format!("ms-ci-{}", WUBRG::render(indicate).to_lowercase()),
        ];

        Node::Element(Element::new(Tag::i).class(class))
    }
}
