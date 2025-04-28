use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::LazyLock;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    ILLEGAL,
    EOF,

    IDENTIFIER(String),
    NUMBER(i64),
    STRING(String),
    BOOLEAN(bool),

    BIND,
    DECLARATION,
    RETURN,
    FUNCTION,
    CALL,
    WHILE,
    FOR,
    NULL,

    END,

    IF,
    ELIF,
    ELSE,
    COMMA,

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

    // STDOUT,
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
            "greater than" => Some(Token::GT),
            "less than" => Some(Token::LT),
            "not equal" => Some(Token::NEQ),
            "less than or equal to" => Some(Token::LEQ),
            "greater than or equal to" => Some(Token::GEQ),
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
    ["touch base", "greater than", "less than"]
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

pub static KEYWORD_MAP: LazyLock<HashMap<&'static str, Token>> = LazyLock::new(|| {
    HashMap::from([
        ("unrealized", Token::NULL),
        // declaration
        ("let", Token::DECLARATION),
        ("actualize", Token::DECLARATION),
        ("initiate", Token::DECLARATION),
        // return
        ("return", Token::RETURN),
        ("synergize", Token::RETURN),
        ("execute", Token::RETURN),
        ("deliver", Token::RETURN),
        // fn
        ("fn", Token::FUNCTION),
        ("plan", Token::FUNCTION),
        ("strategize", Token::FUNCTION),
        ("strat", Token::FUNCTION),
        ("strategy", Token::FUNCTION),
        ("blueprint", Token::FUNCTION),
        // fn call
        ("leverage", Token::CALL),
        ("leveraging", Token::CALL),
        ("utilize", Token::CALL),
        ("engage", Token::CALL),
        ("activate", Token::CALL),
        ("invoke", Token::CALL),
        //loops
        ("while", Token::WHILE),
        ("until", Token::WHILE),
        ("for", Token::FOR),
        // bind / assign
        ("align", Token::BIND),
        ("with", Token::BIND),
        ("to", Token::BIND),
        ("as", Token::BIND),
        // conditional
        ("if", Token::IF),
        ("evaluate", Token::IF),
        ("elif", Token::ELIF),
        ("re_evaluate", Token::ELIF),
        ("pivot", Token::ELSE),
        ("else", Token::ELSE),
        // comparison
        ("is", Token::EQ),
        ("equals", Token::EQ),
        ("less", Token::LT),
        ("below", Token::LT),
        ("above", Token::GT),
        ("exceeds", Token::GT),
        ("achieving", Token::GEQ),
        ("productive", Token::GEQ),
        ("unproductive", Token::LEQ),
        ("ineffectual", Token::LEQ),
        // Add
        ("add", Token::PLUS),
        ("value_add", Token::PLUS),
        ("increase", Token::PLUS),
        ("plus", Token::PLUS),
        // Subtract
        ("reduce", Token::MINUS),
        ("streamline", Token::MINUS),
        ("subtract", Token::MINUS),
        ("cut", Token::MINUS),
        // Mult
        ("multiply", Token::MULTIPLY),
        ("amplify", Token::MULTIPLY),
        ("boost", Token::MULTIPLY),
        // Divide
        ("divide", Token::DIVIDE),
        ("disrupt", Token::DIVIDE),
        // Modulo
        ("modulo", Token::MODULO),
        ("remainder", Token::MODULO),
        ("surplus", Token::MODULO),
        // boolean
        ("not", Token::NEGATE),
        ("true", Token::BOOLEAN(true)),
        ("actionable", Token::BOOLEAN(true)),
        ("false", Token::BOOLEAN(false)),
        ("headwinds", Token::BOOLEAN(false)),
        ("empty", Token::STRING(String::from(""))),
        ("end", Token::END),
    ])
});
// Static keyword set initialized once
// pub static KEYWORDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
//     [
//         "unrealized",       // null
//         "migrate_to_cloud", // never / null
//         // truthy
//         "on_my_radar",
//         "actionable",
//         "stakeholders_pleased",
//         // falsy
//         "headwinds",
//         "deliverables", // start parameter list
//         "with",         // assignment
//         // conditional
//         "pivot", // otherwise / else
//         "else",
//         // loops
//         "optimize",
//         "while",
//         "until",
//         "close_the_loop",
//         "keep_in_the_loop",
//         "looping_in", // looping_in <variable> for in..?
//         // continue
//         "circle_back",
//         "take_offline",
//         // break
//         "punt",
//         "hard_stop",
//         // print  ?
//         "touch_base",
//         // return
//         "execute",
//         // throw
//         "escalate",
//         // equality
//         "is",
//         "equals",
//         // block end
//         "end",
//         // other
//         "no",
//         "and",
//         "touch",
//         "base",
//         "synergize",
//         "exit",
//         "strategy",
//         "circle",
//         "back",
//         "not",
//         "hard",
//         "stop",
//         "backburner",
//         "low-hanging",
//         "fruit,",
//         "resonate",
//         "traction",
//         "results-driven",
//         "downsizing",
//         "stopgap",
//         "let",
//     ]
//     .into_iter()
//     .collect()
// });

/*
[align|actualize] <variable> [with|to] <expression>
*/
impl Token {
    pub fn lookup_keyword(ident: String) -> Token {
        KEYWORD_MAP
            .get(ident.as_str())
            .cloned()
            .unwrap_or(Token::IDENTIFIER(ident))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::ILLEGAL => write!(f, "ILLEGAL"),
            Token::EOF => write!(f, "EOF"),
            Token::IDENTIFIER(s) => write!(f, "IDENTIFIER({})", s),
            Token::NUMBER(n) => write!(f, "NUMBER({})", n),
            Token::STRING(s) => write!(f, "STRING({})", s),
            Token::BOOLEAN(b) => write!(f, "BOOLEAN({})", b),
            Token::DECLARATION => write!(f, "DECLARTION"),

            Token::BIND => write!(f, "="),
            Token::COMMA => write!(f, ","),
            Token::RETURN => write!(f, "return"),
            Token::FUNCTION => write!(f, "function"),
            Token::CALL => write!(f, "call"),
            Token::NULL => write!(f, "null"),
            Token::IF => write!(f, "if"),
            Token::ELIF => write!(f, "elif"),
            Token::ELSE => write!(f, "else"),
            Token::EOL => write!(f, "\\n"),
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

            Token::INDENT => write!(f, "INDENT"),
            Token::DEDENT => write!(f, "DEDENT"),
            Token::BLANK => write!(f, "BLANK"),
            Token::WHILE => write!(f, "WHILE"),
            Token::FOR => write!(f, "FOR"),
        }
    }
}
