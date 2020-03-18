use text_size::{TextRange, TextSize, TextSized};
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct Input {
    pub(crate) str: Box<str>,
    pub(crate) cursor: TextSize
}

impl Input {
    pub fn chomp(&mut self, len: usize) -> TextRange {
        let end = TextSize::of(&*self.str);

        let range = match self.as_ref().char_indices().nth(len - 1)
            .and_then(|(last, c)| TextSize::try_from(last + c.len_utf8()).ok())
        {
            Some(last) => TextRange(self.cursor, self.cursor + last),
            None => TextRange(end, end)
        };
        self.cursor = range.end();

        range
    }
}

impl From<&'_ str> for Input {
    fn from(input: &str) -> Self {
        let str: Box<str> = Box::from(input);
        Self { str, cursor: TextSize::zero() }
    }
}

impl AsRef<str> for Input {
    fn as_ref(&self) -> &str {
        let size = self.str.text_size();
        let range = TextRange(self.cursor, size);
        &self.str[range]
    }
}
