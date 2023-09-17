use super::*;

#[allow(clippy::type_complexity)]
pub enum Constructor<'a, Frag: Token<'a>, Out: Token<'a>> {
    Ref(&'a dyn Fn(&Frag) -> TokParseRes<Frag, Out>),
    Owned(Box<dyn Fn(&Frag) -> TokParseRes<Frag, Out>>),
}

impl<'a, Frag: Token<'a>, Out: Token<'a>> Constructor<'a, Frag, Out> {
    pub fn construct(&self, frag: &'a Frag) -> TokParseRes<'a, Frag, Out> {
        match self {
            Self::Ref(x) => x(frag),
            Self::Owned(x) => x(frag),
        }
    }

    pub fn to_ref(&'a self) -> Self {
        match self {
            Self::Ref(x) => Self::Ref(x),
            Self::Owned(x) => Self::Ref(x.as_ref()),
        }
    }
}