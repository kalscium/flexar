use std::fmt::Debug;
use crate::cursor::Position;

pub trait Token: Debug + Clone {
    fn position(&self) -> Position;
}