use crate::{ast::Node, object::Object};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<FunctionForm, Function>,
}
impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, key: &FunctionForm) -> Option<&Function> {
        self.store.get(key)
    }

    pub fn insert(&mut self, key: FunctionForm, value: Function) -> Option<Function> {
        self.store.insert(key, value)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum FunctionKind {
    Nullary,
    Prefix,
    Infix,
    Postfix,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionForm {
    name: String,
    kind: FunctionKind,
}
impl FunctionForm {
    pub fn new(name: String, kind: FunctionKind) -> Self {
        Self { name, kind }
    }
}

#[derive(Clone)]
pub struct Function {
    node: Box<Node>,
    parameters: Vec<String>,
}
impl Function {
    pub fn new(node: Box<Node>, parameters: Vec<String>) -> Self {
        Self { node, parameters }
    }

    pub fn node(&self) -> Box<Node> {
        self.node.clone()
    }

    pub fn parameters(&self) -> Vec<String> {
        self.parameters.clone()
    }
}
