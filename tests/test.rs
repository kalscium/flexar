use flexar::{cursor::{Position, MutCursor}, compile_error, flexar};
use std::str::FromStr;

pub struct Number(Position, u64);

const NUMBERS: &str = "0123456789";

flexar! {
    [[Number] flext: Flext]
    fn new {
        set (value = String::new());
        while ((flext.current.map(|x| NUMBERS.contains(x)).unwrap_or(false)) {
            value.push(flext.current.unwrap());
            flext.advance();
        } else (flext.current.is_none()) {
            compile_error!((E001, flext.position.position()) flext.current.unwrap())
        });
        ok (Self(flext.position.position(), u64::from_str(&value).unwrap()));
    }
}

compile_error! {
    [[Define]]
    (E001) "invalid number": ((1) "expected number not `", "`");
}

pub struct Flext {
    pub position: MutCursor,
    pub current: Option<char>,
}

impl Flext {
    pub fn advance(&mut self) {
        self.current = self.position.pos_end.get_char();
    }
}