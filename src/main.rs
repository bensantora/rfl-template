mod cli;
mod generator;
mod templates;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use generator::ModuleConfig;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New {
            name,
            author,
            description,
            license,
        } => {
            let config = ModuleConfig {
                name,
                author: author.unwrap_or_else(|| "Unknown Author".to_string()),
                description: description.unwrap_or_else(|| "A Rust for Linux kernel module".to_string()),
                license,
            };

            generator::generate_module(config)?;
        }
    }

    Ok(())
}
