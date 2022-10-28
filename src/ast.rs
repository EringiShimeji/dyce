pub type IntegerType = i128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Integer(IntegerType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}
