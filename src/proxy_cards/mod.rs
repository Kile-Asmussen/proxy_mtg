mod normal;
mod utils;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    atomic_cards::{AtomicCards, Cardoid},
    decklist::{Artoid, DeckList},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ProxyDeck {
    pub proxies: Vec<Proxy>,
    pub tags: BTreeMap<String, usize>,
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
    pub fn build(atomic: &AtomicCards, deck: &DeckList) -> Result<ProxyDeck, String> {
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

        Ok(ProxyDeck {
            proxies,
            tags: BTreeMap::new(),
        })
    }
}
