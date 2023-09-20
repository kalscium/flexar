use crate::cursor::Position;
use super::CompileError;
use std::fmt::{self, Display};
use soulog::*;

pub const LINE_LIMIT: u8 = 16;

impl Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ln = self.position.start().ln.to_string();
        let (line, arrw) = sample(&self.position, &self.msg);

        let out = colour_format![
            red("error["), none(self.error_type), red("]: "), none(&self.msg),
            blue("\n --> "), none(&self.position.start().file_name),
            blue(":"), none(&ln), blue(":"), none(&self.position.start().ln_idx.to_string()),
            blue("\n"), cyan(&ln), blue(" | "), none(&line), // Only works for single line errors, change later if needed
            blue("\n  | "), red(&arrw),
            blue("\n <--"),
        ];

        write!(f, "{}", out)
    }
}

fn sample(position: &Position, msg: &str) -> (String, String) {
    let line = position.start().get_ln().unwrap(); // position should be valid

    let start_trim = cal_trim(position.start().ln_idx, 0);
    let end_trim = line.len() as u16 - cal_trim(line.len() as u16, position.end().ln_idx);

    let mut sample = line[start_trim as usize..end_trim as usize].to_string();

    if start_trim != 0 { sample = colour_format![cyan("..."), none(&sample)]; }
    if end_trim != 0 { sample = colour_format![none(&sample), cyan("...")]; }

    (sample, gen_arrw(position, msg, start_trim))
}

#[inline]
fn cal_trim(actual: u16, desired: u16) -> u16 {
    let dif = actual - desired;
    if dif > LINE_LIMIT as u16 { dif - LINE_LIMIT as u16 + 3 } // accounts for the `...`
    else { 0 }
}

#[inline]
fn gen_arrw(position: &Position, msg: &str, start_trim: u16) -> String {
    // let spaces_since_start = if start_trim > 0 {
    //     position.start().ln_idx - start_trim + 2 // acounts for the `...` and padding
    // } else { 0 };

    let spaces_since_start = position.start().ln_idx - start_trim - 1; // sample is padded by an extra space
    let inbetween = position.end().ln_idx - position.start().ln_idx + 1; // even if it's the same character you still need a pointer

    let mut out = " ".repeat(spaces_since_start as usize);
    out.push_str(&"^".repeat(inbetween as usize));
    out.push(' ');
    out.push_str(msg);
    
    out
}