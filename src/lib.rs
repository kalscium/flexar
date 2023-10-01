//! > **An extremely flexible lexer/parser (get it?) for writing your own programming language**
//! 
//! ## Elements of the `flexar` crate
//! - **Compiler Errors**
//! - **File positions**
//! - **Lexer generation macro**
//! - **Parser generation macros**
//! - (You'll have to implement the compiling / interpreting part of the language yourself)
//! 
//! # Example
//! The following is an exapmle implementation of the flexar crate to create a simple math interpreter with support for varibles
//!
//! **example.fx**
//! ```psudeo
//! 6 / 1 + 2 * 3;
//! (1 + 2) * 3 + 4 / 5 - -3;
//! --5;
//! 1.2 * 4.29 / 36; // yoo even comments work :D
//! -12 + 34 / -3.4;
//! let a-value = 23 * 4;
//! a-value - 92;
//! ```
//! ## Code to execute the above code
//! ```rust
//! use std::collections::HashMap;
//! use flexar::{lext::Lext, flext::Flext, parxt::Parxt, token_node::{Node, TokenToString, self}, compiler_error, cursor::Position};
//!
//! //////////////////////////
//! // Errors
//! //////////////////////////
//!
//! flexar::compiler_error! {
//!     [[Define] Errors]
//!     (E001) "unexpected character": ((1) "character `", "` is unexpected");
//!     (E002) "string not closed": "expected `\"` to close string";
//!     (E003) "expected number": ((1) "expected number, found `", "`.");
//!     (E004) "expected an expr": ((1) "expected expr, found `", "`.");
//!     (E005) "expected `+` or `-` in binary operation": ((1) "expected `+` or `-`, found `", "`.");
//!     (E006) "unexpected token": ((1) "unexpected token `", "`.");
//!     (E007) "unclosed parentheses": "expected `)` to close parentheses";
//!     (E008) "expected identifier in `let` statement": ((1) "expected ident, found `", "`.");
//!     (E009) "expected `=` in `let` statement": ((1) "expected `=`, found `", "`.");
//!     (E010) "expected one of `;`, `+`, `-`, `/` or `*`.": ((1) "expected `;` or operation, found `", "`.");
//!     (RT001) "non-existant varible": ((1) "varible `", "` doesn't exist");
//! }
//!
//! //////////////////////////
//! // Lexer
//! //////////////////////////
//!
//! flexar::lexer! {
//!     [[Token] lext, current, 'cycle]
//!     else flexar::compiler_error!((E001, lext.position()) current).throw();
//!
//!     token_types {
//!         LParen => "(";
//!         RParen => ")";
//!         Int(val: u32) => val;
//!         Float(val: f32) => val;
//!         Plus => "+";
//!         Minus => "-";
//!         Mul => "*";
//!         Div => "/";
//!         Let => "let";
//!         EQ => "=";
//!         Semi => ";";
//!         Ident(val: String) => val;
//!     }
//!
//!     Plus: +;
//!     LParen: '(';
//!     RParen: ')';
//!     Minus: '-';
//!     Mul: *;
//!     EQ: =;
//!     Semi: ;;
//!     [" \n\t"] >> ({ lext.advance(); lext = lext.spawn(); continue 'cycle; });
//!     
//!     / child {
//!         advance: current;
//!         ck (current, /) {
//!             rsome current {
//!                 { if current == '\n' { lext = child; continue 'cycle } };
//!             };
//!         };
//!         advance:();
//!         done Div();
//!     };
//!     
//!     ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_"] child {
//!         set ident { String::new() };
//!         rsome (current, 'ident) {
//!             set matched false;
//!             ck (current, ["abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-0123456789"]) {
//!                 mut matched true;
//!                 { ident.push(current) };
//!             };
//!             { if !matched { break 'ident } };
//!         };
//!         if (ident == "let") { done Let(); };
//!         done Ident(ident);
//!     };
//!
//!     ["0123456789"] child {
//!         set number { String::new() };
//!         set dot false;
//!         rsome (current, 'number) {
//!             set matched false;
//!             ck (current, ["0123456789"]) {
//!                 mut matched true;
//!                 { number.push(current) };
//!             };
//!             ck (current, '.') {
//!                 if (dot) {
//!                     done Float(number.parse().unwrap());
//!                 };
//!                 mut matched true;
//!                 mut dot true;
//!                 { number.push(current) };
//!             };
//!             {if !matched {break 'number}};
//!         };
//!         if (dot) { done Float(number.parse().unwrap()); };
//!         done Int(number.parse().unwrap());
//!     };
//! }
//!
//! //////////////////////////
//! // Parser
//! //////////////////////////
//!
//! #[derive(Debug)]
//! pub enum Stmt {
//!     Expr(Node<Expr>),
//!     Let(String, Node<Expr>),
//! }
//!
//! #[derive(Debug)]
//! pub enum Expr {
//!     Plus(Node<Factor>, Box<Node<Expr>>),
//!     Minus(Node<Factor>, Box<Node<Expr>>),
//!     Factor(Node<Factor>),
//! }
//! 
//! #[derive(Debug)]
//! pub enum Factor {
//!     Mul(Node<Number>, Box<Node<Factor>>),
//!     Div(Node<Number>, Box<Node<Factor>>),
//!     Number(Node<Number>),
//! }
//! 
//! #[derive(Debug)]
//! pub enum Number {
//!     Get(String),
//!     Neg(Box<Node<Number>>),
//!     Expr(Box<Node<Expr>>),
//!     Int(u32),
//!     Float(f32),
//! }
//! 
//! #[derive(Debug)]
//! pub enum ProgramFile {
//!     Single(Node<Stmt>),
//!     Program(Box<[Node<Stmt>]>),
//! }
//!
//! flexar::parser! {
//!     [[Number] parxt: Token]
//!     parse {
//!         (Token::Ident(x)) => (Get(x.clone()));
//!         (Token::Plus), [number: Number::parse] => [number];
//!         (Token::Minus), [number: Number::parse] => (Neg(Box::new(number)));
//!         (Token::Int(x)) => (Int(*x));
//!         (Token::Float(x)) => (Float(*x));
//!         (Token::LParen) => {
//!             [expr: Expr::parse] => {
//!                 (Token::RParen) => (Expr(Box::new(expr)));
//!             } (else Err((E007, parxt.position())))
//!         };
//!     } else Err((E003, parxt.position()) parxt.current_token());
//! }
//!
//! flexar::parser! {
//!     [[Factor] parxt: Token]
//!     parse {
//!         [left: Number::parse] => {
//!             (Token::Mul), [right: Factor::parse] => (Mul(left, Box::new(right)));
//!             (Token::Div), [right: Factor::parse] => (Div(left, Box::new(right)));
//!         } (else Ok(Factor::Number(left)))
//!     } else Other(Number Number::parse(parxt));
//! }
//!
//! flexar::parser! {
//!     [[Expr] parxt: Token]
//!     parse {
//!         [left: Factor::parse] => {
//!             (Token::Plus), [right: Expr::parse] => (Plus(left, Box::new(right)));
//!             (Token::Minus), [right: Expr::parse] => (Minus(left, Box::new(right)));
//!         } (else Ok(Expr::Factor(left)))
//!     } else Err((E004, parxt.position()) parxt.current_token());
//! }
//!
//! flexar::parser! {
//!     [[Stmt] parxt: Token]
//!     parse {
//!         [expr: Expr::parse] => (Expr(expr));
//!         (Token::Let) => {
//!             (Token::Ident(ident)) => {
//!                 (Token::EQ), [expr: Expr::parse] => (Let(ident.clone(), expr));
//!             } (else Err((E009, parxt.position()) parxt.current_token()))
//!         } (else Err((E008, parxt.position()) parxt.current_token()))
//!     } else Err((E006, parxt.position()) parxt.current_token());
//! }
//!
//! flexar::parser! {
//!     [[ProgramFile] parxt: Token]
//!     single {
//!         [stmt: Stmt::parse] => {
//!             (Token::Semi) => (Single(stmt));
//!         } (else Err((E010, parxt.position()) parxt.current_token()))
//!     } else Err((E006, parxt.position()) parxt.current_token());
//! }
//!
//! impl ProgramFile {
//!     pub fn parse(tokens: &[token_node::Token<Token>]) -> Option<Self> {
//!         if tokens.len() == 0 { return None }
//!
//!         let mut parxt = Parxt::new(tokens);
//!         let mut stmts = Vec::new();
//!
//!         while parxt.current().is_some() {
//!             match Self::single(&mut parxt) {
//!                 Ok(Node { node: Self::Single(x), .. }) => stmts.push(x),
//!                 Err((_, x)) => x.throw(),
//!                 _ => panic!("not possible"),
//!             }
//!         }
//!
//!         Some(Self::Program(stmts.into_boxed_slice()))
//!     }
//! }
//!
//! //////////////////////////
//! // Interpreter
//! //////////////////////////
//!
//! pub struct VisitCtx(HashMap<String, f32>, Position);
//! pub trait Visit {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32;
//! }
//!
//! impl ProgramFile {
//!     pub fn visit(&self) {
//!         if let Self::Program(stmts) = self {
//!             let mut visit_ctx = VisitCtx(HashMap::new(), stmts[0].position.clone());
//!             stmts.iter()
//!                 .for_each(|x| {x.visit(&mut visit_ctx);});
//!         }
//!     }
//! }
//!
//! impl<N: Visit> Visit for Node<N> {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
//!         visit_ctx.1 = self.position.clone();
//!         self.node.visit(visit_ctx)
//!     }
//! }
//!
//! impl Visit for Number {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
//!         use Number as N;
//!         match self {
//!             N::Int(x) => *x as f32,
//!             N::Neg(x) => -x.visit(visit_ctx),
//!             N::Float(x) => *x,
//!             N::Get(x) => *visit_ctx.0.get(x).unwrap_or_else(||
//!                 compiler_error!((RT001, visit_ctx.1.clone()) x).throw()
//!             ),
//!             N::Expr(x) => x.visit(visit_ctx)
//!         }
//!     }
//! }
//!
//! impl Visit for Factor {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
//!         use Factor as F;
//!         match self {
//!             F::Mul(a, b) => a.visit(visit_ctx) * b.visit(visit_ctx),
//!             F::Div(a, b) => a.visit(visit_ctx) / b.visit(visit_ctx),
//!             F::Number(x) => x.visit(visit_ctx),
//!         }
//!     }
//! }
//!
//! impl Visit for Expr {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
//!         use Expr as E;
//!         match self {
//!             E::Factor(x) => x.visit(visit_ctx),
//!             E::Plus(a, b) => a.visit(visit_ctx) + b.visit(visit_ctx),
//!             E::Minus(a, b) => a.visit(visit_ctx) - b.visit(visit_ctx),
//!         }
//!     }
//! }
//!
//! impl Visit for Stmt {
//!     fn visit(&self, visit_ctx: &mut VisitCtx) -> f32 {
//!         use Stmt as S;
//!         match self {
//!             S::Expr(x) => println!("{}", x.visit(visit_ctx)),
//!             S::Let(key, x) => {
//!                 let value = x.visit(visit_ctx);
//!                 visit_ctx.0.insert(key.clone(), value);
//!             },
//!         }
//!
//!         0f32 // means nothing
//!     }
//! }

pub mod compile_error;
pub mod cursor;
pub mod lext;
pub mod lexer_macro;
pub mod parser_macro;
pub mod parxt;
pub mod flext;
pub mod token_node;

/// Prelude
pub use flext::Flext;