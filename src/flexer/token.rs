use std::fmt::Debug;
use super::*;
use crate::compile_error::CompileError;

pub enum TokParseRes<'a, Frag: Token<'a>, Out: Token<'a>> {
    Done(Out),
    Failed(CompileError),
    Continue(Constructor<'a, Frag, Out>),
}

pub trait TokenType<'a> {
    type Frag: Token<'a>;
    type Token: Token<'a, Frag = Self::Frag>;
    fn new_token(_: Self) -> Constructor<'a, Self::Frag, Self::Token>;
}

pub trait Token<'a>: Debug {
    type Frag: Token<'a>;
    fn constructors() -> Constructor<'a, Self::Frag, Self> where Self: Sized;
}