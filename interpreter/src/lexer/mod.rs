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
    line: i64,
    col: i64,
    indent: usize,
}

const TAB_WIDTH: usize = 2;

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.trim().chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            col: 0,
            indent: 0,
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn current_indent(&self) -> usize {
        self.indent
    }

    pub fn next_token(&mut self) -> Token {
        if let Some(token) = self.skip_blankline() {
            return token;
        }

        if let Some(token) = self.count_indents() {
            return token;
        }

        self.skip_whitespace();

        let token = match self.ch {
            '\n' => Token::EOL,
            // ';' => Token::END,
            // ';' => Token::SEMICOLON,
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
                    Token::BIND
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
            '"' => self.read_string(),
            'a'..='z' | 'A'..='Z' => return self.read_identifier(),
            '0'..='9' => return self.read_number(),
            '\0' => Token::EOF,
            _ => Token::ILLEGAL,
        };

        self.read_char();
        token
    }

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
            }
        }
        None
    }

    fn read_number(&mut self) -> Token {
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

    fn read_identifier(&mut self) -> Token {
        if let Some(op) = self.read_multiword_operator() {
            return op;
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

    // fn measure_visual_indent()

    fn count_indents(&mut self) -> Option<Token> {
        if self.col == 1 {
            let mut count_indent = 0;

            while self.ch == '\t' || self.ch == ' ' {
                count_indent += match self.ch {
                    ' ' => 1,
                    '\t' => TAB_WIDTH,
                    _ => 0,
                };
                // count_indent += 1;
                self.read_char();
            }

            if count_indent > self.indent {
                self.indent = count_indent;
                return Some(Token::INDENT);
            } else if count_indent < self.indent {
                self.indent = count_indent;
                return Some(Token::DEDENT);
            }
        }

        None
    }

    fn skip_blankline(&mut self) -> Option<Token> {
        if self.ch == '\n' && self.col == 1 {
            self.read_char();
            return Some(Token::BLANK);
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' {
            //|| self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_' || ch == '-'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_ascii_digit()
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // skip opening quote

        let start = self.position;

        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }

        let literal: String = self.input[start..self.position].iter().collect::<String>();

        Token::STRING(literal)
    }
}
