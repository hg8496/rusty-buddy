//! This module provides functionality for managing the chat context in Rusty Buddy.
//!
//! It includes the ability to load files from a specified directory into the
//! chat context for enhanced interactions with the AI assistant. The loading
//! process respects `.gitignore` rules and can filter files based on their
//! extensions or specific filenames. This dynamically enriches the context
//! available to the AI, making responses more relevant to the user's specific
//! environment and needs.
//!
//! ## Key Components
//!
//! - **ContextConsumer Trait**: This trait defines methods for managing context,
//!   allowing any struct implementing it to add context messages and set up
//!   context in a standardized manner. The `ChatService` struct implements
//!   this trait, enabling it to use context management capabilities.
//!
//! ## Functions
//!
//! - `load_files_into_context`: Loads files from the specified directory into
//!   the context of any `ContextConsumer`, filtering based on provided file types
//!   and respecting `.gitignore`.
//!
//! - `add_to_context`: Appends the contents of a file to the context,
//!   including the relative path and its content to aid in maintaining an accurate
//!   context for the conversation.
//!
//! # Examples
//!
//! ```
//! use std::path::Path;
//! use crate::context::load_files_into_context;
//! use crate::chat::service::ChatService;
//!
//! let mut service = ChatService::new(...); // Initialize your ChatService
//! let directory_path = Path::new("./src");
//! let file_types = vec!["rs".to_string(), "md".to_string()];
//! load_files_into_context(&mut service, directory_path, &file_types).unwrap();
//! ```
//!
//! In the example above, `load_files_into_context` is used to populate the context
//! of `ChatService` with all relevant files from the specified directory.
//! This implementation supports a decoupled and reusable context management
//!
//! system, enhancing the application’s modularity.

use ignore::WalkBuilder;
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::Path;

/// This trait defines a common interface for managing context in chat applications.       
/// It allows different components to implement context-related functionalities,           
/// enabling them to add messages, set up the chat context, and leverage                   
/// shared context management capabilities.                                                
///                                                                                        
/// ## Methods                                                                             
///                                                                                        
/// - `add_context_message(system_message: &str)`: Appends a system message to the context,
///   helping maintain relevant contextual information during interactions.                
///                                                                                        
/// - `setup_context()`: Prepares the context for a chat session, which may include        
///   loading files or initializing relevant state information.                            
///                                                                                        
/// ## Usage Example                                                                       
///                                                                                        
/// Implement this trait in your struct to provide context management functionalities:     
///                                                                                        
/// ```rust                                                                                
/// pub struct MyChatService {                                                             
///     // Fields...                                                                       
/// }                                                                                      
///                                                                                        
/// impl ContextConsumer for MyChatService {                                               
///     fn add_context_message(&mut self, system_message: &str) {                          
///         // Implementation...                                                           
///     }                                                                                  
///                                                                                        
///     fn setup_context(&mut self) {                                                      
///         // Implementation...                                                           
///     }                                                                                  
/// }                                                                                      
/// ```

pub trait ContextConsumer {
    fn consume(&mut self, filename: Cow<str>, content: Cow<str>) -> Result<(), Box<dyn Error>>;
}

/// Loads files from a specified directory into a context, filtering based on                                
/// provided file types and specific filenames, while respecting `.gitignore` rules.                         
///                                                                                                          
/// This function recursively walks through the given directory, fetching files                              
/// that match the specified extensions or names, and appends their contents                                 
/// to a provided context, which is operated by the `ContextConsumer`. It ignores                            
/// files and directories specified in `.gitignore`.                                                         
///                                                                                                          
/// # Arguments                                                                                              
///                                                                                                          
/// * `consumer` - A mutable reference to an instance implementing `ContextConsumer`,                        
///   used for adding context messages with the loaded file contents.                                        
/// * `directory` - A reference to the path of the directory to search for files.                            
/// * `file_types_or_names` - A slice of strings representing file extensions or full                        
///   filenames to include in the context.                                                                   
///                                                                                                          
/// # Returns                                                                                                
///                                                                                                          
/// * `Result<(), Box<dyn Error>>` - A result indicating success, or an error,                               
///   where error messages are encapsulated in a `Box` for dynamic error handling.                           
///                                                                                                          
/// # Errors                                                                                                 
///                                                                                                          
/// This function can return errors due to failures in filesystem operations,                                
/// such as reading directory entries or reading files.                                                      
///                                                                                                          
/// # Examples                                                                                               
///                                                                                                          
/// ```                                                                                                      
/// use std::path::Path;                                                                                     
/// use crate::context::load_files_into_context;                                                             
/// use crate::chat::service::ChatService;                                                                   
///                                                                                                          
/// let mut context_service = ChatService::new(...);                                                         
/// let directory_path = Path::new("./src");                                                                 
/// let file_types = vec!["rs".to_string(), "md".to_string()];                                               
/// load_files_into_context(&mut context_service, &directory_path, &file_types)                              
///     .expect("Failed to load files into context");                                                        
/// ```                                                                                                      
///                                                                                                          
/// In this example, `load_files_into_context` is called to populate the context                             
/// of `ChatService` with all relevant `.rs` and `.md` files from the specified directory.                   
pub fn load_files_into_context<T: ContextConsumer>(
    consumer: &mut T,
    directory: &Path,
    file_types_or_names: &[String], // A slice of strings representing file extensions or specific filenames to include
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
                    add_to_context(consumer, file_path)?;
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
/// * `service` - A mutable reference to a `ChatService` instance used for adding
///   context messages with the loaded file contents.
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
fn add_to_context<T: ContextConsumer>(
    consumer: &mut T,
    file_path: &Path,
) -> Result<(), Box<dyn Error>> {
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
    consumer.consume(relative_path, Cow::Owned(content))?;

    Ok(())
}
