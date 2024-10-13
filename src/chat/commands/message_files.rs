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
    action: F,
) -> Result<(), Box<dyn Error>>
where
    F: Fn(usize, &str) -> Result<(), Box<dyn Error>>,
{
    // Regular expression to capture content preceding a code block
    let re = Regex::new(r"(?s)(.*?)```")?;

    if greedy {
        // Find the first and last occurrence of triple backticks
        if let Some(start) = content.find("```") {
            if let Some(end) = content.rfind("```") {
                if start < end {
                    // Extract and prepare the content within these backticks
                    let block_content = &content[start + 3..end].trim();
                    let prepared_content =
                        block_content.lines().skip(1).collect::<Vec<_>>().join("\n");

                    // Apply the action closure to the prepared content
                    action(1, &prepared_content)?;
                }
            }
        }
    } else {
        let mut count = 1;
        // Iterate over individual code blocks matched by the regular expression
        for cap in re.captures_iter(content) {
            let block_content = &cap[1];
            let prepared_content = block_content.lines().skip(1).collect::<Vec<_>>().join("\n");

            // Apply the action closure with the block index
            action(count, &prepared_content)?;

            // Increment the block index for each capture
            count += 1;
        }
    }

    Ok(())
}
