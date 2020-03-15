use parsing_tutorial::lexer::{Lex, Lexer};
use parsing_tutorial::token::TokenKind;

#[derive(Debug, PartialEq)]
pub enum Token {
    Error,
    Atom,
    Trivia, // Whitespace
    OpenParent,
    CloseParent,
}

impl TokenKind for Token {
    fn is_error(&self) -> bool {
        match self {
            Self::Error => true,
            _ => false,
        }
    }
}

pub fn lexer<'a>(input: &'a str) -> impl Lexer<'a, Token> {
    Lex::new(input, |i: &'a str| {
        Some(match i.chars().next()? {
            c if c.is_whitespace() => {
                let rest = i
                    .char_indices()
                    .take_while(|(_, c)| c.is_whitespace())
                    .last()
                    .map(|(idx, c)| idx + c.len_utf8())
                    .unwrap_or_default();
                (Token::Trivia, &i[rest..])
            }
            c if c.is_alphanumeric() => {
                let rest = i
                    .char_indices()
                    .take_while(|(_, c)| c.is_alphanumeric())
                    .last()
                    .map(|(idx, c)| idx + c.len_utf8())
                    .unwrap_or_default();
                (Token::Atom, &i[rest..])
            }
            '(' => (Token::OpenParent, &i[1..]),
            ')' => (Token::CloseParent, &i[1..]),
            _ => (Token::Error, &i[1..]),
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("(add 2 (京 4 5))", "unicode" ; "unicode")]
    #[test_case("(add 2 (+++ 4 5))", "error" ; "error")]
    fn lexer_tests(input: &str, test_case_name: &str) {
        let lexer = lexer(input);

        let res: Vec<_> = lexer.map(|t| t.to_string()).collect();
        parsing_tutorial::testing::snap(
            format!("```\n{}\n```\n\n{:#?}", input, res),
            file!(),
            test_case_name,
        );
    }
}
