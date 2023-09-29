use std::{fs, time::Instant};
use flexar::{lext::Lext, flext::Flext, parxt::Parxt};

flexar::compiler_error! {
    [[Define]]
    (E001) "unexpected character": ((1) "character `", "` is unexpected");
    (E002) "string not closed": "expected `\"` to close string";
    (E003) "expected number": ((1) "expected number, found `", "`.");
    (E004) "expected a binary operation": ((1) "expected binop, found `", "`.");
    (E005) "expected `+` or `-` in binary operation": ((1) "expected `+` or `-`, found `", "`.");
}

#[derive(Debug, Clone, PartialEq)]
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

flexar::lexer! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((E001, lext.position()) current).throw();

    Plus: +;
    LParen: '(';
    RParen: ')';
    Minus: '-';
    Mul: *;
    Div: /;
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
        if (ident == "let") { done Let(); };
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

#[derive(Debug)]
pub enum Expr {
    Number(Number),
    BinOp(BinOp),
    Factor(Factor),
}

#[derive(Debug)]
pub enum BinOp {
    Plus(Factor, Factor),
    Minus(Factor, Factor),
}

#[derive(Debug)]
pub enum Factor {
    Mul(Number, Number),
    Div(Number, Number),
    Number(Number),
}

#[derive(Debug)]
pub enum Number {
    Expr(Box<Expr>),
    Int(u32),
    Float(f32),
}

flexar::parser! {
    [[Number] parxt: Token]
    parse {
        (Token::Int(x)) => (Number::Int(*x));
        (Token::Float(x)) => (Number::Float(*x));
    } else Err((E003, parxt.position()) format!("{:?}", parxt.current()));
}

flexar::parser! {
    [[Factor] parxt: Token]
    parse {

    } else Other(Number Number::parse(parxt));
}

flexar::parser! {
    [[BinOp] parxt: Token]
    parse {
        [left: Factor::parse] => {
            (Token::Plus), [right: Factor::parse] => (BinOp::Plus(left, right));
            (Token::Minus), [right: Factor::parse] => (BinOp::Minus(left, right));
        } (else Err((E005, parxt.position()) format!("{:?}", parxt.current())))
    } else Err((E004, parxt.position()) format!("{:?}", parxt.current()));
}

fn main() {
    let contents = fs::read_to_string("example.fx").unwrap();

    // Lexer
        let time = Instant::now();
    let tokens = Token::tokenize(Lext::new("example.fx".into(), &contents));
        print_time("Tokenising completed in", time);
    println!("{:?}", tokens.iter().map(|x| &x.token_type).collect::<Box<[&Token]>>());

    // Parser
        let time = Instant::now();
    let node = BinOp::parse(&mut Parxt::new(&tokens));
        print_time("Parsing completed in", time);
    match node {
        Ok(x) => println!("{:?}", x),
        Err((_, x)) => x.throw(),
    }
}

fn print_time(str: &str, time: Instant) {
    println!("\x1b[32m{str}: \x1b[33m{}s\x1b[0m", time.elapsed().as_secs_f64());
}