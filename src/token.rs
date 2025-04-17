use std::collections::HashSet;
use std::fmt;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENTIFIER(String),
    NUMBER(i64),
    STRING(String),
    BOOLEAN(bool),

    // OPERATOR(Operator),
    KEYWORD(String),
    ASSIGN,
    DECLARATION,
    RETURN,
    FUNCTION,
    NULL,

    END,

    IF,
    ELIF,
    ELSE,
    SEMICOLON,

    EOL,

    LPAREN,
    RPAREN,

    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    NEGATE,
    AND,
    OR,
    LT,
    GT,
    EQ,
    NEQ,
    LEQ,
    GEQ,
    MODULO,

    STDOUT,

    INDENT,
    DEDENT,

    BLANK,
}

pub static OP_TOKENS: LazyLock<Vec<Token>> = LazyLock::new(|| {
    vec![
        Token::PLUS,
        Token::MINUS,
        Token::MULTIPLY,
        Token::DIVIDE,
        Token::NEGATE,
        Token::AND,
        Token::OR,
        Token::LT,
        Token::GT,
        Token::EQ,
        Token::NEQ,
        Token::LEQ,
        Token::GEQ,
        Token::MODULO,
    ]
});

pub static END_TOKENS: LazyLock<Vec<Token>> =
    LazyLock::new(|| vec![Token::EOF, Token::END, Token::EOL]);

impl Token {
    pub fn from_str(s: &str) -> Option<Token> {
        match s {
            "+" => Some(Token::PLUS),
            "-" => Some(Token::MINUS),
            "*" => Some(Token::MULTIPLY),
            "/" => Some(Token::DIVIDE),
            "!" => Some(Token::NEGATE),
            "&&" => Some(Token::AND),
            "||" => Some(Token::OR),
            "<" => Some(Token::LT),
            ">" => Some(Token::GT),
            // "=" => Some(Token::EQ),
            "!=" => Some(Token::NEQ),
            "<=" => Some(Token::LEQ),
            ">=" => Some(Token::GEQ),
            "%" => Some(Token::MODULO),
            "greater than" => Some(Token::GT),
            "less than" => Some(Token::LT),
            "equals" => Some(Token::EQ),
            "not equal" => Some(Token::NEQ),
            "less than or equal to" => Some(Token::LEQ),
            "greater than or equal to" => Some(Token::GEQ),
            "value-add" => Some(Token::PLUS),
            "add" => Some(Token::PLUS),
            "increase" => Some(Token::PLUS),
            "plus" => Some(Token::PLUS),
            "reduce" => Some(Token::MINUS),
            "streamline" => Some(Token::MINUS),
            "subtract" => Some(Token::MINUS),
            "cut" => Some(Token::MINUS),
            "multiply" => Some(Token::MULTIPLY),
            "amplify" => Some(Token::MULTIPLY),
            "boost" => Some(Token::MULTIPLY),
            "divide" => Some(Token::DIVIDE),
            "disrupt" => Some(Token::DIVIDE),
            "remainder" => Some(Token::MODULO),
            "modulo" => Some(Token::MODULO),
            "touch base" => Some(Token::STDOUT),
            _ => None,
        }
    }

    pub fn is_operator(t: &Token) -> bool {
        OP_TOKENS.contains(t)
    }
    pub fn is_end_token(t: &Token) -> bool {
        END_TOKENS.contains(t)
    }
}

pub static MULTIWORD_OPERATORS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        "touch base",
        "greater than",
        "less than",
        "value-add",  // add
        "add",        // add
        "increase",   // add
        "plus",       // add
        "reduce",     // subtract
        "streamline", // subtract
        "subtract",   // subtract
        "cut",        // subtract
        "multiply",   // multiply
        "amplify",    // multiply
        "boost",      // multiply
        "divide",     // divide
        "disrupt",    // divide
        "remainder",  // modulo
        "modulo",     // modulo
        "surplus",    // modulo
    ]
    .into_iter()
    .collect()
});

pub static MAX_OP_LEN: LazyLock<usize> = LazyLock::new(|| {
    MULTIWORD_OPERATORS
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0)
});

// Static keyword set initialized once
pub static KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    [
        // "actualize",
        // "align",
        // "strategize",
        "with",
        "no",
        "deliverables",
        "and",
        "end",
        "pivot",
        "else",
        "optimize",
        "while",
        "touch",
        "base",
        "synergize",
        "exit",
        "strategy",
        "escalate",
        "execute",
        "equals",
        "circle",
        "back",
        "punt",
        "actionable",
        "not",
        "hard",
        "stop",
        "backburner",
        "headwinds",
        "low-hanging",
        "fruit,",
        "resonate",
        "traction",
        "results-driven",
        "downsizing",
        "stopgap",
        "is",
        "unrealized", // null
        "let",
    ]
    .into_iter()
    .collect()
});

