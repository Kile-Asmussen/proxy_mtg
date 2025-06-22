use clap::Parser;

#[derive(Parser, Debug)]
pub struct Setup;

impl Setup {
    #[allow(unused)]
    fn dispatch() -> anyhow::Result<()> {
        Ok(())
    }
}
