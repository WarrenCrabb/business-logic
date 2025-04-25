use core::fmt;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Identifier(Identifier),
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
    Conditional {
        condition: Box<Expression>,
        then_branch: Block,
        elif_branch: Option<Block>,
        else_branch: Option<Block>,
    },
    FunctionLiteral {
        parameters: Vec<Identifier>,
        body: Block,
    },
    CallExpression {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    // Continue,
    // Break,
    // Loop {
    //     condition: Expression,
    //     body: Block,
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
            Expression::Conditional {
                condition,
                then_branch,
                elif_branch,
                else_branch,
            } => write!(
                f,
                "{:#?} {:#?} {:#?} {:#?}",
                condition, then_branch, elif_branch, else_branch
            ),
            Expression::FunctionLiteral { parameters, body } => {
                write!(f, "{:#?} {:#?}", parameters, body)
            }
            Expression::CallExpression {
                function,
                arguments,
            } => {
                let args: Vec<String> = arguments
                    .clone()
                    .into_iter()
                    .map(|param| format!("{:#?}", param))
                    .collect();

                let args = args.join(", ");

                write!(f, "{:?} ({})", function, args)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Identifier {
        Identifier { value }
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    VariableDeclaration {
        identifier: Identifier,
        value: Expression,
    },
    ReturnStatement(Expression),
    ExpressionStatement(Expression),
    EndLine,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VariableDeclaration { identifier, value } => {
                write!(f, "let {} = {}", identifier, value)
            }
            Statement::ReturnStatement(e) => write!(f, "return {}", e),
            Statement::ExpressionStatement(e) => write!(f, "{}", e),
            Statement::EndLine => writeln!(f),
        }
    }
}

#[derive(Debug, Default)]
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
        Token::MULTIPLY | Token::DIVIDE | Token::MODULO => Precedence::MultDiv,
        Token::CALL => Precedence::Call,
        _ => Precedence::Lowest,
    }
}
