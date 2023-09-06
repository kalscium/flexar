use std::{rc::Rc, cell::RefCell};
use super::*;

/// Flexer Context
#[derive(Debug)]
pub struct Flext<T: Token>(Rc<RefCell<Vec<T>>>);

impl<T: Token> Flext<T> {
    #[inline]
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Vec::new())))
    }

    pub fn parse(&self, token: T) {
        self.0.borrow_mut().push(token);
    }
}

impl<T: Token> Clone for Flext<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Token> Default for Flext<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}