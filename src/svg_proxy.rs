use crate::{cards::Layout, proxy::ProxyTemplate};

struct SvgProxy;

impl ProxyTemplate for SvgProxy {
    type Output = String;

    fn applies_to(&self, layout: crate::cards::Layout) -> bool {
        layout == Layout::Normal
    }

    fn generate(&self, cards: &[crate::cards::Card]) -> Option<Self::Output> {
        let card = cards.first()?;

        return None;
    }
}
