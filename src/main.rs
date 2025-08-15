use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Subcommand)]
#[clap(version)]
enum Command {
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
}

#[derive(Debug, Parser)]
#[command(
    name = "mdtags",
    about = "CLI tool for managing markdown frontmatter"
)]
struct Opt {
    #[command(subcommand)]
    cwd: Command,
}

#[derive(Debug, Serialize, Deserialize)]
struct Frontmatter {
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    aliases: Option<Vec<String>>,

    #[serde(flatten)]
    other: std::collections::HashMap<String, Value>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::parse();

    match opt.cwd {
        Command::Add {
            tags,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| {
            add_tags(content, &tags)
        }),
        Command::Remove {
            tags,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| {
            remove_tags(content, &tags)
        }),
        Command::Replace {
            initial_tag,
            new_tag,
            dir,
            recursive,
        } => process_directory(&dir, recursive, |content| {
            replace_tags(content, &initial_tag, &new_tag)
        }),
        Command::Clear { dir, recursive } => {
            process_directory(&dir, recursive, clear_tags)
        }
        Command::RemoveAliases { dir, recursive } => {
            process_directory(&dir, recursive, remove_blank_aliases)
        }
    }
}

fn process_directory<F>(
    target_rdpath: &Path,
    recursive: bool,
    processor: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn(&str) -> Result<String, Box<dyn std::error::Error>>,
{
    let md_rfpaths = if recursive {
        collect_markdown_files_recursive(target_rdpath)
    } else {
        collect_markdown_files(target_rdpath)
    };

    for md_rfpath in md_rfpaths {
        let content = fs::read_to_string(&md_rfpath)?;

        match processor(&content) {
            Ok(new_content) => {
                if content != new_content {
                    fs::write(&md_rfpath, new_content)?;
                    print!("Updated: {}\n", md_rfpath.display());
                }
            }
            Err(e) => {
                println!("Error processing {}: {}\n", md_rfpath.display(), e);
            }
        }
    }

    Ok(())
}

fn collect_markdown_files(target_rdpath: &Path) -> Vec<PathBuf> {
    fs::read_dir(target_rdpath)
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "md")
        })
        .map(|entry| entry.path())
        .collect()
}

fn collect_markdown_files_recursive(target_rdpath: &Path) -> Vec<PathBuf> {
    WalkDir::new(target_rdpath)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();
            path.is_file() && path.extension().map_or(false, |ext| ext == "md")
        })
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

fn parse_frontmatter(
    content: &str,
) -> Result<(Frontmatter, String), Box<dyn std::error::Error>> {
    let document = YamlFrontMatter::parse::<Frontmatter>(content)?;
    Ok((document.metadata, document.content))
}

fn add_tags(
    content: &str,
    tags_to_add: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => {
            let mut new_frontmatter = Frontmatter {
                tags: Some(vec![]),
                aliases: Some(vec![]),
                other: std::collections::HashMap::new(),
            };

            for tag in tags_to_add {
                new_frontmatter.tags.as_mut().unwrap().push(tag.clone());
            }

            let yaml = serde_yaml::to_string(&new_frontmatter)?;

            return Ok(format!("---\n{}---\n{}", yaml, content));
        }
    };

    let mut tags = frontmatter.tags.unwrap_or_else(Vec::new);
    let mut modified = false;

    for tag in tags_to_add {
        if !tags.contains(tag) {
            tags.push(tag.clone());
            modified = true;
        }
    }

    if modified {
        frontmatter.tags = Some(tags);
        let yaml = serde_yaml::to_string(&frontmatter)?;
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

fn remove_tags(
    content: &str,
    tags_to_remove: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if frontmatter.tags.is_none() {
        return Ok(content.to_string());
    }

    let mut tags = frontmatter.tags.unwrap();
    let original_count = tags.len();

    tags.retain(|t| !tags_to_remove.contains(t));

    if tags.len() < original_count {
        if tags.is_empty() {
            frontmatter.tags = None;
        } else {
            frontmatter.tags = Some(tags);
        }

        let yaml = serde_yaml::to_string(&frontmatter)?;
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

fn replace_tags(
    content: &str,
    initial_tag: &String,
    new_tag: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if frontmatter.tags.is_none() {
        return Ok(content.to_string());
    }

    let mut tags = frontmatter.tags.unwrap();
    let original_count = tags.len();

    tags.retain(|t| !initial_tag.contains(t));

    if tags.len() < original_count {
        if !tags.contains(new_tag) {
            tags.push(new_tag.clone());
        }

        frontmatter.tags = Some(tags);

        let yaml = serde_yaml::to_string(&frontmatter)?;
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

fn clear_tags(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if frontmatter.tags.is_none() {
        return Ok(content.to_string());
    }

    frontmatter.tags = None;

    let yaml = serde_yaml::to_string(&frontmatter)?;
    Ok(format!("---\n{}---\n{}", yaml, content_body))
}

fn remove_blank_aliases(
    content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if let Some(aliases) = &frontmatter.aliases {
        if aliases.is_empty() {
            frontmatter.aliases = None;
            let yaml = serde_yaml::to_string(&frontmatter)?;

            return Ok(format!("---\n{}---\n{}", yaml, content_body));
        }
    }

    Ok(content.to_string())
}
