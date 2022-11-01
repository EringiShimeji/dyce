use crate::token::{Token, TokenKind};

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

    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if !(ch == ' ' || ch == '\r' || ch == '\t') {
                break;
            }

            self.read_char();
        }
    }

    fn read_newlines(&mut self) {
        while let Some('\n') = self.ch {
            self.read_char();
        }
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

    fn read_ident(&mut self) -> String {
        let mut ident = String::new();

        while let Some(ch) = self.ch {
            if !ch.is_ascii_alphabetic() {
                break;
            }

            ident.push(ch);
            self.read_char();
        }

        ident
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let ch = self.ch?;
        let mut literal = ch.to_string();
        let kind = match ch {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '=' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    literal = "==".to_string();
                }

                TokenKind::Eq
            }
            '!' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    literal = "!=".to_string();

                    TokenKind::Ne
                } else {
                    TokenKind::Illegal
                }
            }
            '<' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    literal = "<=".to_string();

                    TokenKind::Le
                } else if let Some('>') = self.peek_char() {
                    self.read_char();
                    literal = "<>".to_string();

                    TokenKind::Ne
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if let Some('=') = self.peek_char() {
                    self.read_char();
                    literal = ">=".to_string();

                    TokenKind::Ge
                } else {
                    TokenKind::Gt
                }
            }
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '\n' => {
                self.read_newlines();

                return Some(Token::new(TokenKind::Separator, literal));
            }
            _ => {
                if ch.is_digit(10) {
                    return Some(Token::new(TokenKind::Number, self.read_number()));
                }
                if ch.is_ascii_alphabetic() {
                    return Some(Token::new(TokenKind::Ident, self.read_ident()));
                }

                TokenKind::Illegal
            }
        };

        self.read_char();

        Some(Token::new(kind, literal))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn separator_tokenize_test() {
        let tests = [(
            "1\n2",
            vec![
                Token::new(TokenKind::Number, "1".to_string()),
                Token::new(TokenKind::Separator, "\n".to_string()),
                Token::new(TokenKind::Number, "2".to_string()),
            ],
        )];

        for (input, expected) in tests {
            let mut lexer = Lexer::new(input.to_string());

            for token in expected {
                assert_eq!(lexer.next_token().unwrap(), token);
            }
        }
    }

    #[test]
    fn ident_tokenize_test() {
        let inputs = ["D", "CCB", "dx"];

        for input in inputs {
            let mut lexer = Lexer::new(input.to_string());

            assert_eq!(
                lexer.next_token().unwrap(),
                Token::new(TokenKind::Ident, input.to_string())
            );
        }
    }

    #[test]
    fn number_tokenize_test() {
        let inputs = ["10", "0"];

        for input in inputs {
            let mut lexer = Lexer::new(input.to_string());

            assert_eq!(
                lexer.next_token().unwrap(),
                Token::new(TokenKind::Number, input.to_string())
            );
        }
    }

    #[test]
    fn arithmetic_expr_tokenize_test() {
        let tests = [
            (
                "1+2-3*4/5",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Minus, "-".to_string()),
                    Token::new(TokenKind::Number, "3".to_string()),
                    Token::new(TokenKind::Asterisk, "*".to_string()),
                    Token::new(TokenKind::Number, "4".to_string()),
                    Token::new(TokenKind::Slash, "/".to_string()),
                    Token::new(TokenKind::Number, "5".to_string()),
                ],
            ),
            (
                "(1+2)*(3-4)/(5-(6-7))",
                vec![
                    Token::new(TokenKind::LParen, "(".to_string()),
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::RParen, ")".to_string()),
                    Token::new(TokenKind::Asterisk, "*".to_string()),
                    Token::new(TokenKind::LParen, "(".to_string()),
                    Token::new(TokenKind::Number, "3".to_string()),
                    Token::new(TokenKind::Minus, "-".to_string()),
                    Token::new(TokenKind::Number, "4".to_string()),
                    Token::new(TokenKind::RParen, ")".to_string()),
                    Token::new(TokenKind::Slash, "/".to_string()),
                    Token::new(TokenKind::LParen, "(".to_string()),
                    Token::new(TokenKind::Number, "5".to_string()),
                    Token::new(TokenKind::Minus, "-".to_string()),
                    Token::new(TokenKind::LParen, "(".to_string()),
                    Token::new(TokenKind::Number, "6".to_string()),
                    Token::new(TokenKind::Minus, "-".to_string()),
                    Token::new(TokenKind::Number, "7".to_string()),
                    Token::new(TokenKind::RParen, ")".to_string()),
                    Token::new(TokenKind::RParen, ")".to_string()),
                ],
            ),
            (
                "1+2<10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Lt, "<".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2<=10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Le, "<=".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2<>10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Ne, "<>".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2>10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Gt, ">".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2>=10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Ge, ">=".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2==10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Eq, "==".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2!=10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Ne, "!=".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "1+2=10",
                vec![
                    Token::new(TokenKind::Number, "1".to_string()),
                    Token::new(TokenKind::Plus, "+".to_string()),
                    Token::new(TokenKind::Number, "2".to_string()),
                    Token::new(TokenKind::Eq, "=".to_string()),
                    Token::new(TokenKind::Number, "10".to_string()),
                ],
            ),
            (
                "CCB<=50",
                vec![
                    Token::new(TokenKind::Ident, "CCB".to_string()),
                    Token::new(TokenKind::Le, "<=".to_string()),
                    Token::new(TokenKind::Number, "50".to_string()),
                ],
            ),
        ];

        for (input, expected) in tests {
            let mut lexer = Lexer::new(input.to_string());

            for token in expected {
                assert_eq!(lexer.next_token().unwrap(), token);
            }
        }
    }
}
