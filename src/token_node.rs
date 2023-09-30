use std::fmt::Debug;
use crate::cursor::Position;

#[derive(Debug, Clone)]
pub struct Token<TT> {
    pub position: Position,
    pub token_type: TT,
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