use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub fn load_files_into_context(
    directory: &Path,
    file_types: &[String], // Use a slice of strings
    context: &mut String,
) -> Result<(), Box<dyn Error>> {
    for entry in WalkDir::new(directory) {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Some(extension) = file_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if file_types.contains(&ext_str.to_string()) {
                        add_to_context(context, file_path)?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn add_to_context(context: &mut String, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory for context: {}", e))?;

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

    let relative_path = file_path
        .strip_prefix(&current_dir)
        .unwrap_or(file_path)
        .to_string_lossy();

    context.push_str(&format!(
        "Filename: {}\nContent:\n{}\n",
        relative_path, content
    ));
    Ok(())
}
