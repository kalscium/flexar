use std::fmt::{Debug, Display};
use crate::cursor::Position;

#[derive(Debug, Clone)]
pub struct Token<TT: Display> {
    pub position: Position,
    pub token_type: TT,
}

impl<TT: Display> Token<TT> {
    pub fn display(this: Option<&Self>) -> String {
        this.map_or(" ".into(), |x| x.token_type.to_string())
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