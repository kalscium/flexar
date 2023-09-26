use crate::cursor::{MutCursor, Cursor};

/// Lexer context for tokenising
pub struct Lext {
    pub cursor: MutCursor,
    pub current: Option<char>,
}

impl Lext {
    pub fn new(file_name: String, contents: &str) -> Self {
        let cursor = MutCursor::new(Cursor::new(file_name, contents));
        let current = cursor.pos_end.get_char();
        Self {
            cursor,
            current,
        }
    }

    /// Advances to the next token
    pub fn advance(&mut self) {
        self.cursor.advance();
        self.current = self.cursor.current_char;
    }

    /// Spawns a child flext
    pub fn spawn(&self) -> Self {
        Self {
            cursor: self.cursor.spawn(),
            current: self.current,
        }
    }
}