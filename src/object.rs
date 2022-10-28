use crate::IntegerType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(IntegerType),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Integer(v) => v.to_string(),
            }
        )
    }
}
