use super::CompileError;
use std::fmt::{self, Display};
use soulog::*;

pub const LINE_LIMIT: u8 = 24;

impl Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ln = self.position.0.ln.to_string();
        let (line, arrw) = if self.position.0.ln == self.position.1.ln {
            sample(
                self.position.0.get_ln().unwrap(), // Position should be valid
                self.position.0.ln_idx,
                self.position.1.ln_idx,
                &self.msg,
                false,
            )
        } else {
            sample(
                self.position.0.get_ln().unwrap(), // Position should be valid
                self.position.0.ln_idx,
                self.position.0.get_ln().unwrap().len() as u16, // Position should be valid
                &self.msg,
                true,
            )
        };

        let out = colour_format![
            red("\nerror["), yellow(self.id), red("]: "), none(self.error_type),
            blue("\n --> "), cyan(&self.position.0.file_name),
            blue(":"), yellow(&ln), blue(":"), yellow(&self.position.0.ln_idx.to_string()),
            none("\n"), yellow(&ln), blue(" | "), none(&line),
            none("\n"), none(&" ".repeat(ln.len())), blue(" | ") red(&arrw),
            blue("\n <--"),
        ];

        write!(f, "{}", out)
    }
}

fn sample(line: &str, start_idx: u16, end_idx: u16, msg: &str, multi_line: bool) -> (String, String) {
    let start_trim = cal_trim(start_idx, 0);
    let end_trim = cal_trim(line.len() as u16, end_idx);

    let mut sample = line[start_trim as usize..line.len() - end_trim as usize].to_string();

    if start_trim != 0 { sample = colour_format![cyan("..."), none(&sample)]; }
    if end_trim != 0 { sample = colour_format![none(&sample), cyan("...")]; }

    if multi_line { sample = colour_format![none(&sample), cyan("\\n"), red("...")]; }

    (sample, gen_arrw(start_idx, end_idx, msg, start_trim, 5 * multi_line as u16))
}

#[inline]
fn cal_trim(actual: u16, desired: u16) -> u16 {
    let dif = actual.saturating_sub(desired);
    if dif > LINE_LIMIT as u16 { dif - LINE_LIMIT as u16 + 3 } // accounts for the `...`
    else { 0 }
}

#[inline]
fn gen_arrw(start_idx: u16, end_idx: u16, msg: &str, start_trim: u16, offset: u16) -> String {
    let spaces_since_start = if start_trim > 0 {
        start_idx - start_trim + 2 // acounts for the `...` and padding
    } else { start_idx -1 };

    let inbetween = end_idx - start_idx + 1 + offset; // even if it's the same character you still need a pointer

    let mut out = " ".repeat(spaces_since_start as usize);
    out.push_str(&"^".repeat(inbetween as usize));
    out.push(' ');
    out.push_str(&msg.replace('\n', "\x1b[36m\\n\x1b[31m"));
    
    out
}