use clap::Parser;

#[derive(Parser, Debug)]
pub struct Setup;

impl Setup {
    fn dispatch() -> anyhow::Result<()> {
        Ok(())
    }
}
