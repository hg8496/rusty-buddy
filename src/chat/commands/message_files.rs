//! This module provides functionality for processing code blocks within a text input.                     
//!                                                                                                        
//! It leverages regular expressions to identify code blocks, allowing users to apply custom actions       
//! to each block through closures. This design supports both greedy and standard processing modes,        
//! enabling flexible operations suited to diverse text analysis needs.                                    
//!                                                                                                        
//! ## Features                                                                                            
//!                                                                                                        
//! - **Flexible Processing**: Choose between greedy mode (processes content between first and last blocks)
//!   and standard mode (processes all code blocks individually).                                          
//! - **Customizable Actions**: Execute user-defined actions on each extracted code block using closures.  
//!                                                                                                        
//! ## Usage                                                                                               
//!                                                                                                        
//! This module is ideal for projects needing to parse and handle embedded code snippets,                  
//! like documentation generators or data processing applications.                                         
use regex::Regex;
use std::error::Error;

/// Processes code blocks from a given content and applies a specified action on each.                  
///                                                                                                     
/// Utilizes a regular expression to detect code blocks within text. It supports                        
/// selecting content between first and last block in greedy mode, or                                   
/// processes each block individually in standard mode.                                                 
///                                                                                                     
/// # Arguments                                                                                         
///                                                                                                     
/// * `content`: Text containing embedded code blocks for processing.                                   
/// * `greedy`: Determines if processing should be greedy or block-wise.                                
/// * `action`: Closure executed on each code block, receiving the block index and content.             
///                                                                                                     
/// # Returns                                                                                           
///                                                                                                     
/// * `Result<(), Box<dyn Error>>`: `Ok` on successful execution, or an error if a step fails.          
///                                                                                                     
/// # Examples                                                                                          
///                                                                                                     
/// ```rust                                                                                             
/// use regex::Regex;                                                                                   
///                                                                                                     
/// // Example without greedy mode                                                                      
/// process_code_blocks("example text with `code block` and `another block`", false, |i, block| {       
///     println!("Block {}: {}", i, block);                                                             
///     Ok(())                                                                                          
/// }).unwrap();                                                                                        
/// ```

pub(super) fn process_code_blocks<F>(
    content: &str,
    greedy: bool,
    mut action: F,
) -> Result<(), Box<dyn Error>>
where
    F: FnMut(usize, &str) -> Result<(), Box<dyn Error>>,
{
    if greedy {
        if let Some(start) = content.find("```") {
            if let Some(end) = content.rfind("```") {
                if start < end {
                    // Extract and prepare the content within these backticks
                    let block_content = &content[start + 3..end].trim();
                    // Apply the action closure to the prepared content
                    action(1, block_content)?;
                }
            }
        }
    } else {
        let re = Regex::new(r"```([a-zA-Z]*\n)?(?s)(.*?)```")?;
        let mut count = 1;
        for cap in re.captures_iter(content) {
            if let Some(block_content) = cap.get(2) {
                action(count, block_content.as_str())?;
                count += 1;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_process_code_blocks_greedy_mode() -> Result<(), Box<dyn Error>> {
        let content = r#"
        Here is a block of code:
        ```
        fn main() {
            println!("Hello, World!");
        }
        ```
        And here's some more text.

        Here’s another block:
        ```
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        ```
        "#;

        let mut blocks = Vec::new();

        // Execute in greedy mode.
        process_code_blocks(content, true, |_, block| {
            blocks.push(block.to_string());
            Ok(())
        })?;

        // There should be only one block captured in greedy mode
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            r#"fn main() {
            println!("Hello, World!");
        }
        ```
        And here's some more text.

        Here’s another block:
        ```
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }"#
        );
        // Verify the content of the block
        assert!(blocks[0].contains("fn main()"));
        Ok(())
    }

    #[test]
    fn test_process_code_blocks_standard_mode() -> Result<(), Box<dyn Error>> {
        let content = r#"
        Here is a block of code:
        ```rust
        fn main() {
            println!("Hello, World!");
        }
        ```
        And other text.
        "#;

        let mut blocks = Vec::new();

        // Execute in standard mode.
        process_code_blocks(content, false, |_, block| {
            blocks.push(block.to_string());
            Ok(())
        })?;

        // There should be one block captured in standard mode
        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0],
            r#"        fn main() {
            println!("Hello, World!");
        }
        "#
        );

        Ok(())
    }
    #[test]
    fn test_process_multi_code_blocks_standard_mode() -> Result<(), Box<dyn Error>> {
        let content = r#"
        Here is a block of code:
        ```
        fn main() {
            println!("Hello, World!");
        }
        ```
        
        Here is some more text.
        And then some more code:
        ```rust
        fn main() {
            println!("Hello, World2!");
        }
        ```
        Followd by more text!
        "#;

        let mut blocks = Vec::new();

        // Execute in standard mode.
        process_code_blocks(content, false, |_, block| {
            blocks.push(block.to_string());
            Ok(())
        })?;

        // There should be one block captured in standard mode
        assert_eq!(blocks.len(), 2);
        assert_eq!(
            blocks[0],
            r#"        fn main() {
            println!("Hello, World!");
        }
        "#
        );
        assert_eq!(
            blocks[1],
            r#"        fn main() {
            println!("Hello, World2!");
        }
        "#
        );

        Ok(())
    }

    #[test]
    fn test_process_code_blocks_no_code_blocks() -> Result<(), Box<dyn Error>> {
        let content = "This text contains no code blocks.";

        let mut blocks = Vec::new();

        // Execute in standard mode.
        process_code_blocks(content, false, |_, block| {
            blocks.push(block.to_string());
            Ok(())
        })?;

        // There should be no blocks captured
        assert!(blocks.is_empty());

        Ok(())
    }
}
