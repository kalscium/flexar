use crate::cursor::Position;

// A context for a lexer / parser (flexar)
pub trait Flext {
    fn advance(&mut self);
    fn revance(&mut self);
    fn spawn(&self) -> Self;
    fn position(&self) -> Position;
    fn rposition(&self) -> Position;
}