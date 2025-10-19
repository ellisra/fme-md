use crate::frontmatter::{indent_yaml, parse_frontmatter, Frontmatter};

pub fn add_tags(
    content: &str,
    tags_to_add: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => {
            let mut new_frontmatter = Frontmatter {
                id: Some(String::new()),
                aliases: Some(vec![]),
                tags: Some(vec![]),
                other: std::collections::HashMap::new(),
            };

            for tag in tags_to_add {
                new_frontmatter.tags.as_mut().unwrap().push(tag.clone());
            }

            let yaml = indent_yaml(&serde_yaml::to_string(&new_frontmatter)?);

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
        let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

pub fn remove_tags(
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

        let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

pub fn replace_tags(
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

        let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);
        Ok(format!("---\n{}---\n{}", yaml, content_body))
    } else {
        Ok(content.to_string())
    }
}

pub fn clear_tags(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if frontmatter.tags.is_none() {
        return Ok(content.to_string());
    }

    frontmatter.tags = None;

    let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);
    Ok(format!("---\n{}---\n{}", yaml, content_body))
}

pub fn remove_blank_aliases(
    content: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if let Some(aliases) = &frontmatter.aliases {
        if aliases.is_empty() {
            frontmatter.aliases = None;
            let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);

            return Ok(format!("---\n{}---\n{}", yaml, content_body));
        }
    }

    Ok(content.to_string())
}

pub fn remove_ids(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (mut frontmatter, content_body) = match parse_frontmatter(content) {
        Ok(result) => result,
        Err(_) => return Ok(content.to_string()),
    };

    if !frontmatter.id.is_none() {
        frontmatter.id = None;
        let yaml = indent_yaml(&serde_yaml::to_string(&frontmatter)?);

        return Ok(format!("---\n{}---\n{}", yaml, content_body));
    }

    Ok(content.to_string())
}
