use crate::offset::Offset;
use crate::peekable::PeekableIterator;
use crate::token::{Token, TokenKind};

pub trait Lexer<'a, K>
where
    Self: PeekableIterator<Item = Token<'a, K>>,
    K: TokenKind,
{
    fn input(&self) -> &'a str;
    fn set_input(&mut self, input: &'a str);
}

pub struct Lex<'a, F, K>
where
    K: TokenKind,
{
    input: &'a str,
    f: F,
    #[allow(clippy::option_option)]
    peeked: Option<Option<Token<'a, K>>>,
}

impl<'a, F, K> Lex<'a, F, K>
where
    K: TokenKind,
    F: Fn(&'a str) -> Option<(K, &'a str)>,
{
    pub fn new(input: &'a str, f: F) -> Self {
        Self {
            input,
            f,
            peeked: None,
        }
    }
}

impl<'a, F, K> Lex<'a, F, K>
where
    F: Fn(&'a str) -> Option<(K, &'a str)>,
    K: TokenKind,
{
    fn lex(&mut self) -> Option<Token<'a, K>> {
        if let Some(peeked) = self.peeked.take() {
            if let Some(peeked) = peeked.as_ref() {
                let input = &self.input[peeked.span.len()..];
                self.input = input;
            }
            return peeked;
        }

        let (kind, rest) = (self.f)(self.input)?;

        let offset = self.input.offset(rest);

        let span = &self.input[0..offset];

        self.input = rest;
        Some(Token::new(span, kind))
    }
}

impl<'a, F, K> Iterator for Lex<'a, F, K>
where
    F: Fn(&'a str) -> Option<(K, &'a str)>,
    K: TokenKind,
{
    type Item = Token<'a, K>;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input;
        let mut first = self.lex()?;

        while first.kind.is_error() {
            match self.peek() {
                Some(token) if token.kind.is_error() => {
                    let first_len = first.span.len();
                    let second_len = token.span.len();
                    let len = first_len + second_len;
                    first.span = &input[..len];
                    self.lex();
                }
                _ => break,
            }
        }
        Some(first)
    }
}

impl<'a, F, K> PeekableIterator for Lex<'a, F, K>
where
    F: Fn(&'a str) -> Option<(K, &'a str)>,
    K: TokenKind,
{
    fn peek(&mut self) -> Option<&Self::Item> {
        if self.peeked.is_none() {
            let i = self.input;
            self.peeked = Some(self.next());
            self.input = i;
        }

        self.peeked.as_ref().and_then(|i| i.as_ref())
    }
}

impl<'a, F, K> Lexer<'a, K> for Lex<'a, F, K>
where
    F: Fn(&'a str) -> Option<(K, &'a str)>,
    K: TokenKind,
{
    fn input(&self) -> &'a str {
        self.input
    }

    fn set_input(&mut self, input: &'a str) {
        self.input = input;
    }
}
