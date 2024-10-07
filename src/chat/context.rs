use ignore::WalkBuilder;
use std::error::Error;
use std::fs;
use std::path::Path;

/// Loads files from a specified directory into a context, filtering based on
/// provided file types and specific filenames, and respecting `.gitignore` rules.
///
/// # Arguments
///
/// * `directory` - A reference to the path of the directory to search for files.
/// * `file_types_or_names` - A slice of strings representing file extensions or full filenames to include in the context.
/// * `context` - A mutable reference to a `String` where the content of the files will be appended.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or an error, with error messages
///   encapsulated in a `Box` for dynamic error handling.
///
/// # Errors
///
/// This function can return errors due to failures in filesystem operations, such
/// as reading directory entries or reading files.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// let mut context = String::new();
/// load_files_into_context(Path::new("./src"), &vec!["rs".to_string(), "Gemfile".to_string()], &mut context).unwrap();
/// println!("{}", context);
/// ```
pub fn load_files_into_context(
    directory: &Path,
    file_types_or_names: &[String], // A slice of strings representing file extensions or specific filenames to include
    context: &mut String,
) -> Result<(), Box<dyn Error>> {
    // Create a directory walker using the `WalkBuilder`, which respects .gitignore rules
    let walker = WalkBuilder::new(directory)
        .standard_filters(true) // Apply standard .gitignore rules
        .build();

    // Iterate over the directory entries returned by the walker
    for result in walker {
        let entry = result?;
        let file_path = entry.path();

        // Check if the current path is a file
        if file_path.is_file() {
            let file_name = file_path.file_name().and_then(std::ffi::OsStr::to_str);

            // Match either by specific filename or by extension
            if let Some(name) = file_name {
                if file_types_or_names
                    .iter()
                    .any(|filter| name == filter || name.ends_with(&format!(".{}", filter)))
                {
                    // Add the file content to the context
                    add_to_context(context, file_path)?;
                }
            }
        }
    }
    Ok(())
}

/// Helper function to append the content of a file to the context, given its path.
///
/// # Arguments
///
/// * `context` - A mutable reference to a `String` where the file content will be appended.
/// * `file_path` - A reference to the path of the file whose content needs to be added.
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - A result indicating success or failure to read the file content.
///
/// # Errors
///
/// This function can return errors due to failures in reading the file or finding the current
/// working directory.
fn add_to_context(context: &mut String, file_path: &Path) -> Result<(), Box<dyn Error>> {
    // Retrieve the current working directory to determine relative file paths
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory for context: {}", e))?;

    // Read the file content, returning a formatted error message if unsuccessful
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file '{}': {}", file_path.display(), e))?;

    // Strip the current directory prefix from the file path to display a relative path
    let relative_path = file_path
        .strip_prefix(&current_dir)
        .unwrap_or(file_path)
        .to_string_lossy();

    // Append the file's relative path and its content to the context string
    context.push_str(&format!(
        "Filename: {}\nContent:\n{}\n",
        relative_path, content
    ));
    Ok(())
}
