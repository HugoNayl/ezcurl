use crate::action::Direction;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone, Copy)]
pub enum Edit {
    Insert(char),
    Backspace,
    Delete,
    Move(Direction),
    Home,
    End,
}

#[derive(Debug, Default, Clone)]
pub struct TextEditor {
    text: String,
    cursor: usize,
}

impl TextEditor {
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let cursor = text.len();
        Self { text, cursor }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_at_start(&self) -> bool {
        self.cursor == 0
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor == self.text.len()
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        let text_before_cursor = &self.text[..self.cursor];
        let line = text_before_cursor.rsplit('\n').next().unwrap_or_default();
        let x = UnicodeWidthStr::width(line).min(u16::MAX as usize) as u16;
        let y = text_before_cursor
            .matches('\n')
            .count()
            .min(u16::MAX as usize) as u16;

        (x, y)
    }

    pub fn edit(&mut self, edit: Edit) {
        match edit {
            Edit::Insert(character) => {
                self.text.insert(self.cursor, character);
                self.cursor += character.len_utf8();
            }
            Edit::Backspace => {
                if let Some(previous) = self.previous_boundary() {
                    self.text.drain(previous..self.cursor);
                    self.cursor = previous;
                }
            }
            Edit::Delete => {
                if let Some(next) = self.next_boundary() {
                    self.text.drain(self.cursor..next);
                }
            }
            Edit::Move(Direction::Left) => {
                if let Some(previous) = self.previous_boundary() {
                    self.cursor = previous;
                }
            }
            Edit::Move(Direction::Right) => {
                if let Some(next) = self.next_boundary() {
                    self.cursor = next;
                }
            }
            Edit::Move(Direction::Up) => self.move_vertically(false),
            Edit::Move(Direction::Down) => self.move_vertically(true),
            Edit::Home => self.cursor = self.line_start(self.cursor),
            Edit::End => self.cursor = self.line_end(self.cursor),
        }
    }

    fn move_vertically(&mut self, down: bool) {
        let current_start = self.line_start(self.cursor);
        let current_end = self.line_end(self.cursor);
        let column = UnicodeWidthStr::width(&self.text[current_start..self.cursor]);

        let (target_start, target_end) = if down {
            if current_end == self.text.len() {
                return;
            }
            let start = current_end + 1;
            (start, self.line_end(start))
        } else {
            if current_start == 0 {
                return;
            }
            let end = current_start - 1;
            (self.line_start(end), end)
        };

        self.cursor = self.byte_at_column(target_start, target_end, column);
    }

    fn previous_boundary(&self) -> Option<usize> {
        self.text[..self.cursor]
            .char_indices()
            .next_back()
            .map(|(index, _)| index)
    }

    fn next_boundary(&self) -> Option<usize> {
        self.text[self.cursor..]
            .char_indices()
            .nth(1)
            .map(|(offset, _)| self.cursor + offset)
            .or((self.cursor < self.text.len()).then_some(self.text.len()))
    }

    fn line_start(&self, cursor: usize) -> usize {
        self.text[..cursor]
            .rfind('\n')
            .map(|index| index + 1)
            .unwrap_or(0)
    }

    fn line_end(&self, cursor: usize) -> usize {
        self.text[cursor..]
            .find('\n')
            .map(|offset| cursor + offset)
            .unwrap_or(self.text.len())
    }

    fn byte_at_column(&self, start: usize, end: usize, target: usize) -> usize {
        let mut width = 0;

        for (offset, character) in self.text[start..end].char_indices() {
            if width >= target {
                return start + offset;
            }

            let next_width = width + character.width().unwrap_or(0);
            if next_width > target {
                return start + offset;
            }
            width = next_width;
        }

        end
    }
}

#[cfg(test)]
mod tests {
    use super::{Edit, TextEditor};
    use crate::action::Direction;

    #[test]
    fn edits_at_the_cursor_instead_of_the_end() {
        let mut editor = TextEditor::new("helo");

        editor.edit(Edit::Move(Direction::Left));
        editor.edit(Edit::Insert('l'));

        assert_eq!(editor.text(), "hello");
        assert_eq!(editor.cursor_position(), (4, 0));
    }

    #[test]
    fn moves_between_lines_and_keeps_the_column() {
        let mut editor = TextEditor::new("one\ntwelve");

        editor.edit(Edit::Move(Direction::Left));
        editor.edit(Edit::Move(Direction::Left));
        editor.edit(Edit::Move(Direction::Up));

        assert_eq!(editor.cursor_position(), (3, 0));
    }

    #[test]
    fn handles_utf8_boundaries() {
        let mut editor = TextEditor::new("cafe");

        editor.edit(Edit::Move(Direction::Left));
        editor.edit(Edit::Backspace);
        editor.edit(Edit::Insert('é'));

        assert_eq!(editor.text(), "caée");
        assert_eq!(editor.cursor_position(), (3, 0));
    }

    #[test]
    fn measures_wide_characters_as_the_terminal_displays_them() {
        let mut editor = TextEditor::new("界a");

        editor.edit(Edit::Move(Direction::Left));

        assert_eq!(editor.cursor_position(), (2, 0));
    }
}
