use std::{fs, time::Instant};
use flexar::{lext::Lext, flext::Flext};

flexar::compiler_error! {
    [[Define]]
    (E001) "unexpected character": ((1) "character `", "` is unexpected");
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
    Ident(String),
}

flexar::flexar! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((E001, lext.position()) current).throw();

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

    ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"] child {
        set ident { String::new() };
        rsome (current, 'ident) {
            set matched false;
            ck (current, ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-0123456789"]) {
                mut matched true;
                { ident.push(current) };
            };
            { if !matched { break 'ident } };
        };
        done Ident(ident);
    };

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
        let time = Instant::now();
    let tokens = Token::tokenise(Lext::new("example.fx".into(), &contents));
        print_time("Tokenising completed in", time);
        let _time = Instant::now();
    println!("{tokens:?}")
}

fn print_time(str: &str, time: Instant) {
    println!("\x1b[32m{str}: \x1b[33m{}s\x1b[0m", time.elapsed().as_secs_f64());
}