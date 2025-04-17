use crate::token::MAX_OP_LEN;
use crate::token::MULTIWORD_OPERATORS;
use crate::token::Token;

#[cfg(test)]
pub mod tests;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        // if let Some(op) = self.read_multiword_operator() {
        //     return Token::OPERATOR(op);
        // }

        let token = match self.ch {
            ';' => Token::END,
            '+' => Token::PLUS,
            '-' => Token::MINUS,
            '*' => Token::MULTIPLY,
            '/' => Token::DIVIDE,
            '%' => Token::MODULO,
            '(' => Token::LPAREN,
            ')' => Token::RPAREN,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NEQ
                } else {
                    Token::NEGATE
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::AND
                } else {
                    Token::ILLEGAL
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    Token::OR
                } else {
                    Token::ILLEGAL
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LEQ
                } else {
                    Token::LT
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GEQ
                } else {
                    Token::GT
                }
            }
            'a'..='z' | 'A'..='Z' => return self.read_identifier(),
            '0'..='9' => return self.read_number(),
            '"' => return self.read_string(),
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };

        self.read_char();
        token
    }

    // pub fn next_token(&mut self) -> Token {
    //     self.skip_whitespace();

    //     // if let Some(op) = self.read_multiword_operator() {
    //     //     return Token::OPERATOR(op);
    //     // }

    //     let token = match self.ch {
    //         '+' => Token::OPERATOR(Operator::PLUS),
    //         '-' => Token::OPERATOR(Operator::MINUS),
    //         '*' => Token::OPERATOR(Operator::MULTIPLY),
    //         '/' => Token::OPERATOR(Operator::DIVIDE),
    //         '%' => Token::OPERATOR(Operator::MODULO),
    //         '=' => Token::ASSIGN,
    //         '!' => {
    //             if self.peek_char() == '=' {
    //                 self.read_char();
    //                 Token::OPERATOR(Operator::NEQ)
    //             } else {
    //                 Token::OPERATOR(Operator::NEGATE)
    //             }
    //         }
    //         '&' => {
    //             if self.peek_char() == '&' {
    //                 self.read_char();
    //                 Token::OPERATOR(Operator::AND)
    //             } else {
    //                 Token::ILLEGAL
    //             }
    //         }
    //         '|' => {
    //             if self.peek_char() == '|' {
    //                 self.read_char();
    //                 Token::OPERATOR(Operator::OR)
    //             } else {
    //                 Token::ILLEGAL
    //             }
    //         }
    //         '<' => {
    //             if self.peek_char() == '=' {
    //                 self.read_char();
    //                 Token::OPERATOR(Operator::LEQ)
    //             } else {
    //                 Token::OPERATOR(Operator::LT)
    //             }
    //         }
    //         '>' => {
    //             if self.peek_char() == '=' {
    //                 self.read_char();
    //                 Token::OPERATOR(Operator::GEQ)
    //             } else {
    //                 Token::OPERATOR(Operator::GT)
    //             }
    //         }
    //         'a'..='z' | 'A'..='Z' => return self.read_identifier(),
    //         '0'..='9' => return self.read_number(),
    //         '"' => return self.read_string(),
    //         '\0' => Token::EOF,
    //         _ => Token::ILLEGAL,
    //     };

    //     self.read_char();
    //     token
    // }

    fn read_multiword_operator(&mut self) -> Option<Token> {
        let max_op_len = *MAX_OP_LEN;

        if (self.position + max_op_len) > self.input.len() {
            return None;
        }

        let lookahead: String = self.input[self.position..]
            .iter()
            .take(max_op_len)
            .collect();

        for op in MULTIWORD_OPERATORS.iter() {
            if lookahead.starts_with(op) {
                // Advance lexer position by length of operator
                for _ in 0..op.len() {
                    self.read_char();
                }

                return Token::from_str(op);

                // return Some(Token::Operator(String::from(op)));
            }
        }
        None
    }

    pub fn read_number(&mut self) -> Token {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }

        Token::NUMBER(
            self.input[position..self.position]
                .iter()
                .collect::<String>()
                .parse::<i64>()
                .expect("Failed to parse number"),
        )
    }

    pub fn read_identifier(&mut self) -> Token {
        if let Some(op) = self.read_multiword_operator() {
            return op; // Token::OPERATOR(op);
        }

        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }

        Token::lookup_keyword(
            self.input[position..self.position]
                .iter()
                .collect::<String>()
                .to_string(),
        )
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\r' || self.ch == '\n' {
            self.read_char();
        }
    }

    pub fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    pub fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_' || ch == '-'
    }

    pub fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    pub fn read_string(&mut self) -> Token {
        self.read_char(); // skip opening quote

        let start = self.position;

        while self.peek_char() != '"' && self.ch != '\0' {
            self.read_char();
        }

        if self.ch != '"' {
            self.read_char();
        }

        let literal = self.input[start..self.position].iter().collect::<String>();

        self.read_char(); // skip closing quote

        Token::STRING(String::from(literal))
    }
}
