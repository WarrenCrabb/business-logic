use std::{fmt, hash::Hash};

pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Object>),
    Null,
}

impl Object {
    pub fn object_type(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Null => "NULL",
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(val) => write!(f, "{}", val),
            Object::Boolean(val) => write!(f, "{}", val),
            Object::String(val) => write!(f, "{}", val),
            Object::ReturnValue(val) => write!(f, "{}", val),
            Object::Null => write!(f, "null"),
        }
    }
}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer(val) => val.hash(state),
            Object::Boolean(val) => val.hash(state),
            Object::String(val) => val.hash(state),
            _ => unreachable!(),
        }
    }
}
