use parsing_tutorial::lexer::{Lex, Lexer};
use parsing_tutorial::token::TokenKind;
use parsing_tutorial::input::Input;

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

pub fn lexer(input: &str) -> impl Lexer<Token> {
    Lex::new(input, |i: &mut Input| {
        Some(match i.as_ref().chars().next()? {
            c if c.is_whitespace() => {
                let rest = i.as_ref()
                    .chars()
                    .take_while(|c| c.is_whitespace())
                    .count();
                (Token::Trivia, i.chomp(rest))
            }
            c if c.is_alphanumeric() => {
                let rest = i.as_ref()
                    .chars()
                    .take_while(|c| c.is_alphanumeric())
                    .count();
                (Token::Atom, i.chomp(rest))
            }
            '(' => (Token::OpenParent, i.chomp(1)),
            ')' => (Token::CloseParent, i.chomp(1)),
            _ => (Token::Error, i.chomp(1)),
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

        let res: Vec<_> = lexer.map(|t| t.display(input).to_string()).collect();
        parsing_tutorial::testing::snap(
            format!("```\n{}\n```\n\n{:#?}", input, res),
            file!(),
            test_case_name,
        );
    }
}

fn main() {}
