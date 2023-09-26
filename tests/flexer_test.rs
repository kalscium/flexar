use flexar::{cursor::{Position, MutCursor, Cursor}, compiler_error, old_flexar};
use std::str::FromStr;

pub struct Number(Position, u64);

const NUMBERS: &str = "0123456789";

old_flexar! {
    [[Number] flext: Flext]
    fn new {
        set (value = String::new());
        while ((flext.current.map(|x| NUMBERS.contains(x)).unwrap_or(false)) {
            value.push(flext.current.unwrap());
            flext.advance();
        } else (flext.current.is_none()) {
            compiler_error!((E001, flext.cursor.position()) flext.current.unwrap())
        });
        ok (Self(flext.cursor.position(), u64::from_str(&value).unwrap()));
    }
}

compiler_error! {
    [[Define]]
    (E001) "invalid number": ((1) "expected number not `", "`");
}

pub struct Flext {
    pub cursor: MutCursor,
    pub current: Option<char>,
}

impl Flext {
    pub fn new(file_name: String, contents: &str) -> Self {
        let cursor = MutCursor::new(Cursor::new(file_name, contents));
        let current = cursor.pos_end.get_char();
        Self {
            cursor,
            current,
        }
    }

    pub fn advance(&mut self) {
        self.cursor.advance();
        self.current = self.cursor.current_char;
    }
}