use crate::model::{Message, Mode, Model};
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use std::time::Duration;

pub fn handle_event(model: &Model) -> color_eyre::Result<Option<Message>> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key(model, key));
            }
        }
    }
    Ok(None)
}

fn handle_key(model: &Model, key: event::KeyEvent) -> Option<Message> {
    match model.mode {
        Mode::Insert => handle_key_insert_mode(key),
        Mode::Normal => handle_key_normal_mode(key),
        Mode::Command => None,
    }
}

fn handle_key_insert_mode(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char(c) => Some(Message::NewChar(c)),
        KeyCode::Backspace => Some(Message::Delete),
        KeyCode::Enter => Some(Message::NewLine),
        KeyCode::Esc => Some(Message::ChangeMode(Mode::Normal)),
        _ => None,
    }
}

fn handle_key_normal_mode(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('i') => Some(Message::ChangeMode(Mode::Insert)),
        _ => None,
    }
}
