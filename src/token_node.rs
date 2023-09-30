use std::{fmt::{Debug, Display}, ops::{Deref, DerefMut}};
use crate::cursor::Position;

#[derive(Debug, Clone)]
pub struct Token<TT: Display> {
    pub position: Position,
    pub token_type: TT,
}

pub trait TokenToString {
    fn to_string(&self) -> String;
}

impl<TT: Display> TokenToString for Option<&Token<TT>> {
    fn to_string(&self) -> String {
        self.map_or(" ".into(), |x| x.token_type.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Node<N> {
    pub position: Position,
    pub node: N,
}

impl<N> Node<N> {
    pub fn new(position: Position, node: N) -> Self {
        Self {
            position,
            node,
        }
    }
}

impl<N> Deref for Node<N> {
    type Target = N;
    fn deref(&self) -> &Self::Target {
        &self.node
    }
}

impl<N> DerefMut for Node<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.node
    }
}