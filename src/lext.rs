use crate::{cursor::{MutCursor, Cursor}, flext::Flext};

/// Lexer context for tokenising
pub struct Lext {
    pub cursor: MutCursor,
    pub current: Option<char>,
}

impl Lext {
    #[inline]
    pub fn new(file_name: String, contents: &str) -> Self {
        let cursor = MutCursor::new(Cursor::new(file_name, contents));
        let current = cursor.pos_end.get_char();
        Self {
            cursor,
            current,
        }
    }
}

impl Flext for Lext {
    /// Advances to the next token
    #[inline]
    fn advance(&mut self) {
        self.cursor.advance();
        self.current = self.cursor.current_char;
    }

    /// Un-Advances
    #[inline]
    fn revance(&mut self) {
        self.cursor.revance();
        self.current = self.cursor.current_char;
    }

    /// Spawns a child flext
    #[inline]
    fn spawn(&self) -> Self {
        Self {
            cursor: self.cursor.spawn(),
            current: self.current,
        }
    }

    /// Gets the current position of the cursor
    #[inline]
    fn position(&self) -> crate::cursor::Position {
        self.cursor.position()
    }

    /// Gets the current position of the cursor (-1 idx)
    #[inline]
    fn rposition(&self) -> crate::cursor::Position {
        let mut clone = self.cursor.clone();
        clone.revance();
        clone.position()
    }
}