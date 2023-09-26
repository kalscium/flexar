use crate::{Flext, token::Token};

/// Lexer context for tokenising
#[derive(Debug, Clone, Copy)]
pub struct Parsxt<'a, T: Token> {
    pub tokens: &'a [T],
    pub idx: u16,
}

impl<'a, T: Token> Parsxt<'a, T> {
    #[inline]
    pub fn new(tokens: &'a [T]) -> Self {
        Self {
            tokens,
            idx: 0,
        }
    }
}

impl<'a, T: Token> Flext for Parsxt<'a, T> {
    /// Advances to the next token
    #[inline]
    fn advance(&mut self) {
        if self.idx < self.tokens.len() as u16 -1 {
            self.idx += 1;
        }
    }

    /// Un-Advances
    #[inline]
    fn revance(&mut self) {
        if self.idx != 0 {
            self.idx -= 1;
        }
    }

    /// Spawns a child flext
    #[inline]
    fn spawn(&self) -> Self {
        self.clone()
    }

    /// Gets the current position of the cursor
    #[inline]
    fn position(&self) -> crate::cursor::Position {
        self.tokens[self.idx as usize].position()
    }
}