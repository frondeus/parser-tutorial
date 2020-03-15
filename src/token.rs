use std::fmt::Debug;

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
