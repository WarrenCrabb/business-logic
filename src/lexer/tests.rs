use crate::{lexer::Lexer, token::Token};

#[test]
fn test_next_token() {
    let input = "let x = 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::ASSIGN,
        Token::NUMBER(5),
        Token::EOF,
    ];
    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_operator_tokens() {
    let input = "initiate x = 5 + 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::ASSIGN,
        Token::NUMBER(5),
        Token::PLUS,
        Token::NUMBER(5),
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_multiword_operator_tokens() {
    let input = "initiate x = 5 greater than 5";

    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("x")),
        Token::ASSIGN,
        Token::NUMBER(5),
        Token::GT,
        Token::NUMBER(5),
        Token::EOF,
    ];

    let mut lexer = Lexer::new(input);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_more_tokens() {
    let source = r#"
strategize main with no deliverables
	actualize synergyScore align 100
	touch base with "Quarterly optimization"

	optimize while momentum below synergyScore
		touch base with "Driving momentum..."
		momentum align momentum value-add 10
	end
end

execute
"#;

    let tests = vec![
        Token::BLANK,
        Token::FUNCTION,
        Token::IDENTIFIER(String::from("main")),
        Token::KEYWORD(String::from("with")),
        Token::KEYWORD(String::from("no")),
        Token::KEYWORD(String::from("deliverables")),
        Token::EOL,
        Token::INDENT,
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::ASSIGN,
        Token::NUMBER(100),
        Token::EOL,
        Token::STDOUT,
        Token::KEYWORD(String::from("with")),
        Token::STRING(String::from("Quarterly optimization")),
        Token::EOL,
        Token::BLANK,
        Token::KEYWORD(String::from("optimize")),
        Token::KEYWORD(String::from("while")),
        Token::IDENTIFIER(String::from("momentum")),
        Token::LT,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::EOL,
        Token::INDENT,
        Token::STDOUT,
        Token::KEYWORD(String::from("with")),
        Token::STRING(String::from("Driving momentum...")),
        Token::EOL,
        Token::IDENTIFIER(String::from("momentum")),
        Token::ASSIGN,
        Token::IDENTIFIER(String::from("momentum")),
        Token::PLUS,
        Token::NUMBER(10),
        Token::EOL,
        Token::DEDENT,
        Token::END,
        Token::EOL,
        Token::DEDENT,
        Token::END,
        Token::EOL,
        Token::BLANK,
        Token::KEYWORD(String::from("execute")),
        Token::EOL,
        Token::EOF,
    ];

    let mut lexer = Lexer::new(source);

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_assignment_and_operators() {
    let input = "actualize synergyScore align 100 value-add 10";
    let mut lexer = Lexer::new(input);
    let tests = vec![
        Token::DECLARATION,
        Token::IDENTIFIER(String::from("synergyScore")),
        Token::ASSIGN,
        Token::NUMBER(100),
        Token::PLUS,
        Token::NUMBER(10),
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}

#[test]
fn test_conditional_branches() {
    let input = r#"
evaluate x > 5
	false
re-evaluate x above 3
	false
pivot
	true 
"#;
    let mut lexer = Lexer::new(input);
    let tests = vec![
        Token::BLANK,
        Token::IF,
        Token::IDENTIFIER(String::from("x")),
        Token::GT,
        Token::NUMBER(5),
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(false),
        Token::EOL,
        Token::DEDENT,
        Token::ELIF,
        Token::IDENTIFIER(String::from("x")),
        Token::GT,
        Token::NUMBER(3),
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(false),
        Token::EOL,
        Token::DEDENT,
        Token::ELSE,
        Token::EOL,
        Token::INDENT,
        Token::BOOLEAN(true),
        Token::EOL,
        Token::DEDENT,
        Token::EOF,
    ];

    for expected in tests {
        let tok = lexer.next_token();
        assert_eq!(tok, expected, "Expected: {:?}, got: {:?}", expected, tok);
    }
}
