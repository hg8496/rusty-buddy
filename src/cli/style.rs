//! This module provides configuration for terminal output styling using the `MadSkin` crate.
//!
//! It customizes the appearance of various text styles, including bold, italic, code blocks,
//! inline code, and headers. The styles are defined to enhance the user experience when
//! presenting text in the terminal.
//!
//! ## Configuring Text Styles
//!
//! The `configure_mad_skin` function initializes a `MadSkin` instance, which allows for
//! rendering stylized text in the terminal. It sets specific foreground and background colors
//! for different text styles, ensuring that output is visually appealing and easily readable.
//!
//! ### Example Usage
//!
//! Here’s how to configure the `MadSkin` instance for styled terminal output:
//!
//! ```rust
//! use termimad::MadSkin;
//!
//! // Configure the terminal styles
//! let skin = configure_mad_skin();
//! skin.print_text("This is styled text!\n");
//! ```
//!
//! ### Styling Details
//!
//! - **Bold:** Text rendered in bold will have a blue foreground.
//! - **Italic:** Italics will be displayed with a dark cyan foreground.
//! - **Code Blocks:** Code blocks will be highlighted with a dark yellow foreground.
//! - **Inline Code:** Inline code sections will appear with a dark magenta foreground.
//! - **Headers:** The first header level will have a dark blue foreground.
//! - **Italics with Background:** Italic text will utilize a dark grey background for enhanced visibility.
use termimad::MadSkin;

/// Configures a `MadSkin` instance with custom styling for terminal output.
/// This function sets specific foreground and background colors for various text styles
/// such as bold, italic, code blocks, inline code, and headers.
/// The created `MadSkin` can be used for rendering styled text in the terminal.
pub fn configure_mad_skin() -> MadSkin {
    let mut skin = MadSkin::default();

    skin.bold.set_fg(termimad::crossterm::style::Color::Blue);
    skin.italic
        .set_fg(termimad::crossterm::style::Color::DarkCyan);
    skin.code_block
        .set_fg(termimad::crossterm::style::Color::DarkYellow);
    skin.inline_code
        .set_fg(termimad::crossterm::style::Color::DarkMagenta);
    skin.headers[0].set_fg(termimad::crossterm::style::Color::DarkBlue);
    skin.italic
        .set_bg(termimad::crossterm::style::Color::AnsiValue(235)); // Dark grey background for italics

    skin
}
