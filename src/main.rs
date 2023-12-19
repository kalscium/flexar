use std::{fs, time::Instant, collections::HashMap, fmt::Debug};
use flexar::prelude::*;

//////////////////////////
// Errors
//////////////////////////

flexar::compiler_error! {
    [[Define] CompileErrors]
    (E001) "unexpected character": "character `", "` is unexpected";
    (E002) "string not closed": "expected `\"` to close string";
    (E003) "expected number": "expected number, found `", "`.";
    (E004) "expected an expr": "expected expr, found `", "`.";
    (E005) "expected `+` or `-` in binary operation": "expected `+` or `-`, found `", "`.";
    (E006) "unexpected token": "unexpected token `", "`.";
    (E007) "unclosed parentheses": "expected `)` to close parentheses";
    (E008) "expected identifier in `let` statement": "expected ident, found `", "`.";
    (E009) "expected `=` in `let` statement": "expected `=`, found `", "`.";
    (E010) "expected one of `;`, `+`, `-`, `/` or `*`.": "expected `;` or operation, found `", "`.";
    (RT001) "non-existant varible": "varible `", "` doesn't exist";
}

//////////////////////////
// Lexer
//////////////////////////

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
    EQ: =;
    Semi: ;;
    [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });
    
    / child {
        advance: current;
        ck (current, /) {
            rsome current {
                { if current == '\n' { lext = child; continue 'cycle } };
            };
        };
        advance:();
        done Div();
    };
    
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

//////////////////////////
// Parser
//////////////////////////

#[derive(Debug)]
pub enum Stmt {
    Expr(Node<Expr>),
    Let(String, Node<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Plus(Node<Factor>, Box<Node<Expr>>),
    Minus(Node<Factor>, Box<Node<Expr>>),
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
    Get(String),
    Neg(Box<Node<Number>>),
    Expr(Box<Node<Expr>>),
    Int(u32),
    Float(f32),
}

#[derive(Debug)]
pub enum ProgramFile {
    Single(Node<Stmt>),
    Program(Box<[Node<Stmt>]>),
}

flexar::parser! {
    [[Number] parxt: Token]
    parse {
        (Ident(x)) => (Get(x.clone()));
        (Plus), [number: Number::parse] => [number];
        (Minus), [number: Number::parse] => (Neg(Box::new(number)));
        (Int(x)) => (Int(*x));
        (Float(x)) => (Float(*x));
        (LParen) => {
            [expr: Expr::parse] => {
                (RParen) => (Expr(Box::new(expr)));
            } (else Err(E007))
        };
    } else Err(E003: parxt.current_token());
}

flexar::parser! {
    [[Factor] parxt: Token]
    parse {
        [left: Number::parse] => {
            (Mul), [right: Factor::parse] => (Mul(left, Box::new(right)));
            (Div), [right: Factor::parse] => (Div(left, Box::new(right)));
        } (else Ok(Factor::Number(left)))
    } else Other(Number Number::parse(parxt));
}

flexar::parser! {
    [[Expr] parxt: Token]
    parse {
        [left: Factor::parse] => {
            (Plus), [right: Expr::parse] => (Plus(left, Box::new(right)));
            (Minus), [right: Expr::parse] => (Minus(left, Box::new(right)));
        } (else Ok(Expr::Factor(left)))
    } else Err(E004: parxt.current_token());
}

flexar::parser! {
    [[Stmt] parxt: Token]
    parse {
        [expr: Expr::parse] => (Expr(expr));
        (Let) => {
            (Ident(ident)) => {
                (EQ), [expr: Expr::parse] => (Let(ident.clone(), expr));
            } (else Err(E009: parxt.current_token()))
        } (else Err(E008: parxt.current_token()))
    } else Err(E006: parxt.current_token());
}

flexar::parser! {
    [[ProgramFile] parxt: Token]
    single {
        [stmt: Stmt::parse] => {
            (Semi) => (Single(stmt));
        } (else Err(E010: parxt.current_token()))
    } else Err(E006: parxt.current_token());
}

impl ProgramFile {
    pub fn parse(tokens: &[token_node::Token<Token>]) -> Option<Self> {
        if tokens.is_empty() { return None }

        let mut parxt = Parxt::new(tokens);
        let mut stmts = Vec::new();

        while parxt.current().is_some() {
            match Self::single(&mut parxt) {
                Ok(Node { node: Self::Single(x), .. }) => stmts.push(x),
                Err((_, x)) => x.throw(),
                _ => panic!("not possible"),
            }
        }

        Some(Self::Program(stmts.into_boxed_slice()))
    }
}

//////////////////////////
// Interpreter
//////////////////////////

pub struct VisitCtx(HashMap<String, f32>, Position);
pub trait Visit {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32;
}

impl ProgramFile {
    pub fn visit(&self) {
        if let Self::Program(stmts) = self {
            let mut visit_ctx = VisitCtx(HashMap::new(), stmts[0].position.clone());
            stmts.iter()
                .for_each(|x| {x.visit(&mut visit_ctx);});
        }
    }
}

impl<N: Visit + Debug> Visit for Node<N> {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
        visit_ctx.1 = self.position.clone();
        self.node.visit(visit_ctx)
    }
}

impl Visit for Number {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
        use Number as N;
        match self {
            N::Int(x) => *x as f32,
            N::Neg(x) => -x.visit(visit_ctx),
            N::Float(x) => *x,
            N::Get(x) => *visit_ctx.0.get(x).unwrap_or_else(||
                compiler_error!((RT001, visit_ctx.1.clone()) x).throw()
            ),
            N::Expr(x) => x.visit(visit_ctx)
        }
    }
}

impl Visit for Factor {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
        use Factor as F;
        match self {
            F::Mul(a, b) => a.visit(visit_ctx) * b.visit(visit_ctx),
            F::Div(a, b) => a.visit(visit_ctx) / b.visit(visit_ctx),
            F::Number(x) => x.visit(visit_ctx),
        }
    }
}

