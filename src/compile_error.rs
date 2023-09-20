pub mod compile_error_format;
pub mod compile_error_macro;
pub mod compile_error_display;
pub use compile_error_format::*;
pub use crate::compilerr_fmt;
pub use crate::compile_error;

use std::error::Error;
use crate::cursor::Position;

#[derive(Clone, Debug)]
pub struct CompileError {
    pub id: &'static str,
    pub error_type: &'static str,
    pub msg: String,
    pub position: Position,
}

impl CompileError {
    #[inline]
    pub fn new(id: &'static str, error_type: &'static str, msg: String, position: Position) -> Self {
        CompileError { id, error_type, msg, position }
    }

    pub fn throw(&self) {
        println!("{}", self);
        std::process::exit(1);
    }
}

impl Error for CompileError {}

pub struct CompileErrorTemplate<const N: usize> {
    pub error_type: &'static str,
    pub fmt: CompileErrFormatter<N>,
}

impl<const N: usize> CompileErrorTemplate<N> {
    #[inline]
    pub const fn new(error_type: &'static str, fmt: CompileErrFormatter<N>) -> Self {
        Self {
            error_type,
            fmt,
        }
    }
}