impl Token {
    pub fn lookup_keyword(ident: String) -> Token {
        match ident.as_str() {
            "let" | "actualize" | "initiate" => Token::DECLARATION,
            "return" | "synergize" => Token::RETURN,
            "strategize" => Token::FUNCTION,
            "unrealized" => Token::NULL,
            "align" => Token::ASSIGN,
            "evaluate" => Token::IF,
            "re-evaluate" => Token::ELIF,
            "pivot" => Token::ELSE,

            "below" => Token::LT,
            "above" | "excelling" => Token::GT,
            "is" => Token::EQ,
            "achieving" => Token::GEQ,
            "add" | "value-add" | "increase" | "plus" => Token::PLUS,
            "reduce" | "streamline" | "subtract" | "cut" => Token::MINUS,
            "multiply" | "amplify" | "boost" => Token::MULTIPLY,
            "divide" | "disrupt" => Token::DIVIDE,
            "modulo" | "remainder" => Token::MODULO,
            "end" => Token::END,

            // "below" => Token::OPERATOR(Token::LT),
            // "above" | "excelling" => Token::OPERATOR(Token::GT),
            // "is" => Token::OPERATOR(Token::EQ),
            // "achieving" => Token::OPERATOR(Token::GEQ),
            // "add" | "value-add" | "increase" | "plus" => Token::OPERATOR(Token::PLUS),
            // "reduce" | "streamline" | "subtract" | "cut" => Token::OPERATOR(Token::MINUS),
            // "multiply" | "amplify" | "boost" => Token::OPERATOR(Token::MULTIPLY),
            // "divide" | "disrupt" => Token::OPERATOR(Token::DIVIDE),
            // "modulo" | "remainder" => Token::OPERATOR(Token::MODULO),
            "true" | "actionable" => Token::BOOLEAN(true),
            "false" => Token::BOOLEAN(false),
            s if KEYWORDS.contains(s) => Token::KEYWORD(ident),
            _ => Token::IDENTIFIER(ident),
        }
    }
}

// impl fmt::Display for Operator {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Token::PLUS => write!(f, "PLUS"),
//             Token::MINUS => write!(f, "MINUS"),
//             Token::MULTIPLY => write!(f, "MULTIPLY"),
//             Token::DIVIDE => write!(f, "DIVIDE"),
//             Token::NEGATE => write!(f, "NEGATE"),
//             Token::AND => write!(f, "AND"),
//             Token::OR => write!(f, "OR"),
//             Token::LT => write!(f, "LT"),
//             Token::GT => write!(f, "GT"),
//             Token::EQ => write!(f, "EQ"),
//             Token::NEQ => write!(f, "NEQ"),
//             Token::LEQ => write!(f, "LEQ"),
//             Token::GEQ => write!(f, "GEQ"),
//             Token::MODULO => write!(f, "MODULO"),
//         }
//     }
// }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::EOF => write!(f, "EOF"),
            Token::IDENTIFIER(s) => write!(f, "IDENTIFIER({})", s),
            Token::NUMBER(n) => write!(f, "NUMBER({})", n),
            Token::STRING(s) => write!(f, "STRING({})", s),
            Token::BOOLEAN(b) => write!(f, "BOOLEAN({})", b),
            // Token::OPERATOR(o) => write!(f, "OPERATOR{}", o),
            Token::DECLARATION => write!(f, "DECLARTION"),

            Token::KEYWORD(k) => write!(f, "KEYWORD({})", k),
            Token::ASSIGN => write!(f, "="),
            Token::RETURN => write!(f, "return"),
            Token::FUNCTION => write!(f, "function"),
            Token::NULL => write!(f, "null"),
            Token::IF => write!(f, "if"),
            Token::ELIF => write!(f, "elif"),
            Token::ELSE => write!(f, "else"),
            Token::SEMICOLON => write!(f, ":"),
            Token::EOL => write!(f, "\\n"),
            // Token::OPERATOR(o) => write!(f, "OPERATOR({})", o),
            Token::PLUS => write!(f, "+"),
            Token::MINUS => write!(f, "-"),
            Token::NEGATE => write!(f, "!"),
            Token::MULTIPLY => write!(f, "*"),
            Token::DIVIDE => write!(f, "/"),
            Token::AND => write!(f, "&&"),
            Token::OR => write!(f, "||"),
            Token::LT => write!(f, "<"),
            Token::GT => write!(f, ">"),
            Token::EQ => write!(f, "=="),
            Token::NEQ => write!(f, "!="),
            Token::LEQ => write!(f, "<="),
            Token::GEQ => write!(f, ">="),
            Token::MODULO => write!(f, "%"),
            Token::END => write!(f, ";"),

            Token::LPAREN => write!(f, "("),
            Token::RPAREN => write!(f, ")"),

            Token::STDOUT => write!(f, "PRINT"),
            Token::INDENT => write!(f, "INDENT"),
            Token::DEDENT => write!(f, "DEDENT"),
            Token::BLANK => write!(f, "BLANK"),
        }
    }
}
