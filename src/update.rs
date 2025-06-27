use crate::edit;
use crate::model::{Message, Mode, Model, RunningState};
use crossterm::ExecutableCommand;
use crossterm::cursor::SetCursorStyle;
use std::io::stdout;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::NewChar(c) => model.buffer.add_char(&mut model.cursor, c),
        Message::Delete => model.buffer.delete_char(&mut model.cursor),
        Message::NewLine => model.buffer.new_line(&mut model.cursor),

        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::ChangeMode(mode) => {
            stdout().execute(mode_to_cursor_style(&mode)).unwrap();
            model.mode = mode;
        }
    }
    None
}

fn mode_to_cursor_style(mode: &Mode) -> SetCursorStyle {
    match mode {
        Mode::Normal => SetCursorStyle::BlinkingBlock,
        Mode::Insert => SetCursorStyle::SteadyBlock,
        Mode::Command => SetCursorStyle::SteadyBlock,
    }
}
