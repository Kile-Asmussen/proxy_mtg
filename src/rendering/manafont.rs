use crate::{
    atomic_cards::types::WUBRG,
    html::{Element, Node, Tag},
    utils::symbolics::RulesTextSymbolReplacer,
};

pub struct ManaFontSymbolics;

impl RulesTextSymbolReplacer for ManaFontSymbolics {
    type Item = Node;

    fn map_symbol(&self, matched: &str) -> Self::Item {
        fn mana(c: &str) -> Node {
            Node::Element(Element::new(Tag::i).class(["ms", c, "ms-cost", "ms-shadow"]))
        }

        // fn split_mana(c: &str) -> Element {
        //     Element::new(Tag::i).class(["ms", c, "ms-cost", "ms-shadow"])
        // }

        match matched {
            "{C}" => mana("ms-c"),
            "{W}" => mana("ms-w"),
            "{U}" => mana("ms-u"),
            "{B}" => mana("ms-b"),
            "{R}" => mana("ms-r"),
            "{G}" => mana("ms-g"),
            "{S}" => mana("ms-s"),
            "{1}" => mana("ms-1"),
            "{2}" => mana("ms-2"),
            "{3}" => mana("ms-3"),
            "{4}" => mana("ms-4"),
            "{5}" => mana("ms-5"),
            "{6}" => mana("ms-6"),
            "{7}" => mana("ms-7"),
            "{8}" => mana("ms-8"),
            "{9}" => mana("ms-9"),
            "{10}" => mana("ms-10"),
            "{11}" => mana("ms-11"),
            "{12}" => mana("ms-12"),
            "{13}" => mana("ms-13"),
            "{14}" => mana("ms-14"),
            "{15}" => mana("ms-15"),
            "{16}" => mana("ms-16"),
            "{17}" => mana("ms-17"),
            "{18}" => mana("ms-18"),
            "{19}" => mana("ms-19"),
            "{20}" => mana("ms-20"),
            "{T}" => mana("ms-tap"),
            "{U}" => mana("ms-untap"),
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
            s => Node::Text(s.to_string()),
        }
    }

    fn intermediate_text(&self, non_matched: &str) -> Self::Item {
        Node::Text(non_matched.to_string())
    }

    fn joiner(&self) -> Option<Self::Item> {
        None
    }

    fn indicator(
        &self,
        indicate: &std::collections::BTreeSet<crate::atomic_cards::types::WUBRG>,
    ) -> Self::Item {
        if indicate.is_empty() {
            return Node::Text("".to_string());
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
