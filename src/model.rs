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
    pub nope: u8,
}

impl Model {
    pub fn new(path_to_file: Option<String>) -> Self {
        Model {
            buffer: Buffer::new(path_to_file),
            cursor: Cursor::default(),
            mode: Mode::default(),
            nope: 0,
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

#[derive(PartialEq, Debug, Default, Clone)]
pub enum Mode {
    #[default]
    Normal,
    Insert,
    Command(CommandLine),
}

#[derive(PartialEq, Clone)]
pub enum Message {
    NewChar(char),
    Delete,
    NewLine,

    Quit,
    ChangeMode(Mode),
    SaveFile,
    Nope,
}

// Structure pour la position du curseur (à mettre dans le Model)
#[derive(Debug, Clone, Copy, Default)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
}
