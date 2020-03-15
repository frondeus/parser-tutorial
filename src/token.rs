use std::fmt::{Debug, Display, Formatter, Result};

pub trait TokenKind: Debug + PartialEq {
    fn is_error(&self) -> bool;
}

#[derive(Debug, PartialEq)]
pub struct Token<'a, K: TokenKind> {
    pub kind: K,
    pub span: &'a str,
}

impl<'a, K> Token<'a, K>
where
    K: TokenKind,
{
    pub fn new(span: &'a str, kind: K) -> Self {
        Self { kind, span }
    }
}

impl<'a, K> Display for Token<'a, K>
where
    K: TokenKind,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.kind)?;
        write!(f, " `{}`", self.span)
    }
}
