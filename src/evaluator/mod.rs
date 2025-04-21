use std::rc::Rc;

use crate::{
    object::{
        FALSE, Function, NULL, Object, TRUE,
        environment::{Env, Environment},
    },
    parser::ast::{Block, Expression, Identifier, Literal, Program, Statement},
    token::Token,
};
use error::EvaluationError;

mod error;

#[cfg(test)]
mod tests;

type EvaluationResult = Result<Object, EvaluationError>;

pub fn eval(program: Program, env: &Env) -> EvaluationResult {
    let mut result = Object::Null;
    for statement in program.body {
        let value = eval_statement(statement, env)?;

        match value {
            Object::ReturnValue(value) => return Ok(*value),
            _ => result = value,
        }
    }

    Ok(result)
}

fn eval_statement(statement: Statement, env: &Env) -> EvaluationResult {
    let result = match statement {
        Statement::ExpressionStatement(expr) => eval_expression(expr, env),
        Statement::ReturnStatement(return_stmt) => {
            let value = eval_expression(return_stmt, env)?;
            Ok(Object::ReturnValue(Box::new(value)))
        }
        Statement::VariableDeclaration { identifier, value } => {
            let value = eval_expression(value, env)?;
            env.borrow_mut().set(identifier.value, value);
            Ok(Object::Null)
        }
        Statement::EndLine => Ok(Object::Null),
    };

    result
}

fn eval_expression(expr: Expression, env: &Env) -> EvaluationResult {
    match expr {
        Expression::FunctionLiteral { parameters, body } => Ok(Object::Function(Function::new(
            parameters,
            body,
            Rc::clone(env),
        ))),
        Expression::CallExpression {
            function,
            arguments,
        } => {
            let func = eval_expression(*function, env)?;
            let args = eval_expressions(arguments, env)?;
            apply_function(func, args)
        }
        Expression::Identifier(ident) => eval_identifier(ident, env),
        Expression::Unary { operator, right } => {
            let right = eval_expression(*right, env)?;
            eval_prefix_expression(operator, right)
        }
        Expression::Binary {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(*left, env)?;
            let right = eval_expression(*right, env)?;
            eval_infix_expression(operator, left, right)
        }
        Expression::Literal(lit) => match lit {
            Literal::Number(val) => Ok(Object::Integer(val)),
            Literal::Boolean(val) => {
                if val {
                    Ok(TRUE)
                } else {
                    Ok(FALSE)
                }
            }
            Literal::String(val) => Ok(Object::String(val)),
        },
        Expression::Conditional {
            condition,
            then_branch,
            elif_branch,
            else_branch,
        } => eval_conditional_expression(*condition, then_branch, elif_branch, else_branch, env),
        _ => Ok(NULL),
    }
}

fn eval_expressions(exprs: Vec<Expression>, env: &Env) -> Result<Vec<Object>, EvaluationError> {
    let mut result = vec![];

    for expr in exprs {
        let evalauated = eval_expression(expr, env)?;
        result.push(evalauated);
    }
    Ok(result)
}

fn extend_function_env(func: &Function, args: Vec<Object>) -> Env {
    let env = Environment::new_enclosed_environment(&func.env);

    for (param_idx, param) in func.parameters.iter().enumerate() {
        env.borrow_mut()
            .set(param.value.clone(), args[param_idx].clone())
    }

    env
}

fn apply_function(function: Object, args: Vec<Object>) -> EvaluationResult {
    match function {
        Object::Function(func) => {
            let ext_env = extend_function_env(&func, args);
            let evaluated = eval_block_statement(func.body, &ext_env)?;
            unwrap_return_value(evaluated)
        }
        _ => Err(EvaluationError::new(format!(
            "Invalid Function: {}",
            function.object_type()
        ))),
    }
}

fn unwrap_return_value(obj: Object) -> EvaluationResult {
    if let Object::ReturnValue(val) = obj {
        Ok(*val)
    } else {
        Ok(obj)
    }
}

