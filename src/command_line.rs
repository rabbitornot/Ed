use crate::model::Message;

#[derive(Debug, PartialEq, Clone)]
pub struct CommandLine {
    pub user_input: String,
}

impl CommandLine {
    pub fn add_char(&mut self, c: char) {
        self.user_input.push(c);
    }

    pub fn remove_char(&mut self) {
        self.user_input.pop();
    }

    pub fn doit(&mut self) -> Option<Vec<Message>> {
        match self.user_input.as_str() {
            "w" => Some(vec![Message::SaveFile]),
            "wq" => Some(vec![Message::SaveFile, Message::Quit]),
            _ => {
                self.user_input.clear();
                None
            }
        }
    }
}
