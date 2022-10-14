use tui::Frame;
use tui::backend::Backend;
use tui::widgets::{Paragraph, Block};
use tui::style::{Color, Style};

/// The hugo banner
pub const HUGO: &str = r#"
 ██░ ██  █    ██   ▄████  ▒█████
▓██░ ██▒ ██  ▓██▒ ██▒ ▀█▒▒██▒  ██▒
▒██▀▀██░▓██  ▒██░▒██░▄▄▄░▒██░  ██▒
░▓█ ░██ ▓▓█  ░██░░▓█  ██▓▒██   ██░
░▓█▒░██▓▒▒█████▓ ░▒▓███▀▒░ ████▓▒░
 ▒ ░░▒░▒░▒▓▒ ▒ ▒  ░▒   ▒ ░ ▒░▒░▒░
 ▒ ░▒░ ░░░▒░ ░ ░   ░   ░   ░ ▒ ▒░
 ░  ░░ ░ ░░░ ░ ░ ░ ░   ░ ░ ░ ░ ▒
 ░  ░  ░   ░           ░     ░ ░
"#;

/// Renders the Hugo logo as a widget
pub fn draw_logo<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let p: Paragraph::new(HUGO)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(p, area);
}