impl Visit for Expr {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
        use Expr as E;
        match self {
            E::Factor(x) => x.visit(visit_ctx),
            E::Plus(a, b) => a.visit(visit_ctx) + b.visit(visit_ctx),
            E::Minus(a, b) => a.visit(visit_ctx) - b.visit(visit_ctx),
        }
    }
}

impl Visit for Stmt {
    fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
        use Stmt as S;
        match self {
            S::Expr(x) => println!("{}", x.visit(visit_ctx)),
            S::Let(key, x) => {
                let value = x.visit(visit_ctx);
                visit_ctx.0.insert(key.clone(), value);
            },
        }

        0f32 // means nothing
    }
}

//////////////////////////
// Main function
//////////////////////////

fn main() {
    let contents = fs::read_to_string("example.fx").unwrap();

    // Lexer
        let first_time = Instant::now();
    let tokens = Token::tokenize(Lext::new("example.fx".into(), &contents));
        print_time("Tokenising completed in", first_time);
    println!("{:?}", tokens.iter().map(|x| &x.token_type).collect::<Box<[&Token]>>());

    // Parser
        let time = Instant::now();
    let node = ProgramFile::parse(&tokens);
        print_time("Parsing completed in", time);
        let node = match node {
            Some(x) => x,
            None => return,
        };
    println!("{:#?}", node);
        
    // Interpreter
        let time = Instant::now();
    println!("\n\x1b[34m=== Program output ===\x1b[0m");
    node.visit();
    println!("\x1b[34m=== Program output ===\x1b[0m\n");
        print_time("Interpreting completed in", time);

    print_time("Full program finished in", time);
}

fn print_time(str: &str, time: Instant) {
    println!("\x1b[32m{str}: \x1b[33m{}s\x1b[0m", time.elapsed().as_secs_f64());
}