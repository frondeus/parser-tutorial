use std::iter::Peekable;

pub trait PeekableIterator: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: Iterator> PeekableIterator for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        Peekable::peek(self)
    }
}
