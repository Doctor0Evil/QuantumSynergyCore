mod commands;
mod aln_serializer;
mod config;

use crate::commands::Cli;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.execute().await
}
