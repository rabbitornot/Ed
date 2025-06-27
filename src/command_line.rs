#[derive(Debug, PartialEq)]
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

    pub fn clear(&mut self) {
        self.user_input.clear();
    }
}
