use std::{fs, time::Instant};
use flexar::{lext::Lext, flext::Flext, parxt::Parxt, token_node::{Node, self}};

flexar::compiler_error! {
    [[Define]]
    (E001) "unexpected character": ((1) "character `", "` is unexpected");
    (E002) "string not closed": "expected `\"` to close string";
    (E003) "expected number": ((1) "expected number, found `", "`.");
    (E004) "expected an expr": ((1) "expected expr, found `", "`.");
    (E005) "expected `+` or `-` in binary operation": ((1) "expected `+` or `-`, found `", "`.");
    (E006) "unexpected token": ((1) "unexpected token `", "`.");
    (E007) "unclosed parentheses": "expected `)` to close parentheses";
}

flexar::lexer! {
    [[Token] lext, current, 'cycle]
    else flexar::compiler_error!((E001, lext.position()) current).throw();

    token_types {
        LParen => "(";
        RParen => ")";
        Int(val: u32) => val;
        Float(val: f32) => val;
        Plus => "+";
        Minus => "-";
        Mul => "*";
        Div => "/";
        Let => "let";
        EQ => "=";
        Semi => ";";
        Ident(val: String) => val;
    }

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
pub enum Stmt {
    Number(Node<Number>),
    Expr(Node<Expr>),
    Factor(Node<Factor>),
}

#[derive(Debug)]
pub enum Expr {
    Plus(Node<Factor>, Node<Factor>),
    Minus(Node<Factor>, Node<Factor>),
    Factor(Node<Factor>),
}

#[derive(Debug)]
pub enum Factor {
    Mul(Node<Number>, Box<Node<Factor>>),
    Div(Node<Number>, Box<Node<Factor>>),
    Number(Node<Number>),
}

#[derive(Debug)]
pub enum Number {
    Neg(Box<Node<Number>>),
    Expr(Box<Node<Expr>>),
    Int(u32),
    Float(f32),
}

flexar::parser! {
    [[Number] parxt: Token]
    parse start {
        (Token::Plus), [number: Number::parse] => [number];
        (Token::Minus), [number: Number::parse] => (Number::Neg(Box::new(number)));
        (Token::Int(x)) => (Number::Int(*x));
        (Token::Float(x)) => (Number::Float(*x));
        (Token::LParen) => {
            [expr: Expr::parse] => {
                (Token::RParen) => (Number::Expr(Box::new(expr)));
            } (else Err((E007, parxt.position())))
        };
    } else Err((E003, parxt.position()) format!("{:?}", parxt.current()));
}

flexar::parser! {
    [[Factor] parxt: Token]
    parse start {
        [left: Number::parse] => {
            (Token::Mul), [right: Factor::parse] => (Factor::Mul(left, Box::new(right)));
            (Token::Div), [right: Factor::parse] => (Factor::Div(left, Box::new(right)));
        } (else Ok(Factor::Number(left)))
    } else Other(Number Number::parse(parxt));
}

flexar::parser! {
    [[Expr] parxt: Token]
    parse start {
        [left: Factor::parse] => {
            (Token::Plus), [right: Factor::parse] => (Expr::Plus(left, right));
            (Token::Minus), [right: Factor::parse] => (Expr::Minus(left, right));
        } (else Ok(Expr::Factor(left)))
    } else Err((E004, parxt.position()) format!("{}", token_node::Token::display(parxt.current_token())));
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse start {
    } else Err((E006, parxt.position()) format!("{}", token_node::Token::display(parxt.current_token())));
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
    let node = Expr::parse(&mut Parxt::new(&tokens));
        print_time("Parsing completed in", time);
    match node {
        Ok(Node {node: x, ..}) => println!("{:?}", x),
        Err((_, x)) => x.throw(),
    }
}

fn print_time(str: &str, time: Instant) {
    println!("\x1b[32m{str}: \x1b[33m{}s\x1b[0m", time.elapsed().as_secs_f64());
}