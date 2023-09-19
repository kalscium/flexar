pub mod compile_error_format;
pub mod compile_error_macro;
pub use compile_error_format::*;
pub use crate::compilerr_fmt;
pub use crate::compile_error;

use std::{error::Error, fmt::{self, Display}};
use crate::cursor::Position;

#[derive(Clone, Debug)]
pub struct CompileError {
    pub origin: &'static str,
    pub msg: String,
    pub position: Position,
}

impl CompileError {
    #[inline]
    pub fn new(origin: &'static str, msg: String, position: Position) -> Self {
        CompileError { origin, msg, position }
    }

    pub fn throw(error: &CompileError) {
        println!("{}", error);
        std::process::exit(1);
    }
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!(); // write some nice compile error display code
    }
}

pub struct CompileErrorTemplate<const N: usize> {
    pub origin: &'static str,
    pub fmt: CompileErrFormatter<N>,
}

impl<const N: usize> CompileErrorTemplate<N> {
    #[inline]
    pub const fn new(origin: &'static str, fmt: CompileErrFormatter<N>) -> Self {
        Self {
            origin,
            fmt,
        }
    }
}