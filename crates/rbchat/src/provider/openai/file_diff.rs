//! This module provides utility functions to create directories and files,
//! as well as to show differences between a specified file and new content
//! using Beyond Compare.
//!
//! The following functions are provided:
//!
//! - `show_diff_in_beyond_compare`: Opens a diff viewer for a specified file
//!   and the new content provided. The new content is written to a temporary
//!   file, which is then opened alongside the original file in Beyond Compare.
//!
//! - `create_directory`: Creates a new directory at the specified path.
//!
//! - `create_file`: Creates or updates a file with the specified content.
//!
//! All functions return a Result type, encapsulating success or any errors that
//! may occur during execution.
//!
//! This module is particularly useful for developers who need to manage files
//! and directories, as well as compare file contents visually using Beyond Compare.
use std::error::Error;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

/// Opens a diff viewer using Beyond Compare to compare an original file
/// with newly provided content. The new content is temporarily stored
/// in a temporary file.
///
/// # Arguments
///
/// * `diff_file` - A string slice that holds the path to the original file.
/// * `diff_content` - A string slice containing the new content to compare.
///
/// # Returns
///
/// Returns a Result that is Ok(()) on success or an error if the operation fails.
pub async fn show_diff_in_beyond_compare(
    diff_file: &str,
    diff_content: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Opening diff for file {}.", diff_file);
    // Create a temporary file for the new content
    let temp_file = NamedTempFile::new()?;

    // Write the new content to the temporary file
    fs::write(temp_file.path(), diff_content)?;

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

/// Creates a new directory at the specified path.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the directory to create.
///
/// # Returns
///
/// Returns a Result that is Ok(()) on success or an error if the operation fails.
pub async fn create_directory(path: &str) -> Result<(), Box<dyn Error>> {
    println!("Create directory {}", path);
    fs::create_dir_all(path)?;
    Ok(())
}

// Method to update a section of the file by replacing content from 'start_line' to 'end_line'
pub async fn update_file_section(
    file_path: &str,
    start_line: usize,
    end_line: usize,
    new_content: &str,
) -> Result<(), Box<dyn Error>> {
    println!("Update file {}.", file_path);
    // Open file for reading and writing
    use std::fs::OpenOptions;
    use std::io::{BufRead, BufReader, Write};

    // Open the file
    let file = OpenOptions::new().read(true).write(true).open(file_path)?;
    let reader = BufReader::new(file);

    // Collect lines into a vector
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // Check for invalid range
    if start_line > end_line || end_line > lines.len() {
        return Err("Invalid line range specified.".into());
    }

    // Replace the lines in the specified range
    lines.splice(start_line..end_line, new_content.lines().map(String::from));

    // Re-open the file for writing
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let mut writer = std::io::BufWriter::new(file);

    // Write lines back to the file
    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    println!(
        "Updated file '{}' from lines {} to {}",
        file_path, start_line, end_line
    );

    Ok(())
}

/// Creates or updates a file with the specified content.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file to create or update.
/// * `content` - A string slice containing the content to write to the file.
///
/// # Returns
///
/// Returns a Result that is Ok(()) on success or an error if the operation fails.
pub async fn create_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    println!("Create or update file {}", path);
    fs::write(path, content)?;
    Ok(())
}
