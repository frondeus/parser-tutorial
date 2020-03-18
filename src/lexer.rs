use crate::peekable::PeekableIterator;
use crate::token::{Token, TokenKind};
use crate::input::Input;
use crate::TextRange;

pub trait Lexer<K>
where
    Self: PeekableIterator<Item = Token<K>>,
    K: TokenKind,
{
    fn input(&self) -> Input;
    fn set_input(&mut self, input: Input);
}

pub struct Lex<F, K>
where
    K: TokenKind,
{
    input: Input,
    f: F,
    #[allow(clippy::option_option)]
    peeked: Option<Option<Token<K>>>,
}

impl<F, K> Lex<F, K>
where
    K: TokenKind,
    F: Fn(&mut Input) -> Option<(K, TextRange)>,
{
    pub fn new(input: &str, f: F) -> Self {
        Self {
            input: input.into(),
            f,
            peeked: None,
        }
    }
}

impl<F, K> Lex<F, K>
where
    F: Fn(&mut Input) -> Option<(K, TextRange)>,
    K: TokenKind,
{
    fn lex(&mut self) -> Option<Token<K>> {
        if let Some(peeked) = self.peeked.take() {
            if let Some(peeked) = peeked.as_ref() {
                self.input.cursor = peeked.span.end();
            }
            return peeked;
        }

        let (kind, span) = (self.f)(&mut self.input)?;

        Some(Token::new(span, kind))
    }
}

impl<F, K> Iterator for Lex<F, K>
where
    F: Fn(&mut Input) -> Option<(K, TextRange)>,
    K: TokenKind,
{
    type Item = Token<K>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut first = self.lex()?;

        while first.kind.is_error() {
            match self.peek() {
                Some(token) if token.kind.is_error() => {
                    first.span = TextRange::covering(first.span, token.span);
                    self.lex();
                }
                _ => break,
            }
        }
        Some(first)
    }
}

impl<F, K> PeekableIterator for Lex<F, K>
where
    F: Fn(&mut Input) -> Option<(K, TextRange)>,
    K: TokenKind,
{
    fn peek(&mut self) -> Option<&Self::Item> {
        if self.peeked.is_none() {
            let i = self.input.cursor;
            self.peeked = Some(self.next());
            self.input.cursor = i;
        }

        self.peeked.as_ref().and_then(|i| i.as_ref())
    }
}

impl<F, K> Lexer<K> for Lex<F, K>
where
    F: Fn(&mut Input) -> Option<(K, TextRange)>,
    K: TokenKind,
{
    fn input(&self) -> Input {
        self.input.clone()
    }

    fn set_input(&mut self, input: Input) {
        self.input = input;
    }
}
