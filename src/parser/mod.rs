use std::{backtrace, sync::RwLock};

use crate::{
    lexer::Lexer,
    token::{OP_TOKENS, Token},
};
use ast::{Block, Expression, Identifier, Literal, Precedence, Program, Statement, precedence_of};

#[cfg(test)]
mod tests;

pub mod ast;

pub struct Parser {
    lexer: Lexer,
    current: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: vec![],
            current: Token::EOF,
            peek_token: Token::EOF,
        };

        parser.advance(); // Initialize current token
        parser.advance(); // Initialize peek token
        parser
    }

    fn advance(&mut self) -> &Token {
        self.current = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
        &self.current
    }

    // fn current_indent(&self) {
    // self.lexer.current_ind()
    // }

    pub fn parse(&mut self) -> Result<Program, Vec<String>> {
        let mut body = vec![];
        while !matches!(self.current, Token::EOF) {
            if self.current != Token::EOL {
                body.push(self.parse_statement());
            }
            self.advance();
        }

        if self.errors.is_empty() {
            Ok(Program { body })
        } else {
            Err(self.errors.clone())
        }
    }

    fn parse_statement(&mut self) -> Statement {
        match &self.current {
            Token::DECLARATION => self.parse_variable_declaration(),
            Token::RETURN => self.parse_return_statement(),
            Token::BLANK | Token::EOL => Statement::EndLine,
            _ => self.parse_expression_statement(),
            // _ => {
            //     self.errors
            //         .push(format!("Unexpected token: {:?}", self.current));
            //     Statement::Print {
            //         value: Expression::Literal(Literal::String(String::from("error"))),
            //     }
            // }
        }
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expression = self.parse_expression(Precedence::Lowest);

        if Token::is_end_token(&self.peek_token) {
            self.advance();
        }

        Statement::ExpressionStatement(expression)
    }

    fn parse_unary_expression(&mut self) -> Expression {
        let operator = self.expect_operator();

        self.advance();
        let right = self.parse_expression(Precedence::Prefix);

        Expression::Unary {
            operator,
            right: Box::new(right),
        }
    }

    fn parse_binary_expression(&mut self, left: Expression) -> Expression {
        let operator = self.expect_operator();
        let prec = self.current_precedence();

        self.advance();

        let right = self.parse_expression(prec);

        Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_of(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        precedence_of(&self.current)
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.advance();
        let e = self.parse_expression(Precedence::Lowest);
        Statement::ReturnStatement(e)
    }

    fn expect_identifier(&mut self) -> Identifier {
        if let Token::IDENTIFIER(name) = &self.current {
            Identifier {
                value: name.clone(),
            }
        } else {
            self.errors
                .push(format!("Expected identifier but got {:?}", self.current));
            Identifier { value: "".into() }
        }
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        self.advance(); // actualize

        // self.expect_peek(Token::IDENTIFIER("synergyScore".into()));
        let name = self.expect_identifier();

        self.expect_peek(Token::ASSIGN);

        self.advance();

        let value = self.parse_expression(Precedence::Lowest);
        Statement::VariableDeclaration {
            identifier: name,
            value,
        }
    }

    fn parse_expression(&mut self, precidence: Precedence) -> Expression {
        let mut left_exp = match &self.current {
            Token::MINUS | Token::NEGATE => self.parse_unary_expression(),
            Token::IDENTIFIER(i) => Expression::Identifier(String::from(i)),
            Token::NUMBER(n) => Expression::Literal(Literal::Number(*n)),
            Token::STRING(s) => Expression::Literal(Literal::String(String::from(s))),
            Token::BOOLEAN(b) => Expression::Literal(Literal::Boolean(*b)),
            Token::LPAREN => self.parse_grouped_expresssion(),
            Token::IF => self.parse_if_expression(),
            _ => panic!("Unexpected expression token {}", &self.current),
        };

        if Token::is_end_token(&self.peek_token) || precidence >= self.peek_precedence() {
            return left_exp;
        }

        while !Token::is_end_token(&self.current) && precidence < self.peek_precedence() {
            self.advance();

            left_exp = self.parse_binary_expression(left_exp);
        }

        left_exp

        // self.advance();
        // return Expression::Literal(Literal::Number(100));
    }

    fn parse_grouped_expresssion(&mut self) -> Expression {
        self.advance();
        let exp = self.parse_expression(Precedence::Lowest);
        self.expect_peek(Token::RPAREN);
        exp
    }

    fn parse_if_expression(&mut self) -> Expression {
        self.advance();

        let expr = self.parse_expression(Precedence::Lowest);

        self.expect_peek(Token::EOL);
        self.expect_peek(Token::INDENT);

        let block = self.parse_block_statement();

        let alt = if self.peek_token == Token::ELSE {
            self.advance(); // skip DEDENT
            self.advance(); // skip EOL

            self.expect_peek(Token::INDENT);

            Some(self.parse_block_statement())
        } else {
            None
        };

        self.expect_peek(Token::END);

        Expression::Conditional {
            condition: Box::new(expr),
            then_branch: block,
            elif_branch: None,
            else_branch: alt,
        }
    }

    fn parse_block_statement(&mut self) -> Block {
        let mut statements = vec![];

        self.advance();

        // self.expect_peek(Token::INDENT);

        while self.current != Token::DEDENT && self.current != Token::EOF {
            let stmt = self.parse_statement();

            statements.push(stmt);

            self.advance();
        }
        // self.advance();

        // if self.current == Token::EOL {
        //     self.advance();
        // }
        Block { statements }
    }

    fn expect_operator(&mut self) -> Token {
        if OP_TOKENS.contains(&self.current) {
            self.current.clone()
        } else {
            self.errors
                .push(format!("Expected Operator but got {:?}", self.current));
            panic!("Expected Operator but got {:?}", self.current);
            // Operator
        }
        // match &self.peek_token {
        //     Token::OPERATOR(op) => Some(op.clone()),
        //     _ => None,
        // }
    }

    // fn expect_operator(&mut self) -> Operator {
    //     if let Token::OPERATOR(op) = &self.current {
    //         op.clone()
    //     } else {
    //         self.errors
    //             .push(format!("Expected Operator but got {:?}", self.current));
    //         panic!("Expected Operator but got {:?}", self.current);
    //         // Operator
    //     }
    //     // match &self.peek_token {
    //     //     Token::OPERATOR(op) => Some(op.clone()),
    //     //     _ => None,
    //     // }
    // }

    fn expect_peek(&mut self, expected: Token) -> Option<Token> {
        if self.peek_token == expected {
            self.advance();
            Some(self.current.clone())
        } else {
            self.peek_error(expected);
            None
        }
    }

    fn peek_error(&mut self, expected: Token) {
        let msg = format!(
            "Expected token {:?}, but got {:?}",
            expected, self.peek_token
        );
        self.errors.push(msg);
    }
}
