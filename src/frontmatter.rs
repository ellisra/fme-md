use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use yaml_front_matter::YamlFrontMatter;

#[derive(Debug, Serialize, Deserialize)]
pub struct Frontmatter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(flatten)]
    pub other: std::collections::HashMap<String, Value>,
}

pub fn parse_frontmatter(
    content: &str,
) -> Result<(Frontmatter, String), Box<dyn std::error::Error>> {
    let document = YamlFrontMatter::parse::<Frontmatter>(content)?;
    Ok((document.metadata, document.content))
}

pub fn indent_yaml(yaml: &str) -> String {
    let mut indented = yaml
        .lines()
        .map(|line| {
            if line.starts_with("- ") {
                format!("  {}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if !indented.ends_with('\n') {
        indented.push('\n');
    }

    indented
}