fn eval_identifier(identifier: Identifier, env: &Env) -> EvaluationResult {
    match env.borrow().get(&identifier.value) {
        Some(value) => Ok(value),
        None => Err(EvaluationError::new(format!(
            "Identifier not found: {}",
            identifier.value,
        ))),
    }
}

fn eval_prefix_expression(operator: Token, right: Object) -> EvaluationResult {
    match operator {
        Token::NEGATE => Ok(eval_negation_expression(right)),
        Token::MINUS => Ok(eval_minus_prefix_expression(right))?,
        _ => Err(EvaluationError::new(format!(
            "invalid operator: {}{}",
            operator,
            right.object_type()
        ))),
    }
}

fn eval_conditional_expression(
    condition: Expression,
    consequence: Block,
    elif: Option<Block>,
    alternative: Option<Block>,
    env: &Env,
) -> EvaluationResult {
    let cond = eval_expression(condition, env)?;

    if is_truthy(cond) {
        eval_block_statement(consequence, env)
    } else {
        match alternative {
            Some(block) => eval_block_statement(block, env),
            _ => Ok(NULL),
        }
    }
}

fn eval_block_statement(block: Block, env: &Env) -> EvaluationResult {
    let mut result = Object::Null;

    for stmt in block.statements {
        let value = eval_statement(stmt, env)?;

        match value {
            Object::ReturnValue(_) => return Ok(value),
            _ => result = value,
        }
    }

    Ok(result)
}

fn eval_infix_expression(operator: Token, left: Object, right: Object) -> EvaluationResult {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix_expression(operator, l, r),
        (Object::Boolean(l), Object::Boolean(r)) => eval_boolean_infix_expression(operator, *l, *r),
        (Object::String(l), Object::String(r)) => eval_string_infix_expression(operator, l, r),
        _ => Err(EvaluationError::new(format!(
            "Infix evaluation type mismatch: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ))),
    }
}

fn eval_string_infix_expression(operator: Token, left: &str, right: &str) -> EvaluationResult {
    match operator {
        Token::PLUS => Ok(Object::String(format!("{}{}", left, right))),
        _ => Err(EvaluationError::new(format!(
            "Invalid Operator: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_integer_infix_expression(operator: Token, left: &i64, right: &i64) -> EvaluationResult {
    match operator {
        Token::PLUS => Ok(Object::Integer(left + right)),
        Token::MINUS => Ok(Object::Integer(left - right)),
        Token::MULTIPLY => Ok(Object::Integer(left * right)),
        Token::DIVIDE => Ok(Object::Integer(left / right)),
        Token::MODULO => Ok(Object::Integer(left % right)),
        Token::GT => Ok(Object::Boolean(left > right)),
        Token::GEQ => Ok(Object::Boolean(left >= right)),
        Token::LT => Ok(Object::Boolean(left < right)),
        Token::LEQ => Ok(Object::Boolean(left <= right)),
        Token::EQ => Ok(Object::Boolean(left == right)),
        Token::NEQ => Ok(Object::Boolean(left != right)),
        _ => Err(EvaluationError::new(format!(
            "Invalid Operator: {} {} {}",
            left, operator, right
        ))),
    }
}

fn eval_boolean_infix_expression(operator: Token, left: bool, right: bool) -> EvaluationResult {
    match operator {
        Token::EQ => Ok(Object::Boolean(left == right)),
        Token::NEQ => Ok(Object::Boolean(left != right)),
        Token::AND => Ok(Object::Boolean(left && right)),
        Token::OR => Ok(Object::Boolean(left || right)),
        _ => Err(EvaluationError::new(format!(
            "Invalid operator: {} {} {}",
            left, operator, right
        ))),
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
fn eval_minus_prefix_expression(right: Object) -> EvaluationResult {
    match right {
        Object::Integer(i) => Ok(Object::Integer(-i)),
        _ => Err(EvaluationError::new(format!(
            "Invalid operator: -{}",
            right.object_type()
        ))),
    }
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        NULL => false,
        TRUE => true,
        FALSE => false,
        _ => true,
    }
}
