use crate::cursor::Position;
use super::CompileError;
use std::fmt::{self, Display};
use soulog::*;

impl Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ln = self.position.0.ln.to_string();

        let out = colour_format![
            red("error["), none(self.error_type), red("]: "), none(&self.msg),
            blue("\n --> "), none(&self.position.0.file_name),
            blue(":"), none(&ln), blue(":"), none(&self.position.1.ln_idx.to_string()),
            blue("\n"), cyan(&ln), blue(" | "), none(self.position.0.get_ln().unwrap()), // Only works for single line errors, change later if needed
        ];

        write!(f, "{}", out)
    }
}

fn shorten(position: &Position, padding: &mut String) {
    position.0.
}