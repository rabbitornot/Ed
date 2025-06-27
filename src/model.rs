use crate::command_line;
use crate::command_line::CommandLine;
use crate::edit::Buffer;
use std::default::Default;

#[derive(Debug)]
pub struct Model {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub mode: Mode,
    pub running_state: RunningState,
}

impl Model {
    pub fn new(path_to_file: Option<String>) -> Self {
        Model {
            buffer: Buffer::new(path_to_file),
            cursor: Cursor::default(),
            mode: Mode::default(),
            running_state: RunningState::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(PartialEq, Debug, Default)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Command(CommandLine),
}

#[derive(PartialEq)]
pub enum Message {
    NewChar(char),
    Delete,
    NewLine,

    Quit,
    ChangeMode(Mode),
}

// Structure pour la position du curseur (à mettre dans le Model)
#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}
