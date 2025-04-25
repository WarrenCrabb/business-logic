use std::{
    fmt::{self},
    hash::Hash,
};

use environment::Env;

use crate::evaluator::builtin::Builtin;
use crate::parser::ast::{Block, Identifier};
pub mod environment;

pub const TRUE: Object = Object::Boolean(true);
pub const FALSE: Object = Object::Boolean(false);
pub const NULL: Object = Object::Null;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    ReturnValue(Box<Object>),
    Function(Function),
    Builtin(Builtin),
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Block,
    pub env: Env,
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: Block, env: Env) -> Function {
        Function {
            parameters,
            body,
            env,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters: Vec<String> = self
            .parameters
            .clone()
            .into_iter()
            .map(|elem| format!("{}", elem))
            .collect();
        let parameters = parameters.join(", ");

        write!(f, "fn({}) {{ {} }}", parameters, self.body)
    }
}

impl Object {
    pub fn object_type(&self) -> &str {
        match self {
            Object::Integer(_) => "INTEGER",
            Object::Boolean(_) => "BOOLEAN",
            Object::String(_) => "STRING",
            Object::ReturnValue(_) => "RETURN_VALUE",
            Object::Function(_) => "FUNCTION",
            Object::Builtin(_) => "BUILTIN",
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
            Object::Function(val) => write!(f, "{}", val),
            Object::Builtin(val) => write!(f, "{}", val),
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
