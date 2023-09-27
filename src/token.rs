use std::fmt::Debug;
use crate::cursor::Position;

#[derive(Debug, Clone)]
pub struct Token<TT> {
    pub position: Position,
    pub token_type: TT,
}