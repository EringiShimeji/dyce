use crate::{ast::Node, eval::eval, object::Object};
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

    pub fn eval(&self, env: &Environment, parameters: Vec<Box<Node>>) -> Result<Object, ()> {
        if parameters.len() != self.parameters.len() {
            return Err(());
        }

        let mut env = env.clone();

        for (i, param) in parameters.into_iter().enumerate() {
            env.insert(
                FunctionForm::new(self.parameters[i].to_string(), FunctionKind::Nullary),
                Function::new(param, Vec::new()),
            );
        }

        eval(self.node.clone(), &env)
    }
}
