use crate::{Flext, token::Token};

/// Lexer context for tokenising
#[derive(Debug, Clone, Copy)]
pub struct Parxt<'a, TT> {
    pub tokens: &'a [Token<TT>],
    pub idx: u16,
    pub done: bool,
}

impl<'a, TT> Parxt<'a, TT> {
    #[inline]
    pub fn new(tokens: &'a [Token<TT>]) -> Self {
        Self {
            tokens,
            idx: 0,
            done: tokens.len() == 0,
        }
    }

    #[inline]
    pub fn current_token(&self) -> &'a Token<TT> {
        &self.tokens[self.idx as usize]
    }

    #[inline]
    pub fn current(&self) -> &'a TT {
        &self.tokens[self.idx as usize].token_type
    }
}

impl<'a, TT> Flext for Parxt<'a, TT> {
    /// Advances to the next token
    #[inline]
    fn advance(&mut self) {
        if self.idx < self.tokens.len() as u16 -1 {
            self.idx += 1;
        } else {
            self.done = true;
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
        Self { tokens: self.tokens, idx: self.idx, done: self.done, }
    }

    /// Gets the current position of the cursor
    #[inline]
    fn position(&self) -> crate::cursor::Position {
        self.tokens[self.idx as usize].position.clone()
    }
}