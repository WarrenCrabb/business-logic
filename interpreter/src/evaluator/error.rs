use std::fmt;

#[derive(Debug)]
pub struct EvaluationError {
    pub msg: String,
}

impl EvaluationError {
    pub fn new(msg: String) -> EvaluationError {
        EvaluationError { msg }
    }
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ERROR: {} ", self.msg)
    }
}
