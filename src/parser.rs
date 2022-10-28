use crate::{
    ast::{BinaryExprKind, Node},
    lexer::Lexer,
    token::{Token, TokenKind},
    IntegerType,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
    next_token: Option<Token>,
}
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            cur_token: None,
            next_token: None,
        };

        parser.read_token();
        parser.read_token();

        parser
    }

    fn read_token(&mut self) {
        self.cur_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn consume(&mut self, expected: TokenKind) -> Option<Token> {
        let token = self.cur_token.as_ref()?;

        if token.kind() != expected {
            return None;
        }

        let token = token.clone();

        self.read_token();

        Some(token)
    }

    fn expect(&mut self, expected: TokenKind) -> Result<Token, ()> {
        let token = self.cur_token.as_ref().ok_or(())?;

        if token.kind() != expected {
            return Err(());
        }

        let token = token.clone();

        self.read_token();

        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Box<Node>, ()> {
        self.expr()
    }

    // expr = add
    fn expr(&mut self) -> Result<Box<Node>, ()> {
        self.add()
    }

    // add = mul ( "+" mul | "-" mul )*
    fn add(&mut self) -> Result<Box<Node>, ()> {
        let mut node = self.mul()?;

        loop {
            if self.consume(TokenKind::Plus).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: node,
                    rhs: self.mul()?,
                })
            } else if self.consume(TokenKind::Minus).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: node,
                    rhs: self.mul()?,
                })
            } else {
                return Ok(node);
            }
        }
    }

    // mul = primary ( "*" primary | "/" primary )*
    fn mul(&mut self) -> Result<Box<Node>, ()> {
        let mut node = self.primary()?;

        loop {
            if self.consume(TokenKind::Asterisk).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: node,
                    rhs: self.primary()?,
                })
            } else if self.consume(TokenKind::Slash).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Div,
                    lhs: node,
                    rhs: self.primary()?,
                })
            } else {
                return Ok(node);
            }
        }
    }

    // primary = number | "(" expr ")"
    fn primary(&mut self) -> Result<Box<Node>, ()> {
        if self.consume(TokenKind::LParen).is_some() {
            let node = self.expr()?;

            self.expect(TokenKind::RParen)?;

            return Ok(node);
        }

        Ok(Box::new(Node::Integer(
            self.expect(TokenKind::Number)?
                .literal()
                .parse::<IntegerType>()
                .or(Err(()))?,
        )))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer_parse_test() {
        let inputs = ["10", "0"];

        for input in inputs {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            assert_eq!(
                parser.parse().unwrap(),
                Box::new(Node::Integer(input.parse::<IntegerType>().unwrap()))
            )
        }
    }

    #[test]
    fn arithmetic_parse_tokenize_test() {
        let tests = [
            (
                "1+2-3*4/5",
                Node::BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Node::Integer(1)),
                        rhs: Box::new(Node::Integer(2)),
                    }),
                    rhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Div,
                        lhs: Box::new(Node::BinaryExpr {
                            kind: BinaryExprKind::Mul,
                            lhs: Box::new(Node::Integer(3)),
                            rhs: Box::new(Node::Integer(4)),
                        }),
                        rhs: Box::new(Node::Integer(5)),
                    }),
                },
            ),
            (
                "1+2*3-4/5",
                Node::BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Node::Integer(1)),
                        rhs: Box::new(Node::BinaryExpr {
                            kind: BinaryExprKind::Mul,
                            lhs: Box::new(Node::Integer(2)),
                            rhs: Box::new(Node::Integer(3)),
                        }),
                    }),
                    rhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Div,
                        lhs: Box::new(Node::Integer(4)),
                        rhs: Box::new(Node::Integer(5)),
                    }),
                },
            ),
            (
                "1+2*(3-4)/5",
                Node::BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(Node::Integer(1)),
                    rhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Div,
                        lhs: Box::new(Node::BinaryExpr {
                            kind: BinaryExprKind::Mul,
                            lhs: Box::new(Node::Integer(2)),
                            rhs: Box::new(Node::BinaryExpr {
                                kind: BinaryExprKind::Sub,
                                lhs: Box::new(Node::Integer(3)),
                                rhs: Box::new(Node::Integer(4)),
                            }),
                        }),
                        rhs: Box::new(Node::Integer(5)),
                    }),
                },
            ),
        ];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            assert_eq!(parser.parse().unwrap(), Box::new(expected));
        }
    }
}
