mod normal;
mod utils;

use crate::{atomic_cards::Cardoid, decklist::Artoid};

pub struct ProxyDeck {
    pub proxies: Vec<Proxy>,
}

pub struct Proxy {
    pub cardoid: Cardoid,
    pub card_artoid: Artoid,
}

pub struct HtmlProxyDeck(pub Vec<HtmlProxyPage>);
pub struct HtmlProxyPage(pub Vec<HtmlProxyCard>);

pub struct HtmlProxyCard(pub String);
