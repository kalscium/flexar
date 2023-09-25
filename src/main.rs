use std::fs;

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
    [[Token] flext: Flext, current, 'cycle]
    else flexar::compiler_error!((E001, flext.cursor.position()) current).throw();

    Plus: +;
    LParen: '(';
    RParen: ')';
    Minus: '-';
    Mul: *;
    Div: /;
    Let: let;
    EQ: =;
    Semi: ;;
    [" \n\t"] >> ({ flext.advance(); flext = flext.spawn(); continue 'cycle; });

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
    let tokens = Token::tokenise(Flext::new("example.fx".into(), &contents));
    println!("{tokens:?}")
}