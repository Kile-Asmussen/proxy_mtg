use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScryfallCard {
    #[serde(flatten, default)]
    pub card_face: ScryfallArt,
    #[serde(default)]
    pub card_faces: Vec<ScryfallArt>,
}

#[derive(Deserialize, Default)]
pub struct ScryfallArt {
    #[serde(default)]
    pub artist: String,
    #[serde(default)]
    pub image_uris: ImageUris,
}

#[derive(Deserialize, Default)]
pub struct ImageUris {
    pub art_crop: String,
}

pub struct ScryfallClient {
    pub force: bool,
    pub client: Client,
}

impl ScryfallClient {
    fn new(force: bool) -> anyhow::Result<Self> {
        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()?;

        Ok(Self { force, client })
    }

    fn get_scryfall_card_art(&self, card_name: &str) -> anyhow::Result<ScryfallCard> {
        let request = self
            .client
            .get(format!("https://api.scryfall.com/cards/named/"))
            .query(&[("exact", card_name)])
            .header(
                "User-Agent",
                format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            )
            .header("Accepts", "text/json")
            .build()?;

        let response = self.client.execute(request)?;

        let body = response.bytes()?.to_vec();

        Ok(serde_json::from_slice(&body)?)
    }
}

#[test]
fn scryfall_client_tests() -> anyhow::Result<()> {
    let client = ScryfallClient::new(false)?;
    let card = client.get_scryfall_card_art("Keeper of the Accord")?;

    assert_ne!(card.card_face.artist, "");
    assert_ne!(card.card_face.image_uris.art_crop, "");
    Ok(())
}
