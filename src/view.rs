use crate::Model;
use crate::model::Mode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Line, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

pub fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
        .split(frame.area());

    let title = Line::from(
        model
            .buffer
            .path_to_file
            .as_deref()
            .unwrap_or("New file")
            .bold(),
    );

    let block = Block::bordered()
        .title_bottom(title.left_aligned())
        .border_set(border::THICK)
        .style(Style::default().fg(mode_to_color(&model.mode)));

    let text = Paragraph::new(model.buffer.get_content())
        .block(block)
        .fg(Color::White);
    if let Mode::Command(command_line) = &model.mode {
        frame.render_widget(
            " :".to_string() + &command_line.user_input,
            *layout.get(1).unwrap(),
        );
    }

    frame.render_widget(text, *layout.get(0).unwrap());
}

pub fn mode_to_color(mode: &Mode) -> Color {
    match mode {
        Mode::Insert => Color::Green,
        Mode::Normal => Color::Yellow,
        Mode::Command(_) => Color::Blue,
    }
}
