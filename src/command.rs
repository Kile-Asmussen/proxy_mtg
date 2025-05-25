use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Opt {
    #[arg(value_name = "FILE")]
    file: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    List {},
    Tags {},
    Colors {},
    Curve {},
    Identity {},
    Types {},
}
