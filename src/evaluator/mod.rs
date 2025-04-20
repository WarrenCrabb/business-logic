use crate::{
    object::{FALSE, NULL, Object, TRUE},
    parser::ast::{Expression, Literal, Program, Statement},
    token::Token,
};
use error::EvaluationError;

mod error;

#[cfg(test)]
mod tests;

type EvaluationResult = Result<Object, EvaluationError>;

pub fn eval(program: Program) -> EvaluationResult {
    let mut result = Object::Null;
    for statement in program.body {
        let value = eval_statement(statement)?;

        match value {
            Object::ReturnValue(value) => return Ok(*value),
            _ => result = value,
        }
    }

    Ok(result)
}

fn eval_statement(statement: Statement) -> EvaluationResult {
    let result = match statement {
        Statement::ExpressionStatement(expr) => eval_expression(expr),
        Statement::ReturnStatement(return_stmt) => {
            let value = eval_expression(return_stmt)?;
            Ok(Object::ReturnValue(Box::new(value)))
        }
        Statement::VariableDeclaration { identifier, value } => {
            let value = eval_expression(value)?;
            Ok(Object::Null)
        }
        Statement::EndLine => Ok(Object::Null),
    };

    result
}

fn eval_expression(expr: Expression) -> EvaluationResult {
    let result = match expr {
        Expression::Unary { operator, right } => {
            let right = eval_expression(*right)?;
            eval_prefix_expression(operator, right)?
        }
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(*left)?;
            let right = eval_expression(*right)?;
            eval_infix_expression(operator, left, right)?
        }
        Expression::Literal(lit) => match lit {
            Literal::Number(val) => Object::Integer(val),
            Literal::Boolean(val) => {
                if val {
                    TRUE
                } else {
                    FALSE
                }
            }
            Literal::String(val) => Object::String(val),
        },
        _ => NULL,
    };

    Ok(result)
}

fn eval_prefix_expression(operator: Token, right: Object) -> EvaluationResult {
    match operator {
        Token::NEGATE => Ok(eval_negation_expression(right)),
        Token::MINUS => Ok(eval_minus_prefix_expression(right)),
        _ => Ok(NULL),
    }
}

fn eval_infix_expression(operator: Token, left: Object, right: Object) -> EvaluationResult {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => {
            Ok(eval_integer_infix_expression(operator, l, r))
        }
        _ => Ok(NULL),
    }
}

fn eval_integer_infix_expression(operator: Token, left: i64, right: i64) -> Object {
    match operator {
        Token::PLUS => Object::Integer(left + right),
        Token::MINUS => Object::Integer(left - right),
        Token::MULTIPLY => Object::Integer(left * right),
        Token::DIVIDE => Object::Integer(left / right),
        Token::MODULO => Object::Integer(left % right),
        _ => NULL,
    }
}

fn eval_negation_expression(right: Object) -> Object {
    match right {
        TRUE => FALSE,
        FALSE => TRUE,
        NULL => TRUE,
        _ => FALSE,
    }
}
fn eval_minus_prefix_expression(right: Object) -> Object {
    match right {
        Object::Integer(i) => Object::Integer(-i),
        _ => NULL,
    }
}
