use crate::{Flext, token::Token};

/// Lexer context for tokenising
#[derive(Debug, Clone, Copy)]
pub struct Parsxt<'a, TT> {
    pub tokens: &'a [Token<TT>],
    pub idx: u16,
}

impl<'a, TT> Parsxt<'a, TT> {
    #[inline]
    pub fn new(tokens: &'a [Token<TT>]) -> Self {
        Self {
            tokens,
            idx: 0,
        }
    }
}

impl<'a, TT> Flext for Parsxt<'a, TT> {
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
        Self { tokens: &self.tokens, idx: self.idx }
    }

    /// Gets the current position of the cursor
    #[inline]
    fn position(&self) -> crate::cursor::Position {
        self.tokens[self.idx as usize].position.clone()
    }
}