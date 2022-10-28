use crate::{
    ast::{BinaryExprKind, Node},
    object::Object,
};

pub fn eval(node: Box<Node>) -> Result<Object, ()> {
    if let Node::Integer(val) = *node {
        return Ok(Object::Integer(val));
    }

    if let Node::BinaryExpr { kind, lhs, rhs } = *node {
        let lhs = eval(lhs)?;
        let rhs = eval(rhs)?;

        if let Object::Integer(lhs) = lhs {
            if let Object::Integer(rhs) = rhs {
                return Ok(Object::Integer(match kind {
                    BinaryExprKind::Add => lhs + rhs,
                    BinaryExprKind::Sub => lhs - rhs,
                    BinaryExprKind::Mul => lhs * rhs,
                    BinaryExprKind::Div => {
                        if rhs == 0 {
                            return Err(());
                        } else {
                            lhs / rhs
                        }
                    }
                }));
            }
        }
    }

    Err(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser, IntegerType};

    #[test]
    fn integer_eval_test() {
        let inputs = ["10", "0"];

        for input in inputs {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(
                eval(node).unwrap(),
                Object::Integer(input.parse::<IntegerType>().unwrap())
            );
        }
    }

    #[test]
    fn arithmetic_expr_eval_test() {
        let tests = [
            ("1+2-3*4-6/2", -12),
            ("1+(2-3)*4-6/2", -6),
            ("1+(2-3)*(4-6)/2", 2),
            ("1+((2-3)*4-6)/2", -4),
        ];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(eval(node).unwrap(), Object::Integer(expected));
        }
    }
}
