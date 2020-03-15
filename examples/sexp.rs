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
    use super::Token::*;
    use super::*;
    use parsing_tutorial::token::Token;

    #[test]
    fn lexer_test() {
        let input = "(add 2 (京 4 5))";
        let mut lexer = lexer(input);
        let res: Vec<_> = lexer.collect();
        assert_eq!(
            res,
            vec![
                Token {
                    kind: OpenParent,
                    span: "(",
                },
                Token {
                    kind: Atom,
                    span: "add",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "2",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: OpenParent,
                    span: "(",
                },
                Token {
                    kind: Atom,
                    span: "京",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "4",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "5",
                },
                Token {
                    kind: CloseParent,
                    span: ")",
                },
                Token {
                    kind: CloseParent,
                    span: ")",
                },
            ]
        );
    }

    #[test]
    fn lexer_error() {
        let input = "(add 2 (+++ 4 5))";
        let mut lexer = lexer(input);
        let res: Vec<_> = lexer.collect();
        assert_eq!(
            res,
            vec![
                Token {
                    kind: OpenParent,
                    span: "(",
                },
                Token {
                    kind: Atom,
                    span: "add",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "2",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: OpenParent,
                    span: "(",
                },
                Token {
                    kind: Error,
                    span: "+++",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "4",
                },
                Token {
                    kind: Trivia,
                    span: " ",
                },
                Token {
                    kind: Atom,
                    span: "5",
                },
                Token {
                    kind: CloseParent,
                    span: ")",
                },
                Token {
                    kind: CloseParent,
                    span: ")",
                },
            ]
        );
    }
}
