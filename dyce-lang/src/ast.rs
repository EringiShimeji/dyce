use crate::IntegerType;

#[derive(Default)]
pub struct Program {
    defs: Vec<CommandDefinition>,
}
impl Program {
    pub fn push(&mut self, def: CommandDefinition) {
        self.defs.push(def)
    }

    pub fn defs(&self) -> &Vec<CommandDefinition> {
        &self.defs
    }
}

pub struct CommandDefinition {
    name: String,
    parameters: Vec<String>,
    expr: Box<Node>,
}
impl CommandDefinition {
    pub fn new(name: String, parameters: Vec<String>, expr: Box<Node>) -> Self {
        Self {
            name,
            parameters,
            expr,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    BinaryExpr {
        kind: BinaryExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ComparisonExpr {
        kind: ComparisonExprKind,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    Integer(IntegerType),
    NullaryCommand(String),
    PrefixCommand {
        name: String,
        rhs: Box<Node>,
    },
    InfixCommand {
        name: String,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    PostfixCommand {
        name: String,
        lhs: Box<Node>,
    },
    FunctionCall {
        name: String,
        parameters: Vec<Box<Node>>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonExprKind {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
