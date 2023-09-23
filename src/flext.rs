use crate::cursor::Position;

pub trait Flext {
    fn get_position(&self) -> Position;
}