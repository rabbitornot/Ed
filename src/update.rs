use crate::edit;
use crate::model::{Message, Mode, Model, RunningState};
use crossterm::ExecutableCommand;
use crossterm::cursor::SetCursorStyle;
use std::io::stdout;

pub fn update(model: &mut Model, msg: Message) -> Option<Message> {
    match msg {
        Message::NewChar(c) => {
            if let Mode::Command(command_line) = &mut model.mode {
                command_line.add_char(c);
                return None;
            }
            model.buffer.add_char(&mut model.cursor, c)
        }
        Message::Delete => {
            if let Mode::Command(command_line) = &mut model.mode {
                command_line.remove_char();
                return None;
            }
            model.buffer.delete_char(&mut model.cursor)
        }
        Message::NewLine => {
            if let Mode::Command(command_line) = &mut model.mode {
                return Some(if let Some(command_line) = command_line.doit() {
                    for command in command_line {
                        update(model, command);
                    }
                    Message::ChangeMode(Mode::Normal)
                } else {
                    Message::Nope
                });
            }
            model.buffer.new_line(&mut model.cursor)
        }

        Message::Quit => {
            model.running_state = RunningState::Done;
        }
        Message::ChangeMode(mode) => {
            stdout().execute(mode_to_cursor_style(&mode)).unwrap();
            model.mode = mode;
        }
        Message::Nope => {
            model.nope = 30;
            return Some(Message::ChangeMode(Mode::Normal));
        }
        Message::SaveFile => {}
    }
    None
}

fn mode_to_cursor_style(mode: &Mode) -> SetCursorStyle {
    match mode {
        Mode::Normal => SetCursorStyle::BlinkingBlock,
        Mode::Insert => SetCursorStyle::SteadyBlock,
        Mode::Command(_) => SetCursorStyle::SteadyBlock,
    }
}
