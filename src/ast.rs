use crate::IntegerType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Integer(IntegerType),
    Variable(String),
    PrefixCall {
        ident: String,
        rhs: Box<Node>,
    },
    InfixCall {
        ident: String,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    PostfixCall {
        ident: String,
        lhs: Box<Node>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
