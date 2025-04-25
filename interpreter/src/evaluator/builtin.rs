use super::{EvaluationResult, error::EvaluationError};
use crate::object::Object;
use std::fmt;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Builtin {
    Len,
    Print,
}

fn validate_arg_length(given: usize, expected: usize) -> Result<(), EvaluationError> {
    if given != expected {
        Err(EvaluationError::new(format!(
            "Invalid argument length. Expected={} got={}",
            expected, given
        )))
    } else {
        Ok(())
    }
}

impl Builtin {
    pub fn lookup(ident: &str) -> Option<Object> {
        match ident {
            "len" => Some(Object::Builtin(Builtin::Len)),
            "print" => Some(Object::Builtin(Builtin::Print)),
            _ => None,
        }
    }

    pub fn apply_func(&self, args: Vec<Object>, output_buffer: &mut String) -> EvaluationResult {
        match self {
            Builtin::Len => {
                validate_arg_length(args.len(), 1)?;
                match &args[0] {
                    Object::String(val) => Ok(Object::Integer(val.len() as i64)),
                    _ => Err(EvaluationError::new(format!(
                        "Invalid argument: {}",
                        &args[0]
                    ))),
                }
            }
            Builtin::Print => {
                for arg in args.iter() {
                    write!(output_buffer, "{}", arg).unwrap();
                }
                writeln!(output_buffer).unwrap();
                Ok(Object::Null)
            }
        }
    }
}

impl fmt::Display for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Builtin::Len => write!(f, "len"),
            // Builtin::First => write!(f, "first"),
            // Builtin::Last => write!(f, "last"),
            // Builtin::Rest => write!(f, "rest"),
            // Builtin::Push => write!(f, "push"),
            Builtin::Print => write!(f, "print"),
        }
    }
}
