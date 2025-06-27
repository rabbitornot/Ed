use crate::Model;
use crate::model::Mode;
use ratatui::Frame;
use ratatui::prelude::{Color, Line, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

pub fn view(model: &Model, frame: &mut Frame) {
    let title = Line::from(model.file_name.as_deref().unwrap_or("New file").bold());

    let block = Block::bordered()
        .title(title.centered())
        .border_set(border::THICK)
        .style(Style::default().fg(mode_to_color(&model.mode)));

    let text = Paragraph::new(model.buffer.get_content())
        .block(block)
        .fg(Color::White);

    frame.render_widget(text, frame.area());
}

pub fn mode_to_color(mode: &Mode) -> Color {
    match mode {
        Mode::Insert => Color::Green,
        Mode::Normal => Color::Yellow,
        Mode::Command => Color::Blue,
    }
}
