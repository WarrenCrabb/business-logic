use core::fmt;
use std::fmt::write;

use crate::token::Token;

/// AST nodes for Business Logic language
// use crate::token::Operator;

#[derive(Debug, Clone)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable(String),
    Conditional {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        elif_branch: Option<Block>,
        else_branch: Option<Block>,
    },
    // Assignment {
    //     name: String,
    //     value: Expression,
    // },
    // Loop {
    //     condition: Expression,
    //     body: Block,
    // },
    // Continue,
    // Break,
    // Conditional {
    //     condition: Expression,
    //     then_branch: Block,
    //     else_branch: Option<Block>,
    // },
    // Print {
    //     value: Expression,
    // },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s),
            Expression::Literal(s) => write!(f, "{}", s),
            Expression::Binary {
                left,
                operator,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::Unary { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Variable(s) => write!(f, "{}", s),
            Expression::Conditional {
                condition,
                then_branch,
                elif_branch,
                else_branch,
            } => write!(f, "{}", "CONDITIONAL"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(i64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: Identifier,
        value: Expression,
    },
    // Assignment {
    //     name: String,
    //     value: Expression,
    // },
    // Loop {
    //     condition: Expression,
    //     body: Block,
    // },
    // Continue,
    // Break,
    // Conditional {
    //     condition: Expression,
    //     then_branch: Block,
    //     else_branch: Option<Block>,
    // },
    // Print {
    //     value: Expression,
    // },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VariableDeclaration { identifier, value } => {
                write!(f, "let {} = {}", identifier, value)
            }
            Statement::ReturnStatement(e) => write!(f, "return {}", e),
            Statement::ExpressionStatement(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.body {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest = 0,
    AndOr,
    Equals,
    LessGreater,
    AddSub,
    MultDiv,
    Prefix,
    Call,
    Index,
}

pub fn precedence_of(op: &Token) -> Precedence {
    match op {
        Token::AND | Token::OR => Precedence::AndOr,
        Token::EQ | Token::NEQ => Precedence::Equals,
        Token::LT | Token::LEQ | Token::GT | Token::GEQ => Precedence::LessGreater,
        Token::PLUS | Token::MINUS => Precedence::AddSub,
        Token::MULTIPLY | Token::DIVIDE => Precedence::MultDiv,
        _ => Precedence::Lowest,
    }
}

// pub fn precedence_of(op: &Operator) -> Precedence {
//     match op {
//         Operator::AND | Operator::OR => Precedence::AndOr,
//         Operator::EQ | Operator::NEQ => Precedence::Equals,
//         Operator::LT | Operator::LEQ | Operator::GT | Operator::GEQ => Precedence::LessGreater,
//         Operator::PLUS | Operator::MINUS => Precedence::AddSub,
//         Operator::MULTIPLY | Operator::DIVIDE => Precedence::MultDiv,
//         _ => Precedence::Lowest,
//     }
// }
