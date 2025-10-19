use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
#[clap(version)]
pub enum Command {
    /// Add tag(s) to notes
    Add {
        #[arg(required = true)]
        tags: Vec<String>,
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },

    /// Remove tag(s) from notes
    Remove {
        #[arg(required = true)]
        tags: Vec<String>,
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },

    /// Replace a tag with another
    Replace {
        #[arg(required = true)]
        initial_tag: String,
        #[arg(required = true)]
        new_tag: String,
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },

    /// Remove all tags from notes
    Clear {
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },

    /// Remove unused alias fields
    RemoveAliases {
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },

    /// Remove Id field
    RemoveId {
        #[arg(short, long)]
        dir: PathBuf,
        #[arg(short, long)]
        recursive: bool,
    },
}

#[derive(Debug, Parser)]
#[command(name = "fme", about = "CLI tool for managing markdown YAML frontmatter")]
pub struct Opt {
    #[command(subcommand)]
    pub cwd: Command,
}
