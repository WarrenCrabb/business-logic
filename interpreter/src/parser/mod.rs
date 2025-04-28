use crate::{
    lexer::Lexer,
    token::{OP_TOKENS, Token},
};
use ast::{Block, Expression, Identifier, Literal, Precedence, Program, Statement, precedence_of};
use error::ParserError;

#[cfg(test)]
mod tests;

pub mod ast;
mod error;

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new_parser(input: String) -> Parser {
        let lexer = Lexer::new(&input);

        let mut parser = Parser {
            lexer,
            errors: vec![],
            current: Token::EOF,
            peek_token: Token::EOF,
        };

        parser.advance();
        parser.advance();
        parser
    }
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: vec![],
            current: Token::EOF,
            peek_token: Token::EOF,
        };

        parser.advance();
        parser.advance();
        parser
    }

    fn advance(&mut self) -> &Token {
        self.current = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        &self.current
    }

    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut body = vec![];
        while !matches!(self.current, Token::EOF) {
            if self.current != Token::EOL && self.current != Token::BLANK {
                body.push(self.parse_statement()?);
            }
            self.advance();
        }

        Ok(Program { body })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match &self.current {
            Token::DECLARATION => self.parse_variable_declaration(),
            Token::RETURN => self.parse_return_statement(),
            Token::BLANK | Token::EOL => Ok(Statement::EndLine),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expression = self.parse_expression(Precedence::Lowest)?;

        if Token::is_end_token(&self.peek_token) {
            self.advance();
        }

        Ok(Statement::ExpressionStatement(expression))
    }

    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let operator = self.expect_operator();

        self.advance();
        let right = self.parse_expression(Precedence::Prefix)?;

        Ok(Expression::Unary {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_binary_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let operator = self.expect_operator();
        let prec = self.current_precedence();

        self.advance();

        let right = self.parse_expression(prec)?;

        Ok(Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_of(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        precedence_of(&self.current)
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.advance();
        let e = self.parse_expression(Precedence::Lowest)?;

        Ok(Statement::ReturnStatement(e))
    }

    fn expect_identifier(&mut self) -> Result<Identifier, ParserError> {
        if let Token::IDENTIFIER(name) = &self.current {
            Ok(Identifier {
                value: name.clone(),
            })
        } else {
            Err(ParserError::new(String::from("Expected Identifier")))
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParserError> {
        self.advance();
        let identifier = self.expect_identifier()?;
        self.expect_peek(Token::BIND)?;
        self.advance();

        let value = self.parse_expression(Precedence::Lowest)?;

        Ok(Statement::VariableDeclaration { identifier, value })
    }

    fn parse_expression(&mut self, precidence: Precedence) -> Result<Expression, ParserError> {
        let mut left_expr = match &self.current {
            Token::MINUS | Token::NEGATE => self.parse_unary_expression()?,
            Token::IDENTIFIER(i) => Expression::Identifier(Identifier::new(i.to_string())),
            Token::NUMBER(n) => Expression::Literal(Literal::Number(*n)),
            Token::STRING(s) => Expression::Literal(Literal::String(String::from(s))),
            Token::BOOLEAN(b) => Expression::Literal(Literal::Boolean(*b)),
            Token::LPAREN => self.parse_grouped_expresssion()?,
            Token::IF => self.parse_if_expression()?,
            Token::FUNCTION => self.parse_function_literal()?,
            _ => {
                return Err(ParserError::new(format!(
                    "No prefix parse function for '{}' found",
                    self.current
                )));
            }
        };

        if Token::is_end_token(&self.peek_token) || precidence >= self.peek_precedence() {
            return Ok(left_expr);
        }

        while !Token::is_end_token(&self.current) && precidence < self.peek_precedence() {
            self.advance();
            if Token::is_operator(&self.current) {
                left_expr = self.parse_binary_expression(left_expr)?;
            } else if self.current == Token::CALL {
                self.advance();
                left_expr = self.parse_call_expression(left_expr)?;
            }
        }

        Ok(left_expr)
    }

    fn parse_call_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        let arguments = self.parse_call_arguments()?;

        Ok(Expression::CallExpression {
            function: Box::new(left),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, ParserError> {
        let mut args = vec![];

        while !Token::is_end_token(&self.current) {
            args.push(self.parse_expression(Precedence::Lowest)?);
            self.advance();
        }

        self.advance();

        Ok(args)
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParserError> {
        self.advance();

        let parameters = self.parse_function_parameters()?;

        self.advance();

        if self.current == Token::RETURN {
            let e = self.parse_return_statement()?;

            let body = Block {
                statements: vec![e],
            };

            return Ok(Expression::FunctionLiteral { parameters, body });
        }

        self.expect_peek(Token::INDENT)?;

        let body = self.parse_block_statement()?;

        Ok(Expression::FunctionLiteral { parameters, body })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>, ParserError> {
        let mut identifiers = vec![];

        while self.current != Token::BIND {
            let i = self.expect_identifier()?;
            identifiers.push(i);
            self.advance();
        }

        Ok(identifiers)
    }

    fn parse_grouped_expresssion(&mut self) -> Result<Expression, ParserError> {
        self.advance();
        let exp = self.parse_expression(Precedence::Lowest);
        self.expect_peek(Token::RPAREN)?;
        exp
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        self.advance();

        let expr = self.parse_expression(Precedence::Lowest)?;

        self.expect_peek(Token::EOL)?;

        let block = if self.peek_token != Token::END {
            self.expect_peek(Token::INDENT)?;
            self.parse_block_statement()?
        } else {
            Block { statements: vec![] }
        };

        let alt = if self.peek_token == Token::ELSE {
            self.advance(); // skip DEDENT
            self.advance(); // skip EOL

            self.expect_peek(Token::INDENT)?;

            Some(self.parse_block_statement()?)
        } else {
            None
        };

        self.expect_peek(Token::END)?;

        Ok(Expression::Conditional {
            condition: Box::new(expr),
            then_branch: block,
            elif_branch: None,
            else_branch: alt,
        })
    }

    fn parse_block_statement(&mut self) -> Result<Block, ParserError> {
        let mut statements = vec![];

        self.advance();

        while self.current != Token::DEDENT && self.current != Token::EOF {
            let stmt = self.parse_statement()?;

            statements.push(stmt);

            self.advance();
        }

        Ok(Block { statements })
    }

    fn expect_operator(&mut self) -> Token {
        if OP_TOKENS.contains(&self.current) {
            self.current.clone()
        } else {
            self.errors
                .push(format!("Expected Operator but got {:?}", self.current));
            panic!("Expected Operator but got {:?}", self.current);
        }
    }

    fn expect_peek(&mut self, expected: Token) -> Result<(), ParserError> {
        if self.peek_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(ParserError::new(format!(
                "expected next token to be '{}', but got '{}'",
                expected, self.peek_token
            )))
        }
    }
}
