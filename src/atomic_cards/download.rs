use std::time::Instant;

use crate::atomic_cards::AtomicCardsFile;

impl AtomicCardsFile {
    pub fn load_raw_file(verbose: bool) -> anyhow::Result<Vec<u8>> {
        if !(std::fs::exists(Self::ATOMIC_CARDS_DUMP)?) {
            return Ok(std::fs::read(Self::ATOMIC_CARDS_DUMP)?);
        }

        let start = Instant::now();
        if verbose {
            eprintln!("{} file not found, downloading...", Self::ATOMIC_CARDS_DUMP);
        }

        let client = reqwest::blocking::ClientBuilder::new()
            .timeout(None)
            .build()?;

        let request = client.get(Self::ATOMIC_CARDS_URL).build()?;

        let response = client.execute(request)?;

        let downloaded = response.bytes()?.to_vec();

        std::fs::write(Self::ATOMIC_CARDS_DUMP, &downloaded)?;

        if verbose {
            eprintln!(
                "Downloaded {} megabytes in {} seconds.",
                downloaded.len() / 1024 / 1000,
                start.elapsed().as_secs()
            );
        }

        return Ok(downloaded);
    }

    pub fn load_json(verbose: bool) -> anyhow::Result<Self> {
        let start = Instant::now();

        let atomic_cards = if std::fs::exists(Self::ATOMIC_CARDS_FILE)? {
            let data = std::fs::read(Self::ATOMIC_CARDS_FILE)?;

            serde_json::from_slice(&data)?
        } else {
            let data = Self::load_raw_file(verbose)?;

            if verbose {
                eprintln!("{} not found, recreating...", Self::ATOMIC_CARDS_FILE);
            }

            let atomic_cards: AtomicCardsFile = serde_json::from_slice(&data)?;

            let data = serde_json::to_vec(&atomic_cards)?;

            std::fs::write(Self::ATOMIC_CARDS_FILE, data)?;

            atomic_cards
        };

        if verbose {
            eprintln!(
                "Loaded {}, {} cards in {} milliseconds.",
                Self::ATOMIC_CARDS_FILE,
                atomic_cards.data.len(),
                start.elapsed().as_millis()
            );
        }

        Ok(atomic_cards)
    }

    pub fn load_sqlite(verbose: bool) -> anyhow::Result<AtomicCardsFile> {
        let start = Instant::now();
        let atomic_cards = if std::fs::exists(Self::ATOMIC_CARDS_DB)? {
            let db = rusqlite::Connection::open(Self::ATOMIC_CARDS_DB)?;

            AtomicCardsFile::load(&db)?
        } else {
            if verbose {
                eprintln!("{} not found, recreating...", Self::ATOMIC_CARDS_DB);
            }
            let atomic_cards = Self::load_json(verbose)?;
            let db = rusqlite::Connection::open(Self::ATOMIC_CARDS_DB)?;
            atomic_cards.store(&db)?;
            db.release_memory()?;
            atomic_cards
        };

        if verbose {
            eprintln!(
                "Loaded {}, {} cards in {} milliseconds.",
                Self::ATOMIC_CARDS_DB,
                atomic_cards.data.len(),
                start.elapsed().as_millis()
            );
        }

        Ok(atomic_cards)
    }
}
