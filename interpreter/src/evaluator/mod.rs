use std::rc::Rc;

use crate::{
    object::{
        FALSE, Function, NULL, Object, TRUE,
        environment::{Env, Environment},
    },
    parser::ast::{Block, Expression, Identifier, Literal, Program, Statement},
    token::Token,
};
use builtin::Builtin;
use error::EvaluationError;

pub mod builtin;
mod error;

#[cfg(test)]
mod tests;

#[derive(Default)]
pub struct Evaluator {
    pub output_buffer: String,
    env: Env,
}

type EvaluationResult = Result<Object, EvaluationError>;

impl Evaluator {
    pub fn flush_output_buffer(&mut self) {
        self.output_buffer = String::new()
    }

    pub fn eval(&mut self, program: Program, env: &Env) -> EvaluationResult {
        let mut result = Object::Null;
        for statement in program.body {
            let value = self.eval_statement(statement, env)?;

            match value {
                Object::ReturnValue(value) => return Ok(*value),
                _ => result = value,
            }
        }

        Ok(result)
    }
    pub fn eval_program(&mut self, program: Program) -> EvaluationResult {
        let mut result = Object::Null;
        for statement in program.body {
            let value = self.eval_statement(statement, &Rc::clone(&self.env))?;

            match value {
                Object::ReturnValue(value) => return Ok(*value),
                _ => result = value,
            }
        }

        Ok(result)
    }

    fn eval_statement(&mut self, statement: Statement, env: &Env) -> EvaluationResult {
        let result = match statement {
            Statement::ExpressionStatement(expr) => self.eval_expression(expr, env),
            Statement::ReturnStatement(return_stmt) => {
                let value = self.eval_expression(return_stmt, env)?;
                Ok(Object::ReturnValue(Box::new(value)))
            }
            Statement::VariableDeclaration { identifier, value } => {
                let value = self.eval_expression(value, env)?;
                env.borrow_mut().set(identifier.value, value);
                Ok(Object::Null)
            }
            Statement::EndLine => Ok(Object::Null),
        };

        result
    }

    fn eval_expression(&mut self, expr: Expression, env: &Env) -> EvaluationResult {
        match expr {
            Expression::FunctionLiteral { parameters, body } => Ok(Object::Function(
                Function::new(parameters, body, Rc::clone(env)),
            )),
            Expression::CallExpression {
                function,
                arguments,
            } => {
                let func = self.eval_expression(*function, env)?;
                let args = self.eval_expressions(arguments, env)?;
                self.apply_function(func, args)
            }
            Expression::Identifier(ident) => self.eval_identifier(ident, env),
            Expression::Unary { operator, right } => {
                let right = self.eval_expression(*right, env)?;
                self.eval_prefix_expression(operator, right)
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.eval_expression(*left, env)?;
                let right = self.eval_expression(*right, env)?;
                self.eval_infix_expression(operator, left, right)
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
            } => self.eval_conditional_expression(
                *condition,
                then_branch,
                elif_branch,
                else_branch,
                env,
            ),
        }
    }

    fn eval_expressions(
        &mut self,
        exprs: Vec<Expression>,
        env: &Env,
    ) -> Result<Vec<Object>, EvaluationError> {
        let mut result = vec![];

        for expr in exprs {
            let evalauated = self.eval_expression(expr, env)?;
            result.push(evalauated);
        }
        Ok(result)
    }

    fn extend_function_env(&self, func: &Function, args: Vec<Object>) -> Env {
        let env = Environment::new_enclosed_environment(&func.env);

        for (param_idx, param) in func.parameters.iter().enumerate() {
            env.borrow_mut()
                .set(param.value.clone(), args[param_idx].clone())
        }

        env
    }

    fn apply_function(&mut self, function: Object, args: Vec<Object>) -> EvaluationResult {
        match function {
            Object::Function(func) => {
                let ext_env = self.extend_function_env(&func, args);
                let evaluated = self.eval_block_statement(func.body, &ext_env)?;
                self.unwrap_return_value(evaluated)
            }
            Object::Builtin(builtin) => builtin.apply_func(args, &mut self.output_buffer),
            _ => Err(EvaluationError::new(format!(
                "Invalid Function: {}",
                function.object_type()
            ))),
        }
    }

