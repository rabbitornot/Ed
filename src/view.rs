use crate::Model;
use crate::model::Mode;
use color_eyre::owo_colors::style;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Line, Style, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};

pub fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(95),
                Constraint::Percentage(3),
                Constraint::Percentage(2),
            ]
            .as_ref(),
        )
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

    frame.render_widget(text, *layout.get(0).unwrap());
    if model.nope > 0 {
        frame.render_widget(
            Line::from(" Nope !!!").style(Style::from(Color::Red)),
            *layout.get(1).unwrap(),
        );
    }

    if let Mode::Command(command_line) = &model.mode {
        frame.render_widget(
            " :".to_string() + &command_line.user_input,
            *layout.get(2).unwrap(),
        );
    }
}

pub fn mode_to_color(mode: &Mode) -> Color {
    match mode {
        Mode::Insert => Color::LightBlue,
        Mode::Normal => Color::Green,
        Mode::Command(_) => Color::Yellow,
    }
}
