use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}
impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> String {
        let mut num = String::new();

        while let Some(ch) = self.ch {
            if !ch.is_digit(10) {
                break;
            }

            num.push(ch);
            self.read_char();
        }

        num
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let ch = self.ch?;
        let mut literal = ch.to_string();
        let kind = match ch {
            ch => {
                if ch.is_digit(10) {
                    literal = self.read_number();
                    TokenKind::Number
                } else {
                    TokenKind::Illegal
                }
            }
        };

        Some(Token::new(kind, literal))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_tokenize_test() {
        let tests = ["10", "0", "43"];

        for input in tests {
            let mut lexer = Lexer::new(input.to_string());

            assert_eq!(
                lexer.next_token().unwrap(),
                Token::new(TokenKind::Number, input.to_string())
            );
        }
    }
}
