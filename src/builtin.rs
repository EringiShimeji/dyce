use crate::{
    ast::Node,
    environment::{Environment, FunctionForm, FunctionKind},
    eval::eval,
    object::Object,
    IntegerType,
};
use rand::{seq::IteratorRandom, thread_rng};

pub fn eval_builtin(
    key: &FunctionForm,
    parameters: Vec<Box<Node>>,
    env: &Environment,
) -> Option<Object> {
    match key.name() {
        "D" | "d" => {
            if key.kind() == &FunctionKind::Infix {
                let count = eval(parameters.iter().nth(0)?.clone(), env).ok()?;
                let kind = eval(parameters.iter().nth(1)?.clone(), env).ok()?;

                if let Object::Integer(count) = count {
                    if let Object::Integer(kind) = kind {
                        return Some(Object::Integer(roll(count, kind)?));
                    }
                }

                return None;
            }
        }
        _ => {}
    }

    None
}

fn roll(count: IntegerType, kind: IntegerType) -> Option<IntegerType> {
    if count < 0 || kind < 1 {
        return None;
    }

    (0..count)
        .map(|_| (1..=kind).choose(&mut thread_rng()))
        .sum()
}
