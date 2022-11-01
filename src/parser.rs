use crate::{
    ast::{BinaryExprKind, ComparisonExprKind, Node},
    lexer::Lexer,
    token::{Token, TokenKind},
    IntegerType,
};

pub struct Parser {
    lexer: Lexer,
    cur_token: Option<Token>,
}
impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Self {
            lexer,
            cur_token: None,
        };

        parser.read_token();

        parser
    }

    fn read_token(&mut self) {
        self.cur_token = self.lexer.next_token();
    }

    fn peek(&mut self, expected: TokenKind) -> bool {
        if let Some(t) = &self.cur_token {
            t.kind() == expected
        } else {
            false
        }
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
        self.equality()
    }

    // equality = relational ( "=" relational | "==" relational | "!=" relational | "<>" relational )
    fn equality(&mut self) -> Result<Box<Node>, ()> {
        let mut node = self.relational()?;

        if self.consume(TokenKind::Eq).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Eq,
                lhs: node,
                rhs: self.relational()?,
            })
        } else if self.consume(TokenKind::Ne).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Ne,
                lhs: node,
                rhs: self.relational()?,
            })
        }

        Ok(node)
    }

    // relational = add ( "<" add | "<=" add | ">" add | ">" add )
    fn relational(&mut self) -> Result<Box<Node>, ()> {
        let mut node = self.add()?;

        if self.consume(TokenKind::Lt).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Lt,
                lhs: node,
                rhs: self.add()?,
            })
        } else if self.consume(TokenKind::Le).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Le,
                lhs: node,
                rhs: self.add()?,
            })
        } else if self.consume(TokenKind::Gt).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Gt,
                lhs: node,
                rhs: self.add()?,
            })
        } else if self.consume(TokenKind::Ge).is_some() {
            node = Box::new(Node::ComparisonExpr {
                kind: ComparisonExprKind::Ge,
                lhs: node,
                rhs: self.add()?,
            })
        }

        Ok(node)
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

    // mul = call ( "*" call | "/" call )*
    fn mul(&mut self) -> Result<Box<Node>, ()> {
        let mut node = self.call()?;

        loop {
            if self.consume(TokenKind::Asterisk).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: node,
                    rhs: self.call()?,
                })
            } else if self.consume(TokenKind::Slash).is_some() {
                node = Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Div,
                    lhs: node,
                    rhs: self.call()?,
                })
            } else {
                return Ok(node);
            }
        }
    }

    // call = primary | string | string primary | primary string primary | primary string
    fn call(&mut self) -> Result<Box<Node>, ()> {
        // string | string primary
        if let Some(token) = self.consume(TokenKind::Ident) {
            let ident = token.literal();

            // string primary
            if self.peek(TokenKind::Number) || self.peek(TokenKind::LParen) {
                let rhs = self.primary()?;

                return Ok(Box::new(Node::PrefixCall { ident, rhs }));
            }

            // string
            return Ok(Box::new(Node::NullaryCall(ident)));
        }

        // primary | primary string primary | primary string
        let lhs = self.primary()?;
        let ident = match self.consume(TokenKind::Ident) {
            Some(t) => t.literal(),
            _ => return Ok(lhs), // primary
        };

        // primary string primary
        if self.peek(TokenKind::Number) || self.peek(TokenKind::LParen) {
            let rhs = self.primary()?;

            return Ok(Box::new(Node::InfixCall { ident, lhs, rhs }));
        }

        // primary string
        Ok(Box::new(Node::PostfixCall { ident, lhs }))
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
    fn call_parse_test() {
        let tests = [
            (
                "1D6",
                Node::InfixCall {
                    ident: "D".to_string(),
                    lhs: Box::new(Node::Integer(1)),
                    rhs: Box::new(Node::Integer(6)),
                },
            ),
            ("CCB", Node::NullaryCall("CCB".to_string())),
            (
                "d6",
                Node::PrefixCall {
                    ident: "d".to_string(),
                    rhs: Box::new(Node::Integer(6)),
                },
            ),
            (
                "2d",
                Node::PostfixCall {
                    ident: "d".to_string(),
                    lhs: Box::new(Node::Integer(2)),
                },
            ),
        ];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);

            assert_eq!(parser.parse().unwrap(), Box::new(expected));
        }
    }

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
            (
                "(1+2)D6",
                Node::InfixCall {
                    ident: "D".to_string(),
                    lhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Node::Integer(1)),
                        rhs: Box::new(Node::Integer(2)),
                    }),
                    rhs: Box::new(Node::Integer(6)),
                },
            ),
            (
                "1D(2*(1+2))",
                Node::InfixCall {
                    ident: "D".to_string(),
                    lhs: Box::new(Node::Integer(1)),
                    rhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Mul,
                        lhs: Box::new(Node::Integer(2)),
                        rhs: Box::new(Node::BinaryExpr {
                            kind: BinaryExprKind::Add,
                            lhs: Box::new(Node::Integer(1)),
                            rhs: Box::new(Node::Integer(2)),
                        }),
                    }),
                },
            ),
            (
                "3D6+3",
                Node::BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(Node::InfixCall {
                        ident: "D".to_string(),
                        lhs: Box::new(Node::Integer(3)),
                        rhs: Box::new(Node::Integer(6)),
                    }),
                    rhs: Box::new(Node::Integer(3)),
                },
            ),
            (
                "1+2=3",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Eq,
                    lhs: Box::new(Node::BinaryExpr {
                        kind: BinaryExprKind::Add,
                        lhs: Box::new(Node::Integer(1)),
                        rhs: Box::new(Node::Integer(2)),
                    }),
                    rhs: Box::new(Node::Integer(3)),
                },
            ),
            (
                "2==2",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Eq,
                    lhs: Box::new(Node::Integer(2)),
                    rhs: Box::new(Node::Integer(2)),
                },
            ),
            (
                "2!=2",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Ne,
                    lhs: Box::new(Node::Integer(2)),
                    rhs: Box::new(Node::Integer(2)),
                },
            ),
            (
                "2<>2",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Ne,
                    lhs: Box::new(Node::Integer(2)),
                    rhs: Box::new(Node::Integer(2)),
                },
            ),
            (
                "10<12",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Lt,
                    lhs: Box::new(Node::Integer(10)),
                    rhs: Box::new(Node::Integer(12)),
                },
            ),
            (
                "CCB<=100",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Le,
                    lhs: Box::new(Node::NullaryCall("CCB".to_string())),
                    rhs: Box::new(Node::Integer(100)),
                },
            ),
            (
                "10>12",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Gt,
                    lhs: Box::new(Node::Integer(10)),
                    rhs: Box::new(Node::Integer(12)),
                },
            ),
            (
                "CCB>=10",
                Node::ComparisonExpr {
                    kind: ComparisonExprKind::Ge,
                    lhs: Box::new(Node::NullaryCall("CCB".to_string())),
                    rhs: Box::new(Node::Integer(10)),
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
