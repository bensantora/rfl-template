use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rfl-template")]
#[command(about = "Generate Rust for Linux kernel module templates", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new kernel module
    New {
        /// Name of the kernel module
        name: String,

        /// Author name
        #[arg(short, long)]
        author: Option<String>,

        /// Module description
        #[arg(short, long)]
        description: Option<String>,

        /// License (default: GPL-2.0)
        #[arg(short, long, default_value = "GPL-2.0")]
        license: String,
    },
}
