use crate::edit::Buffer;

#[derive(Debug, Default)]
pub struct Model {
    pub buffer: Buffer,
    pub cursor: Cursor,
    pub mode: Mode,
    pub running_state: RunningState,
    pub file_name: Option<String>,
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
    Command,
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
