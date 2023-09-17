pub mod flext;
pub mod token;
pub mod constructor;

pub use token::Token;
pub use flext::Flext;
pub use constructor::Constructor;

use std::cell::RefCell;
use token::TokParseRes;

use crate::compile_error::CompileError;

use self::flext::FlextResult;

/// **note:** Clone doesn't clone internal value; reference counted smart pointer
pub struct Flexer<'a, Frag: Token<'a>, Out: Token<'a, Frag = Frag>> {
    constructors: Box<[Constructor<'a, Frag, Out>]>,
    flexts: Box<[Flext<'a, Frag, Out>]>,
    tok_stream: RefCell<Vec<Out>>,
    err_stream: RefCell<Vec<Option<CompileError>>>,
    idx: u16,
}

impl<'a, Frag: Token<'a>, Out: Token<'a, Frag = Frag>> Flexer<'a, Frag, Out> {
    #[inline]
    pub fn new(constructors: Box<[Constructor<'a, Frag, Out>]>) -> Self {
        Self {
            constructors,
            flexts: Box::new([]),
            tok_stream: RefCell::new(Vec::new()),
            err_stream: RefCell::new(Vec::new()),
            idx: 0,
        }
    }

    pub fn parse(&mut self, frag: &'a Frag) {
        let flexts = std::mem::replace(&mut self.flexts, Box::new([]));
        let mut new_stream: bool = false; // So that there is only one new outstream per parse cycle

        self.flexts = flexts.into_vec().into_iter()
            .map(|x| match x.parse(frag) {
                FlextResult::Continue(x) => Some(x),
                FlextResult::Done => if !new_stream { new_stream = true; self.idx += 1; Some(Flext::new(self.idx, self)) } else { None },
                FlextResult::Failed(x) => { self }
            }).collect();
    }
}

pub enum Or<A, B> {
    A(A),
    B(B),
}

// impl<Frags: Token, Out: Token> Clone for Flexer<Frags, Out> {
//     fn clone(&self) -> Self {
//         Self {
//             constructors: self.constructors.clone(),
//             constructed: self.constructed.clone(),
//         }
//     }
// }