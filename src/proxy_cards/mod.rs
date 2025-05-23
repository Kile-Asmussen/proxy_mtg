mod normal;
mod utils;

use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{AtomicCards, Cardoid},
    decklist::{Artoid, DeckList},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProxyDeck {
    pub proxies: Vec<Proxy>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Proxy {
    pub cardoid: Cardoid,
    pub artoid: Artoid,
}

pub struct HtmlProxyDeck(pub Vec<HtmlProxyPage>);
pub struct HtmlProxyPage(pub Vec<HtmlProxyCard>);

pub struct HtmlProxyCard(pub String);

impl ProxyDeck {
    pub fn new(atomic: &AtomicCards, deck: &DeckList) -> Result<ProxyDeck, String> {
        let artoids = deck.vec();

        let mut proxies = vec![];

        for artoid in &artoids {
            if atomic.data.contains_key(&artoid.name) {
                proxies.push(Proxy {
                    artoid: artoid.clone(),
                    cardoid: atomic.data[&artoid.name].clone(),
                })
            } else {
                return Err(artoid.name.clone());
            }
        }

        Ok(ProxyDeck { proxies })
    }
}
