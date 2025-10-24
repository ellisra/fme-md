mod cli;
mod commands;
mod frontmatter;
mod fs_utils;

use clap::{error::Result, Parser};
use cli::{Command, Opt};
use commands::{add_tags, clear_tags, remove_blank_aliases, remove_ids, remove_tags, replace_tags};
use fs_utils::process_directory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    match opt.cwd {
        Command::Add {
            tags,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| add_tags(content, &tags)),
        Command::Remove {
            tags,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| remove_tags(content, &tags)),
        Command::Replace {
            initial_tag,
            new_tag,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| {
            replace_tags(content, &initial_tag, &new_tag)
        }),
        Command::Clear {
            dir,
            recursive,
        } => process_directory(&dir, recursive, clear_tags),
        Command::RemoveAliases {
            dir,
            recursive,
        } => process_directory(&dir, recursive, remove_blank_aliases),
        Command::RemoveId {
            dir,
            recursive,
        } => process_directory(&dir, recursive, remove_ids),
    }
}
