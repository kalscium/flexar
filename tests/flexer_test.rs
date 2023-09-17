use matcha::flexer::{Flexer, token::{Token, TokParseRes}};

#[test]
pub fn flext_multipush() {
    #[derive(Clone, Debug)]
    enum MyToken {
        String(String),
        Int(i32),
    }
}