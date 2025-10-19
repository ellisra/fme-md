use std::{
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn process_directory<F>(
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

pub fn collect_markdown_files(target_rdpath: &Path) -> Vec<PathBuf> {
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

pub fn collect_markdown_files_recursive(target_rdpath: &Path) -> Vec<PathBuf> {
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
