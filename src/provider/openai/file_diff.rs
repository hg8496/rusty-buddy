use std::error::Error;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

// Function to show the difference between the original file and new content using Beyond Compare.
pub async fn show_diff_in_beyond_compare(
    diff_file: &str,
    diff_content: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Opening diff for file {}.", diff_file);
    // Create a temporary file for the new content
    let temp_file = NamedTempFile::new()?;

    // Write the new content to the temporary file
    std::fs::write(temp_file.path(), diff_content)?;

    // Attempt to launch Beyond Compare with the original file and the temp file
    let status = Command::new("bcomp") // Use "bcomp" for the command, if that's the correct one
        .arg(diff_file)
        .arg(temp_file.path())
        .status()?; // This will block until the command is finished

    // Check if the Beyond Compare command was successful
    if !status.success() {
        eprintln!("Failed to launch Beyond Compare: {:?}", status);
    }

    Ok(())
}

// Function to create a new directory
pub async fn create_directory(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Create directory {}", path);
    fs::create_dir_all(path)?;
    Ok(())
}

// Function to create a new file with given content
pub async fn create_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    println!("Create or update file {}", path);
    std::fs::write(path, content)?;
    Ok(())
}
