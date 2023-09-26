use std::fs;

use flexar::lext::Lext;

flexar::compiler_error! {
    [[Define]]
    (E001) "invalid character": ((1) "character `", "` is invalid");
    (E002) "string not closed": "expected `\"` to close string";
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Int(u32),
    Float(f32),
    Plus,
    Minus,
    Mul,
    Div,
    Let,
    EQ,
    Semi,
}

flexar::lexer! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((E001, lext.cursor.position()) current).throw();

    Plus: +;
    LParen: '(';
    RParen: ')';
    Minus: '-';
    Mul: *;
    Div: /;
    Let: ( l e t );
    EQ: =;
    Semi: ;;
    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });

    ["0123456789"] child {
        set number { String::new() };
        set dot false;
        rsome (current, 'number) {
            set matched false;
            ck (current, ["0123456789"]) {
                mut matched true;
                { number.push(current) };
            };
            ck (current, '.') {
                if (dot) {
                    done Float(number.parse().unwrap());
                };
                mut matched true;
                mut dot true;
                { number.push(current) };
            };
            {if !matched {break 'number}};
        };
        if (dot) { done Float(number.parse().unwrap()); };
        done Int(number.parse().unwrap());
    };
}

fn main() {
    let contents = fs::read_to_string("example.fx").unwrap();
    let tokens = Token::tokenise(Lext::new("example.fx".into(), &contents));
    println!("{tokens:?}")
}