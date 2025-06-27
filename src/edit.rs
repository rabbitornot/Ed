use crate::model::Cursor;

#[derive(Debug, Default)]
pub struct Buffer {
    content: Vec<String>,
}

impl Buffer {
    pub fn add_char(&mut self, cursor: &mut Cursor, c: char) {
        self.ensure_not_empty();

        let line = match self.content.get_mut(cursor.line) {
            Some(line) => line,
            None => return,
        };

        line.insert(cursor.column, c);
        cursor.column += 1;
    }

    pub fn delete_char(&mut self, cursor: &mut Cursor) {
        self.ensure_not_empty();

        if cursor.line >= self.content.len() {
            return;
        }

        if cursor.column > 0 {
            self.delete_char_same_line(cursor);
            return;
        }

        if cursor.line == 0 {
            return;
        }

        self.merge_with_previous_line(cursor);
    }

    fn delete_char_same_line(&mut self, cursor: &mut Cursor) {
        let line = &mut self.content[cursor.line];
        line.remove(cursor.column - 1);
        cursor.column -= 1;
    }

    fn merge_with_previous_line(&mut self, cursor: &mut Cursor) {
        let current_line = self.content.remove(cursor.line);
        cursor.line -= 1;

        let previous_line = &mut self.content[cursor.line];
        cursor.column = previous_line.len();
        previous_line.push_str(&current_line);
    }

    pub fn new_line(&mut self, cursor: &mut Cursor) {
        self.ensure_not_empty();

        let line = match self.content.get_mut(cursor.line) {
            Some(line) => line,
            None => return,
        };

        let new_line = line.split_off(cursor.column);
        self.content.insert(cursor.line + 1, new_line);
        cursor.line += 1;
        cursor.column = 0;
    }

    fn ensure_not_empty(&mut self) {
        if self.content.is_empty() {
            self.content.push(String::new());
        }
    }

    pub fn get_line_length(&self, line: usize) -> Option<usize> {
        self.content.get(line).map(|l| l.len())
    }

    pub fn validate_cursor(&self, cursor: &mut Cursor) {
        if self.content.is_empty() {
            return;
        }

        cursor.line = cursor.line.min(self.content.len() - 1);

        let line_length = self.content[cursor.line].len();
        cursor.column = cursor.column.min(line_length);
    }

    pub fn get_content(&self) -> String {
        self.content.join("\n")
    }
}
