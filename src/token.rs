use std::fmt::{Debug, Display, Formatter, Result};
use text_size::TextRange;

pub trait TokenKind: Debug + PartialEq {
    fn is_error(&self) -> bool;
}

#[derive(PartialEq)]
pub struct Token<K: TokenKind> {
    pub kind: K,
    pub span: TextRange,
}

impl<K> Token<K>
where
    K: TokenKind,
{
    pub fn new(span: TextRange, kind: K) -> Self {
        Self { kind, span }
    }

    pub fn display<'k, 's>(&'k self, str: &'s str) -> DisplayToken<'k, 's, K> {
        DisplayToken {  str, kind: &self.kind, span: self.span }
    }
}

impl<K> Debug for Token<K>
    where
        K: TokenKind,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}@{:?}", self.kind, self.span)
    }
}

pub struct DisplayToken<'k, 's, K: TokenKind> {
    str: &'s str,
    kind: &'k K,
    span: TextRange
}

impl<'k, 's, K> Display for DisplayToken<'k, 's, K>
where
    K: TokenKind,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?} `{}`", self.kind, &self.str[self.span])
    }
}
