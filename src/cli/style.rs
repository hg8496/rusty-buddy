use termimad::MadSkin;

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
