use crate::{
    ast::{BinaryExprKind, ComparisonExprKind, Node},
    environment::{Environment, Function, FunctionForm, FunctionKind},
    object::Object,
};

pub fn eval(node: Box<Node>, env: &Environment) -> Result<Object, ()> {
    if let Node::Integer(val) = *node {
        return Ok(Object::Integer(val));
    }

    if let Node::BinaryExpr { kind, lhs, rhs } = *node {
        let lhs = eval(lhs, env)?;
        let rhs = eval(rhs, env)?;

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

        return Err(());
    }

    if let Node::ComparisonExpr { kind, lhs, rhs } = *node {
        let lhs = eval(lhs, env)?;
        let rhs = eval(rhs, env)?;

        if kind == ComparisonExprKind::Eq {
            return Ok(Object::Boolean(lhs == rhs));
        }

        if kind == ComparisonExprKind::Ne {
            return Ok(Object::Boolean(lhs != rhs));
        }

        if let Object::Integer(lhs) = lhs {
            if let Object::Integer(rhs) = rhs {
                return Ok(Object::Boolean(match kind {
                    ComparisonExprKind::Lt => lhs < rhs,
                    ComparisonExprKind::Le => lhs <= rhs,
                    ComparisonExprKind::Gt => lhs > rhs,
                    ComparisonExprKind::Ge => lhs >= rhs,
                    _ => return Err(()),
                }));
            }
        }

        return Err(());
    }

    if let Node::NullaryCall(ref name) = *node {
        if let Some(f) = env.get(&FunctionForm::new(name.clone(), FunctionKind::Nullary)) {
            return eval(f.node(), env);
        }
    }

    if let Node::PrefixCall { ref ident, ref rhs } = *node {
        if let Some(f) = env.get(&FunctionForm::new(ident.clone(), FunctionKind::Prefix)) {
            let mut env = env.clone();
            let rhs_name = f.parameters().iter().nth(0).ok_or(())?.clone();

            env.insert(
                FunctionForm::new(rhs_name, FunctionKind::Nullary),
                Function::new(rhs.clone(), Vec::new()),
            );

            return eval(f.node(), &env);
        }
    }

    if let Node::InfixCall {
        ref ident,
        ref lhs,
        ref rhs,
    } = *node
    {
        if let Some(f) = env.get(&FunctionForm::new(ident.clone(), FunctionKind::Infix)) {
            let mut env = env.clone();
            let lhs_name = f.parameters().iter().nth(0).ok_or(())?.clone();
            let rhs_name = f.parameters().iter().nth(1).ok_or(())?.clone();

            env.insert(
                FunctionForm::new(lhs_name, FunctionKind::Nullary),
                Function::new(lhs.clone(), Vec::new()),
            );
            env.insert(
                FunctionForm::new(rhs_name, FunctionKind::Nullary),
                Function::new(rhs.clone(), Vec::new()),
            );

            return eval(f.node(), &env);
        }
    }

    if let Node::PostfixCall { ref ident, ref lhs } = *node {
        if let Some(f) = env.get(&FunctionForm::new(ident.clone(), FunctionKind::Postfix)) {
            let mut env = env.clone();
            let lhs_name = f.parameters().iter().nth(0).ok_or(())?.clone();

            env.insert(
                FunctionForm::new(lhs_name, FunctionKind::Nullary),
                Function::new(lhs.clone(), Vec::new()),
            );

            return eval(f.node(), &env);
        }
    }

    Err(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser, IntegerType};

    #[test]
    fn nullary_call_eval_test() {
        let tests = [("A", 1), ("B", 2), ("C", 3), ("A+B-C", 0)];
        let mut env = Environment::new();

        for (input, expected) in tests {
            env.insert(
                FunctionForm::new(input.to_string(), FunctionKind::Nullary),
                Function::new(Box::new(Node::Integer(expected)), Vec::new()),
            );
        }

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(eval(node, &env).unwrap(), Object::Integer(expected));
        }
    }

    #[test]
    fn prefix_call_eval_test() {
        let tests = [("Succ3", 4), ("Pre3", 2), ("Succ(3+Pre3)+Succ0", 7)];
        let mut env = Environment::new();

        env.insert(
            FunctionForm::new("Succ".to_string(), FunctionKind::Prefix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(Node::NullaryCall("x".to_string())),
                    rhs: Box::new(Node::Integer(1)),
                }),
                vec!["x".to_string()],
            ),
        );
        env.insert(
            FunctionForm::new("Pre".to_string(), FunctionKind::Prefix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(Node::NullaryCall("x".to_string())),
                    rhs: Box::new(Node::Integer(1)),
                }),
                vec!["x".to_string()],
            ),
        );

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(eval(node, &env).unwrap(), Object::Integer(expected));
        }
    }

    #[test]
    fn infix_call_eval_test() {
        let tests = [("1plus3", 4), ("6minus3", 3), ("1plus(2minus3)", 0)];
        let mut env = Environment::new();

        env.insert(
            FunctionForm::new("plus".to_string(), FunctionKind::Infix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Add,
                    lhs: Box::new(Node::NullaryCall("x".to_string())),
                    rhs: Box::new(Node::NullaryCall("y".to_string())),
                }),
                vec!["x".to_string(), "y".to_string()],
            ),
        );
        env.insert(
            FunctionForm::new("minus".to_string(), FunctionKind::Infix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Sub,
                    lhs: Box::new(Node::NullaryCall("x".to_string())),
                    rhs: Box::new(Node::NullaryCall("y".to_string())),
                }),
                vec!["x".to_string(), "y".to_string()],
            ),
        );

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(eval(node, &env).unwrap(), Object::Integer(expected));
        }
    }

    #[test]
    fn postfix_call_eval_test() {
        let tests = [("1x", 2), ("2y", 6), ("2x+3y", 13)];
        let mut env = Environment::new();

        env.insert(
            FunctionForm::new("x".to_string(), FunctionKind::Postfix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: Box::new(Node::NullaryCall("n".to_string())),
                    rhs: Box::new(Node::Integer(2)),
                }),
                vec!["n".to_string()],
            ),
        );
        env.insert(
            FunctionForm::new("y".to_string(), FunctionKind::Postfix),
            Function::new(
                Box::new(Node::BinaryExpr {
                    kind: BinaryExprKind::Mul,
                    lhs: Box::new(Node::NullaryCall("n".to_string())),
                    rhs: Box::new(Node::Integer(3)),
                }),
                vec!["n".to_string()],
            ),
        );

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(eval(node, &env).unwrap(), Object::Integer(expected));
        }
    }

    #[test]
    fn integer_eval_test() {
        let inputs = ["10", "0"];

        for input in inputs {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(
                eval(node, &Environment::new()).unwrap(),
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

            assert_eq!(
                eval(node, &Environment::new()).unwrap(),
                Object::Integer(expected)
            );
        }
    }

    #[test]
    fn boolean_eval_test() {
        let tests = [
            ("1=1", true),
            ("1=2", false),
            ("1==1", true),
            ("1==2", false),
            ("1!=2", true),
            ("1!=1", false),
            ("1<>2", true),
            ("1<>1", false),
            ("1<2", true),
            ("1<1", false),
            ("1<=1", true),
            ("1<=0", false),
            ("2>1", true),
            ("1>1", false),
            ("1>=1", true),
            ("0>=1", false),
            ("1+2=3", true),
            ("1+2=3+1", false),
            ("1+2==3", true),
            ("1+2==3+1", false),
            ("1+2!=3+1", true),
            ("1+2!=3", false),
            ("1+2<>3+1", true),
            ("1+2<>3", false),
            ("1+2<3+1", true),
            ("1+2<3", false),
            ("1+2<=3", true),
            ("1+2<=3-1", false),
            ("3+1>2", true),
            ("3-1>2", false),
            ("3-1>=2", true),
            ("3-1>=2+1", false),
            ("(1=1)=(2=2)", true),
            ("(1=1)=(1=2)", false),
            ("(1=1)==(2=2)", true),
            ("(1=1)==(1=2)", false),
            ("(1=1)!=(1=2)", true),
            ("(1=1)!=(2=2)", false),
            ("(1=1)<>(1=2)", true),
            ("(1=1)<>(2=2)", false),
        ];

        for (input, expected) in tests {
            let lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(lexer);
            let node = parser.parse().unwrap();

            assert_eq!(
                eval(node, &Environment::new()).unwrap(),
                Object::Boolean(expected)
            );
        }
    }
}
