use crate::compile_error::CompileError;
use super::*;

pub struct Flext<'a, Frag: Token<'a>, Out: Token<'a>> {
    idx: u16,
    constructors: Vec<Constructor<'a, Frag, Out>>,
    tok_stream: &'a RefCell<Vec<Out>>,
    err_stream: &'a RefCell<Vec<Out>>,
    failures: Vec<CompileError>,
}

impl<'a, Frag: Token<'a>, Out: Token<'a, Frag = Frag>> Flext<'a, Frag, Out> {
    pub fn new(idx: u16, flexer: &'a Flexer<'a, Frag, Out>) -> Self {
        Self {
            idx,
            tok_stream: &flexer.tok_stream,
            failures: Vec::new(),
            constructors: flexer.constructors
                .iter()
                .map(|x| x.to_ref())
                .collect(),
        }
    }

    pub fn parse(mut self, frag: &'a Frag) -> FlextResult<'a, Frag, Out> {
        let mut constructors = Vec::<Constructor<'a, Frag, Out>>::new();
        
        self.constructors.into_iter()
            .map(|x| x.construct(frag))
            .for_each(|r| 
                match r {
                    TokParseRes::Done(x) => self.tok_stream.borrow_mut()[self.idx as usize] = x, // Takes the most complex token (change to push for all)
                    TokParseRes::Continue(x) => constructors.push(x),
                    TokParseRes::Failed(x) => self.failures.push(x),
                }
            );

        if constructors.is_empty() {
            if self.failures.is_empty() { FlextResult::Done }
            else { FlextResult::Failed(self.failures[0]) } // Use simplest error instead of most complex
        } else {
            FlextResult::Continue(
                Self {
                    idx: self.idx,
                    tok_stream: self.tok_stream,
                    failures: Vec::new(),
                    constructors,
                }
            )
        }
    }
}