    fn unwrap_return_value(&self, obj: Object) -> EvaluationResult {
        if let Object::ReturnValue(val) = obj {
            Ok(*val)
        } else {
            Ok(obj)
        }
    }

    fn eval_identifier(&self, identifier: Identifier, env: &Env) -> EvaluationResult {
        if let Some(identifier) = env.borrow().get(&identifier.value) {
            return Ok(identifier);
        };

        match Builtin::lookup(&identifier.value) {
            Some(builtin) => Ok(builtin),
            None => Err(EvaluationError::new(format!(
                "Identifier not found: {}",
                identifier.value,
            ))),
        }
    }

    fn eval_prefix_expression(&self, operator: Token, right: Object) -> EvaluationResult {
        match operator {
            Token::NEGATE => Ok(self.eval_negation_expression(right)),
            Token::MINUS => Ok(self.eval_minus_prefix_expression(right))?,
            _ => Err(EvaluationError::new(format!(
                "invalid operator: {}{}",
                operator,
                right.object_type()
            ))),
        }
    }

    fn eval_conditional_expression(
        &mut self,
        condition: Expression,
        consequence: Block,
        _elif: Option<Block>,
        alternative: Option<Block>,
        env: &Env,
    ) -> EvaluationResult {
        let cond = self.eval_expression(condition, env)?;

        if self.is_truthy(cond) {
            self.eval_block_statement(consequence, env)
        } else {
            match alternative {
                Some(block) => self.eval_block_statement(block, env),
                _ => Ok(NULL),
            }
        }
    }

    fn eval_block_statement(&mut self, block: Block, env: &Env) -> EvaluationResult {
        let mut result = Object::Null;

        for stmt in block.statements {
            let value = self.eval_statement(stmt, env)?;

            match value {
                Object::ReturnValue(_) => return Ok(value),
                _ => result = value,
            }
        }

        Ok(result)
    }

    fn eval_infix_expression(
        &self,
        operator: Token,
        left: Object,
        right: Object,
    ) -> EvaluationResult {
        match (&left, &right) {
            (Object::Integer(l), Object::Integer(r)) => {
                self.eval_integer_infix_expression(operator, l, r)
            }
            (Object::Boolean(l), Object::Boolean(r)) => {
                self.eval_boolean_infix_expression(operator, *l, *r)
            }
            (Object::String(l), Object::String(r)) => {
                self.eval_string_infix_expression(operator, l, r)
            }
            _ => Err(EvaluationError::new(format!(
                "Infix evaluation type mismatch: {} {} {}",
                left.object_type(),
                operator,
                right.object_type()
            ))),
        }
    }

    fn eval_string_infix_expression(
        &self,
        operator: Token,
        left: &str,
        right: &str,
    ) -> EvaluationResult {
        match operator {
            Token::PLUS => Ok(Object::String(format!("{}{}", left, right))),
            _ => Err(EvaluationError::new(format!(
                "Invalid Operator: {} {} {}",
                left, operator, right
            ))),
        }
    }

    fn eval_integer_infix_expression(
        &self,
        operator: Token,
        left: &i64,
        right: &i64,
    ) -> EvaluationResult {
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

    fn eval_boolean_infix_expression(
        &self,
        operator: Token,
        left: bool,
        right: bool,
    ) -> EvaluationResult {
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

    fn eval_negation_expression(&self, right: Object) -> Object {
        match right {
            TRUE => FALSE,
            FALSE => TRUE,
            NULL => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_prefix_expression(&self, right: Object) -> EvaluationResult {
        match right {
            Object::Integer(i) => Ok(Object::Integer(-i)),
            _ => Err(EvaluationError::new(format!(
                "Invalid operator: -{}",
                right.object_type()
            ))),
        }
    }

    fn is_truthy(&self, obj: Object) -> bool {
        match obj {
            NULL => false,
            TRUE => true,
            FALSE => false,
            _ => true,
        }
    }
}
